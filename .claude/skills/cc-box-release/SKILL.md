---
name: cc-box-release
description: CC-Box 发布（桌面安装包 / WebUI / relay / 官网 / Claude Code 参考文档）。当用户说"发布"、"release"、"版本更新"、"部署 webui"、"部署 relay"、"部署官网"、"更新文档"时使用此 skill。
---

# CC-Box 发布

## 先判断：这次要发什么

CC-Box 有 **5 个相互独立的发布对象**。桌面版本发布 ≠ 其他生效；不同改动只需要发对应部分。
先读 `git log` / `git diff --stat` 与用户意图，按下表决定动作，再逐项执行。

| # | 发布对象 | 内容 | 命令 | 什么时候需要 |
|---|---|---|---|---|
| 1 | **桌面程序包** | 各平台安装包 + `latest.json`（GitHub Release + OSS） | `npm run release -- --bump patch --notes "..."` | 任何 `src-tauri/`、`src/`（桌面内嵌 UI）、版本号、发布说明改动 |
| 2 | **WebUI** | 手机/浏览器页面（生产 relay 磁盘目录 `/opt/cc-box/web`） | `npm run deploy:web` | `src/` 前端改动，且希望手机端立刻看到新 UI |
| 3 | **relay 二进制** | 服务端（认证/配对/透传） | `npm run deploy:relay` | `remote/relay/`、`remote/web-assets/` 服务逻辑改动 |
| 4 | **官网** | `website/`（ccbox.orczh.top） | `npm run deploy:site` | `website/` 改动，或参考文档数据更新后需要上线 |
| 5 | **Claude Code 参考文档** | `website/content/claude-code-docs/` 数据源 | `npm run sync:cc-docs`（只更新本地） | 需要同步最新官方文档时；上线还要跟 #4 |

**常见组合**

| 本次改动 | 需要发布 |
|---|---|
| 只改桌面 Rust（`src-tauri/`） | 仅 #1 |
| 改前端（`src/`），用户希望手机端也更新 | #1（桌面内嵌 UI）+ #2（手机/浏览器） |
| 改 relay 服务逻辑 | #1（可选，若同时带桌面修复）+ #3 |
| 改协议（`remote/protocol/`） | #1 + #2（relay 只透传，不用重编） |
| 改官网 | #4 |
| 更新 Claude Code 文档数据 | #5，再 #4 上线 |

**顺序规则（涉及多个对象时）**

1. **先桌面 #1，后 WebUI #2 / relay #3**：新桌面向后兼容旧 UI；反序会出现新 UI 调用旧桌面 `rpc_dispatch` 白名单外的命令
2. 官网与参考文档互不依赖桌面版本，可随时独立发布

---

## 1. 桌面程序包

### 发布前检查（必做）

1. **测试全绿**：`cd src-tauri && cargo test` + `npm test`（`remote/web-assets` 改动时另跑 `cd remote/web-assets && cargo test`）
2. **协议改动**：确认四处同步（crate 定义 / protocol.ts / samples / remote/docs/protocol.md）
3. **敏感文件与无关改动**：`git status` 不应有 oss-config.json、截图、日志、本地测试安装包（`*-setup-local.exe` 已在 .gitignore）。**release.js 提交是 `git add -A`**——无关改动会一并进 Release commit，发现时先单独提交或暂存
4. **版本号一致性**：`package.json` 与 `src-tauri/tauri.conf.json` 由脚本统一 bump；手工本地试构建不要只改其中一个（`APP_VERSION` 取 `package.json`，包版本取 `tauri.conf.json`，二者不一致会让更新标记 `fromVersion` 判等失效）

### 标准流程

```bash
npm run release -- --bump <level> --notes "<notes>"
```

自动更新版本号 → CHANGELOG → 提交推送 → 标签 → CI → GitHub Release → OSS（产物 + `latest.json`）。

- **断点续跑**：任何一步失败 → **重跑与首次完全相同的命令**（含 `--bump` 与 `--notes`），脚本幂等并自动锁定在途版本。不要改成 `--exact`
- **网络域**：GitHub 走代理（默认 `127.0.0.1:33210`，`CC_BOX_PROXY` 可覆盖），Gitee/OSS 强制直连。代理不可用但 GitHub 直连正常时，用 `NO_PROXY="github.com,githubusercontent.com,objects.githubusercontent.com"` 前缀直连续跑（`release.js` 只覆盖 `*_PROXY`，不碰 `NO_PROXY`）
- **分仓库推送**：GitHub 失败阻断；Gitee 失败仅告警（GitHub Actions **不会**自动同步 Gitee，需按提示补推）

### 特殊情况

| 场景 | 处理 |
|------|------|
| 脚本中断/失败 | 重跑同一命令（首选） |
| 仅补传 OSS | `npm run release -- --oss-only vX.Y.Z` |
| CI 已完成的重发 | `--exact --skip-ci` |
| 指定级别 | `--bump patch/minor/major`（**major/minor 仅在用户明确要求时使用**） |
| 没有变更 | 提示用户，中止 |
| 撤回某版本后重发下一版 | **先清 CHANGELOG 顶部段 + 三处 tag**，见「删除/回滚已发布版本」 |
| 产物下载失败（CDN `PROTOCOL_ERROR` / 代理不通） | 手动断点续传预下载到 `releases/vX.Y.Z/`，再 `--oss-only`（脚本按 size+digest 校验，完整即跳过下载） |

**产物下载失败时的补救**（代理长期不可用、CDN 间歇性中断）：

```bash
mkdir -p releases/vX.Y.Z && cd releases/vX.Y.Z
# 逐个文件断点续传循环，直连 + HTTP/1.1 比 HTTP/2 稳
for f in <资产名...>; do
  for i in $(seq 1 40); do
    curl -sSL --http1.1 -C - --max-time 120 --speed-time 20 --speed-limit 1024 \
      -o "$f" "https://github.com/orczh-hj/cc-box/releases/download/vX.Y.Z/$f" && break
    sleep 2
  done
done
```

资产名与大小用 `gh api repos/orczh-hj/cc-box/releases/tags/vX.Y.Z --jq '.assets[] | "\(.name) \(.size)"'` 对照。下全后回到仓库根执行 `npm run release -- --oss-only vX.Y.Z`。

### Notes 格式（**纯文本，不用 Markdown**）

设置页的更新说明**不渲染 Markdown**：只会把换行转成 `<br>`，并把行首的 `#` 当加粗。写 `### Features`、`- 列表` 会原样显示成 `### Features` 和连字符列表，很难看。

正确写法 = 一句标题行 + 「新增 / 修复 / 调整」分组词独占一行 + 每条一行：

```
本次更新

新增
新增自动更新：检测到新版本自动下载，无窗口运行时无人值守完成安装升级
托盘菜单新增「检查更新」；有新版本时显示「更新到 vX.Y.Z」，二次确认后直接更新

修复
修复更新完成后后台服务没有自动重新启动的问题

提示：手机/浏览器请刷新页面获取新前端。
```

- 每条以「新增 / 修复 / 调整 / 移除」开头（会自然读成一句话）
- **不用** `#` 标题、`-` / `*` 列表、`**加粗**`、代码块、链接语法
- 空行仅用于分组；中文全角标点

**传参必须用单行 + `\n` 转义**（`--notes '本次更新\n\n新增\n...'`）：多行字符串经 `npm run` 传参只会保留首行，其余行被丢弃——CHANGELOG 与 Release notes 会变成只有标题的空段。若已发生：补全 CHANGELOG 后 `git commit --amend --no-edit`（未推送时），再用正确的单行 notes 重跑即可，`updateChangelog` 会跳过已存在的段、只补用 Release notes。

### 已发布后修正 notes

Release 已上线但 notes 需要改（格式/错别字）：

```bash
# 1) 写纯文本到文件，改 GitHub Release
gh release edit vX.Y.Z --notes-file notes.txt
# 2) 同步 OSS latest.json（--oss-only 因幂等会跳过，需直接改字段再上传）
curl -s https://cc-box.oss-cn-beijing.aliyuncs.com/cc-box/latest.json -o latest.json
node -e "const fs=require('fs');const j=JSON.parse(fs.readFileSync('latest.json','utf8'));j.notes=fs.readFileSync('notes.txt','utf8').trim();fs.writeFileSync('latest.json',JSON.stringify(j,null,2))"
./ossutil64.exe cp latest.json "oss://cc-box/cc-box/latest.json" -f
```

`--oss-only` 的幂等判据只看 version + 三产物，**不会**发现 notes 变化。

### 删除/回滚已发布版本

需要撤回某个版本时（用户明确要求才做），四处都要清：

```bash
# GitHub Release + tag（--cleanup-tag 同时删远端 tag）
NO_PROXY="github.com,githubusercontent.com" gh release delete vX.Y.Z --repo orczh-hj/cc-box --yes --cleanup-tag
# Gitee tag
git push https://gitee.com/orczh/cc-box.git :refs/tags/vX.Y.Z
# 本地 tag
git tag -d vX.Y.Z
# OSS 产物与 latest.json（ossutil64.exe 在仓库根）
./ossutil64.exe rm "oss://cc-box/cc-box/vX.Y.Z/" -r -f
./ossutil64.exe rm "oss://cc-box/cc-box/latest.json" -f
```

删除 `latest.json` 期间更新通道不可用，应尽快重新发布以恢复。**安全红线**：只删 `cc-box/vX.Y.Z/` 前缀与 `latest.json`，绝不 `rm oss://cc-box/cc-box/` 或 `oss://cc-box/` 整体。

**撤回后立即发下一版：必须先清干净，否则会被锁回被删版本。** `resolveTargetVersion` 的断点锁定判据是「版本文件 == CHANGELOG 顶部版本 且该版本未完整发布」——撤回后版本文件仍是旧版本、CHANGELOG 顶部段还在，会被误判为「在途发布」并**重新打上被删版本的 tag**（实测：删 0.17.3 后 `--bump patch` 又发了 0.17.3）。正确顺序：

1. 删除 CHANGELOG 顶部的该版本段（或改成下一版应有的内容）
2. 删除本地 / GitHub / Gitee 三处 tag（`git tag -d` + 两条 `git push <url> :refs/tags/vX.Y.Z`）
3. 确认 `gh release list` 已无该版本
4. 再执行 `--bump patch`

若已经误锁并推出了 run：先 `gh run cancel`，再按上面 1–3 清理，然后重来。

---

## 2. WebUI（手机/浏览器前端）

relay 二进制**不内嵌 UI**；生产 UI 唯一来源是磁盘目录 `CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web`。因此前端改动免重编、免重启 relay。

```bash
npm run deploy:web            # 构建 → staging → md5 校验 → 原子切换 → 线上 bundle 校验
npm run deploy:web -- --skip-build
npm run deploy:web -- --keep-prev   # 保留上一版目录供回滚
```

- **不要**用 `scp -r dist/*` 直接覆盖：历史 `index-*.js` 会永久堆叠
- 部署后手机/浏览器**强刷**即生效；已配对设备无需重新扫码
- 校验：`curl -s https://orczh.top:8800/ | grep -o 'assets/index-[^"]*'` 与本地 `dist/assets/` 一致

---

## 3. relay 二进制

仅在 `remote/relay/` 或 `remote/web-assets/`（服务逻辑）改动时需要。

```bash
npm run deploy:relay
```

musl 交叉编译（**不构建前端、不依赖 dist**）→ ssh 预检（磁盘 Web 目录必配）→ scp `.new` + md5 → **切换前人工确认**（`--yes` 跳过）→ 重启 → 三查（md5 / 服务端 bundle hash / HTTP 特征）。

- 重启 = 内存态清空；**token v2 存于桌面，已配对手机用本地 token 自动重连，无需重新扫码**（仅「刷新密钥」后才需重新配对）
- 细节与故障处理 → `remote/docs/deploy.md` §3.5

---

## 4. 官网

`website/` 是独立的 Vue 3 + Vite 站点（与主应用零共享、仅浅色主题），部署在 OSS bucket `cc-box` 根路径，对外 `https://ccbox.orczh.top/`。

```bash
npm run dev:site      # 本地预览
npm run build:site    # 构建
npm run deploy:site                 # 部署
npm run deploy:site -- --skip-build
npm run deploy:site -- --clean-assets
```

- 版本号与下载链接**动态 fetch `/cc-box/latest.json`**：桌面发版后官网自动更新，**无需重新部署官网**
- 改入口页 / 域名时同步 sitemap 与 canonical
- **安全红线**：bucket 根下有更新通道 `cc-box/` 与依赖 `deps/`，绝不 `rm -r oss://cc-box/`；清旧 hash 产物只能用 `--clean-assets`
- 实现细节 → `docs/website.md`

---

## 5. Claude Code 参考文档

`website/content/claude-code-docs/` 是参考文档页的数据源。

```bash
npm run sync:cc-docs     # 拉取最新官方文档到本地（需要代理）
```

- **只更新本地，不自动部署**；要让线上生效，再执行 `npm run deploy:site`
- 同步后检查 `git status` 的变更范围，只提交文档数据本身

---

## 验收标准

按本次实际发布的对象逐项检查：

| 对象 | 检查方式 | 预期 |
|------|----------|------|
| 桌面：GitHub Release | `gh release view v<x.y.z>` | 存在、非 draft、资产齐全（3 平台安装包 + `.sig` + mac `.app.tar.gz`） |
| 桌面：Notes | `gh release view` | 非字面 `\n`，内容与改动匹配 |
| 桌面：Gitee | `git ls-remote https://gitee.com/orczh/cc-box.git refs/heads/main` | 与 GitHub main 一致 |
| 桌面：OSS | curl `https://cc-box.oss-cn-beijing.aliyuncs.com/cc-box/latest.json` | `version` 正确（无 `v` 前缀）；`v<x.y.z>/` 下三平台安装包 HEAD 200 |
| 桌面：本地 | `git describe --tags` | 与发布版本一致 |
| WebUI | curl relay 首页 bundle hash | 与本地 `dist/assets/` 一致 |
| relay | deploy:relay 自动三查 | 新二进制已上线 |
| 官网 | 打开 `https://ccbox.orczh.top/` | 页面正常、版本号/下载链接正确 |
| 文档 | 打开 `https://ccbox.orczh.top/reference.html` | 新内容已上线 |

注：Gitee Release 自 2026-08 起不再创建（仅同步代码），无需验收。

任一失败 → 桌面流程优先「重跑同一命令」续跑；其他对象各自重跑对应 deploy 命令（均幂等）。
