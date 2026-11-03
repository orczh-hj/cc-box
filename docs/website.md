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
- 路由 `#/<相对路径>`（非 `/` 开头 hash 视为页内锚点）；正文中 code.claude.com 文档链接与英文 slug 锚点均拦截为站内导航（`resolveAnchor` 按链接文字↔标题模糊兜底）；链接解析在 `lib/internalLinks.ts` 纯函数（**slug 允许多段嵌套路径**如 `plugins/mods/interface.md`，单段正则会漏拦截致跳外链，测试 tests/website/internalLinks.test.ts）

## 全文搜索

三页共用 `SiteHeader` 内的 `SearchBox`（**压缩语料 + 浏览器内暴力扫描，无倒排索引**）：

- **构建期**（`website/vite.config.mts` 的 `ccDocsSearchPlugin`）：读全部语料（`claude-code-docs/**/*.md` + `cc-box.md` + `claude-code.md`），`stripForSearch` 剥 code fence 与表格行，拼成单文本（每篇一行 `@@@CCBOXDOC ty=… key=… title=…` 分隔头）；产物 `assets/search-corpus-<hash>.txt`（`emitFile`，构建在 `buildStart` 无条件产出），doc 更新 → hash 变 → 旧缓存自然失效
- **虚拟模块** `virtual:cc-docs-search` 只下发 `corpusUrl`；dev 由 `configureServer` 中间件返回同一纯函数生成的未压缩语料（`/__search-corpus.txt`），保证 dev/prod 行为一致
- **运行时**（`lib/search.ts`，纯函数）：首次搜索才 fetch 语料（模块级 Promise 缓存）→ `parseCorpus` 切文档/章节 → `searchCorpus` 按空白拆词、**AND 语义**、标题命中优先排序、前 12 条 → `makeSnippet` 截命中前后 ~40 字并以 `«»` 标记（前端渲染 `<mark>`）
- **跳转**：reference → `reference.html#/<key>?a=<encodeURIComponent(章节标题)>`（ReferenceApp 用文字兜底 `resolveAnchor` 定位，构建期不算锚点 id）；docs → `docs.html#<slug(标题)>`（与 `lib/markdown.ts` h2 id 规则一致）
- **缓存语义**：内容 hash 文件名 + `deploy:site` 上传时预 gzip 并设 `Content-Encoding: gzip` / `Cache-Control: max-age=31536000, immutable`；语料文件单独上传，批量上传以 `--exclude "search-corpus-*.txt"` 防止未压缩版覆盖；HEAD 校验含语料 URL 与其 gzip 头（另见 `--clean-assets` 随 assets 目录整体替换）
- **已知限制**：无模糊/拼写容错；code block 与表格内容不可搜；docs.html 两篇文档 h2 撞名时只落第一篇

## 手机适配

- **SiteHeader 断点**：`>1024px` 常驻搜索框（nav 左侧 240px）；`≤1024px` 搜索收成图标 + header 下方浮层（header 根节点 `@click.stop` + document 监听实现点击外部收起）；`≤768px` 六链接收进汉堡菜单（含搜索框，链接点击后收起）
- **docs.html 目录**：`≤900px` 不再 `display:none`，改为「本页目录」可折叠块（默认收起，复刻 reference `ref-nav-toggle` 模式；`>900px` 侧栏 sticky 常驻）
- 触屏适配与工具条规则见 CLAUDE.md「响应式适配」

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

纯逻辑在 `website/src/lib/`：`ua/format/latest`（浏览器/格式化/版本）、`refTree/toc`（参考文档树与目录）、`internalLinks`（文档链接拦截）、`markdown`（渲染与 h2 锚点）、`search`（语料切分/扫描/排序/跳转 URL）、`refHash`（`#/key?a=` 路由 hash 解析）；测试在 `tests/website/`（`npx vitest run tests/website/`）。
