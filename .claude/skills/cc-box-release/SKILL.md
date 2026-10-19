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

桌面版本发布 ≠ 全部生效。按本次改动追加：

| 本次改动 | relay 部署 |
|---|---|
| 前端（src/）/ relay（remote/relay/）/ 协议（remote/protocol/） | **必须**（手机 UI 是 relay 编译时嵌入的 dist） |
| 仅桌面功能 / 仅桌面 Rust（src-tauri/） | 不需要 |

**顺序：先桌面版本，后 relay 部署**（新桌面向后兼容旧 UI，反序会出现新 UI 调用旧桌面 rpc 白名单外的命令）。

relay 部署命令（完整版见 remote/docs/deploy.md §3.5）：`npm run build` → musl 交叉编译 → scp + systemctl restart → 三查（md5 / bundle hash / 特征字符串）。

relay 重启 = 内存态清空：**所有手机需重新扫码配对**（桌面面板自动出新码）。

## 验收标准

发布完成后，验证以下项：

| 项目 | 检查方式 | 预期 |
|------|----------|------|
| GitHub Release | `gh release view v<x.y.z>` | 存在，非 draft，有 3 个资产 |
| Release Notes | `gh release view` | 格式正确，非字面 `\n`，动词开头，内容与 diff 匹配 |
| OSS latest.json | curl `https://<bucket>/cc-box/latest.json` | version 字段正确 |
| OSS 安装包 | curl HEAD 检查 | 3 个文件均返回 200 |
| 本地版本号 | `git describe --tags` | 与发布版本一致 |
| relay（部署时） | md5 比对 + bundle hash + 特征字符串 | 新二进制已上线 |
| 桌面远程（部署时） | `~/.cc-box/logs/` 有 `relay connected/registered` | 桌面已重连中继 |

注：Gitee Release 已于 2026-08 起不再创建（仓库同步保留），无需验收。

全部通过 → 发布成功。

任一失败 → 排查并手动修复。
