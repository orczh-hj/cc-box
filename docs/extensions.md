# 扩展管理正本：编辑器、Hook 面板与 Plugin 市场

侧边栏对 Claude Code 五类资源（skills / agents / mcp / plugin / hooks）的管理能力：浏览 + 开关 + **表单化新增/编辑**（四类）+ **标准市场浏览安装**（plugin）。本文是行为正本；速查见 CLAUDE.md。

## 设计定位

- **CLI 优先**：所有能力都落在「CLI 做不好」的事上——skill/agent 无 CLI 创建命令、MCP `add` 参数难记、plugin 市场操作全靠斜杠命令。
- **来源分组统一命名**：五个面板（skills/agents/mcp/plugins/hooks）的作用域分组标题统一为 **个人 / 项目 / 本地 / 插件**（agents/mcp 用裸键 `user/project/local/plugin`，skills/plugins 用 `xxxSkills/xxxPlugins` 后缀键但值同为单词，hooks 用 `hookGroup_*`；英文 Personal/Project/Local/Plugins）。plugin 来源条目一律归「插件」组（只读，随插件启停；MCP 面板此前漏渲染 local 来源，已补）。
- **不建第二份状态**：编辑直接写 Claude Code 原生文件（`~/.claude.json`、`.mcp.json`、`skills/agents` 目录、`settings.json` hooks）；市场注册事实源是 Claude 自己的 `known_marketplaces.json`。
- **不做官方推荐**：不做 CC-Box 自有市场格式与内容运营；plugin 走标准原生 marketplace 协议。

## 编辑器（四类统一）

入口：面板头部「＋」（新建）、条目「编辑」按钮（悬停显示，触屏常显）。点击条目展开详情（既有行为保留）。

| 编辑器 | 表单字段 | 源码/JSON 模式 | 落盘 |
|---|---|---|---|
| `McpEditor.vue` | 名称(可改)/作用域/类型(stdio\|http\|sse)/命令+参数/环境变量/URL+Headers | server 配置 JSON（双向同步） | 三作用域，见下 |
| `SkillEditor.vue` | 名称(可改)/作用域/描述/正文 | SKILL.md 全文（校验 frontmatter 可解析 + name 一致） | `<scope skills>/<name>/SKILL.md` |
| `AgentEditor.vue` | 名称(可改)/作用域/描述/tools/model/正文 | md 全文 | `<scope agents>/<name>.md` |
| `HookEditor.vue` | 事件/作用域/类型(command\|http\|prompt\|agent\|mcp_tool)/匹配器/按类型字段/超时 | matcher 条目 JSON | settings.json `hooks.<event>` 合并 |

通用约定：

- **作用域**：skill/agent = user/project；mcp/hook = user/project/local，**编辑中可改**——保存时跨作用域移动（skill/agent 整目录 move + disabled 备份随迁；MCP 从旧作用域配置移除原 key；hook 从旧 settings 移除原条目，均不留 disabled 备份——条目是迁移不是删除）。删除/定位按打开时的原作用域（用户改选未保存不影响）。项目级写入前提示（`.mcp.json`/`.claude/` 会随 git 提交）。
- **原子写 + 跨进程锁**：JSON 走 `write_json_atomic`；锁文件集中放 `~/.cc-box/locks/<hash>.lock`（不在项目根落 `config.lock`，避免污染用户 git 工作区）。
- **可逆**：**删除一律彻底删**（四类资源，二次确认后直接移除并清理历史 disabled 备份，不可恢复）；可逆性由「开关」承担——禁用 = 移入 `~/.cc-box/disabled/`，启用移回。二次确认统一用前端 `uiConfirm`（`ConfirmDialog.vue`，App.vue 挂载一份），不用浏览器原生 confirm。
- **描述解析**：skill/agent 的 frontmatter description 支持普通单行、单/双引号与 YAML 块标量（`>-` 折叠 / `|` 字面量，可带 -/+），前后端各有一份同语义实现（`store.rs::parse_block_scalar` / `model.ts::parseBlockScalar`）；曾把 `description: >-` 误解析为字面量「>-」。
- **重命名**：skill 整目录 move（附属文件保留）、agent/md 文件 move、MCP 移除旧 key；与跨作用域移动可组合（一步到位 move）。
- **外部打开（可选入口）**：skill 与 agent 编辑器底部统一「打开目录」——skill 打开自身目录（`open_in_file_manager`），agent 在文件管理器中定位其 md（`reveal_in_file_manager`：Windows `explorer /select,`、macOS `open -R`、Linux 打开父目录）；两者均为桌面专属命令（EXCLUDED），远程（浏览器）降级为「复制路径」（`copyText`，非安全上下文回退 execCommand）。
- **Esc/脏检查**：`EditorModal.vue` 外壳统一处理（有未保存修改时确认丢弃）；手机端弹窗 = 底部抽屉（同一断点 CSS）。
- **MCP 密钥与变量引用**：只直写目标配置，CC-Box 不存储、不打日志；表单原样保留 `${VAR}` / `${VAR:-default}` 引用（Claude Code 对 command/args/env/url/headers 统一扩展，不展开）；http/sse 也可编辑 env；url 以 `${` 开头时放行静态校验。**args 与数组的转换是引号感知的**（`argsJoin`/`argsSplit`）：含空格的参数（如 `C:/Program Files (x86)/...`）join 时自动加双引号，裸空白 split 会把单个参数拆断（历史 bug）。
- **禁用条目查看**：skill/agent 被开关移入 `~/.cc-box/disabled/` 后，编辑器从备份目录读源码（`get_skill_source`/`get_agent_source` 兜底），不再报「Failed to read」。
- **样式归位**：`.editor-btn` 家族（primary/ghost/danger）定义在 `global.css`（非 scoped）——scoped 样式不作用于 slot 内按钮与 MarketBrowser 等独立组件（曾致打开目录/安装按钮裸样式）。

后端命令链：`store_edit.rs` → `commands.rs`（`#[cc_command(write = sect)]`）→ `rpc_dispatch.rs registry!` → `lib.rs generate_handler!` → `api/tauri.ts`。写命令自动广播 state-changed（mcp/skills/agents/hooks section），刷新回调链不含写操作（防写→广播→读→写死循环）。

## Hook 面板（第五面板）

- 数据：`get_hook_list` 合并 user（`~/.claude/settings.json`）/ project（`.claude/settings.json`）/ local（`.claude/settings.local.json`）三来源 + disabled 备份条目（`enabled: false`，分组单独展示）。
- **多类型**：hook 类型 = command / http / prompt / agent / mcp_tool（官方 hooks 文档）。表单按类型出字段（command→命令、http→URL+Headers、prompt/agent→Prompt(+model)）；mcp_tool 字段文档未稳定，编辑器自动切 JSON 模式原样保存。面板条目对非 command 类型显示类型徽标，摘要取 command→url→prompt 回退链。
- 条目定位：hooks 数组条目无 id，定位键 = `hash(event | matcher | hook0 签名)`（`hook_entry_key`）；command 类型签名 = 命令原文（老 disabled 备份文件名兼容），其他类型 = hook 对象 canonical JSON。前端一律使用列表下发的 `key`（`HookListItem.key`）做启停/删除/编辑定位；外部手改后失配报「配置已被外部修改，请刷新」——不做模糊匹配。
- **插件 hooks 分组**：Hooks 面板末尾有「插件」组（只读，头部/指示器/行样式与其他组同构——注意 `.expand-icon` 样式须在 HooksPanel 本地定义，HookGroup 的 scoped 样式不外溢），条目来自已安装插件的 hooks——`PluginInfo.hooks` 为 `PluginHookEntry[]`（事件 + matcher + 类型 + 摘要 + 超时），由 `parse_plugin_hooks_detail` 逐条展开（hooks/hooks.json、根 hooks.json、plugin.json 内联 object 或路径式声明；摘要取 command→url→prompt 回退）。行标题只显示事件名，点击展开只读详情（plugin/matcher/type/timeout/cmd），无开关、无编辑。Plugins 面板的 hooks 明细展示事件 + matcher + 摘要。
- 开关：禁用 = 条目摘出存 `disabled/hooks/hook-<key>.json`；启用 = 找回写回。前端与 skills/agents/mcp 同一模式：store `toggleHookEnabled` 乐观更新 + `markSelfWrite('hooks')` 抑制本端回环刷新，其他端经 state-changed 静默重拉。
- 事件清单：前端下拉为常用 13 事件（已对 Claude Code `HOOK_EVENTS` 源码核实），后端只校验形态（`^[A-Za-z_]{2,40}$`），向前兼容新事件。
- `_ccbox` 标记方案**已废弃**（本轮不需要商店所有权），hook 条目保持 Claude 原生结构零污染。

## Plugin 市场

- **市场与注册解耦**：「市场」= 可浏览的数据源；「注册」只在安装插件时懒发生。市场栏三段分组（后端 `get_plugin_marketplaces` 统一合并，条目带 `origin`/`registered`/`source`）：
  - **我的市场**（origin=user，存 `config.json::customMarkets`）——添加即入列表可浏览，不碰 Claude；带「已注册/未注册」徽标
  - **Claude 注册**（origin=registered）——CLI 等途径注册的 `known_marketplaces.json` 市场，可浏览/注销
  - **精选推荐**（origin=preset）——OSS 清单里未添加的，仅元数据（不拉取），一键「＋」加入列表
- **未注册市场浏览缓存**（仅 origin=user 且未注册）：本地目录直读；marketplace.json URL → HTTP GET（直连优先/系统代理兜底）存 `~/.cc-box/cache/markets/src-<hash>.json`；git 仓库（URL/owner-repo 简写）→ `git clone --depth 1`（120s 超时）到 `src-<hash>/`。刷新 = 删缓存重拉；移除 = 连缓存清理；**注册后浏览切换到 Claude 的克隆，CC-Box 缓存即清**（不两份并存）。
- **安装懒注册**：`install_plugin` 带可选 `market_source`；市场未在 `known_marketplaces.json` 时先 `claude plugin marketplace add <source>` 再 install（前端对未注册市场自动传参）。
- **推荐市场不写死在代码里**：数据文件 `${OSS}/data/plugin-markets.json`（仓库内 `data/plugin-markets.json`，`npm run deploy:data` 上传；`data/` 与官网、更新通道 `cc-box/`、`deps/` 完全独立）。后端 `get_market_presets` 拉取（OSS 直连、5s 超时），内存缓存 6h + 磁盘缓存 `~/.cc-box/cache/plugin-markets.json` 供离线兜底；拿不到就返回空列表（不显示推荐分组）。schema：`{ "version":1, "updatedAt":"…", "markets":[{ "name","source","description?" }] }`，按名称去重、丢弃空项。
- **排序 = 已安装 → 精选 → 热度 → 名称**：**已安装置顶（第一优先级，无论精选/热度）** → `pinned` 置顶 → `order` 升序（缺省排后）→ 安装量降序 → 名称。精选字段写在 marketplace.json 条目里（Claude 忽略未知字段，仅 CC-Box 解析）：
  `"ccbox": { "pinned": true, "order": 10, "category": "文档处理", "icon": "📄" }`。
  `category` 在条目没有顶层 `category` 时作回退；`icon` 支持 emoji / 短文本 / `http(s)` 图片 URL（图片加载失败时整块隐藏）。OSS 推荐市场清单（`data/plugin-markets.json`）同样支持顶层 `pinned` / `order` / `icon` / `category`。热度来自 Claude 的 `plugins/plugin-catalog-cache.json::catalog.plugins[*].unique_installs`（键 `name@marketplace`）；无热度数据排最后。行尾显示 `↓ 3.4k`（悬停说明数据来源），置顶条目名称前带 ★；纯本地读取，无额外网络。
- **自有精选市场 `ccbox-market`（完全自包含）**：本机开发目录 `C:/ccbox-plugins`（公开 git 仓库，推 Gitee）。**插件内容直接内嵌**（`plugins/<name>/` 完整源码，条目 source 全为相对路径）——浏览无需安装即可列出全部组件，安装走 Gitee 直连不需要代理。**该市场不可移除/卸载**（后端 `remove_plugin_marketplace` 守卫 `CCBOX_MARKET_NAME`，前端隐藏按钮）。结构、精选字段与发布方式见该仓库 README。
- **ccbox-market 是首个/默认市场**：面向用户的源是 `https://gitee.com/orczh/ccbox-market.git`（OSS 推荐清单 `pinned: true, order: 1`）；市场列表里置顶（`model.ts::orderMarkets`），打开面板时默认选中它；未注册时退回第一个已注册市场。开发期本地注册：`claude plugin marketplace add C:/ccbox-plugins`（注册名取自 marketplace.json 的 `name`），两者同名 `ccbox-market`。
- **已注册市场的缓存归 Claude 所有**：克隆在 `known_marketplaces.json::installLocation` 并记 `lastUpdated`，浏览读本地副本不联网。CC-Box 只把**解析结果**内存缓存 60s（组件目录扫描 + `plugins list` 代价随插件增长），市场增删/刷新、插件安装或启停时失效。
- **刷新分流**：当前市场已注册 → Claude 的 `marketplace update <name>`（git pull，需访问 GitHub，无代理失败时文案补提示）；未注册 → 重拉 CC-Box 浏览缓存。本地缓存新旧用列表头「更新于 YYYY-MM-DD」展示（注册 = Claude `lastUpdated`；未注册 = 缓存目录 mtime）。
- **组件明细（详情页「包含内容」）**：解析顺序是「清单/plugin.json 声明 → 扫描插件目录」：`commands|agents/*.md` 取文件名、`skills/*/SKILL.md` 取目录名、`hooks/hooks.json` 取事件名、`.mcp.json`/`mcp.json`（agent-plugins 新 schema）或 `mcpServers` 对象取服务器名、条目或 plugin.json 内联 `lspServers` / `.lsp.json` 取 LSP 服务器名（plugin.json 支持根目录 fallback）。**commands 是老版 skill 形态，市场详情与插件面板均并入 Skills 展示不区分**。源码目录优先取市场仓库内相对 `source`；远程源未安装时退到 `~/.claude/plugins/cache/<市场>/<插件>/<版本>/`，两者都没有则显示「未声明内容」提示。
- **安装/卸载/更新**：CLI 包装，显式 `-s user|project|local`（project 以项目路径为 cwd），成功后 `invalidate_plugins_list_cache()`；120s 超时（`run_command_with_timeout`，argv + cwd + 管道独立线程 + 超时树杀）。
- **信任确认**：安装前 confirm 展示名称/来源/作用域，明示插件可能包含可执行的 skills/agents/hooks/MCP（对齐 Claude Code `PluginTrustWarning` 的意图）。
- **UI**：`MarketBrowser.vue` 宽屏三栏（市场栏三段：我的市场（注册徽标）/ Claude 注册 / 精选推荐（＋ 添加）→ 插件列表（搜索 + 已安装徽标）→ 详情（简介 + 组件分组 + 作用域/安装操作））。**移除与取消注册是两个独立按钮**：「移除该市场」= 仅从 CC-Box 列表删除（不动 Claude 注册）；「取消注册」= 仅 `claude plugin marketplace remove`（列表条目保留仍可浏览）；按 origin/registered 条件显示。**≤860px（含桌面窄窗）**：市场栏改为**左侧抽屉**（头部「市场选择器」打开，竖向列表，数量多也不挤），内容区为列表/详情钻取（返回键）；**手机（≤768px 且触屏）**再降级为底部抽屉并放大触控目标（插件行 ≥56px、按钮 ≥38px、开关 38×22）。
- **进行中反馈**：添加/刷新/安装都可能同步阻塞几十秒，卡片底部统一进度条（spinner + 文案 + 已耗时秒数）；添加表单内另有「已等待 N 秒（首次克隆最长约 2 分钟）」提示。
- **单动作忙标记**：`busyKey` 按「市场/插件 + 动作」区分——安装某个插件时只有该行与详情按钮显示「处理中…」，其余按钮仅禁用，避免无关插件也被标成更新中。

## Plugins 面板（已安装插件管理）

- 分组 = 项目 / 个人（scope）；条目展开显示**全部七类组件明细**：skills / agents（可调用）/ mcpServers / hooks / lsp / monitors / themes。`commands` 是老版 skill 形态，**并入 skills 展示不区分**（invoke 格式 `/name`）。
- 解析单点 `parse_plugin_commands_hooks` + `parse_plugin_extra_components`（store.rs）：每类组件都覆盖「默认位置独立文件 + plugin.json 声明/内联」双形态——commands/ 目录与声明路径；hooks/hooks.json、根 hooks.json 与 plugin.json 内联 object（键=事件名）/路径（字符串或数组）；`.lsp.json` 与 `lspServers` 内联/路径；`monitors/monitors.json` 与 `monitors` 内联条目（取 name）/路径；`themes/` 目录与 `themes` 路径（目录→扫 *.json，文件→名）。声明值落得到真实文件按文件读，落不到即为内联名。
- **条目管理**：悬停（触屏常显）「更新 / 卸载」——包装 `update_plugin` / `uninstall_plugin`（CLI，project 作用域 cwd=projectPath），卸载走 `uiConfirm` 二次确认（明示会一并移除其 skills/agents/mcp/hooks）；成功后重拉列表（写命令出口也会广播 plugins state-changed 供其他端刷新）。启停开关与官方 `claude plugin enable/disable` 同一逻辑。

## 项目级开关

`set_skill_enabled` / `set_agent_enabled` / `set_mcp_server_enabled` 增加可选 `sourceType/scope + projectPath`（缺省 = user，旧调用兼容）：

- skill/agent project：active = `<proj>/.claude/skills|agents`，disabled = `~/.cc-box/disabled/<category>/<projectKey>/`。
- mcp project：剪切 `<proj>/.mcp.json::mcpServers`；local：剪切 `~/.claude.json::projects[cwd]::mcpServers`。
- 前端 `findToggleIndex` 按条目自身 `sourceType` 精确定位（同名 project/user 行由点击行决定；未传来源的旧调用只匹配 user，防误改）。

## 已知边界

- hook 定位依赖条目键精确匹配，用户手改 settings.json 后需刷新（不模糊匹配是刻意的）。
- hook http/prompt/agent 类型编辑从面板摘要回填（url/prompt 单字段）；多 hook 命令条目仍需先合并再编辑。
- plugin 安装无取消能力（同步 CLI + 120s 超时）；失败透传 CLI stderr。
- `vue-tsc` 存量类型告警（App.vue/TerminalView 等）与本次无关，见任务记录。

测试：后端 `src-tauri/src/tests/{plugin_market,store_edit}.rs`；前端 `tests/editor/model.test.ts`（表单↔配置纯函数真值对拍）。
