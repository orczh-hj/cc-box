---
name: cc-box-release
description: CC-Box 版本发布。当用户说"发布"、"release"、"版本更新"、"准备发布"、"上传 OSS"时使用此 skill。
---

# CC-Box Release

## 发布前检查（必做）

1. **测试全绿**：`cd src-tauri && cargo test` + `npm test`（前端/远程 Rust 改动时另跑 `cd remote/web-assets && cargo test`）
2. **协议改动**：确认四处同步（crate 定义 / protocol.ts / samples / remote/docs/protocol.md）
3. **敏感文件**：确认 `git status` 无 oss-config.json、截图、日志等不该提交的内容
4. **改动范围**：`git status` 里不应混入与本次发布无关的开发中改动——release.js 提交是 `git add -A` 全量语义，无关改动会被一并收进 Release commit（发现时提醒用户先单独提交或暂存）

## 标准流程

```bash
npm run release -- --bump <level> --notes "<notes>"
```

自动更新版本号 → CHANGELOG → 提交推送 → 标签 → CI → GitHub Release → OSS。

**2026-10 重构后特性（两阶段 + 断点续跑）**：

- **开着代理一条命令跑完**：脚本按命令分网络域——GitHub（推送/CI/产物下载）走代理（默认 127.0.0.1:33210，`CC_BOX_PROXY` 可覆盖），Gitee/OSS/阿里云强制直连（清空代理 env）。**不需要中途开关代理**
- **断点续跑**：每阶段幂等（版本文件/CHANGELOG 段/commit/远端 ref/CI 结论/Release 状态/产物 digest/OSS HEAD 逐项检查）。**任何一步失败 → 重跑与首次相同的完整命令（含 `--bump` 与 `--notes`）**，自动跳过已完成部分——这是绝大多数故障的正确处理方式。**不要改成 `--exact`**（没必要，且 `--notes` 用于补齐未完成阶段的 Release notes）
- **断点锁定（防二次 bump）**：`--bump` 重跑时若检测到在途发布（版本文件与 CHANGELOG 已 bump、但 tag/Release/OSS 未完整），自动锁定该版本续跑（输出 `检测到未完成的发布 vX.Y.Z——锁定为续跑目标`），不会基于已变的版本号再 bump 一次；上个版本已完整发布时正常计算新版本
- **分仓库推送**：GitHub（代理，失败阻断）+ Gitee（直连，失败仅告警——**GitHub Actions 并不会自动同步 Gitee**，告警时按提示手动补推）
- 开头会打印代理预检结果；代理未开时 GitHub 阶段会失败，开代理重跑同一命令即续

## 特殊情况

| 场景 | 处理 |
|------|------|
| 脚本中断/某步失败 | **重跑同一命令**（幂等续跑，首选） |
| 仅上传 OSS | `--oss-only v0.6.2`（产物已完整时全程免代理） |
| CI 已完成的重发 | `--exact --skip-ci`（常规断点续跑不需要这些参数） |
| 用户指定级别 | `--bump patch/minor/major`（跳过自动判断） |
| 没有变更 | 提示用户，中止 |

## Notes 格式

```markdown
### Fixed
- Fix <具体问题>

### Features
- Add <具体功能>
```

英文，`\n` 换行，动词开头。**含远程改动时**末尾加用户提示行（如 `Note: phone clients need to re-pair after relay update; refresh browser to get the new bundle.`）。

## 远程组件发布（relay 部署）

**协作机制**：桌面 Rust 是唯一业务执行端；relay（orczh.top:8800）只做认证/配对/连接管理与消息透传，不承载业务。手机/浏览器 UI = 主应用 dist，由 `remote/web-assets` 共享 crate 供给——**服务器配置 `CC_BOX_RELAY_WEB_DIR` 指向磁盘目录时优先读盘（仅前端更新 scp dist 即生效、免重编），未配置/磁盘 miss 回退编译期内嵌**。架构详见 remote/README.md。

桌面版本发布 ≠ 全部生效。按本次改动追加：

| 本次改动 | relay 部署 |
|---|---|
| 前端（src/） | **按通道**：服务器已配 `CC_BOX_RELAY_WEB_DIR` → `npm run build` 后 scp dist 到该目录即生效（免重编免重启）；未配置（默认）→ 必须重编 relay |
| relay（remote/relay/）/ web-assets（remote/web-assets/） | **必须**重编（独立二进制） |
| 协议（remote/protocol/） | **必须**重编（三端同步：桌面 + relay + Web UI） |
| 仅桌面功能 / 仅桌面 Rust（src-tauri/） | 不需要（relay 透传原则，不解析应用层 payload） |

**relay 重编部署（2026-10 起一键化）**：`npm run deploy:relay`——自动完成前端构建 → musl 交叉编译（RUSTFLAGS=lld）→ ssh 预检 → scp `.new` + md5 校验 → **切换前人工确认**（`--yes` 跳过）→ 切换重启 → 三查（md5 / bundle hash / HTTP 特征）。手动细节与故障处理见 remote/docs/deploy.md §3.5。

**仅前端更新（scp 通道）**：`npm run build` → `timeout 90 scp -r dist/* root@orczh.top:<目录>/` → `curl -s https://orczh.top:8800/ | grep -o 'assets/index-[^"]*'` 确认 bundle hash 为本次构建。免重启：**手机无需重新配对**，浏览器强刷页面即可。

**顺序（涉及桌面版本时）：先桌面版本，后 relay 部署**（新桌面向后兼容旧 UI，反序会出现新 UI 调用旧桌面 rpc 白名单外的命令）。

relay 重启（仅重编通道）= 内存态清空：**所有手机需重新扫码配对**（桌面面板自动出新码）。

## 验收标准

发布完成后，验证以下项：

| 项目 | 检查方式 | 预期 |
|------|----------|------|
| GitHub Release | `gh release view v<x.y.z>` | 存在，非 draft，7 个资产（3 平台安装包 + .sig + mac .app.tar.gz） |
| Release Notes | `gh release view` | 格式正确，非字面 `\n`，动词开头，内容与 diff 匹配 |
| Gitee 同步 | `git ls-remote https://gitee.com/orczh/cc-box.git refs/heads/main` | 与 GitHub main 一致（发布时脚本已推，失败会有告警日志） |
| OSS latest.json | curl `https://<bucket>/cc-box/latest.json` | version 字段正确（无 v 前缀） |
| OSS 安装包 | curl HEAD 检查 | 3 个主安装包均返回 200 |
| 本地版本号 | `git describe --tags` | 与发布版本一致 |
| relay（重编部署时） | md5 比对 + bundle hash + 特征字符串 | 新二进制已上线（deploy:relay 已自动三查） |
| relay（scp 通道时） | curl 首页 bundle hash | 与本地 `dist/assets/` 一致 |
| 桌面远程（部署时） | `~/.cc-box/logs/` 有 `relay connected/registered` | 桌面已重连中继 |

注：Gitee Release 已于 2026-08 起不再创建（代码仓库同步靠 release.js 脚本推送，GitHub Actions 不做同步），无需验收 Release。

全部通过 → 发布成功。

任一失败 → 排查并手动修复；release 脚本类失败优先"重跑同一命令"（断点续跑）。
