// CC-Box 官网部署：构建 website/ → ossutil 上传到 OSS bucket 根路径 → HEAD 校验
//
// 用法：
//   npm run deploy:site                  # 构建 + 上传 + 校验
//   npm run deploy:site -- --skip-build  # 跳过构建（website/dist 已存在）
//   npm run deploy:site -- --clean-assets # 上传前清空 oss://bucket/assets/（删旧 hash 产物；
//                                         # 清理与新上传之间旧页面有短暂 404 窗口，故非默认）
//
// ⚠️ 安全红线：bucket 根下有更新通道（cc-box/）与内嵌依赖（deps/），
//    绝不允许 `rm -r oss://cc-box/`。清理旧 hash 产物只允许
//    `rm -r oss://cc-box/assets/`，且必须在新 index.html 上传成功之后手动执行。
//
// ossutil 调用模式复制自 scripts/release.js（uploadToOSS），未直接 import：
// release.js 是 CJS 且未导出这些函数，为部署官网重构发布链路不值得。

import { execSync } from 'node:child_process'
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const PROJECT_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const OSS_CONFIG_PATH = path.join(PROJECT_ROOT, 'scripts/oss-config.json')
const SITE_DIST = path.join(PROJECT_ROOT, 'website', 'dist')
const NULL_DEVICE = process.platform === 'win32' ? 'NUL' : '/dev/null'

const logStep = (m) => console.log(`\n\x1b[36m==> ${m}\x1b[0m`)
const logOk = (m) => console.log(`\x1b[32m✓ ${m}\x1b[0m`)
const logErr = (m) => console.log(`\x1b[31m✗ ${m}\x1b[0m`)
const logInfo = (m) => console.log(m)

// 阿里云直连：清空代理 env（本地代理会污染 ossutil/阿里云请求）
function sh(cmd, { silent = false, allowFail = false } = {}) {
  const env = { ...process.env }
  for (const v of ['HTTP_PROXY', 'HTTPS_PROXY', 'http_proxy', 'https_proxy']) delete env[v]
  try {
    return execSync(cmd, { encoding: 'utf-8', stdio: silent ? 'pipe' : 'inherit', env, cwd: PROJECT_ROOT })
  } catch (e) {
    if (allowFail) {
      logInfo(`[allowFail] 命令失败: ${cmd}`)
      if (String(e.stderr || '').trim()) logInfo(String(e.stderr).trim().split('\n').slice(-2).join('\n'))
      return null
    }
    logErr(`命令失败: ${cmd}`)
    if (String(e.stderr || '').trim()) logInfo(String(e.stderr).trim().split('\n').slice(-3).join('\n'))
    process.exit(1)
  }
}

function loadOssConfig() {
  if (!fs.existsSync(OSS_CONFIG_PATH)) {
    logErr(`OSS 配置文件不存在: ${OSS_CONFIG_PATH}`)
    process.exit(1)
  }
  const config = JSON.parse(fs.readFileSync(OSS_CONFIG_PATH, 'utf-8'))
  if (!config.bucketName || !config.region || !config.accessKeyId || !config.accessKeySecret) {
    logErr('OSS 配置缺失,请检查 scripts/oss-config.json')
    process.exit(1)
  }
  return config
}

function ensureOssutil() {
  const name =
    process.platform === 'win32' ? 'ossutil64.exe' : process.platform === 'darwin' ? 'ossutilmac64' : 'ossutil64'
  const ossUtilPath = path.join(PROJECT_ROOT, name)
  if (!fs.existsSync(ossUtilPath)) {
    logInfo(`下载 ossutil (${name})...`)
    sh(
      `node -e "const https=require('https'),fs=require('fs');const ws=fs.createWriteStream(process.argv[1]);https.get(process.argv[2],r=>r.pipe(ws));" "${ossUtilPath}" "https://gosspublic.alicdn.com/ossutil/1.7.14/${name}"`,
    )
    if (process.platform !== 'win32') fs.chmodSync(ossUtilPath, 0o755)
  }
  return ossUtilPath
}

function httpHeadOk(url) {
  return (sh(`curl -s -o ${NULL_DEVICE} -w "%{http_code}" --max-time 10 -I "${url}"`, { silent: true, allowFail: true }) || '').trim() === '200'
}

// OSS 静态页面托管的默认首页只处理 GET：HEAD / 仍按列桶校验返回 403，根路径校验必须用 GET
function httpGetOk(url) {
  return (sh(`curl -s -o ${NULL_DEVICE} -w "%{http_code}" --max-time 10 "${url}"`, { silent: true, allowFail: true }) || '').trim() === '200'
}

function main() {
  const args = process.argv.slice(2)
  const skipBuild = args.includes('--skip-build')
  const cleanAssets = args.includes('--clean-assets')

  // [1/4] 构建
  if (!skipBuild) {
    logStep('[1/4] 构建官网')
    sh('npm run build:site')
  } else {
    logStep('[1/4] 跳过构建（--skip-build）')
  }

  if (!fs.existsSync(path.join(SITE_DIST, 'index.html'))) {
    logErr(`产物缺失: ${path.join(SITE_DIST, 'index.html')}（先跑 npm run build:site）`)
    process.exit(1)
  }
  // 多入口构建：main-*.js（首页）、docs-*.js（文档页）、reference-*.js（参考文档页）
  const assetFiles = fs.readdirSync(path.join(SITE_DIST, 'assets'))
  const mainBundle = assetFiles.find((f) => /^main-.*\.js$/.test(f))
  const docsBundle = assetFiles.find((f) => /^docs-.*\.js$/.test(f))
  const referenceBundle = assetFiles.find((f) => /^reference-.*\.js$/.test(f))
  if (!mainBundle || !docsBundle || !referenceBundle) {
    logErr('产物缺失: website/dist/assets/{main,docs,reference}-*.js')
    process.exit(1)
  }
  logOk('产物完整')

  // [2/4] ossutil 准备
  logStep('[2/4] 准备 ossutil')
  const { bucketName, region, accessKeyId, accessKeySecret } = loadOssConfig()
  const ossutil = ensureOssutil()
  sh(`"${ossutil}" config -e ${region}.aliyuncs.com -i ${accessKeyId} -k ${accessKeySecret} -L CH`)
  const siteBase = `https://${bucketName}.${region}.aliyuncs.com`
  logOk(`目标: ${siteBase}/`)

  // [3/4] 上传：静态资源先行，index.html 最后（引用一致性窗口最小化）
  logStep('[3/4] 上传到 OSS')
  if (cleanAssets) {
    sh(`"${ossutil}" rm -r "oss://${bucketName}/assets/"`)
    logOk('已清空 assets/（旧 hash 产物）')
  }
  sh(`"${ossutil}" cp -r "website/dist/" "oss://${bucketName}/" -f --exclude "index.html"`)
  sh(`"${ossutil}" cp "website/dist/index.html" "oss://${bucketName}/index.html" -f`)
  logOk('上传完成')

  // [4/4] HEAD 校验
  logStep('[4/4] HEAD 校验')
  let failed = false
  for (const url of [
    `${siteBase}/index.html`,
    `${siteBase}/docs.html`,
    `${siteBase}/reference.html`,
    `${siteBase}/robots.txt`,
    `${siteBase}/sitemap.xml`,
    `${siteBase}/assets/${mainBundle}`,
    `${siteBase}/assets/${docsBundle}`,
    `${siteBase}/assets/${referenceBundle}`,
  ]) {
    if (httpHeadOk(url)) {
      logOk(`200 ${url}`)
    } else {
      logErr(`校验失败(非 200): ${url}`)
      failed = true
    }
  }
  if (failed) process.exit(1)

  // 正式域名（自定义域名，OSS 默认域名对网页文件强制下载，仅它可对外）
  const SITE_DOMAIN = 'https://ccbox.orczh.top'
  if (httpGetOk(`${SITE_DOMAIN}/`)) {
    logOk(`200 ${SITE_DOMAIN}/ （官网正式域名）`)
  } else {
    logErr(`官网正式域名校验失败(非 200): ${SITE_DOMAIN}/`)
    process.exit(1)
  }

  if (httpGetOk(`${siteBase}/`)) {
    logOk(`200 ${siteBase}/ （静态页面托管已生效）`)
  } else {
    logInfo(
      `\x1b[33m⚠ 根路径 ${siteBase}/ 非 200：请到 OSS 控制台 → bucket ${bucketName} → 数据管理 → 静态页面 → 默认首页设为 index.html\x1b[0m`,
    )
  }

  logOk('官网部署完成')
  logInfo(`地址: https://ccbox.orczh.top/（文档: https://ccbox.orczh.top/docs.html）`)
  logInfo('提示: 旧 hash 产物会随部署堆积，需要时用 --clean-assets 在上传前清理（勿在上传后手动 rm，会误删新产物）')
}

main()
