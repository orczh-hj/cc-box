#!/usr/bin/env node

/**
 * relay 部署脚本(npm run deploy:relay)
 *
 * 背景:relay 二进制**不内嵌 Web UI**(web-assets 以 --no-default-features 构建),
 * 只做消息透传 + 磁盘静态服务——前端与 relay 完全独立:
 *   - 纯前端改动: npm run build + scp dist 到 CC_BOX_RELAY_WEB_DIR,无需本脚本
 *   - relay 源码/服务逻辑改动: 本脚本(无需前端构建,编译不依赖 dist)
 *   - 磁盘目录 CC_BOX_RELAY_WEB_DIR 是硬性要求(未配置则页面 404)
 *
 * 流程: musl 交叉编译 → ssh 预检(必须配置磁盘目录) → scp .new + md5 校验 →
 * 【人工确认】切换 + systemctl restart → 校验(md5 / 服务端 bundle hash / 特征)。
 * 切换保留人工确认:relay 重启断开在途连接(已配对手机凭 token v2 自动重连
 * 恢复,无需重扫;仅升级到 v2 版 relay 时存量 v1 手机需各重扫一次)。
 *
 * 全程国内直连(ssh/scp),无需代理。
 * 用法:npm run deploy:relay [-- --yes 跳过切换确认]
 */
import { execSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import fs from 'node:fs'
import path from 'node:path'
import readline from 'node:readline'
import { fileURLToPath } from 'node:url'

const PROJECT_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const SERVER = 'root@orczh.top'
const REMOTE_BIN = '/usr/local/bin/cc-box-relay'
const RELAY_URL = 'https://orczh.top:8800'
const MUSL_TARGET = 'x86_64-unknown-linux-musl'

const logStep = (m) => console.log(`\n\x1b[36m==> ${m}\x1b[0m`)
const logOk = (m) => console.log(`\x1b[32m✓ ${m}\x1b[0m`)
const logErr = (m) => console.log(`\x1b[31m✗ ${m}\x1b[0m`)
const log = (m) => console.log(m)

/** 全部命令国内直连:清空代理 env;envExtra 注入额外变量(如 RUSTFLAGS) */
function sh(cmd, { silent = false, allowFail = false, cwd, envExtra } = {}) {
  const env = { ...process.env, ...envExtra }
  for (const v of ['HTTP_PROXY', 'HTTPS_PROXY', 'http_proxy', 'https_proxy']) delete env[v]
  if (/^(git|ssh|scp) /.test(cmd)) env.GIT_TERMINAL_PROMPT = '0'
  try {
    return execSync(cmd, { encoding: 'utf-8', stdio: silent ? 'pipe' : 'inherit', env, cwd })
  } catch (e) {
    if (allowFail) return null
    logErr(`命令失败: ${cmd}`)
    if (!silent) process.exit(1)
    throw e
  }
}

const askConfirm = (q) =>
  new Promise((resolve) => {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout })
    rl.question(q, (a) => { rl.close(); resolve(a.toLowerCase() === 'y') })
  })

const sleep = (ms) => new Promise((r) => setTimeout(r, ms))

const md5 = (p) => createHash('md5').update(fs.readFileSync(p)).digest('hex')

async function main() {
  const skipConfirm = process.argv.includes('--yes')

  logStep('[1/5] musl 交叉编译 relay')
  // Windows 无 musl-gcc,统一用 rust-lld 链接(见 remote/docs/deploy.md)
  sh(`rustup target add ${MUSL_TARGET}`, { silent: true, allowFail: true })
  sh(`cargo build --release --target ${MUSL_TARGET}`, {
    cwd: path.join(PROJECT_ROOT, 'remote/relay'),
    envExtra: { RUSTFLAGS: '-C linker=rust-lld' },
  })
  const bin = path.join(PROJECT_ROOT, 'remote/relay/target', MUSL_TARGET, 'release/cc-box-relay')
  if (!fs.existsSync(bin)) {
    logErr(`编译产物不存在: ${bin}`)
    process.exit(1)
  }
  const localMd5 = md5(bin)
  logOk(`产物就绪(${(fs.statSync(bin).size / 1024 / 1024).toFixed(1)} MB, md5 ${localMd5.slice(0, 8)}...)`)

  logStep('[2/5] 服务器预检')
  const pre = sh(`ssh -o BatchMode=yes -o ConnectTimeout=10 ${SERVER} "systemctl is-active cc-box-relay && (systemctl cat cc-box-relay | grep -c CC_BOX_RELAY_WEB_DIR || true)"`, { silent: true, allowFail: true })
  if (!pre) {
    logErr(`无法连接 ${SERVER} 或 relay 服务不存在——检查 ssh 免密与 systemd 单元`)
    process.exit(1)
  }
  // relay 已不内嵌 UI：磁盘目录是 UI 的唯一来源，未配置时必须中止
  if (!pre.includes('1')) {
    logErr('systemd 未配置 CC_BOX_RELAY_WEB_DIR——relay 不内嵌 Web UI，页面将全部 404')
    log('  在单元文件中添加 Environment=CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web 后重试（见 remote/docs/deploy.md）')
    process.exit(1)
  }
  log('✓ 磁盘 Web 目录已配置（前端可独立 scp 更新，无需重编 relay）')

  logStep('[3/5] 上传 .new 并校验')
  sh(`scp "${bin}" ${SERVER}:${REMOTE_BIN}.new`)
  const remoteMd5 = sh(`ssh -o BatchMode=yes ${SERVER} "md5sum ${REMOTE_BIN}.new"`, { silent: true }).trim().split(/\s+/)[0]
  if (remoteMd5 !== localMd5) {
    logErr(`md5 不一致(本地 ${localMd5.slice(0, 8)} vs 远端 ${remoteMd5.slice(0, 8)})——重新运行本脚本`)
    process.exit(1)
  }
  logOk('md5 一致')

  logStep('[4/5] 切换(需确认)')
  if (!skipConfirm) {
    log('\x1b[33mrelay 重启将短暂断开全部连接——已配对手机自动重连恢复(token v2 存活);\x1b[0m')
    log('\x1b[33m若本次是从 v1 版本升级,存量 v1 token 手机需各重扫一次!\x1b[0m')
    const ok = await askConfirm('确认切换并重启 relay?(y/n) ')
    if (!ok) {
      log('已取消。.new 已就位,可稍后手动执行:')
      log(`  ssh ${SERVER} 'cd /usr/local/bin && mv cc-box-relay cc-box-relay.old && mv cc-box-relay.new cc-box-relay && chmod +x cc-box-relay && systemctl restart cc-box-relay'`)
      process.exit(0)
    }
  }
  sh(`ssh -o BatchMode=yes ${SERVER} "cd /usr/local/bin && mv cc-box-relay cc-box-relay.old && mv cc-box-relay.new cc-box-relay && chmod +x cc-box-relay && systemctl restart cc-box-relay && sleep 2 && systemctl is-active cc-box-relay"`)
  logOk('relay 已切换并重启(旧版备份为 cc-box-relay.old)')

  logStep('[5/5] 校验:md5 / 服务端 bundle / HTTP 特征')
  const activeMd5 = sh(`ssh -o BatchMode=yes ${SERVER} "md5sum ${REMOTE_BIN}"`, { silent: true }).trim().split(/\s+/)[0]
  console.log(activeMd5 === localMd5 ? '✓ md5 一致' : `✗ md5 不一致(${activeMd5})`)
  // 用 Node fetch 而非 curl：Windows 下 `curl -o /dev/null` 因 NUL 路径写失败
  // （exit 23）会恒报"请求失败"（与 relay 状态无关的误报）；重启窗口内带重试
  const httpGet = async (p) => {
    for (let attempt = 0; attempt < 3; attempt++) {
      try {
        const res = await fetch(`${RELAY_URL}${p}`)
        return { status: res.status, body: await res.text() }
      } catch {
        await new Promise((r) => setTimeout(r, 1000))
      }
    }
    return { status: 0, body: '' }
  }
  const index = await httpGet('/')
  const remoteHash = (index.body.match(/assets\/index-[^"']+/) || [''])[0]
  const distAssets = path.join(PROJECT_ROOT, 'dist/assets')
  const localHashFile = fs.existsSync(distAssets)
    ? fs.readdirSync(distAssets).find((f) => /^index-.*\.js$/.test(f))
    : undefined
  const localHash = localHashFile ? `assets/${localHashFile}` : ''
  if (!remoteHash) {
    console.log(`✗ 服务端页面未取到 bundle（HTTP ${index.status}；检查 CC_BOX_RELAY_WEB_DIR 与磁盘目录内容）`)
  } else if (localHash) {
    console.log(remoteHash === localHash
      ? `✓ 服务端 bundle 与本地一致(${remoteHash})`
      : `ℹ 服务端 bundle=${remoteHash}，本地 dist=${localHash}（前端未 scp 时不一致属正常）`)
  } else {
    console.log(`ℹ 服务端 bundle=${remoteHash}（本地无 dist，跳过比对）`)
  }
  const icon = await httpGet('/icon.png')
  console.log(icon.status === 200
    ? '✓ HTTP 特征(icon.png 200)'
    : `✗ icon.png: HTTP ${icon.status || '请求失败'}（磁盘目录缺 icon.png 或页面未部署）`)

  log('\n部署完成。提醒:')
  log('  - 已配对手机自动重连恢复(v2 token 存活);从 v1 升级则存量手机各重扫一次')
  log('  - 浏览器远程端强刷页面获取新 bundle')
}

main().catch((e) => {
  logErr(e.message)
  process.exit(1)
})
