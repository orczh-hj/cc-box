# 官网手机适配 + 全文搜索 设计方案

日期：2026-11-03。范围：`website/`（独立 Vite 站点，三个入口 index/docs/reference.html）。

## 意图

1. 官网在手机宽度（375px 及以下）存在明显挤压，需要系统修复。
2. 228 篇 Claude Code 中文文档 + 2 篇使用文档没有任何搜索入口，需要全文搜索（用户定位：**用户靠关键词搜索**，召回完整性优先于模糊匹配）。

## 调研结论（现状）

- 三页均无页面级横向溢出（表格/代码块已有横向滚动包装）。
- **主要挤压点 = 顶部导航**：≤360px 时 brand 与 6 个 nav 链接零间距贴死，更窄必溢出；无汉堡菜单。
- **次要点 = docs.html 目录**：≤900px 时 DocsToc 直接 `display:none`，手机上两篇长文档失去全部导航。
- reference.html 侧边栏已有「展开目录」toggle，可用。
- 语料实测：228 篇 md，剥 code fence/表格后 4.74MB；**gzip 2.4MB / brotli 1.65MB**；6 关键词全量 `indexOf` 扫描 **8ms**。

## 方案 A：手机适配

1. **SiteHeader 汉堡菜单**（三页共用）：≤768px 隐藏横向 nav，显示汉堡按钮，点击展开垂直菜单面板（含全部 6 个链接与搜索入口，点击跳转后自动收起）；桌面端不变。纯 CSS 断点 + 组件内状态，不引入 useIsMobile。
2. **docs.html 手机目录**：≤900px 不再隐藏 DocsToc，改为内容区上方的「本页目录」可折叠块（默认收起，对齐 reference 页 `ref-nav-toggle` 模式）。
3. **搜索框响应式**（与方案 B 联动）：>1024px header 内常驻输入框；≤1024px 收成图标按钮，点击弹出搜索浮层（与结果面板同一套 UI），避免在 769–1000px 平板宽度重新制造导航挤压。
4. 修完用 Playwright 375px/320px 复测三页无溢出、导航可用。

非目标：深色模式、首页各区块视觉重设计。

## 方案 B：全文搜索（压缩语料 + 客户端暴力扫描，无倒排索引）

曾评估 MiniSearch 截断索引（每章节前 N 字），因「关键词可能出现在章节任意位置，截断必漏召回」被否。选定方案：**构建期把语料压成一个文本文件下发，浏览器内全文扫描**。零依赖、召回完整、体积比索引更小。

### 构建期（vite 插件）

- 新插件在 `website/vite.config.mts`：读取全部语料（`content/claude-code-docs/**/*.md` + `content/cc-box.md` + `content/claude-code.md`），剥 code fence 与表格行，按文档拼接为单一大文本。
- **语料格式**：每篇文档前插一行机器可读分隔头（含 `ty: reference|docs`、文档 key、文档标题），正文按原文（含各级标题行）。
- **产物**：`assets/search-corpus-<contenthash>.txt`（`emitFile` 带哈希文件名）；同时虚拟模块 `virtual:cc-docs-search` 只下发极小元数据（corpus URL），进入主 bundle。文档更新 → hash 变 → 新 URL，旧缓存自然失效。
- **本地 dev**：插件 `configureServer` 中间件按相同逻辑生成并直接返回未压缩语料（保证 dev 与 prod 行为一致）。

### 部署与缓存

- `deploy:site`：对 `search-corpus-*.txt` 预 gzip + ossutil 设 `Content-Encoding: gzip` 与 `Cache-Control: max-age=31536000, immutable`（浏览器 fetch 自动解压，运行时代码零感知）。
- 缓存语义（用户要求）：URL 不变 → 浏览器 HTTP 缓存命中、二次搜索零下载；文档更新 → 新 hash URL 自动拉新；旧文件随 `deploy:site --clean-assets` 清理（语料放 `assets/` 下正好纳入该机制）。不额外使用 localStorage/Cache Storage。

### 运行时（website/src/lib/search.ts + SiteHeader 搜索 UI）

- **首次搜索才 fetch 语料**（懒加载），fetch + 客户端切分：扫描分隔头切出文档，再按 `## ` 切出章节 → 偏移表 `{ty, key, docTitle, sectionTitle, start, end}`（纯函数，可测）。
- **查询**：按空白拆关键词，每词 `indexOf` 全量扫描；结果 = 含全部关键词的章节（AND），排序：章节标题命中 > 文档标题命中 > 正文命中次数，最多 12 条。
- **摘要**：现场从语料截取首个命中位置前后 ~40 字，高亮关键词。
- **跳转**：`ty=reference` → `reference.html#/<key>?a=<编码章节标题>`；`ty=docs` → `docs.html#<锚点>`。
- 150ms 防抖，≥2 字符触发；无结果时提示换关键词。

### 已知限制（接受）

- 无模糊/拼写容错（中文关键词场景不需要）。
- docs.html 两篇文档 h2 撞名时 `getElementById` 只落第一篇（概率低，不为它改共享渲染器）。
- code block 与表格内容不可搜（它们本身不是关键词目标，且是体积大头）。

## Builder 检查结论（已定案）

1. **跨页锚点传输**：扩展 hash 为 `#/<key>?a=<编码锚点>`，ReferenceApp 路由解析剥离 `?a=` 段，跳转后走已有 `resolveAnchor` 文字兜底——搜索传章节标题即可，**构建期不需要算锚点 id**（语料方案天然规避 id 同源问题）。
2. **docs.html 锚点撞名**：首个命中，记为已知限制。
3. **平板宽度搜索框**：≤1024px 收成图标 + 浮层。

## 测试

- `tests/website/`：语料切分纯函数（文档头解析、章节切分、边界）、搜索排序（标题命中优先、AND 语义、命中计数）、摘要截取与高亮位置、跳转 URL 构造（reference 含 `?a=`、docs 两类）、ReferenceApp hash 解析（`#/<key>?a=` 剥离）。
- 手机适配纯 CSS，不写自动测试；Playwright 375/320px 视口复测三页。

## 部署

- 前端与 vite 配置 + deploy 脚本改动，无 OSS 控制台配置变更（ossutil 参数即达）。
- 文档同步：`docs/website.md` 补「搜索」「手机适配」小节。

## 决策记录

- 否决 Pagefind：正文客户端渲染，dist HTML 无内容。
- 否决 MiniSearch 截断索引：关键词搜索要求全章节召回，截断与需求矛盾；且压缩语料（2.4MB gz）比索引（2–4MB）更小、零依赖、扫描仅 8ms。
- 缓存走内容 hash 文件名 + HTTP immutable，不引入存储型缓存。
