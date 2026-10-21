---
name: cc-box-release
description: CC-Box 版本发布。当用户说"发布"、"release"、"版本更新"、"准备发布"、"上传 OSS"时使用此 skill。
---

# CC-Box Release

## 发布前检查（必做）

1. **测试全绿**：`cd src-tauri && cargo test` + `npm test`
2. **协议改动**：确认四处同步（crate 定义 / protocol.ts / samples / remote/docs/protocol.md）
3. **敏感文件**：确认 `git status` 无 oss-config.json、截图、日志等不该提交的内容

## 标准流程

```bash
npm run release -- --bump <level> --notes "<notes>"
```

自动更新版本号 → CHANGELOG → 提交推送 → 标签 → CI → GitHub Release → OSS。

## 特殊情况

| 场景 | 命令 |
|------|------|
| 仅上传 OSS | `--oss-only v0.6.2` |
| 重新发布当前版本 | `--exact --skip-ci` |
| 用户指定级别 | `--bump patch/minor/major`（跳过自动判断） |
| CI 已完成 | `--skip-ci` |
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

不确定服务器是否配置磁盘通道时：`ssh root@orczh.top 'systemctl cat cc-box-relay | grep Environment'`——有 `CC_BOX_RELAY_WEB_DIR` 即走 scp 通道。

**仅前端更新（scp 通道）**：`npm run build` → `timeout 90 scp -r dist/* root@orczh.top:<目录>/` → `curl -s https://orczh.top:8800/ | grep -o 'assets/index-[^"]*'` 确认 bundle hash 为本次构建。免重启：**手机无需重新配对**，浏览器强刷页面即可。

**relay 重编部署**（改 Rust / 未配磁盘通道）：`npm run build` → `cd remote/relay && RUSTFLAGS="-C linker=rust-lld" cargo build --release --target x86_64-unknown-linux-musl` → scp 为 `.new` → ssh 切换 + `systemctl restart` → 三查（md5 / bundle hash / 特征字符串）。完整命令见 remote/docs/deploy.md §3.5。

**顺序（涉及桌面版本时）：先桌面版本，后 relay 部署**（新桌面向后兼容旧 UI，反序会出现新 UI 调用旧桌面 rpc 白名单外的命令）。

relay 重启（仅重编通道）= 内存态清空：**所有手机需重新扫码配对**（桌面面板自动出新码）。

## 验收标准

发布完成后，验证以下项：

| 项目 | 检查方式 | 预期 |
|------|----------|------|
| GitHub Release | `gh release view v<x.y.z>` | 存在，非 draft，有 3 个资产 |
| Release Notes | `gh release view` | 格式正确，非字面 `\n`，动词开头，内容与 diff 匹配 |
| OSS latest.json | curl `https://<bucket>/cc-box/latest.json` | version 字段正确 |
| OSS 安装包 | curl HEAD 检查 | 3 个文件均返回 200 |
| 本地版本号 | `git describe --tags` | 与发布版本一致 |
| relay（重编部署时） | md5 比对 + bundle hash + 特征字符串 | 新二进制已上线 |
| relay（scp 通道时） | curl 首页 bundle hash | 与本地 `dist/assets/` 一致 |
| 桌面远程（部署时） | `~/.cc-box/logs/` 有 `relay connected/registered` | 桌面已重连中继 |

注：Gitee Release 已于 2026-08 起不再创建（仓库同步保留），无需验收。

全部通过 → 发布成功。

任一失败 → 排查并手动修复。
