# 官网手机适配 + 全文搜索 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 官网三页适配手机宽度（汉堡菜单 + docs 目录 + 搜索框响应式），并新增基于「压缩语料 + 客户端暴力扫描」的全文搜索。

**Architecture:** 构建期 vite 插件把 228+2 篇文档剥 code/表格后拼成语料文件（内容 hash 文件名）；运行时首次搜索才 fetch，浏览器内按分隔头/`## ` 切分后 `indexOf` 扫描。搜索 UI 在 SiteHeader，三页共用，平板/手机收成图标浮层。

**Tech Stack:** Vue 3 + Vite 插件（虚拟模块 + emitFile + dev 中间件）+ ossutil（gzip + immutable 缓存头）。零新增 npm 依赖。

**Spec:** `docs/superpowers/specs/2026-11-03-website-mobile-search-design.md`

## Global Constraints

- 官网与主应用 `src/` 零共享；纯函数放 `website/src/lib/`，测试放 `tests/website/`（Vitest）
- vite.config.mts 已有从 `./src/lib/refTree` import 纯函数的先例——构建期逻辑可复用 `src/lib` 中的纯函数
- 语料分隔头格式（锁定，构建与解析两端共用）：单行 `@@@CCBOXDOC ty=<reference|docs> key=<文档key> title=<文档标题>`，后跟正文
- 跳转 URL：reference 类 → `reference.html#/<key>?a=<encodeURIComponent(章节标题)>`；docs 类 → `docs.html#<slug(章节标题)>`（slug = trim + 空白转 `-`，与 `lib/markdown.ts` 的 h2 id 规则一致）
- 不引入 minisearch/其他依赖；不使用 localStorage/Cache Storage（缓存全靠内容 hash 文件名 + HTTP immutable）
- deploy 红线：bucket 根下 `cc-box/`、`deps/` 绝不 rm；语料文件放 `assets/` 下

## Review Focus

- **多关键词 AND 语义**：`MCP 配置` 应只返回同时含两词的章节，含一个词的不进结果 → Task 1 测试
- **关键词出现在章节后段**：不可截断语料，长章节尾部命中必须可搜到 → Task 1 测试（构造 >500 字章节，关键词仅在末尾）
- **语料头与正文撞车**：正文出现 `@@@CCBOXDOC` 字样不能切出新文档（只认行首且带完整 ty/key/title 结构）→ Task 1 测试
- **`?a=` 与现有路由互斥**：`#/<key>?a=x` 不能把 `?a=x` 混进 key（现有 `key.replace(/\/+$/,'')` 不处理 `?`）；纯锚点 hash（非 `/` 开头）不受影响 → Task 3 测试
- **dev/prod 语料一致性**：dev 中间件与 build emitFile 必须产出逐字节相同的语料（同一纯函数生成）→ Task 2 以「两端调用同一 `buildCorpusText`」保证，Task 6 手测 dev 搜索

---

### Task 1: 搜索纯函数库 `website/src/lib/search.ts`

**Files:**
- Create: `website/src/lib/search.ts`
- Test: `tests/website/search.test.ts`

**Interfaces:**
- Produces（Task 2/4 依赖，签名锁定）:
  - `interface CorpusDoc { ty: 'reference' | 'docs'; key: string; title: string; body: string }`
  - `interface CorpusSection { ty: 'reference' | 'docs'; key: string; docTitle: string; sectionTitle: string; start: number; end: number }`
  - `interface SearchHit { ty: 'reference' | 'docs'; key: string; docTitle: string; sectionTitle: string; snippet: string }`
  - `stripForSearch(md: string): string` — 剥 code fence 与表格行（与 spike 相同的两条 replace）
  - `buildCorpusText(docs: CorpusDoc[]): string` — 拼接：每篇 `@@@CCBOXDOC ty=… key=… title=…` 头行 + `\n` + body，篇间 `\n\n`
  - `parseCorpus(text: string): { corpus: string; sections: CorpusSection[] }` — 扫头行切文档、按行首 `## ` 切章节，返回原文与带偏移的章节表（首个章节 = 文档头行后到第一个 `## `，sectionTitle 取文档标题）
  - `searchCorpus(corpus: string, sections: CorpusSection[], query: string): SearchHit[]` — 空白拆词（≤0 词返回 []），AND 语义，排序：章节标题包含任一关键词 > 文档标题包含 > 正文命中次数，截前 12 条
  - `makeSnippet(corpus: string, start: number, end: number, keywords: string[]): string` — 首个命中位置前后 ~40 字，关键词替换为 `«»` 包裹（前端高亮渲染用）
  - `buildJumpUrl(hit: Pick<SearchHit,'ty'|'key'|'sectionTitle'>): string` — 按 Global Constraints 的 URL 规则

- [ ] **Step 1: 写失败测试** `tests/website/search.test.ts`

用例（中文注释，命名 `Feature_SeqNum`）：
- `stripForSearch` 剥 ``` 围栏与 `|a|b|` 表格行、保留正文文字
- `buildCorpusText`/`parseCorpus` round-trip：2 篇文档（一篇 reference 一篇 docs）→ 解析回相同篇数、ty/key/title 正确、偏移量落在对应文本上
- `parseCorpus` 只认结构完整的头行：正文里出现 `@@@CCBOXDOC ty=…` 但缺 key/title 的行不切分；行中（非行首）出现不切分
- `searchCorpus` AND 语义：章节只含「MCP」不含「配置」时，查 `MCP 配置` 不返回该章节；两词都含时返回
- 长章节尾部命中：构造 600 字章节、关键词仅出现在第 550 字，能命中（回归「截断索引」错误方案）
- 排序：章节标题命中排在仅正文命中的前面
- `makeSnippet`：命中位置前后各 ~40 字、关键词被 `«»` 包裹、命中在开头时起点钳到 0
- `buildJumpUrl`：reference → `reference.html#/getting-started/overview?a=%E5%AE%89%E8%A3%85`；docs → `docs.html#快速开始`（空白转 `-`）

- [ ] **Step 2: 跑测试确认失败**

Run: `npx vitest run tests/website/search.test.ts`
Expected: FAIL（模块不存在）

- [ ] **Step 3: 实现 `website/src/lib/search.ts`**

全部纯函数、无浏览器/Node API。章节切分注意：`## ` 匹配 `/^## /`（不匹配 `###`）；sectionTitle 取该行 `## ` 之后原文 trim。

- [ ] **Step 4: 跑测试确认通过**

Run: `npx vitest run tests/website/search.test.ts`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add website/src/lib/search.ts tests/website/search.test.ts
git commit -m "feat(website): 搜索纯函数库（语料切分/扫描/排序/跳转 URL）"
```

---

### Task 2: vite 插件——语料构建 + 虚拟模块 + dev 中间件

**Files:**
- Modify: `website/vite.config.mts`（新增 `ccDocsSearchPlugin`，注册进 plugins）
- Modify: `website/src/env.d.ts`（补 `virtual:cc-docs-search` 声明）

**Interfaces:**
- Consumes: Task 1 的 `stripForSearch`、`buildCorpusText`、`CorpusDoc`
- Produces: `import { corpusUrl } from 'virtual:cc-docs-search'`（Task 4 使用）
  - build：`corpusUrl = '/assets/search-corpus-<sha256前12位>.txt'`（插件 `emitFile({type:'asset', fileName 固定含 hash, source})`）
  - dev：`corpusUrl = '/__search-corpus.txt'`，`configureServer` 中间件返回未压缩语料（内存缓存，首次生成）
- 语料采集：`content/claude-code-docs/**/*.md`（复用 `scanDocsManifest` 的遍历取 key/title，ty=reference；文件读原文传 `stripForSearch`）+ `content/cc-box.md`（ty=docs, key='cc-box'）+ `content/claude-code.md`（ty=docs, key='claude-code'），docs 两篇标题取各自首个 h1（无则用 key）
- `emitFile` 只能在 build 调用；dev/build 分支用 `configResolved` 记录 `config.command === 'serve'` 判断

- [ ] **Step 1: 实现 `ccDocsSearchPlugin` 并注册**

按上述 Interfaces 实现；虚拟模块 resolveId/load 模式复制现有 `ccDocsManifestPlugin`。虚拟模块代码：`export const corpusUrl = ${JSON.stringify(url)}`。

- [ ] **Step 2: 验证 dev**

Run: `npm run dev:site`（后台），`curl -s http://localhost:4173/__search-corpus.txt | head -c 300`
Expected: 输出以 `@@@CCBOXDOC ty=reference` 开头的语料；`curl … | wc -c` 约 4.7MB 量级

- [ ] **Step 3: 验证 build 产物**

Run: `npm run build:site && ls website/dist/assets/ | grep search-corpus`
Expected: 恰好一个 `search-corpus-<hash>.txt`；`grep -c '@@@CCBOXDOC' <该文件>` = 230

- [ ] **Step 4: Commit**

```bash
git add website/vite.config.mts website/src/env.d.ts
git commit -m "feat(website): 构建期语料产物（emitFile 哈希文件名 + dev 中间件）"
```

---

### Task 3: ReferenceApp 支持 `#/<key>?a=<锚点>` 跳转

**Files:**
- Create: `website/src/lib/refHash.ts`
- Modify: `website/src/ReferenceApp.vue`（`syncFromHash`/`onHashChange` 改用解析函数；`?a=` 存在时在 `loadDoc` 完成后滚到锚点）
- Test: `tests/website/refHash.test.ts`

**Interfaces:**
- Produces: `parseRefHash(hash: string): { key: string; anchor?: string }`——入参为 `location.hash`（含或不含前导 `#`）；非 `/` 开头返回 `{ key: '' }`（页内锚点，维持现有行为）；`key` 剥 `?` 及之后与尾部 `/`
- ReferenceApp 行为：`?a=` 值 `decodeURIComponent` 后作为 `pendingAnchor = { id: <值>, text: <值> }`（id/text 同值——`resolveAnchor` 的文字兜底正好吃 text）

- [ ] **Step 1: 写失败测试**：`#/a/b?x` → key=`a/b`；`#/a/b?a=%E5%AE%89%E8%A3%85` → key=`a/b`, anchor=`安装`；`#安装` → `{key:''}`；`#/a/` → key=`a`

- [ ] **Step 2: 跑测试确认失败**（Run: `npx vitest run tests/website/refHash.test.ts`，Expected: FAIL）

- [ ] **Step 3: 实现 `refHash.ts` + 改造 ReferenceApp 两处 hash 入口**（`syncFromHash` 用 `parseRefHash`；`onHashChange` 路由分支同改，`?a=` 时设置 pendingAnchor 再走现有 `watch → loadDoc → scrollToAnchor` 链路；同 key 仅锚点变化时直接 `scrollToAnchor`）

- [ ] **Step 4: 跑测试确认通过 + 手动验证**

Run: `npx vitest run tests/website/refHash.test.ts` → PASS
浏览器（dev）访问 `http://localhost:4173/reference.html#/getting-started/overview` 后改 hash 加 `?a=安装`，确认滚动到「安装」章节（resolveAnchor 文字兜底生效）

- [ ] **Step 5: Commit**

```bash
git add website/src/lib/refHash.ts tests/website/refHash.test.ts website/src/ReferenceApp.vue
git commit -m "feat(website): reference 路由支持 ?a= 章节锚点（文字兜底跳转）"
```

---

### Task 4: SearchBox 组件 + SiteHeader 集成（含手机汉堡菜单）

**Files:**
- Create: `website/src/components/SearchBox.vue`
- Modify: `website/src/components/SiteHeader.vue`

**Interfaces:**
- Consumes: Task 1 `parseCorpus`/`searchCorpus`/`makeSnippet`/`buildJumpUrl`、Task 2 `corpusUrl`
- SearchBox.vue：`<SearchBox />` 无 props 无 emits，自带 input + 结果面板 + 键盘上下/Enter/Escape；内部状态：首次输入触发 `fetch(corpusUrl).then(r=>r.text())`（模块级 Promise 缓存，三页不共享但同页多次开合只 fetch 一次）；选中/点击结果 → `location.href = buildJumpUrl(hit)`；`makeSnippet` 的 `«»` 渲染为 `<mark>`
- SiteHeader 断点（锁定）：
  - `>1024px`：SearchBox 常驻 header（nav 左侧，宽 ~240px）
  - `≤1024px`：搜索图标按钮，点击在 header 下方展开浮层（`position:absolute` 面板含 SearchBox，点击页面其他处收起）
  - `≤768px`：nav 六链接隐藏，汉堡按钮toggle 垂直菜单面板（含 6 链接 + SearchBox），链接点击后收起
- 加载态：首次 fetch 中显示「加载搜索索引…」；fetch 失败显示「搜索暂不可用」可重试
- 输入 <2 字符（或英文 <2 字符）清空结果

- [ ] **Step 1: 实现 SearchBox.vue**（样式对齐现有浅色主题变量 `--bg-*`/`--border-light`/`--accent-primary`）

- [ ] **Step 2: 改造 SiteHeader.vue**（`ref` 状态 `menuOpen`/`searchOpen`；现有 6 链接与 brand 不动；模板加按钮与面板；样式按断点锁定值）

- [ ] **Step 3: Playwright 手动验证（dev）**

- 桌面 1280px：header 搜索框输入 `权限`，结果 ≤12 条、摘要含 `<mark>`；点 reference 类结果跳到对应文档并滚到章节；点 docs 类结果跳 docs.html 对应锚点
- 输入 `MCP 配置` 验证 AND；输入单字 `权` 无结果触发
- 375px：汉堡开合、菜单内搜索可用；320px 无横向溢出
- 800px（平板）：搜索图标浮层可用、nav 不溢出

- [ ] **Step 4: 跑全量前端 website 测试确认无回归**

Run: `npx vitest run tests/website/`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add website/src/components/SearchBox.vue website/src/components/SiteHeader.vue
git commit -m "feat(website): 站内全文搜索 UI（桌面常驻/平板浮层/手机入菜单）"
```

---

### Task 5: docs.html 手机目录折叠

**Files:**
- Modify: `website/src/DocsApp.vue`

**Interfaces:**
- Consumes: 无新接口；复用现有 `DocsToc` 组件
- 行为：`≤900px` 时 `.doc-layout > nav` 不再 `display:none`，改为「本页目录」toggle 按钮（默认收起，样式复制 ReferenceApp 的 `ref-nav-toggle` CSS）；`>900px` 桌面行为不变（sidebar 常驻、无按钮）

- [ ] **Step 1: 改造 DocsApp.vue**（加 `tocOpen` ref + toggle 按钮；CSS：≤900px nav 隐藏/展开由 `tocOpen` 类控制）

- [ ] **Step 2: Playwright 验证** 375px：按钮显示、点击展开目录、点目录项滚动正常且目录可保持展开；1280px：无按钮、侧边栏照旧

- [ ] **Step 3: Commit**

```bash
git add website/src/DocsApp.vue
git commit -m "feat(website): 文档页手机目录折叠（替代直接隐藏）"
```

---

### Task 6: deploy 脚本 gzip + 缓存头 + 文档同步 + 收尾验证

**Files:**
- Modify: `scripts/deploy-site.mjs`
- Modify: `docs/website.md`

**Interfaces:**
- deploy-site 改动（锁定）：
  - 上传前检测 `website/dist/assets/search-corpus-*.txt`（缺失则报错退出——主 bundle 已引用，漏传=线上搜索必坏）
  - `node:zlib` 预 gzip → 临时文件 `<原名>.gz`
  - 单独上传：`ossutil cp <gz> oss://<bucket>/assets/<原名>(不带.gz) -f --meta "Content-Encoding:gzip#Cache-Control:max-age=31536000,immutable"`
  - 批量上传加 `--exclude "search-corpus-*.txt"`（防未压缩版覆盖）
  - HEAD 校验列表加语料 URL
- docs/website.md：新增「全文搜索」（架构：语料/虚拟模块/dev 中间件/缓存语义/已知限制）与「手机适配」小节，并更新「纯逻辑与测试」清单补 search.ts/refHash.ts

- [ ] **Step 1: 改 deploy-site.mjs**（按上述锁定行为；日志沿用 logStep/logOk 风格）

- [ ] **Step 2: 全量验证**

Run: `npm run build:site && npx vitest run tests/website/`
Expected: 构建产物含语料文件；测试全 PASS

- [ ] **Step 3: Playwright 用 build 产物（`npx vite preview --config website/vite.config.mts`）复测** 搜索全链路 + 375/320px 三页无溢出（preview 不经 gzip 中间件，验证未压缩路径；gzip 路径待真实部署后 `curl -I https://ccbox.orczh.top/assets/search-corpus-*.txt` 确认 Content-Encoding，不在本任务内）

- [ ] **Step 4: 更新 docs/website.md**

- [ ] **Step 5: Commit**

```bash
git add scripts/deploy-site.mjs docs/website.md
git commit -m "feat(website): 部署语料 gzip+immutable 缓存头；文档同步"
```
