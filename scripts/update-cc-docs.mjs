#!/usr/bin/env node
// Claude Code 官方文档抓取与完整更新脚本（官网 reference.html 数据源）
// 用法：npm run sync:cc-docs [-- --proxy http://127.0.0.1:33210] [-- --incremental] [-- --concurrency 8]
// 行为：拉取 llms.txt 索引 → 解析章节分类 → 并发下载 zh-CN .md → 按 <顶级分类>/<子分类>/ 存放；
//       写 order.json（官方索引顺序的相对路径清单，无 .md 后缀），官网 reference.html 侧栏按它排序。
// 模式：默认完整更新——重下全部文档并清理官方已下线的旧文件与空目录；
//       --incremental 补充模式——只下载本地没有的文档，不删除任何现有文件，order.json 仍全量刷新。
// 仅本地更新 website/content/claude-code-docs/，不触发任何部署——上线走 npm run deploy:site。
// 例外：changelog 307 跳 GitHub 原始 CHANGELOG（官方无中文）；claude-tag 跳 claude.com（无中文，存英文）

import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { existsSync } from 'node:fs';
import { mkdir, writeFile, readdir, rm, stat } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const execFileP = promisify(execFile);
const PROJECT_ROOT = dirname(dirname(fileURLToPath(import.meta.url)));
const OUT_DIR = join(PROJECT_ROOT, 'website', 'content', 'claude-code-docs');
const argProxy = process.argv.includes('--proxy')
  ? process.argv[process.argv.indexOf('--proxy') + 1]
  : null;
const PROXY = argProxy ?? process.env.CC_DOCS_PROXY ?? 'http://127.0.0.1:33210';
const INDEX_URL = 'https://code.claude.com/docs/llms.txt';

// 经 curl 请求（-x 走代理，不跟随重定向，返回状态码 + 响应头 + 正文；瞬时失败重试 3 次）
async function get(url, follow = false) {
  const args = ['-s', '--max-time', '60'];
  if (follow) args.push('-L');
  args.push('-w', '\n%CURL_STATUS%{http_code}\t%CURL_LOC%{redirect_url}');
  if (PROXY) args.unshift('-x', PROXY);
  args.push(url);
  let last;
  for (let attempt = 0; attempt < 3; attempt++) {
    try {
      const { stdout } = await execFileP('curl', args, { maxBuffer: 64 * 1024 * 1024 });
      const m = stdout.match(/\n%CURL_STATUS(\d{3})\t(.*)\n?$/);
      const r = { status: Number(m[1]), text: stdout.slice(0, m.index), location: m[2] };
      if (r.status !== 0) return r;
      last = r;
    } catch (e) {
      last = { status: 0, text: '', location: '' };
    }
  }
  return last;
}

// ---- 解析索引：URL → [顶级分类, 子分类]（Map 保持 llms.txt 首次出现顺序 = 官方目录顺序）----
function parseIndex(text) {
  const map = new Map(); // file -> {top, sub}
  let top = '';
  let sub = '';
  for (const line of text.split('\n')) {
    let m = line.match(/^##\s+(.+)/);
    if (m) { top = m[1].trim(); sub = ''; continue; }
    m = line.match(/^###+\s+(.+)/);
    if (m) { sub = m[1].trim(); continue; }
    // 捕获组必须含 .md 后缀：丢后缀会导致落盘文件无扩展名、cleanStale 误删正牌 .md
    m = line.match(/\]\(https:\/\/code\.claude\.com\/docs\/en\/([a-z0-9-]+(?:\/[a-z0-9-]+)*\.md)\)/);
    if (m && top && sub && !map.has(m[1])) map.set(m[1], { top, sub });
  }
  return map;
}

// ---- 章节名 → 目录名（安全化）----
const dirName = (s) =>
  s.replace(/[^\w\s&/-]/g, '').trim().replace(/\s+/g, '-').toLowerCase();

// ---- 完整更新：删除官方索引中已不存在的 md 与随之变空的目录 ----
async function cleanStale(expectedFiles) {
  const stale = [];
  const walk = async (dir) => {
    for (const name of await readdir(dir)) {
      const full = join(dir, name);
      if ((await stat(full)).isDirectory()) {
        await walk(full);
        try { await rm(full); } catch { /* 非空目录删除失败 = 仍有内容，保留 */ }
      } else if (name.endsWith('.md') && !expectedFiles.has(relative(OUT_DIR, full).replaceAll('\\', '/'))) {
        await rm(full);
        stale.push(name);
      }
    }
  };
  await walk(OUT_DIR);
  return stale;
}

async function main() {
  const incremental = process.argv.includes('--incremental');
  const concurrency = Math.max(1, Number(process.argv[process.argv.indexOf('--concurrency') + 1]) || 8);
  const { text: index } = await get(INDEX_URL);
  const map = parseIndex(index);
  console.log(`索引解析：${map.size} 篇 → ${OUT_DIR}${incremental ? '（补充模式：只下载缺失）' : ''}，并发 ${concurrency}`);

  const overrides = new Map([
    ['changelog.md', async () => {
      const r = await get('https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md', true);
      return r.status === 200 ? r.text : null;
    }],
    ['claude-tag.md', async () => {
      const r = await get('https://claude.com/docs/claude-tag.md', true);
      return r.status === 200 ? r.text : null;
    }],
  ]);

  let ok = 0, skipped = 0, done = 0;
  const fail = [];
  const orderedPaths = [];
  const jobs = [];
  let seq = 0;
  for (const [file, cat] of map) {
    // 子分类与顶级同名（如 Getting started 下还有 #### Getting started）时不再嵌套
    const parts = dirName(cat.sub) === dirName(cat.top)
      ? [dirName(cat.top)]
      : [dirName(cat.top), dirName(cat.sub)];
    // file 可含子路径（如 plugins/overview.md、agent-sdk/hooks.md），并入目标目录；
    // 其前导段与分类目录重复时丢弃（plugins/use-plugins + plugins/install → plugins/use-plugins/install）
    const segs = file.split('/');
    while (segs.length > 1 && parts.includes(segs[0])) segs.shift();
    const name = segs.pop();
    orderedPaths.push([...parts, ...segs, name.replace(/\.md$/, '')].join('/'));
    jobs.push({ seq: ++seq, file, dir: join(OUT_DIR, ...parts, ...segs), name });
  }
  const total = jobs.length;

  // 并发池：每个任务下载一篇；进度按完成序打印（seq 为官方索引序号）
  let next = 0;
  async function worker() {
    while (next < jobs.length) {
      const { seq, file, dir, name } = jobs[next++];
      await mkdir(dir, { recursive: true });
      const dest = join(dir, name);
      let msg;
      if (incremental && existsSync(dest)) {
        skipped++;
        msg = `已有，跳过 ${file}`;
      } else {
        let body = null;
        let note = '';
        if (overrides.has(file)) {
          body = await overrides.get(file)();
          note = '（override）';
        } else {
          const r = await get(`https://code.claude.com/docs/zh-CN/${file}`);
          if (r.status === 200) body = r.text;
        }
        if (body) {
          await writeFile(dest, body);
          ok++;
          msg = `下载 ${file}${note}`;
        } else {
          fail.push(file);
          msg = `失败 ${file}`;
        }
      }
      done++;
      console.log(`  [${done}/${total}] #${seq} ${msg}`);
    }
  }
  await Promise.all(Array.from({ length: Math.min(concurrency, jobs.length) }, worker));
  await writeFile(join(OUT_DIR, 'order.json'), JSON.stringify(orderedPaths, null, 2) + '\n');

  // 完整模式：下载完成后按官方索引清理已下线旧文件。期望集必须是落盘的完整相对路径
  // （分类目录 + 文件自带子路径），用 llms 索引键对比会误删全部文件。补充模式不删除。
  if (incremental) {
    console.log(`完成（补充模式）：下载 ${ok} 篇，跳过已有 ${skipped} 篇；失败：${fail.length}${fail.length ? ' → ' + fail.join(', ') : ''}；order.json ${orderedPaths.length} 条`);
  } else {
    const stale = await cleanStale(new Set(orderedPaths.map((p) => `${p}.md`)));
    console.log(`完成：${ok} 篇；失败：${fail.length}${fail.length ? ' → ' + fail.join(', ') : ''}；order.json ${orderedPaths.length} 条；清理旧文件 ${stale.length}${stale.length ? ' → ' + stale.join(', ') : ''}`);
  }
  if (fail.length) process.exitCode = 1;
}

main().catch((e) => { console.error(e); process.exit(1); });
