# 版本发布流程

## 远程组件发布（每次版本发布的附加动作）

一套 UI 三端运行（桌面 Tauri / 宽屏浏览器 / 手机浏览器），除桌面版本外多出两条发布线：**手机/浏览器 UI = 主应用 dist**。**relay 二进制不内嵌 UI**（web-assets 以 `--no-default-features` 构建），生产（orczh.top:8800）由 `CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web` 指向的磁盘目录供给 UI：前端改动 scp dist 即生效，免重编、免重启；relay 与前端部署完全独立。**relay 重启 = 内存态清空，已配对手机凭 token v2 自动重连，无需重新扫码**（重新配对仅在桌面「刷新密钥」后需要）。

### 改动类型 → 发布动作

| 本次改动 | 桌面版本发布 | relay 部署 | 说明 |
|---|---|---|---|
| 仅桌面功能（无前端、无远程改动） | 需要 | 不需要 | 标准流程 |
| 前端（src/ 任意改动） | 需要 | **不需要**（生产已磁盘分离） | `npm run build` + `scp -r dist/* root@orczh.top:/opt/cc-box/web/`（deploy.md §3.5）；未配置磁盘目录的部署才需重编 relay |
| 桌面 Rust（src-tauri/） | 需要 | 不需要 | relay 透传原则，不解析应用层 payload |
| relay / web-assets（remote/） | 不需要 | **必须** | 独立部署 |
| 协议（remote/protocol/） | 需要 | 不需要（改了 relay 层消息才需要） | 桌面 + Web UI dist 同步；relay 只透传应用层 payload |

### 每次版本发布的完整清单

1. **发布前检查**
   - `cd src-tauri && cargo test` + `npm test` 全绿
   - 协议改动：确认四处同步（crate 定义 / protocol.ts / samples / remote/docs/protocol.md，见 docs/development.md 新增功能检查清单）
   - 远程功能改动：remote/docs/manual-test-cases.md U/R 系列关键条目真机走查
2. **桌面版本发布**：`npm run release -- --bump <type> --notes "..."`（见下节全自动流程）
3. **relay 部署**（仅 relay 服务逻辑改动时需要）：`npm run deploy:relay`（一键：musl 交叉编译 → 预检磁盘 Web 目录 → scp `.new` + md5 校验 → 确认后切换重启 → 校验自动执行；不构建前端；手动细节见 remote/docs/deploy.md §3.5）
   - **纯前端改动**：`npm run build` + scp dist 到 `/opt/cc-box/web/`，不重编、不重启
   - **顺序：先桌面版本，后 relay 部署**——新桌面向后兼容旧 UI；反序会出现新 UI 调用旧桌面 rpc 白名单外的命令
4. **发布后验证**
   - relay 三查：md5 比对 / bundle hash / 新功能特征字符串（deploy.md §3.5 第 4 步）
   - 桌面：`~/.cc-box/logs/` 看 `relay connected/registered` 与新配对码
   - 手机：**无需重新扫码**——已配对 token 存于手机 localStorage，桌面重连并重新注册设备后手机自动重连（relay 重启仅丢失内存路由表）→ 打开会话走查
5. **Release notes 用户提示**（本次含远程改动时）
   - 浏览器端刷新页面拿新 bundle（缓存旧 JS 是"升级没反应"的头号原因）；已配对手机自动重连，无需重新扫码

### 协议兼容约束

- `PROTOCOL_VERSION` 目前握手携带但**不强校验**；兼容性靠同源部署保证（手机 UI 与 relay 同 binary 嵌入、桌面协议随桌面版本）
- 新增可选字段/消息属非破坏性（旧端忽略未知消息）；**破坏性变更**必须桌面与 relay 同窗口期更新，并递增 `PROTOCOL_VERSION` 考虑握手校验

## 自动化发布（推荐）

使用 `scripts/release.js` 脚本全自动发布（2026-10 重构为**两阶段 + 断点续跑**版）：

```bash
# 全程开着代理即可一次跑完（脚本按命令分域,不需要中途开关）
npm run release -- --bump patch --notes "### Fixed\n- Fix terminal copy issue"

# 或直接运行脚本
node scripts/release.js --bump minor --notes "### Features\n- Add sidebar panel\n\n### Fixed\n- Memory leak"
```

### 设计要点（为什么这样）

- **网络域路由**：每条网络命令显式声明域——GitHub（推送/CI/产物下载,注入代理,默认 `127.0.0.1:33210`,`CC_BOX_PROXY` 可覆盖）/ Gitee·OSS·阿里云（强制直连,清空代理 env,不受外部污染）。**开着代理可一次跑完**,阶段二的 ossutil 永不走代理
- **断点续跑**：每个阶段幂等（版本文件/CHANGELOG 段/Release commit/远端 ref/CI 结论/Release 状态/产物 digest/OSS HEAD 逐项检查）,任何一步失败后**重跑同一命令自动跳过已完成部分**——不再需要 `--exact/--skip-ci` 组合记忆
- **断点锁定（防二次 bump）**：`--bump` 重跑时,若检测到在途发布（版本文件与 CHANGELOG 顶部已是某版本、但该版本 tag/Release/OSS 未完整）,自动**锁定该版本续跑**而不是基于已变的 currentVersion 再 bump 一次——0.16.1 中断后重跑 `--bump patch` 曾二次 bump 出孤儿版本 0.16.2,此逻辑为该教训的修复。判定依据 `releaseFullyDone`（tag + 非 draft Release + OSS 三项全过才算完整）;gh 查询失败按未完整处理,锁定续跑只会触发幂等跳过,无破坏性
- **分仓库推送**：origin 配置了双 push URL,一条 `git push origin` 跨两域必挂一半——脚本改为单 URL 显式分推：GitHub（代理,失败阻断）+ Gitee（直连,失败仅告警可后补）。**注意：GitHub Actions 并不会自动同步 Gitee**（release.yml 注释声称同步但无此代码）,Gitee 同步只能靠脚本的手动推送
- **CI 按 tag 匹配**：轮询等待该 tag 的 run 出现再 watch（避免错拿别的 run）;CI 失败立即中止（旧版继续走会在 Release 步骤卡死）
- **产物完整性**：size + sha256 digest 双校验（防半截文件,也防 CI re-run 后同 size 不同内容的漂移）
- **GIT_TERMINAL_PROMPT=0**：git 凭据缺失时直接失败,不在 execSync 下挂死等输入

### 参数说明

| 参数 | 必填 | 说明 |
|------|------|------|
| `--bump` | ✓* | 版本更新类型：`major` / `minor` / `patch`（与 `--exact` 二选一） |
| `--exact` | ✓* | 使用当前版本发布，不 bump 版本号（断点续跑时通常不需要——直接重跑原命令） |
| `--notes` | ✓ | Release notes 内容，用 `\n` 表示换行 |
| `--skip-ci` |  | 跳过 CI 监控（用于已构建的标签） |
| `--oss-only` |  | 仅下载指定版本并上传 OSS（如 `--oss-only v0.5.1`；产物已完整时全程免代理） |

### 执行流程

```
阶段一 GitHub 域（代理）
  1. 更新版本号（Cargo.toml, package.json, tauri.conf.json）
  2. 更新 CHANGELOG.md（幂等:已含该版本段则跳过）
  3. Git 提交（幂等:Release commit 已存在则跳过）
  4. 分仓库推送 main + tag（GitHub 阻断 / Gitee 告警）
  5. 按 tag 匹配并监控 CI（失败中止）
  6. GitHub Release:draft 转正式 + 定制 notes（幂等）
  7. 下载产物到 releases/<ver>/（size+digest 校验,完整则跳过）

阶段二 国内直连域（清空代理）
  8. OSS 上传（幂等:latest.json version + 三产物 HEAD 校验）
```

失败时脚本会打印续跑指引——**开/关代理（如需）后重跑同一命令**即可。

#### 重跑操作提示（重要）

- **重跑请用与首次相同的完整命令**（含 `--bump` 与 `--notes`）,不要改成 `--exact`——脚本会自动锁定在途版本,`--notes` 用于补齐未完成阶段的 Release notes
- 重跑输出 `检测到未完成的发布 vX.Y.Z——锁定为续跑目标` 即为锁定生效,属正常行为
- 判断"在途"的三要素：版本文件已 bump + CHANGELOG 顶部段 = 该版本 + 该版本未完整发布（tag 未建 / Release 是 draft 或不存在 / OSS 未更新,任一即算未完成）
- 上个版本已完整发布时重跑 `--bump`,正常计算新版本（不会误锁）

### Release Notes 格式

```bash
# 多行格式示例（用 \n 表示换行；内容用中文）
--notes "### Bug Fixes\n- 修复问题 A\n- 修复问题 B\n\n### Features\n- 新增功能 X"
```

**写作角度**：中文书写，**从用户使用的角度出发**——写「修好了什么影响你的问题、你能用到什么新能力」，不罗列提交/模块/实现细节（用户不关心 broker revision、attach 语义这类内部概念）。

### 常用示例

```bash
# 新版本发布
npm run release -- --bump patch --notes "### Fixed\n- 修复复制问题"
npm run release -- --bump minor --notes "### Features\n- 新增侧边栏面板"

# 重新发布当前版本（CI 已构建）
npm run release -- --exact --notes "### Fixed\n- Fix issue" --skip-ci

# 仅上传 OSS（补传某个版本）
npm run release -- --oss-only v0.5.1
```

## 仅上传到 OSS

已有 Release 产物，只需上传到 OSS：

```bash
# 产物已完整（releases/<ver>/ 通过 size+digest 校验）时全程免代理；
# 产物不完整时会先从 GitHub 下载（需代理，失败打印断点提示）
npm run release:oss -- v0.5.1

# 或直接运行
node scripts/release.js --oss-only v0.5.1
```

OSS 侧幂等：latest.json version 正确且三主安装包 HEAD 200 时自动跳过重复上传。

## OSS 配置

OSS 配置文件：`scripts/oss-config.json`

```json
{
  "bucketName": "cc-box",
  "region": "oss-cn-beijing",
  "accessKeyId": "YOUR_ACCESS_KEY_ID",
  "accessKeySecret": "YOUR_ACCESS_KEY_SECRET"
}
```

**注意**：此文件已加入 `.gitignore`，不会提交到仓库。首次使用需复制示例文件：

```bash
cp scripts/oss-config.example.json scripts/oss-config.json
# 编辑填入阿里云 AccessKey
```

## 版本号更新位置

三个文件的版本号必须保持一致：

| 文件 | 路径 |
|------|------|
| Cargo.toml | `src-tauri/Cargo.toml` → `version` |
| package.json | `package.json` → `version` |
| tauri.conf.json | `src-tauri/tauri.conf.json` → `version` |

## 手动发布（备用）

```bash
# 1. 更新版本号（编辑三个文件）

# 2. 提交并推送（需要代理）
export HTTP_PROXY=http://127.0.0.1:33210
git add -A && git commit -m "Release v0.2.5"
git push origin main

# 3. 创建并推送标签（触发 CI 构建）
git tag -a v0.2.5 -m "Release v0.2.5"
git push origin v0.2.5

# 4. 监控 CI 构建
gh run watch <run-id> --exit-status
# 或访问 https://github.com/orczh-hj/cc-box/actions

# 5. 发布 Release
gh release edit v0.2.5 --draft=false --notes "## What's Changed\n\n..."

# 6. 上传到 OSS（无需代理）
node scripts/release.js --oss-only v0.2.5
```

## 构建产物

CI 自动构建并上传：

| 平台 | 产物 |
|------|------|
| Windows (x64) | `.exe` (NSIS) + `.exe.sig` |
| macOS (ARM) | `.dmg` + `.app.tar.gz` + `.app.tar.gz.sig` |
| Linux (x64) | `.AppImage` + `.AppImage.sig` |

**本地 `npm run tauri:build` 只产出 NSIS exe、不生成签名产物**（`tauri.conf.json` 默认 `targets: "nsis"` 且未启用 `createUpdaterArtifacts`，本地无私钥也不会报错）。签名产物（自动更新必需的 `*.sig`）由 Release CI 通过 `--config '{"bundle":{"createUpdaterArtifacts":true}}'` 启用，并用 secret `TAURI_SIGNING_PRIVATE_KEY` 签名；macOS/Linux 的打包目标由 `tauri.macos.conf.json` / `tauri.linux.conf.json` 平台专属配置提供。MSI 已移除（CI 从未上传过）。

## CHANGELOG 格式

`CHANGELOG.md` 用于记录版本变更：

```markdown
## [v0.2.5] - 2026-05-05

### Bug Fixes
- 修复终端复制（Ctrl+C）

### Features
- 新增 Alt+N/Alt+R 快捷键
```

## 重要提醒

1. **每次发布必须编写 release notes**，说明变更内容
2. **release notes 与 CHANGELOG.md 用中文编写**（2026-10 起），**从用户使用角度出发**：写用户可感知的修复与新增能力，不罗列提交、模块名或内部实现概念
3. **GitHub 操作需要代理**（国内环境），OSS 操作无需代理
4. **OSS 配置文件包含敏感信息**，已在 `.gitignore` 中排除
5. **新增 Rust crate 时必须把它的 `target/` 加入 `.gitignore`**——release 脚本 `git add -A` 会把构建产物（.o/.pdb/.dll，数百文件）提交进 Release commit
6. **OSS 安装包路径带 v 前缀**：`/cc-box/v<x.y.z>/`（验收时注意，latest.json 内 url 为准）

## 发布故障恢复（脚本中断后续跑）

脚本若在某步失败退出,2026-10 重构后**首选重跑同一命令**（各阶段幂等,自动跳过已完成部分）；以下为仍需人工介入的场景：

1. **Gitee 推送告警**（脚本内直连失败,不阻断发布）——一次性禁校验补推（公开代码无风险,不改全局配置）：

   ```bash
   git -c http.sslVerify=false push https://gitee.com/orczh/cc-box.git main --tags
   ```

   注意：多 URL origin 下手动 `git push origin` 仍会连推两仓库（代理环境 Gitee 必失败）——**始终分 URL 推送**：GitHub 用代理、Gitee 直连。

2. **commit 需要修正**（如误入构建产物）：先确认 GitHub 未收到（`git ls-remote https://github.com/orczh-hj/cc-box.git refs/heads/main` 是否为本地 HEAD），未收到才可安全 amend；产物文件用 `git rm -r --cached <target>` 移出跟踪后 amend。

3. **特殊情况手动步骤**（脚本已覆盖常规断点,以下仅在脚本本身故障时使用）：

   ```bash
   git tag -a v<x.y.z> -m "Release v<x.y.z>" && git push <两远程> v<x.y.z>   # 触发 CI
   gh run watch <run-id> --exit-status                                        # 监控 CI
   sed -n '/^## \[<ver>\]/,/^## \[/p' CHANGELOG.md | sed '$d' > /tmp/notes.txt
   gh release edit v<x.y.z> --draft=false --notes-file /tmp/notes.txt         # CI 建 draft 后转正式
   npm run release -- --oss-only v<x.y.z>                                     # OSS 上传
   ```

## 回滚流程

```bash
git push origin :refs/tags/v0.2.5
git tag -d v0.2.5
gh release delete v0.2.5 --yes
```