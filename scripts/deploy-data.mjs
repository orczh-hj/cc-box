// OSS 数据文件部署：data/ → oss://<bucket>/data/（推荐市场等可热更新的数据）
//
// 用法：
//   npm run deploy:data
//
// 设计：与官网（website/dist）和更新通道（cc-box/）、依赖（deps/）完全解耦，
// 数据文件放 bucket 根下的独立 data/ 目录，客户端按文件名单独拉取。
// 只上传 data/ 下的文件，绝不删除 bucket 其他目录。
//
// ⚠️ 安全红线：不得对 oss://<bucket>/ 或 cc-box/、deps/ 执行 rm。

import { execSync } from 'node:child_process'
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const PROJECT_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const OSS_CONFIG_PATH = path.join(PROJECT_ROOT, 'scripts/oss-config.json')
const DATA_DIR = path.join(PROJECT_ROOT, 'data')
const PUBLIC_BASE = 'https://ccbox.orczh.top'
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

function httpGetOk(url) {
  return (sh(`curl -s -o ${NULL_DEVICE} -w "%{http_code}" --max-time 10 "${url}"`, { silent: true, allowFail: true }) || '').trim() === '200'
}

function main() {
  if (!fs.existsSync(DATA_DIR)) {
    logErr(`数据目录不存在: ${DATA_DIR}`)
    process.exit(1)
  }
  const files = fs.readdirSync(DATA_DIR).filter((f) => f.endsWith('.json'))
  if (files.length === 0) {
    logErr('data/ 下没有 .json 文件')
    process.exit(1)
  }

  logStep('[1/3] 校验 JSON 格式')
  for (const f of files) {
    try {
      JSON.parse(fs.readFileSync(path.join(DATA_DIR, f), 'utf-8'))
      logOk(f)
    } catch (e) {
      logErr(`${f} JSON 解析失败: ${e.message}`)
      process.exit(1)
    }
  }

  logStep('[2/3] 上传到 OSS')
  const { bucketName, region, accessKeyId, accessKeySecret } = loadOssConfig()
  const ossutil = ensureOssutil()
  sh(`"${ossutil}" config -e ${region}.aliyuncs.com -i ${accessKeyId} -k ${accessKeySecret} -L CH`)
  // 单文件逐个上传（-f 覆盖），不删除目录内其他文件
  for (const f of files) {
    sh(`"${ossutil}" cp "${path.join(DATA_DIR, f)}" "oss://${bucketName}/data/${f}" -f`)
    logOk(`oss://${bucketName}/data/${f}`)
  }

  logStep('[3/3] 公网校验')
  let failed = false
  for (const f of files) {
    const url = `${PUBLIC_BASE}/data/${f}`
    if (httpGetOk(url)) {
      logOk(`200 ${url}`)
    } else {
      logErr(`校验失败(非 200): ${url}`)
      failed = true
    }
  }
  if (failed) process.exit(1)
  logOk('数据文件部署完成')
}

main()
