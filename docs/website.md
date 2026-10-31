# 产品官网正本（website/）

> CLAUDE.md「产品官网（速查）」的实现细节正本；速查只留命令与红线。

## 定位与部署

- **官网 = `website/`**：Vue 3 独立 Vite 站点，复用根 node_modules，与主应用 `src/` 零共享、仅浅色主题
- 部署在 OSS bucket `cc-box` 根路径，**对外地址 `https://ccbox.orczh.top/`**（自定义域名，OSS 托管 DigiCert 免费证书；OSS 默认域名对网页文件强制 `Content-Disposition: attachment` 下载，仅可作资源直链如安装包下载）
- 多入口：`index.html`（主页）+ `docs.html`（使用文档）+ `reference.html`（Claude Code 参考文档），bundle 名为 `main-*`/`docs-*`/`reference-*`

## 文档页（docs.html）

- 内容存 `website/content/*.md`（claude-code.md / cc-box.md，客观清单风格、自包含不外链），经 marked 运行时渲染（`lib/markdown.ts`：h2 生成锚点 id + 提取 TOC，DocsApp 双栏布局 + scrollspy）；客户端渲染导致 hash 定位需挂载后补跳（DocsApp onMounted）

## 参考文档页（reference.html）

分层浏览 `website/content/claude-code-docs/` 全部中文官方文档：

- **数据源由 `npm run sync:cc-docs`（`scripts/update-cc-docs.mjs`）本地更新**：拉 llms.txt → 并发下载（默认 8，`--concurrency` 可调）zh-CN md → 按官方索引顺序写 order.json；默认完整更新并清理官方已下线旧文件，`--incremental` 补充模式只下载缺失不删除；需代理，默认 `http://127.0.0.1:33210`，可用 `--proxy` 覆盖；**只更新本地，不部署**，上线走 `npm run deploy:site`；228 篇 ≈15MB
- vite 虚拟模块 `virtual:cc-docs-manifest`（vite.config.mts 内插件扫描目录，标题取首个 h1）下发扁平清单；`lib/refTree.ts`（纯函数，测试 tests/website/refTree.test.ts）构建三层树（类→组→文档，中文标签映射 + kebab 兜底）
- **排序以 `claude-code-docs/order.json` 为准**（插件 `applyDocOrder` 应用，类/组/文档三层按首次出现顺序，无 order.json 回退字母序）
- 正文用 `import.meta.glob('../content/claude-code-docs/**/*.md', {query:'?raw'})` **每篇一个懒加载 chunk**（勿全量内联）；渲染前 `stripLeadingBlockquote` 剥掉每篇开头的 `Documentation Index` 引用块
- 路由 `#/<相对路径>`（非 `/` 开头 hash 视为页内锚点）；正文中 code.claude.com 文档链接与英文 slug 锚点均拦截为站内导航（`resolveAnchor` 按链接文字↔标题模糊兜底）

## SEO 与版本号

- 三个页面 head 已含 keywords/canonical/Open Graph/Twitter（主页另有 JSON-LD SoftwareApplication），`public/robots.txt` + `sitemap.xml` 随 dist 根上传并由 deploy-site 校验；**改入口或域名需同步 sitemap 与 canonical**
- 版本号/下载链接**动态 fetch `/cc-box/latest.json`**（同 bucket 同源无 CORS；本地 dev 走 vite proxy，port 4173）——发版后官网自动更新，无需重新部署

## 命令

```bash
npm run dev:site        # 本地开发（port 4173）
npm run build:site      # 构建
npm run deploy:site     # 构建 → ossutil 上传 assets 先、index.html 后 → HEAD 校验（--skip-build / --clean-assets 可选）
npm run sync:cc-docs    # 更新参考文档数据源（只本地，不部署）
```

- `--clean-assets`：上传前清 `assets/`（完整镜像替换、自动清理旧 bundle）；**勿在上传后手动 rm 旧产物**
- 前端（主应用 src/）改动发布只需 `npm run deploy:web`（见 CLAUDE.md「版本发布」），与 deploy:site 是两条独立链路

## 安全红线

- bucket 根下有更新通道 `cc-box/` 与依赖 `deps/`，**绝不 `rm -r oss://cc-box/`**

## 证书自动续签

DigiCert 免费证书（3 个月）由 ortab 服务器 `update_ssl.py` 每天 3 点自动续签（剩 7 天触发、复核过期时间刷新、邮件通知）→ [docs/release-process.md](release-process.md)「产品官网」

## 纯逻辑与测试

纯逻辑在 `website/src/lib/`（ua/format/latest），测试在 `tests/website/`。
