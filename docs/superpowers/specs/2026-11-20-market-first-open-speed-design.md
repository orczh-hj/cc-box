# 插件市场首次打开提速（方案一）设计

## 意图

首次打开插件市场（无安装、无缓存）约 30s 转圈，主因：浏览副本 `git clone --depth 1`（数秒~几十秒）+ `claude plugins list --json` 子进程（1~3s）+ OSS 推荐清单，三者**串行**。目标：冷路径 ≤3s，热路径（TTL 过期重开）<1s。

**核心认知（用户确认）**：未安装时浏览只需要 `marketplace.json` 清单，无需整个仓库；安装时由 Claude CLI 负责克隆。

## 改动

### 1. 浏览副本：git 源优先 raw JSON 直拉（store.rs `market_browse_local_copy`）

- Git 分支现路径「无缓存 → clone」改为「无缓存 → `marketplace_raw_urls(url)` 逐个 `http_get_market_json` → 命中即存 `src-<hash>.json`（与 JsonUrl 源同款缓存文件形态）」
- raw 全部候选 URL 失败 → **回退现有 clone 路径**（私有仓库/非标准 git 主机的兼容兜底，语义不收窄；clone 前清残留逻辑保留）
- 旧目录缓存（`src-<hash>/.claude-plugin/marketplace.json`）命中逻辑不变，自然过渡
- 刷新（`refresh_market_browse_copy`）同规则：优先 raw 重拉 json，失败回退 clone 换入
- raw 推导复用已有 `marketplace_raw_urls`（gitee/github 专用路径 + 通用 `/raw/` 兜底）

### 2. 三步并行（store.rs `load_plugin_marketplaces`）

`plugins_list_json_in` / `get_app_config + get_market_presets` / 市场清单装配三者独立，`std::thread::scope` 三路并行，总耗时 ≈ 最慢一路（plugins list）。市场间多源拉取暂不并行（冷路径只有内置 ccbox-market 一个未注册源）。

### 3. 转圈加文案（前端 MarketBrowser）

loading 状态显示「正在拉取市场清单…」（i18n zh/en）。

## 非目标

- 启动预热（预检查写浏览缓存）——落地后按体感决定
- 市场间并行拉取、前端渐进渲染

## 测试

- 复现「raw 优先」：注入 fake fetcher（参数化 `market_browse_local_copy` 的拉取函数），git 源 URL → 断言走 raw 且写 `src-<hash>.json`；raw 全失败 → 回退 clone（现有 file:// 测试即此路径）
- 并行化不改变结果：现有 `get_plugin_marketplaces_in` 测试覆盖装配逻辑不变

## 取舍（我的判断，可否决）

- raw 失败保留 clone 回退（而非直接报错）：浏览能力对私有源不收窄
- 缓存从目录改文件后，「更新于」取文件 mtime，展示语义不变
