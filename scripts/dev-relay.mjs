#!/usr/bin/env node
/**
 * 本地中继全链路开发启动（改远程相关代码后的标准动作）：
 * 一次拉起 relay（remote/relay，0.0.0.0:8800）+ 桌面 tauri dev（注入
 * CC_BOX_RELAY_URL=<自动探测的局域网IP>:8800，勿用 127.0.0.1——手机打不开
 * 配对 URL）。手机访问 http://<局域网IP>:8800。
 *
 * 多设备联调：设备列表存在浏览器 localStorage，按页面 origin 隔离。
 * 两台电脑各自跑 dev:relay 会出现两个 origin，手机永远只看到当前一台。
 * 让第二台电脑连接第一台的 relay（不启动本地 relay）：
 *   CC_BOX_RELAY_URL=192.168.5.121:8800 npm run dev:relay
 * 然后手机只打开第一台的 relay 页面，分别配对两台电脑即可互相切换。
 *
 * 任一子进程退出即全部退出；Ctrl+C 统一杀进程树（Windows taskkill /T）。
 */
import { execSync, spawn } from 'node:child_process'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { lanIp } from './lib/lan-ip.mjs'
import { resolveRelayTarget } from './lib/relay-target.mjs'

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const PORT = 8800
const IS_WIN = process.platform === 'win32'

// CC_BOX_RELAY_URL：显式指定共享 relay（多设备联调）。设置后不启动本地 relay。
const relayOverride = process.env.CC_BOX_RELAY_URL?.trim()
// CC_BOX_RELAY_IP 可覆盖自动探测（多网卡/虚拟网卡干扰时手工指定）
const ip = relayOverride ? '' : (process.env.CC_BOX_RELAY_IP?.trim() || lanIp())
const target = resolveRelayTarget({ override: relayOverride, ip, port: PORT })
if (!target.relayUrl) {
  console.error('[dev-relay] 未探测到局域网 IPv4 地址（请检查网络连接），中止')
  process.exit(1)
}

if (target.shared) {
  console.log(`[dev-relay] 共享 relay: ${target.relayUrl}（不启动本地 relay）`)
  console.log(`[dev-relay] 手机访问: ${target.phoneUrl}`)
} else {
  console.log(`[dev-relay] 局域网 IP: ${ip}`)
  console.log(`[dev-relay] 手机访问: ${target.phoneUrl}（扫码或输入地址）`)
}

let exiting = false

/** Windows 下杀进程树（tauri dev 会嵌套 spawn vite/cargo/app）；POSIX 组信号直达。
 * 必须同步等待 taskkill 完成：process.exit 会立刻终止本进程，异步 spawn 的
 * taskkill 还没跑完就没了，relay/vite 会残留在 8800/1420。 */
function killTree(p) {
  if (p.exitCode !== null) return
  if (IS_WIN) {
    if (p.pid) {
      try {
        execSync(`taskkill /F /T /PID ${p.pid}`, { stdio: 'ignore', windowsHide: true })
      } catch { /* 已退出 */ }
    }
  } else {
    try { p.kill('SIGTERM') } catch { /* 已退出 */ }
  }
}

function shutdown(code = 0) {
  if (exiting) return
  exiting = true
  for (const p of procs) killTree(p)
  process.exit(code)
}

/** 前缀输出子进程日志（按行加 [relay]/[app] 标签） */
function start(name, cmd, args, opts) {
  // Windows: npm 是 .cmd，须 shell 解析；参数为常量无注入面
  const p = spawn(cmd, args, { stdio: ['ignore', 'pipe', 'pipe'], shell: IS_WIN, ...opts })
  const pipe = (stream, out) => {
    let buf = ''
    stream.on('data', (chunk) => {
      buf += chunk.toString()
      const lines = buf.split('\n')
      buf = lines.pop() ?? ''
      for (const line of lines) out.write(`[${name}] ${line}\n`)
    })
    stream.on('end', () => { if (buf) out.write(`[${name}] ${buf}\n`) })
  }
  pipe(p.stdout, process.stdout)
  pipe(p.stderr, process.stderr)
  p.on('exit', (code) => {
    if (exiting) return
    console.error(`[dev-relay] ${name} 已退出（code=${code}），关闭全部进程`)
    shutdown(code === null ? 1 : code)
  })
  return p
}

const procs = []
if (!target.shared) {
  procs.push(
    // BIND 只接受纯 IP（端口由 CC_BOX_RELAY_PORT 控制，默认 8800）。
    // CARGO_TARGET_DIR 指到不被 git 跟踪的独立目录：remote/relay/target 下
    // 的 debug 产物历史上被误跟踪，dev 构建会弄脏工作区。
    // WEB_DIR：relay 以 --no-default-features 构建（无内嵌 UI），只服务磁盘
    // 目录——不注入则手机打开配对页得到纯文字 "not found"。指到仓库 dist
    // （改前端后需 npm run build，手机刷新即生效）
    start('relay', 'cargo', ['run'], {
      cwd: path.join(ROOT, 'remote', 'relay'),
      env: {
        ...process.env,
        CC_BOX_RELAY_BIND: '0.0.0.0',
        CC_BOX_RELAY_PORT: String(PORT),
        CC_BOX_RELAY_WEB_DIR: path.join(ROOT, 'dist'),
        CARGO_TARGET_DIR: path.join(ROOT, 'remote', 'relay', 'target-dev'),
      },
    }),
  )
}
procs.push(
  start('app', 'npm', ['run', 'tauri:dev'], {
    cwd: ROOT,
    env: { ...process.env, CC_BOX_RELAY_URL: target.relayUrl },
  }),
)

process.on('SIGINT', () => shutdown(0))
process.on('SIGTERM', () => shutdown(0))
