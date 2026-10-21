#!/usr/bin/env node

/**
 * CC-Box 自动化发布脚本(两阶段 + 断点续跑版)
 *
 * 设计(2026-10 重构,失败史驱动的修正):
 * - **网络域路由**:每条网络命令显式声明所属域——'github'(注入代理)/
 *   'direct'(清空代理)/'auto'(直连优先失败换代理)。全程开着代理可一次
 *   跑完;阶段二的 ossutil/Gitee 永不受外部代理环境污染。
 * - **断点续跑**:每个阶段幂等(isDone 检查),任何一步失败后**重跑同一命令
 *   自动跳过已完成部分**。失败提示统一打印续跑指引,不再依赖 --exact/
 *   --skip-ci/--oss-only 组合记忆。
 * - **分仓库推送**:origin 配置了 Gitee+GitHub 双 push URL,一条命令跨两域
 *   必挂一半(代理不容 Gitee、直连不通 GitHub)——改为单 URL 显式分推:
 *   GitHub(代理,失败阻断)与 Gitee(直连,失败仅告警可后补)。
 * - **CI 按 tag 匹配**:轮询等待该 tag 的 run 出现再 watch(旧版取 list 第一
 *   条会错拿别的 run);CI 失败立即中止(旧版继续走会卡死在 Release 步骤)。
 *
 * 阶段划分(逻辑分组):
 *   阶段一 GitHub 域:prepare(bump/CHANGELOG/commit,逐项幂等)→ push → tag
 *            → CI → Release(draft 转正式 + 定制 notes)→ 下载产物
 *   阶段二 国内直连域:OSS 上传(HEAD 校验幂等)
 *
 * 用法:
 *   npm run release -- --bump patch --notes "### Fixed\n- Fix issue"
 *   npm run release -- --oss-only v0.16.0   # 补传 OSS(产物完整时免代理)
 *
 * 环境变量:CC_BOX_PROXY 覆盖默认代理地址(127.0.0.1:33210)
 */

const fs = require('fs')
const path = require('path')
const os = require('os')
const crypto = require('crypto')
const { execSync } = require('child_process')

// ============================================
// 配置
// ============================================

const PROJECT_ROOT = path.resolve(__dirname, '..')
const VERSION_FILES = {
  cargoToml: path.join(PROJECT_ROOT, 'src-tauri/Cargo.toml'),
  packageJson: path.join(PROJECT_ROOT, 'package.json'),
  tauriConf: path.join(PROJECT_ROOT, 'src-tauri/tauri.conf.json'),
  changelog: path.join(PROJECT_ROOT, 'CHANGELOG.md'),
}
const OSS_CONFIG_PATH = path.join(PROJECT_ROOT, 'scripts/oss-config.json')
const GITHUB_URL = 'https://github.com/orczh-hj/cc-box.git'
const GITEE_URL = 'https://gitee.com/orczh/cc-box.git'
const MAIN_BRANCH = 'main'
const PROXY = process.env.CC_BOX_PROXY || 'http://127.0.0.1:33210'
const PROXY_VARS = ['HTTP_PROXY', 'HTTPS_PROXY', 'http_proxy', 'https_proxy']

// ============================================
// 日志与断点提示
// ============================================

function logStep(msg) { console.log(`\n\x1b[36m==> ${msg}\x1b[0m`) }
function logSuccess(msg) { console.log(`\x1b[32m✓ ${msg}\x1b[0m`) }
function logError(msg) { console.log(`\x1b[31m✗ ${msg}\x1b[0m`) }
function logInfo(msg) { console.log(msg) }

function printResumeHint() {
  logInfo('\x1b[33m⟳ 断点续跑:开启代理(如需)后重跑同一命令,已完成的步骤会自动跳过\x1b[0m')
}

// ============================================
// 网络域路由执行器
// ============================================

/**
 * 按命令所属网络域执行:
 * - 'github' 注入代理 env(GitHub 推送/大文件下载,必须代理)
 * - 'direct' 清空代理 env(Gitee/ossutil/阿里云,不容代理污染)
 * - 'auto'   直连优先,失败换代理重试(gh api 轻调用,两路都可能通)
 * 失败时:allowFail 返回 null;否则打印断点提示并以非零码退出。
 * git 网络命令统一注入 GIT_TERMINAL_PROMPT=0——凭据缺失时直接失败,
 * 不在 execSync 下挂死等输入。
 */
function run(cmd, domain = 'direct', options = {}) {
  const makeEnv = (withProxy) => {
    const env = { ...process.env }
    if (withProxy) {
      for (const v of PROXY_VARS) env[v] = PROXY
    } else {
      for (const v of PROXY_VARS) delete env[v]
    }
    if (/^git /.test(cmd)) env.GIT_TERMINAL_PROMPT = '0'
    return env
  }

  const attempt = (env) =>
    execSync(cmd, {
      encoding: 'utf-8',
      stdio: options.silent ? 'pipe' : 'inherit',
      env,
      ...(options.silent ? { input: undefined } : {}),
    })

  try {
    return attempt(makeEnv(domain === 'github'))
  } catch (e1) {
    // auto:直连失败后换代理再试一次
    if (domain === 'auto') {
      try {
        return attempt(makeEnv(true))
      } catch {}
    }
    if (options.allowFail) return null
    logError(`命令执行失败(${domain}): ${cmd}`)
    if (String(e1.stderr || '').trim()) logInfo(String(e1.stderr).trim().split('\n').slice(-3).join('\n'))
    printResumeHint()
    process.exit(1)
  }
}

const execQuiet = (cmd, domain = 'direct') => run(cmd, domain, { silent: true, allowFail: true })

function askConfirm(question) {
  const readline = require('readline')
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout })
  return new Promise(resolve => {
    rl.question(question, answer => {
      rl.close()
      resolve(answer.toLowerCase() === 'y')
    })
  })
}

// ============================================
// 版本工具
// ============================================

function getCurrentVersion() {
  const pkg = JSON.parse(fs.readFileSync(VERSION_FILES.packageJson, 'utf-8'))
  return pkg.version
}

function bumpVersion(version, bumpType) {
  const parts = version.split('.').map(Number)
  if (bumpType === 'major') { parts[0]++; parts[1] = 0; parts[2] = 0 }
  else if (bumpType === 'minor') { parts[1]++; parts[2] = 0 }
  else { parts[2]++ }
  return parts.join('.')
}

// ============================================
// 阶段一:prepare(逐项幂等)
// ============================================

function versionFilesReady(newVersion) {
  try {
    const pkg = JSON.parse(fs.readFileSync(VERSION_FILES.packageJson, 'utf-8'))
    const conf = JSON.parse(fs.readFileSync(VERSION_FILES.tauriConf, 'utf-8'))
    const cargo = fs.readFileSync(VERSION_FILES.cargoToml, 'utf-8')
    return (
      pkg.version === newVersion &&
      conf.version === newVersion &&
      new RegExp(`^version\\s*=\\s*"${newVersion}"`, 'm').test(cargo)
    )
  } catch {
    return false
  }
}

function updateVersionFiles(newVersion) {
  logStep('更新版本号文件...')
  let cargo = fs.readFileSync(VERSION_FILES.cargoToml, 'utf-8')
  cargo = cargo.replace(/^version\s*=\s*"[^"]*"/m, `version = "${newVersion}"`)
  fs.writeFileSync(VERSION_FILES.cargoToml, cargo)
  logSuccess(`Cargo.toml -> v${newVersion}`)

  const pkg = JSON.parse(fs.readFileSync(VERSION_FILES.packageJson, 'utf-8'))
  pkg.version = newVersion
  fs.writeFileSync(VERSION_FILES.packageJson, JSON.stringify(pkg, null, 2) + '\n')
  logSuccess(`package.json -> v${newVersion}`)

  const conf = JSON.parse(fs.readFileSync(VERSION_FILES.tauriConf, 'utf-8'))
  conf.version = newVersion
  fs.writeFileSync(VERSION_FILES.tauriConf, JSON.stringify(conf, null, 2) + '\n')
  logSuccess(`tauri.conf.json -> v${newVersion}`)
}

/** CHANGELOG 段已存在判断(updateChangelog 写入格式为 `## [x.y.z] - 日期`,无 v 前缀) */
function changelogHasEntry(content, newVersion) {
  return content.includes(`\n## [${newVersion}] - `)
}

function updateChangelog(newVersion, releaseNotes) {
  logStep('更新 CHANGELOG.md...')
  const content = fs.readFileSync(VERSION_FILES.changelog, 'utf-8')
  if (changelogHasEntry(content, newVersion)) {
    logSuccess(`CHANGELOG 已含 [${newVersion}] 段,跳过`)
    return
  }
  const today = new Date().toISOString().split('T')[0]
  const entry = `\n## [${newVersion}] - ${today}\n\n${releaseNotes}\n`
  const match = content.match(/(.*?# Changelog.*?\n)/s)
  if (!match) {
    logError('无法解析 CHANGELOG.md 格式')
    process.exit(1)
  }
  fs.writeFileSync(VERSION_FILES.changelog, match[1] + entry + content.slice(match[1].length))
  logSuccess('CHANGELOG.md 已更新')
}

function hasReleaseCommit(version) {
  return !!execQuiet(`git log --oneline -1 --grep "Release v${version}"`)
}

function gitCommit(version) {
  logStep('Git 提交...')
  const status = execQuiet('git status --porcelain')
  if (!status) {
    if (hasReleaseCommit(version)) {
      logSuccess(`Release v${version} commit 已存在,跳过`)
      return
    }
    logError('工作区无变更且未找到 Release commit——状态异常,请人工检查')
    process.exit(1)
  }
  run('git add -A', 'direct')
  run(`git commit -m "Release v${version}"`, 'direct')
  logSuccess(`提交完成: Release v${version}`)
}

// ============================================
// 阶段一:push(分仓库,幂等)
// ============================================

function localRefSha(ref) {
  return (execQuiet(`git rev-parse ${ref}`) || '').trim()
}

/** 远端是否已有该 ref 且指向给定 sha(网络失败按未同步处理,交给推送重试) */
function remoteHasRef(url, ref, sha, domain) {
  const out = execQuiet(`git ls-remote ${url} ${ref}`, domain)
  if (!out) return false
  return out.trim().split(/\s+/)[0] === sha
}

function pushRefToBoth(refSpec, version, { tag = false } = {}) {
  // GitHub:主通道,失败阻断
  const localSha = localRefSha(tag ? `v${version}` : MAIN_BRANCH)
  if (remoteHasRef(GITHUB_URL, refSpec, localSha, 'github')) {
    logSuccess(`GitHub ${refSpec} 已同步,跳过`)
  } else {
    run(`git push ${GITHUB_URL} ${refSpec}`, 'github')
    logSuccess(`GitHub ${refSpec} 已推送`)
  }
  // Gitee:国内镜像,失败仅告警(release.yml 不再自动同步 Gitee,但镜像晚一点无碍主发布)
  if (remoteHasRef(GITEE_URL, refSpec, localSha, 'direct')) {
    logSuccess(`Gitee ${refSpec} 已同步,跳过`)
  } else {
    const ok = execQuiet(`git push ${GITEE_URL} ${refSpec}`)
    if (ok !== null) {
      logSuccess(`Gitee ${refSpec} 已推送(直连)`)
    } else {
      logError(`Gitee ${refSpec} 推送失败(不阻断发布)——稍后可手动: git push ${GITEE_URL} ${refSpec}`)
    }
  }
}

function pushRelease(version) {
  logStep(`推送 ${MAIN_BRANCH} 与 tag v${version}(分仓库)`)
  pushRefToBoth(MAIN_BRANCH, version)
  if (!execQuiet(`git tag -l v${version}`)) {
    run(`git tag -a v${version} -m "Release v${version}"`, 'direct')
    logSuccess(`tag v${version} 已创建`)
  } else {
    logSuccess(`tag v${version} 已存在,跳过创建`)
  }
  pushRefToBoth(`v${version}`, version, { tag: true })
}

// ============================================
// 阶段一:CI(按 tag 匹配 + 失败中止)
// ============================================

const sleep = (ms) => new Promise(r => setTimeout(r, ms))

async function watchCIBuild(version) {
  logStep(`监控 CI 构建(tag v${version})...`)
  // 轮询等待该 tag 的 run 注册(tag 刚推时 list 可能尚未出现,旧版取
  // list 第一条会错拿别的 run)
  let runId = null
  for (let i = 0; i < 24 && !runId; i++) {
    const json = execQuiet(`gh run list --branch v${version} --limit 3 --json databaseId,status,conclusion,displayTitle`, 'auto')
    if (json) {
      const runs = JSON.parse(json)
      if (runs.length) {
        runId = runs[0].databaseId
        logInfo(`找到 run: ${runs[0].displayTitle} (ID: ${runId})`)
      }
    }
    if (!runId) await sleep(5000)
  }
  if (!runId) {
    logError(`2 分钟内未找到 tag v${version} 的 CI run——请人工检查 Actions 后重跑(将跳过已完成步骤)`)
    printResumeHint()
    process.exit(1)
  }

  // 已完成(断点重跑场景):直接按结论判定
  const viewJson = () => JSON.parse(execQuiet(`gh run view ${runId} --json status,conclusion`, 'auto') || 'null')
  let st = viewJson()
  if (st && st.status === 'completed') {
    if (st.conclusion === 'success') {
      logSuccess('CI 已构建成功(断点跳过)')
      return
    }
    logError(`CI 失败(conclusion: ${st.conclusion}): https://github.com/orczh-hj/cc-box/actions`)
    printResumeHint()
    process.exit(1)
  }

  // watch(--exit-status:失败以非零码退出;此处失败必须中止——继续走会在
  // Release 步骤对不存在的产物卡死)
  run(`gh run watch ${runId} --exit-status --interval 10`, 'github')
  logSuccess('CI 构建成功')
}

// ============================================
// 阶段一:GitHub Release(幂等)
// ============================================

function releaseState(version) {
  const json = execQuiet(`gh release view v${version} --json isDraft,body`, 'github')
  if (!json) return null
  try {
    return JSON.parse(json)
  } catch {
    return null
  }
}

function publishRelease(version, releaseNotes) {
  logStep('发布 GitHub Release...')

  const firstNoteLine = releaseNotes.split('\n').find(l => l.trim())
  const state = releaseState(version)
  if (state && !state.isDraft && state.body && firstNoteLine && state.body.includes(firstNoteLine)) {
    logSuccess(`Release v${version} 已发布且 notes 匹配,跳过`)
    return
  }

  const fullNotes = `## What's Changed\n\n${releaseNotes}`
  const notesFile = path.join(os.tmpdir(), `cc-box-release-notes-${version}.txt`)
  fs.writeFileSync(notesFile, fullNotes)
  try {
    run(`gh release edit v${version} --draft=false --notes-file "${notesFile}"`, 'github')
    logSuccess(`GitHub Release v${version} 已发布!`)
    logInfo(`查看: https://github.com/orczh-hj/cc-box/releases/tag/v${version}`)
  } finally {
    try { fs.unlinkSync(notesFile) } catch {}
  }
}

// ============================================
// 阶段一:产物下载(digest + size 双校验,幂等)
// ============================================

/**
 * 完整性校验:size + sha256 digest 双比对。
 * 历史教训(v0.13.7/8):脚本被杀时 gh download 子进程仍写文件,半截产物
 * 被传 OSS 致用户更新签名校验失败——size 比对拦截半截;digest 比对拦截
 * 「CI re-run 后新旧产物同 size 不同内容」的漂移(签名文件 base64 长度
 * 固定,size 相同内容已换)。gh 无 digest 字段时退化为仅 size。
 */
function expectedAssets(version) {
  const json = execQuiet(`gh release view v${version} --json assets`, 'auto')
  if (!json) return null
  const table = {}
  for (const a of JSON.parse(json).assets || []) {
    table[a.name] = { size: a.size, digest: typeof a.digest === 'string' ? a.digest.replace(/^sha256:/, '') : null }
  }
  return table
}

function downloadComplete(versionDir, table) {
  if (!table || !fs.existsSync(versionDir)) return false
  const names = Object.keys(table)
  if (names.length === 0) return false
  return names.every((name) => {
    const p = path.join(versionDir, name)
    if (!fs.existsSync(p)) return false
    if (fs.statSync(p).size !== table[name].size) return false
    if (table[name].digest) {
      const local = crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex')
      if (local !== table[name].digest) {
        logInfo(`digest 不匹配: ${name}(内容与官方资产不一致,将重新下载)`)
        return false
      }
    }
    return true
  })
}

function downloadGitHubRelease(version) {
  logStep('下载 GitHub Release 产物...')
  const versionDir = path.join(PROJECT_ROOT, 'releases', version)

  const table = expectedAssets(version)
  if (!table) {
    logError('无法获取 GitHub 资产清单(gh release view 失败)')
    printResumeHint()
    return null
  }
  if (downloadComplete(versionDir, table)) {
    logSuccess(`产物已存在且校验完整(size+digest): ${versionDir}`)
    return { versionDir }
  }

  fs.mkdirSync(versionDir, { recursive: true })
  run(
    `gh release download ${version} --dir "${versionDir}" --pattern "*.exe" --pattern "*.exe.sig" --pattern "*.app.tar.gz" --pattern "*.app.tar.gz.sig" --pattern "*.dmg" --pattern "*.AppImage" --pattern "*.AppImage.sig" --clobber`,
    'github',
    { allowFail: true },
  )
  if (!downloadComplete(versionDir, table)) {
    logError('产物下载失败或不完整(size/digest 与官方资产不一致)')
    printResumeHint()
    logInfo(`稍后可补传: npm run release -- --oss-only ${version}`)
    return null
  }
  logSuccess('产物下载完成且校验完整')
  return { versionDir }
}

// ============================================
// 阶段二:OSS 上传(direct 域,幂等)
// ============================================

function loadOssConfig() {
  if (!fs.existsSync(OSS_CONFIG_PATH)) {
    logError(`OSS 配置文件不存在: ${OSS_CONFIG_PATH}`)
    return null
  }
  const config = JSON.parse(fs.readFileSync(OSS_CONFIG_PATH, 'utf-8'))
  if (!config.bucketName || !config.region || !config.accessKeyId || !config.accessKeySecret) {
    logError('OSS 配置缺失,请检查 scripts/oss-config.json')
    return null
  }
  return config
}

/** OSS 侧幂等:latest.json version 正确且三主产物 HEAD 200 */
function ossAlreadyDone(version, config) {
  const v = version.replace(/^v/, '')
  const base = `https://${config.bucketName}.${config.region}.aliyuncs.com/cc-box`
  const head = (url) => (execQuiet(`curl -s -o /dev/null -w "%{http_code}" --max-time 10 -I "${url}"`) || '').trim() === '200'
  const latest = execQuiet(`curl -s --max-time 10 "${base}/latest.json"`)
  if (!latest) return false
  try {
    if (JSON.parse(latest).version !== v) return false
  } catch {
    return false
  }
  return (
    head(`${base}/${version}/CC-Box_${v}_x64-setup.exe`) &&
    head(`${base}/${version}/CC-Box_${v}_aarch64.dmg`) &&
    head(`${base}/${version}/CC-Box_${v}_amd64.AppImage`)
  )
}

function uploadToOSS(version, versionDir, releaseNotes) {
  logStep('上传到阿里云 OSS...')
  const config = loadOssConfig()
  if (!config) process.exit(1)

  if (ossAlreadyDone(version, config)) {
    logSuccess(`OSS 已是 ${version}(latest.json + 三产物校验通过),跳过`)
    return
  }

  const { bucketName, region, accessKeyId, accessKeySecret } = config
  const endpoint = `${region}.aliyuncs.com`

  // ossutil:首次使用时下载(阿里云 CDN 直连,无需代理;同步等待完成)
  const ossUtilPath = path.join(PROJECT_ROOT, 'ossutil64.exe')
  if (!fs.existsSync(ossUtilPath)) {
    logInfo('下载 ossutil...')
    run(
      `node -e "const https=require('https'),fs=require('fs');const ws=fs.createWriteStream(process.argv[1]);https.get(process.argv[2],r=>r.pipe(ws));" "${ossUtilPath}" "https://gosspublic.alicdn.com/ossutil/1.7.14/ossutil64.exe"`,
      'direct',
    )
  }

  // ossutil 全部 direct 域(阿里云不容代理污染)
  run(`"${ossUtilPath}" config -e ${endpoint} -i ${accessKeyId} -k ${accessKeySecret} -L CH`, 'direct')

  const files = fs.readdirSync(versionDir).map(f => ({ name: f, path: path.join(versionDir, f) }))
  for (const file of files) {
    run(`"${ossUtilPath}" cp "${file.path}" "oss://${bucketName}/cc-box/${version}/" -f`, 'direct')
    logSuccess(`已上传: ${file.name}`)
  }

  const readFileContent = (filePath) => {
    if (!filePath || !fs.existsSync(filePath)) return ''
    return fs.readFileSync(filePath, 'utf-8')
  }
  const getFileSize = (filePath) => {
    if (!filePath) return 0
    try { return fs.statSync(filePath).size } catch { return 0 }
  }

  const latestJson = generateLatestJson({ version, releaseNotes, files, bucketName, endpoint, readFileContent, getFileSize })
  const jsonPath = path.join(versionDir, 'latest.json')
  fs.writeFileSync(jsonPath, JSON.stringify(latestJson, null, 2))
  run(`"${ossUtilPath}" cp "${jsonPath}" "oss://${bucketName}/cc-box/latest.json" -f`, 'direct')
  logSuccess('latest.json 已上传(双格式:官方 + 旧版本兼容)')
}

// ============================================
// 双格式 JSON 生成(可独立测试)
// ============================================

/**
 * 生成双格式 latest.json(同时兼容 Tauri 官方插件和旧版本自定义 updater)
 */
function generateLatestJson({ version, releaseNotes, files, bucketName, endpoint, readFileContent, getFileSize }) {
  const winFile = files.find(f => /-setup\.exe$/i.test(f.name))
  const winSigFile = files.find(f => /-setup\.exe\.sig$/i.test(f.name))
  const macTarGzFile = files.find(f => /\.app\.tar\.gz$/i.test(f.name))
  const macTarGzSigFile = files.find(f => /\.app\.tar\.gz\.sig$/i.test(f.name))
  const macDmgFile = files.find(f => /\.dmg$/i.test(f.name))
  const linuxFile = files.find(f => /\.AppImage$/i.test(f.name))
  const linuxSigFile = files.find(f => /\.AppImage\.sig$/i.test(f.name))

  const macArch = macDmgFile?.name?.includes('aarch64') ? 'aarch64' : 'x86_64'
  const macPlatformKey = `darwin-${macArch}`

  const versionWithoutV = version.replace(/^v/, '')
  const makeUrl = (file) => `https://${bucketName}.${endpoint}/cc-box/${version}/${file?.name || ''}`

  return {
    version: versionWithoutV,
    notes: releaseNotes || '',
    pub_date: new Date().toISOString(),
    platforms: {
      'windows-x86_64': {
        signature: readFileContent(winSigFile?.path),
        url: makeUrl(winFile),
      },
      [macPlatformKey]: {
        signature: readFileContent(macTarGzSigFile?.path),
        url: makeUrl(macTarGzFile),
      },
      'linux-x86_64': {
        signature: readFileContent(linuxSigFile?.path),
        url: makeUrl(linuxFile),
      },
    },

    release_date: new Date().toISOString().split('T')[0],
    release_notes: releaseNotes || '',
    release_notes_url: `https://github.com/orczh-hj/cc-box/releases/tag/${version}`,
    assets: {
      windows: { url: makeUrl(winFile), size: getFileSize(winFile?.path) },
      macos: { url: makeUrl(macDmgFile), size: getFileSize(macDmgFile?.path) },
      linux: { url: makeUrl(linuxFile), size: getFileSize(linuxFile?.path) },
    },
  }
}

module.exports = { generateLatestJson, changelogHasEntry, bumpVersion }

// ============================================
// 独立 OSS 上传命令(断点续跑友好:产物完整时全程免代理)
// ============================================

function ossUploadOnly(version) {
  const result = downloadGitHubRelease(version)
  if (!result) {
    logError('产物下载失败,无法上传 OSS')
    printResumeHint()
    process.exit(1)
  }
  // notes 从 GitHub API 获取(auto:直通常可用,失败换代理)
  const releaseNotes = (execQuiet(`gh release view ${version} --json body --jq .body`, 'auto') || '').trim()
  uploadToOSS(version, result.versionDir, releaseNotes)
  logSuccess(`OSS 上传完成: ${version}`)
}

// ============================================
// 代理预检(信息性,不阻断——失败靠断点续跑兜底)
// ============================================

function reportProxyStatus() {
  const code = execQuiet(`curl -s -o /dev/null -w "%{http_code}" --max-time 8 -x ${PROXY} https://api.github.com/zen`)
  if ((code || '').trim() === '200') {
    logSuccess(`代理可用(${PROXY})——GitHub 阶段(推送/CI/下载)将走代理,OSS/Gitee 直连不受影响`)
  } else {
    logInfo(`\x1b[33m⚠ 代理(${PROXY})当前不可用:GitHub 推送与产物下载将会失败。\n  失败后开启代理并重跑同一命令即可从断点继续。\x1b[0m`)
  }
}

// ============================================
// 参数解析
// ============================================

function parseArgs() {
  const args = process.argv.slice(2)
  const parsed = { bumpType: null, releaseNotes: null, skipCI: false, ossOnly: null, yes: true, exact: false }

  for (let i = 0; i < args.length; i++) {
    switch (args[i]) {
      case '--bump':
        parsed.bumpType = args[++i]
        break
      case '--notes':
        parsed.releaseNotes = args[++i]?.replace(/\\n/g, '\n')
        break
      case '--skip-ci':
        parsed.skipCI = true
        break
      case '--oss-only':
        parsed.ossOnly = args[++i]
        break
      case '--yes':
        parsed.yes = true
        break
      case '--no':
        parsed.yes = false
        break
      case '--exact':
        parsed.exact = true
        break
      case '--help':
      case '-h':
        console.log(`
CC-Box 自动化发布脚本(两阶段 + 断点续跑)

用法:
  npm run release -- --bump <type> --notes "<notes>"    发布新版本
  npm run release -- --exact --notes "<notes>"          发布当前版本(不 bump)
  npm run release -- --oss-only <version>               仅下载并上传 OSS

设计:
  - 每个阶段幂等:任何一步失败后重跑同一命令,自动跳过已完成部分
  - 网络域路由:GitHub 走代理(默认 ${PROXY},CC_BOX_PROXY 可覆盖),
    Gitee/OSS/阿里云强制直连(不受外部代理环境污染)——全程开着代理可一次跑完
  - 分仓库推送:GitHub(阻断)与 Gitee(告警可后补)独立推送

参数:
  --bump <type>      版本类型: major / minor / patch(与 --exact 二选一)
  --exact            使用当前版本发布,不 bump 版本号
  --notes "<notes>"  Release notes,用 \\n 表示换行(必填)
  --skip-ci          跳过 CI 监控(tag 已构建完成时使用)
  --oss-only <ver>   仅处理指定版本的 OSS(如 --oss-only v0.16.0;产物完整时免代理)
  --yes              自动确认(默认)
  --no               显示确认提示(人工执行时使用)

示例:
  npm run release -- --bump patch --notes "### Fixed\\n- Fix copy issue"
  npm run release -- --oss-only v0.16.0
`)
        process.exit(0)
    }
  }
  return parsed
}

// ============================================
// 主流程
// ============================================

async function main() {
  const args = parseArgs()

  // 独立 OSS 上传模式
  if (args.ossOnly) {
    ossUploadOnly(args.ossOnly)
    return
  }

  if (!args.exact && !args.bumpType) {
    logError('需要指定 --bump <type> 或 --exact')
    process.exit(1)
  }
  if (args.bumpType && !['major', 'minor', 'patch'].includes(args.bumpType)) {
    logError(`无效的 bump 类型: ${args.bumpType}`)
    process.exit(1)
  }
  if (!args.releaseNotes) {
    logError('缺少必填参数 --notes')
    process.exit(1)
  }

  console.log('\x1b[35m======================================')
  console.log('     CC-Box 自动化发布脚本(断点续跑版)')
  console.log('======================================\x1b[0m')

  const currentVersion = getCurrentVersion()
  const newVersion = args.exact ? currentVersion : bumpVersion(currentVersion, args.bumpType)
  console.log(args.exact ? `\n\x1b[33m发布当前版本: v${newVersion}\x1b[0m` : `\n\x1b[33m版本更新: v${currentVersion} -> v${newVersion}(${args.bumpType})\x1b[0m`)

  console.log('\nRelease Notes 预览:')
  console.log(args.releaseNotes)

  reportProxyStatus()

  if (!args.yes) {
    const confirmed = await askConfirm('\n是否继续?(y/n) ')
    if (!confirmed) {
      logInfo('已取消发布')
      process.exit(0)
    }
  }

  // ===== 阶段一:GitHub 域 =====
  logStep('\n—— 阶段一:GitHub(推送/CI/Release/产物下载)——')
  if (!args.exact && !versionFilesReady(newVersion)) {
    updateVersionFiles(newVersion)
  } else if (versionFilesReady(newVersion)) {
    logSuccess(`版本文件已是 v${newVersion},跳过 bump`)
  }
  updateChangelog(newVersion, args.releaseNotes)
  gitCommit(newVersion)
  pushRelease(newVersion)

  if (!args.skipCI) {
    await watchCIBuild(newVersion)
  } else {
    logInfo('--skip-ci:跳过 CI 监控')
  }

  publishRelease(newVersion, args.releaseNotes)

  const downloaded = downloadGitHubRelease(`v${newVersion}`)

  // ===== 阶段二:国内直连域 =====
  logStep('\n—— 阶段二:OSS 上传(直连,不受代理影响)——')
  let ossOk = false
  if (downloaded) {
    uploadToOSS(`v${newVersion}`, downloaded.versionDir, args.releaseNotes)
    ossOk = true
  }

  console.log(`\n\x1b[32m======================================`)
  console.log(`     发布完成!v${newVersion}`)
  console.log('======================================\x1b[0m')
  if (!ossOk) {
    console.log('\n\x1b[33m⚠ OSS 未上传,发布不完整——代理开启后补传:\x1b[0m')
    console.log(`  npm run release -- --oss-only v${newVersion}`)
  }
  logInfo('远程组件有改动时记得部署 relay: npm run deploy:relay')
}

// 只在直接运行时执行主流程(被 require 时不执行)
if (require.main === module) {
  main().catch(e => {
    logError(e.message)
    printResumeHint()
    process.exit(1)
  })
}
