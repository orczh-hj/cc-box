#!/usr/bin/env node

/**
 * Web UI 部署（npm run deploy:web）
 *
 * relay 二进制不内嵌 UI：生产 UI 的唯一来源是磁盘目录
 * （CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web）。前端改动只需更新该目录，
 * 免重编、免重启 relay。
 *
 * 部署策略：完整镜像替换，不叠加历史产物——
 *   构建 → 上传到 web.next → md5 校验 → 原子切换（web→web.prev,
 *   web.next→web）→ 校验线上 bundle → 删除 web.prev。
 *   校验失败自动回滚。
 *
 * 不要再用 `scp -r dist/*` 直接覆盖：旧 index-*.js 会永久堆积在 assets。
 *
 * 用法：
 *   npm run deploy:web
 *   npm run deploy:web -- --skip-build
 *   npm run deploy:web -- --keep-prev    # 保留上一版目录供手动回滚
 */

import { execSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { scpTargets, indexBundleOf, servedBundleFromHtml, isSafeName } from './lib/deploy-web.mjs'

const PROJECT_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const SERVER = 'root@orczh.top'
const WEB = '/opt/cc-box/web'
const STAGING = '/opt/cc-box/web.next'
const PREV = '/opt/cc-box/web.prev'
const RELAY_URL = 'https://orczh.top:8800'

const logStep = (m) => console.log(`\n\x1b[36m==> ${m}\x1b[0m`)
const logOk = (m) => console.log(`\x1b[32m✓ ${m}\x1b[0m`)
const logErr = (m) => console.log(`\x1b[31m✗ ${m}\x1b[0m`)

/** ssh/scp 国内直连：清空代理 env（本地代理会让 OpenSSH 连不上） */
function sh(cmd, { silent = false, allowFail = false } = {}) {
  const env = { ...process.env }
  for (const v of ['HTTP_PROXY', 'HTTPS_PROXY', 'http_proxy', 'https_proxy']) delete env[v]
  try {
    return execSync(cmd, {
      encoding: 'utf-8',
      stdio: silent ? 'pipe' : 'inherit',
      env,
      cwd: PROJECT_ROOT,
    })
  } catch (e) {
    if (allowFail) return null
    logErr(`命令失败: ${cmd}`)
    process.exit(1)
  }
}

const md5 = (p) => createHash('md5').update(fs.readFileSync(p)).digest('hex')

async function servedBundle() {
  for (let attempt = 0; attempt < 3; attempt++) {
    try {
      const res = await fetch(`${RELAY_URL}/`)
      const found = servedBundleFromHtml(await res.text())
      if (found) return found
    } catch {
      // 网络抖动重试
    }
    await new Promise(r => setTimeout(r, 1000))
  }
  return ''
}

async function main() {
  const args = process.argv.slice(2)
  const skipBuild = args.includes('--skip-build')
  const keepPrev = args.includes('--keep-prev')

  if (!skipBuild) {
    logStep('[1/5] 构建前端')
    sh('npm run build')
    logOk('构建完成')
  } else {
    logStep('[1/5] 跳过构建（--skip-build）')
  }

  const dist = path.join(PROJECT_ROOT, 'dist')
  const entries = fs.readdirSync(dist)
  const bundle = indexBundleOf(fs.readdirSync(path.join(dist, 'assets')))
  if (!bundle) {
    logErr('dist/assets 缺少 index-*.js——先运行 npm run build')
    process.exit(1)
  }
  if (![...entries, bundle].every(isSafeName)) {
    logErr(`产物名含意外字符，拒绝拼接远端命令: ${[...entries, bundle].filter(n => !isSafeName(n)).join(', ')}`)
    process.exit(1)
  }
  const localMd5 = md5(path.join(dist, 'assets', bundle))
  logOk(`本地 bundle: ${bundle}（md5 ${localMd5.slice(0, 8)}…）`)

  logStep('[2/5] 上传到临时目录')
  sh(`ssh ${SERVER} "rm -rf ${STAGING} && mkdir -p ${STAGING}"`)
  sh(`scp -r ${scpTargets(entries)} ${SERVER}:${STAGING}/`)
  logOk('上传完成')

  logStep('[3/5] 校验上传完整性')
  const remoteMd5 = sh(`ssh ${SERVER} "md5sum ${STAGING}/assets/${bundle}"`, { silent: true })
    .trim().split(/\s+/)[0]
  if (remoteMd5 !== localMd5) {
    sh(`ssh ${SERVER} "rm -rf ${STAGING}"`, { allowFail: true })
    logErr(`md5 不一致（本地 ${localMd5.slice(0, 8)} vs 远端 ${remoteMd5.slice(0, 8)}）——线上版本未动`)
    process.exit(1)
  }
  logOk('md5 一致')

  logStep('[4/5] 原子切换到生产目录')
  sh(
    `ssh ${SERVER} "rm -rf ${PREV}; if [ -d ${WEB} ]; then mv ${WEB} ${PREV}; fi; mv ${STAGING} ${WEB}"`,
  )
  logOk('已切换')

  logStep('[5/5] 校验线上页面')
  const served = await servedBundle()
  if (served !== `assets/${bundle}`) {
    // 首次部署没有 PREV 可回滚：保留现场（新版本）便于排查，不得删掉后 404
    logErr(`线上仍引用 ${served || '（取不到页面）'}，回滚（无上一版时保留新版本现场）`)
    sh(
      `ssh ${SERVER} "if [ -d ${PREV} ]; then rm -rf ${WEB} && mv ${PREV} ${WEB}; fi"`,
      { allowFail: true },
    )
    process.exit(1)
  }
  logOk(`线上已引用 ${served}`)

  if (keepPrev) {
    console.log(`\x1b[33mℹ 已保留上一版目录 ${PREV}（手动回滚：rm -rf ${WEB} && mv ${PREV} ${WEB}）\x1b[0m`)
  } else {
    sh(`ssh ${SERVER} "rm -rf ${PREV}"`, { allowFail: true })
    logOk('已清理上一版，目录保持完整镜像（无历史产物堆叠）')
  }

  console.log('\n部署完成。手机/浏览器强刷（或关闭标签重开）获取新 bundle。')
}

main().catch((e) => {
  logErr(e.message)
  process.exit(1)
})
