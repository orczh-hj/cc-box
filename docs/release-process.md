# 版本发布流程

## 远程组件发布（每次版本发布的附加动作）

一套 UI 三端运行（桌面 Tauri / 宽屏浏览器 / 手机浏览器），除桌面版本外多出两条发布线：**手机/浏览器 UI = 主应用 dist**（由 `remote/web-assets` 共享 crate 供给：服务器配置 `CC_BOX_RELAY_WEB_DIR` 指向磁盘目录时优先读盘、miss 回退编译期内嵌）；**relay 二进制独立部署**（重启 = 内存态清空，所有手机配对失效）。

### 改动类型 → 发布动作

| 本次改动 | 桌面版本发布 | relay 部署 | 说明 |
|---|---|---|---|
| 仅桌面功能（无前端、无远程改动） | 需要 | 不需要 | 标准流程 |
| 前端（src/ 任意改动） | 需要 | **按通道** | 服务器已配 `CC_BOX_RELAY_WEB_DIR`：scp dist 即生效免重编（deploy.md §3.5「仅前端更新」）；未配置（默认内嵌）：必须重编 relay |
| 桌面 Rust（src-tauri/） | 需要 | 不需要 | relay 透传原则，不解析应用层 payload |
| relay / web-assets（remote/） | 不需要 | **必须** | 独立部署 |
| 协议（remote/protocol/） | 需要 | **必须** | 三端同步（桌面 + relay + Web UI） |

### 每次版本发布的完整清单

1. **发布前检查**
   - `cd src-tauri && cargo test` + `npm test` 全绿
   - 协议改动：确认四处同步（crate 定义 / protocol.ts / samples / remote/docs/protocol.md，见 CLAUDE.md 检查清单）
   - 远程功能改动：remote/docs/manual-test-cases.md U/R 系列关键条目真机走查
2. **桌面版本发布**：`npm run release -- --bump <type> --notes "..."`（见下节全自动流程）
3. **relay 部署**（上表"必须"时）：完整命令见 remote/docs/deploy.md §3.5（npm run build → musl 交叉编译 → scp → systemctl restart → 三查）
   - **顺序：先桌面版本，后 relay 部署**——新桌面向后兼容旧 UI；反序会出现新 UI 调用旧桌面 rpc 白名单外的命令
4. **发布后验证**
   - relay 三查：md5 比对 / bundle hash / 新功能特征字符串（deploy.md §3.5 第 4 步）
   - 桌面：`~/.cc-box/logs/` 看 `relay connected/registered` 与新配对码
   - 手机：**重新扫码配对**（relay 重启清空内存态）→ 打开会话走查
5. **Release notes 用户提示**（本次含远程改动时）
   - 手机端需重新扫码配对；浏览器端刷新页面拿新 bundle（缓存旧 JS 是"升级没反应"的头号原因）

### 协议兼容约束

- `PROTOCOL_VERSION` 目前握手携带但**不强校验**；兼容性靠同源部署保证（手机 UI 与 relay 同 binary 嵌入、桌面协议随桌面版本）
- 新增可选字段/消息属非破坏性（旧端忽略未知消息）；**破坏性变更**必须桌面与 relay 同窗口期更新，并递增 `PROTOCOL_VERSION` 考虑握手校验

## 自动化发布（推荐）

使用 `scripts/release.js` 脚本全自动发布：

```bash
# 设置代理（GitHub 访问需要）
export HTTP_PROXY=http://127.0.0.1:33210
export HTTPS_PROXY=http://127.0.0.1:33210

# 使用 npm 命令
npm run release -- --bump patch --notes "### Fixed\n- Fix terminal copy issue"

# 或直接运行脚本
node scripts/release.js --bump minor --notes "### Features\n- Add sidebar panel\n\n### Fixed\n- Memory leak"
```

### 参数说明

| 参数 | 必填 | 说明 |
|------|------|------|
| `--bump` | ✓* | 版本更新类型：`major` / `minor` / `patch`（与 `--exact` 二选一） |
| `--exact` | ✓* | 使用当前版本发布，不 bump 版本号（用于重新发布） |
| `--notes` | ✓ | Release notes 内容，用 `\n` 表示换行 |
| `--skip-ci` |  | 跳过 CI 监控（用于已构建的标签） |
| `--oss-only` |  | 仅下载指定版本并上传 OSS（如 `--oss-only v0.5.1`） |

### 执行流程

脚本自动执行以下步骤：
1. 更新版本号（Cargo.toml, package.json, tauri.conf.json）
2. 更新 CHANGELOG.md
3. Git 提交并推送（自动使用代理）
4. 创建并推送标签
5. 监控 CI 构建
6. 发布 GitHub Release
7. 下载 Release 产物并上传到阿里云 OSS（国内更新渠道）

### Release Notes 格式

```bash
# 多行格式示例（用 \n 表示换行）
--notes "### Bug Fixes\n- Fix issue A\n- Fix issue B\n\n### Features\n- Add feature X"
```

### 常用示例

```bash
# 新版本发布
npm run release -- --bump patch --notes "### Fixed\n- Fix copy issue"
npm run release -- --bump minor --notes "### Features\n- Add sidebar panel"

# 重新发布当前版本（CI 已构建）
npm run release -- --exact --notes "### Fixed\n- Fix issue" --skip-ci

# 仅上传 OSS（补传某个版本）
npm run release -- --oss-only v0.5.1
```

## 仅上传到 OSS

已有 Release 产物，只需上传到 OSS：

```bash
# 无需代理（OSS 国内直连）
npm run release:oss -- v0.5.1

# 或直接运行
node scripts/release.js --oss-only v0.5.1
```

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

## CHANGELOG 格式

`CHANGELOG.md` 用于记录版本变更：

```markdown
## [v0.2.5] - 2026-05-05

### Bug Fixes
- Fix terminal copy (Ctrl+C)

### Features
- Add Alt+N/Alt+R shortcuts
```

## 重要提醒

1. **每次发布必须编写 release notes**，说明变更内容
2. **CHANGELOG.md 用英文编写**，便于国际用户阅读
3. **GitHub 操作需要代理**（国内环境），OSS 操作无需代理
4. **OSS 配置文件包含敏感信息**，已在 `.gitignore` 中排除

## 回滚流程

```bash
git push origin :refs/tags/v0.2.5
git tag -d v0.2.5
gh release delete v0.2.5 --yes
```