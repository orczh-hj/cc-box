#!/usr/bin/env node

/**
 * CC-Box 依赖下载脚本
 *
 * 从 Claude 官方和 GitHub 下载最新版本，上传到阿里云 OSS
 *
 * 用法：
 *   set HTTP_PROXY=http://127.0.0.1:33210
 *   npm run download-deps
 *
 * 下载内容：
 *   - Claude CLI (各平台版本)
 *   - Git for Windows (便携版)
 *
 * 功能：
 *   - 显示下载/上传进度
 *   - 跳过已下载且 checksum 验证通过的文件
 *   - 跳过 OSS 已存在的文件
 */

const fs = require('fs')
const path = require('path')
const crypto = require('crypto')
const { execSync, spawn } = require('child_process')

// ============================================
// 配置
// ============================================

const PROJECT_ROOT = path.resolve(__dirname, '..')
const OSS_CONFIG_PATH = path.join(PROJECT_ROOT, 'scripts/oss-config.json')
const RELEASES_DIR = path.join(PROJECT_ROOT, 'releases/deps')

// Claude CLI 下载源 = npm 镜像平台分包（国内直连免代理）。
// npm 包的编译二进制与官方 downloads.claude.ai 完全一致，且 registry 元数据
// 提供 SHA1 真值可校验。官方源（需代理且节点要过 Cloudflare）仅作备用：
//   https://downloads.claude.ai/claude-code-releases/<v>/manifest.json
const NPM_REGISTRY = 'https://registry.npmmirror.com'
const NPM_CDN = 'https://cdn.npmmirror.com/packages'
const NPM_SCOPE = '@anthropic-ai'
const GIT_RELEASES_API = 'https://api.github.com/repos/git-for-windows/git/releases/latest'

// 平台白名单（全部 8 个形态；默认只处理三主平台——与 CC-Box 自身发布面
// 对齐：Windows x64 / macOS Apple Silicon / Linux x64 glibc）
const ALL_PLATFORMS = [
  'win32-x64',
  'win32-arm64',
  'darwin-x64',
  'darwin-arm64',
  'linux-x64',
  'linux-x64-musl',
  'linux-arm64',
  'linux-arm64-musl',
]
const DEFAULT_PLATFORMS = ['win32-x64', 'darwin-arm64', 'linux-x64']

// 待执行的历史版本清单（2026-11 选定，原因见 docs/release-process.md
// 「Claude CLI 版本存档」）：留空 = 只处理 registry 最新版
const CANDIDATE_VERSIONS = []

// ============================================
// 工具函数
// ============================================

function logStep(msg) {
  console.log(`\n\x1b[36m==> ${msg}\x1b[0m`)
}

function logSuccess(msg) {
  console.log(`\x1b[32m✓ ${msg}\x1b[0m`)
}

function logError(msg) {
  console.log(`\x1b[31m✗ ${msg}\x1b[0m`)
}

function logInfo(msg) {
  console.log(msg)
}

function logSkip(msg) {
  console.log(`\x1b[33m⊙ ${msg}\x1b[0m`)
}

// 格式化文件大小
function formatSize(bytes) {
  if (bytes < 1024) return `${bytes}B`
  if (bytes < 1024 * 1024) return `${Math.round(bytes / 1024)}KB`
  return `${Math.round(bytes / 1024 / 1024)}MB`
}

// 格式化进度条
function formatProgress(percent, downloaded, total) {
  const barWidth = 40
  const filled = Math.round(barWidth * percent / 100)
  const empty = barWidth - filled
  const bar = '\x1b[36m' + '█'.repeat(filled) + '\x1b[0m' + '░'.repeat(empty)
  return `${bar} ${percent.toFixed(1)}% (${formatSize(downloaded)}/${formatSize(total)})`
}

// 使用 curl 下载（支持进度显示）
async function curlDownload(url, outputPath, expectedChecksum = null) {
  const proxy = process.env.HTTP_PROXY || process.env.HTTPS_PROXY

  // 检查本地文件是否已存在且 checksum 匹配
  if (fs.existsSync(outputPath)) {
    if (expectedChecksum) {
      const localChecksum = await calculateChecksum(outputPath)
      if (localChecksum === expectedChecksum.toLowerCase()) {
        logSkip(`已存在且校验通过: ${path.basename(outputPath)} (${formatSize(fs.statSync(outputPath).size)})`)
        return { success: true, skipped: true }
      } else {
        logInfo(`本地文件 checksum 不匹配，重新下载...`)
        fs.unlinkSync(outputPath)
      }
    } else {
      logSkip(`已存在: ${path.basename(outputPath)} (${formatSize(fs.statSync(outputPath).size)})`)
      return { success: true, skipped: true }
    }
  }

  logInfo(`下载: ${url}`)
  if (proxy) logInfo(`使用代理: ${proxy}`)

  // 创建临时文件用于下载
  const tempPath = outputPath + '.tmp'
  // 清理可能存在的临时文件（上次中断残留的 curl 可能短暂占用 → 提示重试而非中断全流程）
  try {
    if (fs.existsSync(tempPath)) fs.unlinkSync(tempPath)
  } catch (e) {
    logError(`临时文件被占用（可能有残留 curl 进程）: ${tempPath}`)
    throw e
  }

  try {
    // 构建 curl 参数
    const curlArgs = [
      '-fsSL',
      '--ssl-no-revoke',
      '-#', // 简短进度条（类似 wget）
      '-o', tempPath,
    ]
    if (proxy) {
      curlArgs.push('--proxy', proxy)
    }
    curlArgs.push(url)

    // 使用 spawn 显示 curl 自带的进度条
    const curl = spawn('curl', curlArgs, {
      stdio: ['inherit', 'inherit', 'inherit']
    })
    // Ctrl+C 时连带杀掉 curl，避免残留子进程占用 .tmp（Windows 下子进程不随父进程退出）
    const onSigint = () => { try { curl.kill() } catch {} process.exit(130) }
    process.once('SIGINT', onSigint)

    // 等待下载完成
    try {
      await new Promise((resolve, reject) => {
        curl.on('close', (code) => {
          if (code === 0) resolve()
          else reject(new Error(`curl exited with code ${code}`))
        })
        curl.on('error', reject)
      })
    } finally {
      process.removeListener('SIGINT', onSigint)
    }

    // 移动临时文件到目标路径
    fs.renameSync(tempPath, outputPath)

    logSuccess(`下载完成: ${path.basename(outputPath)} (${formatSize(fs.statSync(outputPath).size)})`)
    return { success: true, skipped: false }
  } catch (e) {
    // 清理临时文件
    if (fs.existsSync(tempPath)) fs.unlinkSync(tempPath)
    logError(`下载失败: ${e.message}`)
    return { success: false, skipped: false }
  }
}

// 使用 curl 获取内容（自动支持 HTTP_PROXY 环境变量）
function curlFetch(url) {
  const proxy = process.env.HTTP_PROXY || process.env.HTTPS_PROXY
  const proxyArg = proxy ? `--proxy "${proxy}"` : ''

  if (proxy) logInfo(`使用代理: ${proxy}`)

  // 网络抖动重试 3 次（DNS 失败/超时常见）
  let lastErr
  for (let i = 1; i <= 3; i++) {
    try {
      return execSync(`curl -fsSL ${proxyArg} --ssl-no-revoke --max-time 60 "${url}"`, {
        encoding: 'utf-8',
        // npm 包级元数据（557 个版本）远超 execSync 默认 1MB 缓冲
        maxBuffer: 64 * 1024 * 1024,
      })
    } catch (e) {
      lastErr = e
      if (i < 3) {
        logInfo(`请求失败，2s 后重试 (${i}/3): ${url}`)
        execSync('node -e "setTimeout(()=>{},2000)"')
      }
    }
  }
  // 保留 curl 退出码（fetchExistingVersionsJson 用 22=HTTP 404 区分「首次无文件」）
  throw Object.assign(new Error(`请求失败: ${lastErr.message}`), { status: lastErr.status })
}

// 计算文件 SHA256
function calculateChecksum(filePath) {
  return new Promise((resolve, reject) => {
    const hash = crypto.createHash('sha256')
    const stream = fs.createReadStream(filePath)
    stream.on('data', chunk => hash.update(chunk))
    stream.on('end', () => resolve(hash.digest('hex')))
    stream.on('error', reject)
  })
}

// 加载 OSS 配置
function loadOssConfig() {
  if (!fs.existsSync(OSS_CONFIG_PATH)) {
    logError(`OSS 配置文件不存在: ${OSS_CONFIG_PATH}`)
    return null
  }
  const config = JSON.parse(fs.readFileSync(OSS_CONFIG_PATH, 'utf-8'))
  if (!config.bucketName || !config.region || !config.accessKeyId || !config.accessKeySecret) {
    logError('OSS 配置缺失，请检查 scripts/oss-config.json')
    return null
  }
  return config
}

// 检查 OSS 文件是否存在且完整（大小匹配）
function checkOssFileValid(ossPath, expectedSize) {
  const config = loadOssConfig()
  if (!config) return false

  const { bucketName, region, accessKeyId, accessKeySecret } = config
  const endpoint = `${region}.aliyuncs.com`
  const ossUtilPath = path.join(PROJECT_ROOT, 'ossutil64.exe')

  if (!fs.existsSync(ossUtilPath)) return false

  try {
    const result = execSync(`"${ossUtilPath}" stat "oss://${bucketName}/${ossPath}" -e ${endpoint} -i ${accessKeyId} -k ${accessKeySecret}`, {
      encoding: 'utf-8',
      stdio: ['pipe', 'pipe', 'pipe']
    })

    // 提取 Content-Length
    const sizeMatch = result.match(/Content-Length\s*:\s*(\d+)/)
    if (!sizeMatch) return false

    const ossSize = parseInt(sizeMatch[1])
    // 大小必须匹配（允许 0 字节的 latest.json）
    return ossSize === expectedSize || expectedSize === 0
  } catch {
    return false
  }
}

// OSS 上传（带进度显示）
async function uploadToOSS(localPath, ossPath, skipIfExists = true) {
  const config = loadOssConfig()
  if (!config) return { success: false, skipped: false }

  const fileSize = fs.statSync(localPath).size

  // 检查 OSS 文件是否已存在且完整
  if (skipIfExists && checkOssFileValid(ossPath, fileSize)) {
    logSkip(`OSS 已存在且完整: ${ossPath} (${formatSize(fileSize)})`)
    return { success: true, skipped: true }
  }

  const { bucketName, region, accessKeyId, accessKeySecret } = config
  const endpoint = `${region}.aliyuncs.com`
  const ossUtilPath = path.join(PROJECT_ROOT, 'ossutil64.exe')

  // 检查 ossutil
  if (!fs.existsSync(ossUtilPath)) {
    logInfo('下载 ossutil...')
    await downloadOssUtil(ossUtilPath)
  }

  logInfo(`上传: ${path.basename(localPath)} (${formatSize(fileSize)}) → oss://${bucketName}/${ossPath}`)

  try {
    // 使用 spawn 显示 ossutil 自带的进度输出
    const ossutil = spawn(ossUtilPath, [
      'cp',
      localPath,
      `oss://${bucketName}/${ossPath}`,
      '-f',
      '-e', endpoint,
      '-i', accessKeyId,
      '-k', accessKeySecret
    ], {
      stdio: ['inherit', 'inherit', 'inherit']
    })

    // 等待上传完成
    await new Promise((resolve, reject) => {
      ossutil.on('close', (code) => {
        if (code === 0) resolve()
        else reject(new Error(`ossutil exited with code ${code}`))
      })
      ossutil.on('error', reject)
    })

    logSuccess(`已上传: ${ossPath}`)
    return { success: true, skipped: false }
  } catch (e) {
    logError(`上传失败: ${e.message}`)
    return { success: false, skipped: false }
  }
}

// 下载 ossutil
async function downloadOssUtil(targetPath) {
  const url = 'https://gosspublic.alicdn.com/ossutil/1.7.14/ossutil64.exe'
  const result = await curlDownload(url, targetPath)
  if (!result.success) {
    throw new Error('下载 ossutil 失败')
  }
}

// ============================================
// Claude CLI 下载（npm 镜像平台分包）
// ============================================

// ---- 纯函数（导出供测试） ----

/** 解析逗号/空白分隔的版本列表：修剪、去重保序、非法版本号直接抛错 */
function parseVersionList(raw) {
  const parts = String(raw || '')
    .split(/[,\s]+/)
    .map(s => s.trim())
    .filter(Boolean)
  const out = []
  for (const p of parts) {
    if (!/^\d+\.\d+\.\d+$/.test(p)) {
      throw new Error(`非法版本号: ${p}（需形如 2.1.316）`)
    }
    if (!out.includes(p)) out.push(p)
  }
  return out
}

/** 解析平台列表：未知平台丢弃；缺省返回三主平台 */
function parsePlatformList(raw) {
  if (!raw) return [...DEFAULT_PLATFORMS]
  const out = []
  for (const p of String(raw).split(/[,\s]+/).map(s => s.trim()).filter(Boolean)) {
    if (ALL_PLATFORMS.includes(p) && !out.includes(p)) out.push(p)
  }
  return out
}

/** npm 平台包 tarball 文件名 */
function npmTarballName(platform, version) {
  return `claude-code-${platform}-${version}.tgz`
}

/** npm 平台包 tarball CDN 地址（registry 302 的最终目标，免代理）。
 * 形态（实测）：/packages/%40anthropic-ai/claude-code-<platform>/<v>/<tgz>
 * 只编码 @ 符号，scope 与包名之间的 / 保持字面（整段编码会变成 %2F 打不开） */
function npmTarballUrl(platform, version) {
  return `${NPM_CDN}/%40anthropic-ai/claude-code-${platform}/${version}/${npmTarballName(platform, version)}`
}

/** npm 平台包元数据地址（含 dist.shasum=SHA1 真值与 dist.size） */
function npmMetaUrl(platform, version) {
  return `${NPM_REGISTRY}/${NPM_SCOPE}/claude-code-${platform}/${version}`
}

/** npm 主包元数据地址（含 time 字段 = 各版本真实发布日期） */
function npmRootMetaUrl() {
  return `${NPM_REGISTRY}/${NPM_SCOPE}/claude-code`
}

/** tarball 内二进制相对路径（win32 带 .exe；实测包内仅此一个可执行文件） */
function binaryRelPathInTarball(platform) {
  return platform.startsWith('win32') ? 'package/claude.exe' : 'package/claude'
}

/** 二进制在本地 releases 目录中的文件名 */
function binaryFileName(platform) {
  return platform.startsWith('win32') ? 'claude.exe' : 'claude'
}

// ---- 下载流程 ----

/** 拉主包元数据：latest + 各版本发布日期 */
function fetchRootMeta() {
  const meta = JSON.parse(curlFetch(npmRootMetaUrl()))
  const latest = meta['dist-tags'] && meta['dist-tags'].latest
  return { latest, time: meta.time || {} }
}

/** 下载单个平台包的二进制并校验 SHA1 → 返回 SHA256 等信息 */
async function downloadPlatformBinary(version, platform, versionDir) {
  const meta = JSON.parse(curlFetch(npmMetaUrl(platform, version)))
  const expectedSha1 = meta.dist && meta.dist.shasum
  if (!expectedSha1) throw new Error(`${platform}@${version} 元数据缺少 shasum`)

  const platformDir = path.join(versionDir, platform)
  fs.mkdirSync(platformDir, { recursive: true })
  const binPath = path.join(platformDir, binaryFileName(platform))
  const tgzPath = path.join(platformDir, npmTarballName(platform, version))

  // 已存在裸二进制 → 跳过下载（重跑脚本时省流量；checksum 复用本地计算结果）
  if (fs.existsSync(binPath)) {
    logSkip(`已存在: ${platform}/${path.basename(binPath)} (${formatSize(fs.statSync(binPath).size)})`)
    const sha256 = await calculateChecksum(binPath)
    return { platform, filename: path.basename(binPath), checksum: sha256, size: fs.statSync(binPath).size, skipped: true }
  }

  // 下载 tgz（curlDownload 的 expectedChecksum 是 SHA256 语义，这里传 null 后单独校验 SHA1）
  const dl = await curlDownload(npmTarballUrl(platform, version), tgzPath)
  if (!dl.success) throw new Error(`${platform} tarball 下载失败`)

  const actualSha1 = await calculateSha1(tgzPath)
  if (actualSha1 !== String(expectedSha1).toLowerCase()) {
    fs.unlinkSync(tgzPath)
    throw new Error(`${platform} tarball SHA1 不匹配（期望 ${expectedSha1}，实际 ${actualSha1}）`)
  }
  logSuccess(`${platform} SHA1 校验通过 (${expectedSha1.slice(0, 12)}…)`)

  // 解包二进制（Windows 10+ 自带 bsdtar；Git Bash 亦有 tar）
  extractFromTarball(tgzPath, platformDir, platform)
  if (!fs.existsSync(binPath)) {
    throw new Error(`${platform} 解包后未找到 ${binaryRelPathInTarball(platform)}`)
  }
  // 保留裸二进制、删除 tarball（OSS 只存二进制，客户端无需解压步骤）
  fs.unlinkSync(tgzPath)

  const checksum = await calculateChecksum(binPath)
  const size = fs.statSync(binPath).size
  logSuccess(`${platform} 就绪: ${path.basename(binPath)} (${formatSize(size)})`)
  return { platform, filename: path.basename(binPath), checksum, size, skipped: false }
}

/** 从 tarball 解出二进制到目标目录（package/ 前缀剥离为扁平文件名）。
 * 注意：参数一律用相对路径 + cwd——Git Bash 的 GNU tar 会把 `-C D:\...`
 * 的盘符当成远程主机（rsh），报 "Cannot connect to D:"（与 scp 同源坑） */
function extractFromTarball(tgzPath, destDir, platform) {
  const { spawnSync } = require('child_process')
  const relPathInTarball = binaryRelPathInTarball(platform)
  const tmpDirName = '.extract-tmp'
  const tmpDir = path.join(destDir, tmpDirName)
  fs.mkdirSync(tmpDir, { recursive: true })
  const r = spawnSync('tar', ['-xzf', path.basename(tgzPath), '-C', tmpDirName, relPathInTarball], {
    cwd: destDir,
    stdio: ['ignore', 'inherit', 'inherit'],
  })
  if (r.error) throw new Error(`解包失败（需要系统 tar）: ${r.error.message}`)
  if (r.status !== 0) throw new Error(`解包失败，tar 退出码 ${r.status}`)

  const extractedPath = path.join(tmpDir, relPathInTarball)
  fs.copyFileSync(extractedPath, path.join(destDir, binaryFileName(platform)))
  fs.rmSync(tmpDir, { recursive: true, force: true })
}

/** 计算文件 SHA1（npm registry 真值校验用） */
function calculateSha1(filePath) {
  return new Promise((resolve, reject) => {
    const hash = crypto.createHash('sha1')
    const stream = fs.createReadStream(filePath)
    stream.on('data', chunk => hash.update(chunk))
    stream.on('end', () => resolve(hash.digest('hex')))
    stream.on('error', reject)
  })
}

/** 下载并整理指定版本的全部平台 → { version, versionDir, platformInfos } */
async function downloadClaudeVersion(version, platforms, releaseDate, rootLatest) {
  logStep(`下载 Claude CLI v${version}（平台: ${platforms.join(', ')}）`)

  const versionDir = path.join(RELEASES_DIR, 'claude', version)
  fs.mkdirSync(versionDir, { recursive: true })

  const platformInfos = []
  const downloadResults = { downloaded: 0, skipped: 0, failed: 0 }

  for (const platform of platforms) {
    logInfo(`\n--- ${platform} ---`)
    try {
      const info = await downloadPlatformBinary(version, platform, versionDir)
      platformInfos.push(info)
      if (info.skipped) downloadResults.skipped++
      else downloadResults.downloaded++
    } catch (e) {
      logError(`${platform} 失败: ${e.message}`)
      downloadResults.failed++
    }
  }

  logInfo(`\n下载统计: 新下载 ${downloadResults.downloaded}, 跳过 ${downloadResults.skipped}, 失败 ${downloadResults.failed}`)
  if (platformInfos.length === 0) {
    throw new Error(`v${version} 没有成功下载任何平台版本`)
  }

  // latest.json 只在该版本 = registry 最新版时生成（历史版本批量导入不覆盖 latest）
  if (version === rootLatest) {
    const latestJson = {
      version,
      release_date: releaseDate || new Date().toISOString().split('T')[0],
      platforms: {},
    }
    for (const info of platformInfos) {
      latestJson.platforms[info.platform] = {
        url: `deps/claude/${version}/${info.platform}/${info.filename}`,
        checksum: info.checksum,
        size: info.size,
      }
    }
    const latestJsonPath = path.join(RELEASES_DIR, 'claude', 'latest.json')
    fs.writeFileSync(latestJsonPath, JSON.stringify(latestJson, null, 2) + '\n')
    logSuccess(`latest.json 已生成（v${version} 为 registry 最新）`)
  } else {
    logInfo(`v${version} 非 registry 最新（${rootLatest}），不覆盖 latest.json`)
  }

  return { version, versionDir, platformInfos, releaseDate }
}

// ============================================
// Git 便携版下载
// ============================================

async function downloadGitPortable() {
  logStep('获取 Git for Windows 最新版本...')

  // GitHub API 获取最新 release（使用 curl）
  const releaseText = curlFetch(GIT_RELEASES_API)
  let release
  try {
    release = JSON.parse(releaseText)
  } catch (e) {
    logError('解析 GitHub release 失败')
    throw e
  }

  const version = release.tag_name
  logSuccess(`最新版本: ${version}`)

  // 找到 PortableGit 文件
  const portableAsset = release.assets.find(a =>
    a.name.match(/PortableGit-[\d.]+-64-bit\.7z\.exe/i)
  )

  if (!portableAsset) {
    logError('未找到 PortableGit-64-bit.7z.exe')
    throw new Error('PortableGit asset not found')
  }

  logInfo(`找到: ${portableAsset.name}`)
  logInfo(`下载 URL: ${portableAsset.browser_download_url}`)

  // 创建 Git 目录
  const gitDir = path.join(RELEASES_DIR, 'git')
  fs.mkdirSync(gitDir, { recursive: true })
  const outputPath = path.join(gitDir, portableAsset.name)

  logInfo(`\n--- ${portableAsset.name} ---`)
  const result = await curlDownload(portableAsset.browser_download_url, outputPath)

  if (!result.success) {
    throw new Error('下载 Git 便携版失败')
  }

  const fileSize = fs.statSync(outputPath).size
  const downloadResult = result.skipped ? '跳过' : '新下载'
  logSuccess(`下载完成 (${downloadResult}): ${formatSize(fileSize)}`)

  // 生成 latest.json
  const latestJson = {
    version,
    release_date: new Date().toISOString().split('T')[0],
    file: portableAsset.name,
    url: `deps/git/${portableAsset.name}`,
    size: fileSize,
  }

  const latestJsonPath = path.join(gitDir, 'latest.json')
  fs.writeFileSync(latestJsonPath, JSON.stringify(latestJson, null, 2) + '\n')
  logSuccess(`latest.json 已生成: ${latestJsonPath}`)

  return { version, outputPath, portableAsset, skipped: result.skipped }
}

// ============================================
// 上传到 OSS
// ============================================

// 构建 versions.json 中的单条 entry（纯函数）。
// releaseDate 缺省用当天：批量导入历史版本时应传 npm registry 的真实发布日期
function buildVersionEntryFromPlatformInfos(version, platformInfos, releaseDate) {
  const platforms = {}
  for (const info of platformInfos) {
    platforms[info.platform] = {
      url: `deps/claude/${version}/${info.platform}/${info.filename}`,
      checksum: info.checksum,
      size: info.size,
    }
  }
  return {
    version,
    release_date: releaseDate || new Date().toISOString().split('T')[0],
    platforms,
  }
}

// 比较两个版本号，降序（纯函数）
function compareVersionsDesc(a, b) {
  const pa = String(a.version).split('.').map(n => parseInt(n, 10) || 0)
  const pb = String(b.version).split('.').map(n => parseInt(n, 10) || 0)
  const len = Math.max(pa.length, pb.length)
  for (let i = 0; i < len; i++) {
    const va = pa[i] || 0
    const vb = pb[i] || 0
    if (va !== vb) return vb - va
  }
  return 0
}

// 合并现有版本列表与新 entry：同版本按平台合并（补传部分平台时不丢其他平台）
// + 去重 + 降序排序（纯函数）
function mergeVersionEntries(existing, newEntry) {
  const map = new Map()
  for (const e of existing || []) {
    if (e && e.version) map.set(e.version, e)
  }
  if (newEntry && newEntry.version) {
    const old = map.get(newEntry.version)
    if (old) {
      map.set(newEntry.version, {
        ...old,
        ...newEntry,
        release_date: newEntry.release_date || old.release_date,
        platforms: { ...(old.platforms || {}), ...(newEntry.platforms || {}) },
      })
    } else {
      map.set(newEntry.version, newEntry)
    }
  }
  return Array.from(map.values()).sort(compareVersionsDesc)
}

// 拉取 OSS 上的 versions.json。
// 返回 null 仅表示「文件不存在」（首次部署）；**网络失败必须抛错**——
// 否则合并时用空列表覆盖，会把已存档的历史版本条目全部丢掉（曾发生）
function fetchExistingVersionsJson() {
  const config = loadOssConfig()
  if (!config) return null
  const { bucketName, region } = config
  const endpoint = `${region}.aliyuncs.com`
  const url = `https://${bucketName}.${endpoint}/deps/claude/versions.json`
  try {
    return JSON.parse(curlFetch(url))
  } catch (e) {
    // curl -f 对 HTTP 404 返回 22（文件尚不存在）
    if (e.status === 22) return null
    throw new Error(`无法读取 OSS 上的 versions.json，已中止写入以免覆盖历史版本: ${e.message}`)
  }
}

// 维护 deps/claude/versions.json：拉取现有 → 合并当前版本 → 上传。
// latest 取合并后版本号最大者（批量导入历史版本时不得把 latest 降级）
async function mergeAndUploadVersionsJson(version, platformInfos, releaseDate) {
  logInfo('\n--- 维护 versions.json ---')
  try {
    const existing = fetchExistingVersionsJson() || { latest: '', updated_at: '', versions: [] }
    const newEntry = buildVersionEntryFromPlatformInfos(version, platformInfos, releaseDate)
    const mergedVersions = mergeVersionEntries(existing.versions, newEntry)
    const merged = {
      latest: mergedVersions.length > 0 ? mergedVersions[0].version : version,
      updated_at: new Date().toISOString(),
      versions: mergedVersions,
    }

    const versionsJsonPath = path.join(RELEASES_DIR, 'claude', 'versions.json')
    fs.mkdirSync(path.dirname(versionsJsonPath), { recursive: true })
    fs.writeFileSync(versionsJsonPath, JSON.stringify(merged, null, 2) + '\n')

    const result = await uploadToOSS(versionsJsonPath, 'deps/claude/versions.json', false)
    if (result.success) {
      logSuccess(`versions.json 已更新（共 ${mergedVersions.length} 个版本，latest v${merged.latest}）`)
    } else {
      throw new Error('上传失败')
    }
  } catch (e) {
    logError(`versions.json 维护失败（不影响主流程）: ${e.message}`)
  }
}

async function uploadClaudeToOSS(version, versionDir, platformInfos, releaseDate, isLatest) {
  logStep('上传 Claude CLI 到 OSS...')

  const uploadResults = { uploaded: 0, skipped: 0, failed: 0 }

  // 上传各平台文件
  const platforms = fs.readdirSync(versionDir)
  for (const platform of platforms) {
    const platformDir = path.join(versionDir, platform)
    if (!fs.statSync(platformDir).isDirectory()) continue

    const files = fs.readdirSync(platformDir)
    for (const file of files) {
      const localPath = path.join(platformDir, file)
      const ossPath = `deps/claude/${version}/${platform}/${file}`

      logInfo(`\n--- 上传 ${platform}/${file} ---`)
      const result = await uploadToOSS(localPath, ossPath)

      if (result.success) {
        if (result.skipped) uploadResults.skipped++
        else uploadResults.uploaded++
      } else {
        uploadResults.failed++
      }
    }
  }

  // 上传 latest.json（仅该版本是 registry 最新时；历史版本导入不覆盖）
  if (isLatest) {
    logInfo(`\n--- 上传 latest.json ---`)
    const latestJsonPath = path.join(RELEASES_DIR, 'claude', 'latest.json')
    const result = await uploadToOSS(latestJsonPath, 'deps/claude/latest.json', false)
    if (result.success) {
      uploadResults.uploaded++
    } else {
      uploadResults.failed++
    }
  } else {
    logSkip('非最新版本，跳过 latest.json')
  }

  // 维护 versions.json（失败不影响主流程）
  if (platformInfos && platformInfos.length > 0) {
    await mergeAndUploadVersionsJson(version, platformInfos, releaseDate)
  }

  logInfo(`\n上传统计: 新上传 ${uploadResults.uploaded}, 跳过 ${uploadResults.skipped}, 失败 ${uploadResults.failed}`)
}

module.exports = {
  buildVersionEntryFromPlatformInfos,
  compareVersionsDesc,
  mergeVersionEntries,
  parseVersionList,
  parsePlatformList,
  npmTarballName,
  npmTarballUrl,
  npmMetaUrl,
  binaryRelPathInTarball,
}

/** 命令行参数解析：
 *   无参数                        → registry 最新版 × 三主平台
 *   --claude-versions 2.1.318,2.1.316 → 指定历史版本列表
 *   --platforms win32-x64,linux-x64   → 覆盖平台（限白名单）
 *   --skip-git                    → 只处理 Claude CLI（Git 走 GitHub 需代理）
 */
function parseArgs(argv) {
  const args = { versions: [], platforms: parsePlatformList(undefined), skipGit: false }
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]
    if (a === '--claude-versions') args.versions = parseVersionList(argv[++i])
    else if (a === '--platforms') args.platforms = parsePlatformList(argv[++i])
    else if (a === '--skip-git') args.skipGit = true
  }
  return args
}

async function uploadGitToOSS(outputPath, portableAsset, skipped) {
  logStep('上传 Git 便携版到 OSS...')

  // uploadToOSS 会自动检查 OSS 文件完整性
  const uploadResults = { uploaded: 0, skipped: 0, failed: 0 }

  logInfo(`\n--- 上传 ${portableAsset.name} ---`)
  const result = await uploadToOSS(outputPath, `deps/git/${portableAsset.name}`)

  if (result.success) {
    if (result.skipped) uploadResults.skipped++
    else uploadResults.uploaded++
  } else {
    uploadResults.failed++
  }

  // 上传 latest.json（强制更新）
  logInfo(`\n--- 上传 latest.json ---`)
  const latestJsonPath = path.join(RELEASES_DIR, 'git', 'latest.json')
  const jsonResult = await uploadToOSS(latestJsonPath, 'deps/git/latest.json', false)
  if (jsonResult.success) {
    uploadResults.uploaded++
  } else {
    uploadResults.failed++
  }

  logInfo(`\n上传统计: 新上传 ${uploadResults.uploaded}, 跳过 ${uploadResults.skipped}, 失败 ${uploadResults.failed}`)
}

// ============================================
// 主流程
// ============================================

async function main() {
  const args = parseArgs(process.argv.slice(2))

  console.log('\x1b[35m======================================')
  console.log('     CC-Box 依赖下载脚本')
  console.log('======================================\x1b[0m')
  logInfo(`Claude CLI 源: npm 镜像（${NPM_REGISTRY}，免代理）`)
  logInfo(`平台: ${args.platforms.join(', ')}`)
  logInfo(`版本: ${args.versions.length ? args.versions.join(', ') : 'registry 最新版'}`)
  logInfo(`Git: ${args.skipGit ? '跳过（--skip-git）' : '处理'}`)

  // Git 走 GitHub，需代理；Claude 走 npm 镜像不需要
  if (!args.skipGit && !process.env.HTTP_PROXY && !process.env.HTTPS_PROXY) {
    logInfo('\n提示: 未配置代理，Git 下载可能失败（可用 --skip-git 只处理 Claude CLI）')
    logInfo('建议设置: set HTTP_PROXY=http://127.0.0.1:33210')
  }

  try {
    // 确定版本列表：显式指定 或 registry 最新
    const root = fetchRootMeta()
    const versions = args.versions.length ? args.versions : [root.latest]
    logSuccess(`目标版本: ${versions.join(', ')}（registry latest = ${root.latest}）`)

    // 单版本失败不中断后续版本（网络抖动常见）；最后汇总失败清单
    const failedVersions = []
    for (const version of versions) {
      const releaseDate = root.time[version] ? root.time[version].slice(0, 10) : undefined
      if (!releaseDate) logInfo(`提示: registry time 中无 v${version} 日期，回退为当天`)

      try {
        const claudeResult = await downloadClaudeVersion(version, args.platforms, releaseDate, root.latest)
        await uploadClaudeToOSS(
          claudeResult.version,
          claudeResult.versionDir,
          claudeResult.platformInfos,
          claudeResult.releaseDate,
          claudeResult.version === root.latest,
        )
      } catch (e) {
        logError(`v${version} 整体失败（继续下一版本）: ${e.message}`)
        failedVersions.push(version)
      }
    }
    if (failedVersions.length > 0) {
      logError(`\n以下版本未完成：${failedVersions.join(', ')}（重跑同一命令会自动续传）`)
      process.exitCode = 1
    }

    // 下载 Git（仅 Windows 便携版；可跳过）
    if (!args.skipGit) {
      const gitResult = await downloadGitPortable()
      await uploadGitToOSS(gitResult.outputPath, gitResult.portableAsset, gitResult.skipped)
    }

    console.log('\n\x1b[32m======================================')
    console.log('     全部完成！')
    console.log('======================================\x1b[0m')

    logInfo('\nOSS 文件结构:')
    logInfo('  deps/claude/latest.json（仅最新版更新）')
    logInfo('  deps/claude/versions.json')
    for (const version of versions) {
      logInfo(`  deps/claude/${version}/<platform>/claude[.exe]`)
    }
    if (!args.skipGit) logInfo('  deps/git/latest.json + PortableGit')

  } catch (e) {
    logError(`\n执行失败: ${e.message}`)
    process.exit(1)
  }
}

// 仅在直接执行时运行 main（允许测试时 require 纯函数）
if (require.main === module) {
  main()
}