#!/usr/bin/env node

/**
 * relay 部署脚本(npm run deploy:relay)
 *
 * 背景:桌面版本发布 ≠ 全部生效——前端(src/)或 relay/web-assets 改动后,
 * relay(orczh.top:8800,未配置 CC_BOX_RELAY_WEB_DIR 磁盘通道)必须重编部署
 * (手机 Web UI 是 relay 编译期内嵌的 dist)。
 *
 * 流程:前端构建 → musl 交叉编译 → ssh 预检 → scp .new + md5 校验 →
 * 【人工确认】切换 + systemctl restart → 三查(md5 / bundle hash / 特征)。
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

  logStep('[1/6] 前端构建(dist 将内嵌进 relay)')
  sh('npm run build', { cwd: PROJECT_ROOT })
  if (!fs.existsSync(path.join(PROJECT_ROOT, 'dist', 'index.html'))) {
    logErr('dist/index.html 不存在——构建失败?')
    process.exit(1)
  }

  logStep('[2/6] musl 交叉编译 relay')
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

  logStep('[3/6] 服务器预检')
  const pre = sh(`ssh -o BatchMode=yes -o ConnectTimeout=10 ${SERVER} "systemctl is-active cc-box-relay && (systemctl cat cc-box-relay | grep -c CC_BOX_RELAY_WEB_DIR || true)"`, { silent: true, allowFail: true })
  if (!pre) {
    logErr(`无法连接 ${SERVER} 或 relay 服务不存在——检查 ssh 免密与 systemd 单元`)
    process.exit(1)
  }
  const webDirConfigured = pre.includes('1')
  if (webDirConfigured) {
    log('⚠ 服务器已配置 CC_BOX_RELAY_WEB_DIR(磁盘通道):仅前端更新时 scp dist 即可,无需重编(见 remote/docs/deploy.md)')
  }

  logStep('[4/6] 上传 .new 并校验')
  sh(`scp "${bin}" ${SERVER}:${REMOTE_BIN}.new`)
  const remoteMd5 = sh(`ssh -o BatchMode=yes ${SERVER} "md5sum ${REMOTE_BIN}.new"`, { silent: true }).trim().split(/\s+/)[0]
  if (remoteMd5 !== localMd5) {
    logErr(`md5 不一致(本地 ${localMd5.slice(0, 8)} vs 远端 ${remoteMd5.slice(0, 8)})——重新运行本脚本`)
    process.exit(1)
  }
  logOk('md5 一致')

  logStep('[5/6] 切换(需确认)')
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

  logStep('[6/6] 三查:md5 / bundle hash / HTTP 特征')
  const activeMd5 = sh(`ssh -o BatchMode=yes ${SERVER} "md5sum ${REMOTE_BIN}"`, { silent: true }).trim().split(/\s+/)[0]
  console.log(activeMd5 === localMd5 ? '✓ md5 一致' : `✗ md5 不一致(${activeMd5})`)
  const remoteIndex = sh(`curl -s --max-time 10 ${RELAY_URL}/`, { silent: true, allowFail: true }) || ''
  const remoteHash = (remoteIndex.match(/assets\/index-[^"']+/) || [''])[0]
  const localHashFile = fs.readdirSync(path.join(PROJECT_ROOT, 'dist/assets')).find((f) => /^index-.*\.js$/.test(f))
  const localHash = localHashFile ? `assets/${localHashFile}` : ''
  console.log(remoteHash && remoteHash === localHash ? `✓ bundle hash 一致(${remoteHash})` : `✗ bundle hash 不一致(remote=${remoteHash} local=${localHash})`)
  const iconCode = (sh(`curl -s -o /dev/null -w "%{http_code}" --max-time 10 ${RELAY_URL}/icon.png`, { silent: true, allowFail: true }) || '').trim()
  console.log(iconCode === '200' ? '✓ HTTP 特征(icon.png 200)' : `⚠ icon.png: ${iconCode || '请求失败'}`)

  log('\n部署完成。提醒:')
  log('  - 已配对手机自动重连恢复(v2 token 存活);从 v1 升级则存量手机各重扫一次')
  log('  - 浏览器远程端强刷页面获取新 bundle')
}

main().catch((e) => {
  logErr(e.message)
  process.exit(1)
})
