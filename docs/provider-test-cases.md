# Provider 管理测试条目

## 1. 数据存储与读取

### 1.1 初始化

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 1.1.1 | `~/.cc-box/providers.json` 不存在时加载 | 返回默认空配置：providers=[]、commonConfig.enabled=false、activeProviderId=null |
| 1.1.2 | `~/.cc-box/providers.json` 存在且格式正确时加载 | 正确解析所有字段 |
| 1.1.3 | `~/.cc-box/providers.json` 格式损坏时加载 | 返回错误，前端显示加载失败（不崩溃） |
| 1.1.4 | `~/.cc-box/` 目录不存在时保存 | 自动创建目录后保存 |

### 1.2 配置读写

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 1.2.1 | 保存 providers.json 后重新读取 | 数据一致，字段无丢失 |
| 1.2.2 | 中文/特殊字符的 name/notes | 保存和读取后 UTF-8 编码正确 |
| 1.2.3 | settingsConfig 包含嵌套对象和数组 | 序列化/反序列化后结构不变 |

## 2. Provider CRUD

### 2.1 创建

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 2.1.1 | 从预设模板创建 Provider | 正确填充 settingsConfig、websiteUrl、category、icon、iconColor |
| 2.1.2 | 预设含 templateValues 时创建 | 占位符 `${key}` 被替换为 editorValue/defaultValue |
| 2.1.3 | templateValues 中占位符出现在 env 值里 | env 中的占位符也被替换 |
| 2.1.4 | 创建后 providers.json 中多一条记录 | 新 Provider 追加到 providers 数组末尾 |
| 2.1.5 | 创建后自动进入编辑面板 | editingProvider 为新创建的 Provider |
| 2.1.6 | 快速连续创建多个 Provider | 每次生成不同 UUID，无数据丢失 |

### 2.2 编辑

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 2.2.1 | 修改名称并保存 | providers.json 中 name 更新 |
| 2.2.2 | 修改备注并保存 | notes 正确保存 |
| 2.2.3 | 清空备注并保存 | notes 保存为 null（前端显示无备注） |
| 2.2.4 | 修改环境变量值并保存 | settingsConfig.env 中对应值更新 |
| 2.2.5 | 添加自定义环境变量并保存 | 出现在 settingsConfig.env 中 |
| 2.2.6 | 删除自定义环境变量并保存 | 从 settingsConfig.env 中移除 |
| 2.2.7 | 无法删除 6 个必需环境变量 | 删除按钮不显示或点击无效 |
| 2.2.8 | 编辑 JSON 高级配置并保存 | settingsConfig 中非 env 字段更新 |
| 2.2.9 | 编辑 JSON 输入非法格式 | 显示错误提示，保存按钮禁用 |
| 2.2.10 | 编辑后不点保存直接关闭 | 不保存修改，下次打开显示旧数据 |
| 2.2.11 | 切换 commonConfigEnabled 开关并保存 | meta.commonConfigEnabled 正确更新 |

### 2.3 删除

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 2.3.1 | 点击删除按钮 | 显示自定义确认对话框，含 Provider 名称和"此操作不可撤销" |
| 2.3.2 | 确认对话框中点击取消 | 对话框关闭，Provider 仍在 |
| 2.3.3 | 确认对话框中点击删除 | Provider 从列表移除，providers.json 同步更新 |
| 2.3.4 | 删除当前激活的 Provider | Provider 移除 + activeProviderId 清空为 null |
| 2.3.5 | 删除非激活的 Provider | activeProviderId 不变 |
| 2.3.6 | 删除后列表为空 | 显示空状态提示 |

## 3. 激活流程

### 3.1 基本激活

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 3.1.1 | 激活 Provider A | `~/.claude/settings.json` 被完整替换为 Provider A 的 settingsConfig |
| 3.1.2 | 切换激活：A → B | settings.json 完整替换为 Provider B 的 settingsConfig |
| 3.1.3 | 激活后 activeProviderId 更新 | providers.json 中 activeProviderId 指向新激活的 Provider |
| 3.1.4 | 激活后卡片显示"使用中"标记 | 对应 ProviderCard 显示 active 状态（琥珀金边框） |
| 3.1.5 | 激活不存在的 Provider ID | 返回错误，前端显示错误信息 |

### 3.2 通用配置合并

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 3.2.1 | Provider 启用通用配置时激活 | settings.json = deep_merge(provider.settingsConfig, commonConfig.settings)，通用配置覆盖同名 key |
| 3.2.2 | Provider 禁用通用配置时激活 | settings.json = provider.settingsConfig（不合并） |
| 3.2.3 | commonConfigEnabled 未设置（undefined）时激活 | 默认合并（等同 true） |
| 3.2.4 | 合并：通用配置有新 key | Provider 中不存在的 key 被添加 |
| 3.2.5 | 合并：通用配置覆盖同名 key | Provider 中已存在的 key 被通用配置值替换 |
| 3.2.6 | 合并：嵌套对象递归合并 | env 等嵌套对象内逐 key 合并，不是整体替换 |
| 3.2.7 | 合并：数组字段 | 通用配置的数组整体替换 Provider 的数组（不连接） |
| 3.2.8 | 通用配置为空对象 `{}` 时激活 | settings.json 等于 provider.settingsConfig（合并无变化） |

### 3.3 settings.json 写入验证

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 3.3.1 | 写入后读取 settings.json | 内容与合并结果一致 |
| 3.3.2 | 写入的 JSON 格式化 | 使用 `to_string_pretty`（缩进 2 空格） |
| 3.3.3 | `~/.claude/` 目录不存在时激活 | 自动创建目录后写入 |

## 4. 通用配置管理

### 4.1 编辑通用配置

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 4.1.1 | 打开通用配置面板 | 显示当前 commonConfig.settings 的 JSON |
| 4.1.2 | 通用配置为空时打开 | 显示 `{\n  \n}` |
| 4.1.3 | 编辑 JSON 并保存 | providers.json 中 commonConfig 更新 |
| 4.1.4 | 编辑 JSON 格式错误 | 显示错误提示，保存按钮禁用 |
| 4.1.5 | 点击 Format 按钮 | JSON 自动格式化（缩进 2 空格） |

### 4.2 保存后响应

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 4.2.1 | 保存通用配置，当前有激活的 Provider | 自动重新激活当前 Provider（重新合并写入 settings.json） |
| 4.2.2 | 保存通用配置，当前无激活的 Provider | 只保存，不触发写入 settings.json |
| 4.2.3 | 修改通用配置后 settings.json 内容 | 包含通用配置 + Provider 配置的合并结果 |

### 4.3 从编辑内容中提取

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 4.3.1 | 从 Provider 编辑面板点击"编辑通用配置" | 打开通用配置面板，显示"从编辑内容中提取"按钮 |
| 4.3.2 | 点击"从编辑内容中提取" | 提取当前 Provider JSON 中除 env 和默认 model 外的字段 |
| 4.3.3 | 提取结果中不含 env | env 被过滤掉 |
| 4.3.4 | 提取结果中不含默认 model 值 | `claude-sonnet-4-6`、`claude-haiku-4-5-20251001`、`claude-opus-4-7` 被过滤 |
| 4.3.5 | 直接从工具栏点击"编辑通用配置" | 不显示"从编辑内容中提取"按钮（sourceJson 为 null） |

## 5. 预设模板

### 5.1 预设选择

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 5.1.1 | 打开预设面板 | 显示分类筛选标签 + 所有非 hidden 预设 |
| 5.1.2 | 按分类筛选 | 只显示对应分类的预设 |
| 5.1.3 | 选择"全部"分类 | 显示所有非 hidden 预设 |
| 5.1.4 | 点击预设卡片 | 创建 Provider 并进入编辑面板 |

### 5.2 预设内容

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 5.2.1 | Anthropic Official 预设 | settingsConfig 包含官方 API 端点和标准环境变量 |
| 5.2.2 | DeepSeek 预设 | category=cn_official、正确的 Base URL |
| 5.2.3 | OpenRouter 预设 | category=aggregator、正确的 Base URL |
| 5.2.4 | AWS Bedrock 预设 | 含 templateValues（AWS_REGION、AWS_ACCESS_KEY_ID、AWS_SECRET_ACCESS_KEY） |
| 5.2.5 | hidden=true 的预设不出现在列表中 | 预设面板中不显示 |

## 6. 拖拽排序

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 6.1 | 拖拽卡片到新位置 | Provider 列表顺序改变 |
| 6.2 | 拖拽后 sortIndex 更新 | providers.json 中 sortIndex 按新顺序递增 |
| 6.3 | 拖拽后页面刷新 | 排序保持（从 providers.json 读取） |
| 6.4 | 通过拖拽 handle（⋮⋮）拖动 | 正常拖拽 |
| 6.5 | 不通过 handle 拖动（点击卡片其他区域） | 不触发拖拽 |
| 6.6 | 卡片之间有纵向间隙 | gap: 8px 生效 |

## 7. cc-switch 数据导入

### 7.1 导入检测

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 7.1.1 | cc-switch 数据库不存在 | "从 cc-switch 导入"按钮不显示 |
| 7.1.2 | cc-switch 数据库存在 | "从 cc-switch 导入"按钮显示 |
| 7.1.3 | 数据库文件存在但无法打开 | 返回错误，前端显示"导入失败" |

### 7.2 Provider 导入

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 7.2.1 | 数据库有 3 个 Provider，CC-Box 无 Provider | 导入 3 个，提示"成功导入 3 个" |
| 7.2.2 | 数据库有 Provider，CC-Box 已有同 ID 的 | 跳过已存在的，只导入新的 |
| 7.2.3 | 导入后 Provider 字段完整 | id、name、settingsConfig、websiteUrl、category、createdAt、sortIndex、notes、icon、iconColor、meta、inFailoverQueue 全部保留 |
| 7.2.4 | 导入的 settingsConfig 正确解析 | JSON 字符串正确反序列化为对象 |
| 7.2.5 | 导入的 meta 字段正确解析 | JSON 字符串正确反序列化为 ProviderMeta（含 isPartner 等扩展字段） |
| 7.2.6 | 只导入 app_type='claude' 的记录 | 其他 app_type 的记录不导入 |

### 7.3 通用配置导入

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 7.3.1 | settings 表有 common_config_claude | commonConfig.enabled=true、settings 为对应 JSON |
| 7.3.2 | settings 表无 common_config_claude | commonConfig 保持不变 |
| 7.3.3 | common_config_claude 值为空 JSON `{}` | commonConfig 保持不变（不设置 enabled=true） |
| 7.3.4 | common_config_claude 值为 null | commonConfig 保持不变 |

## 8. 前端 UI 交互

### 8.1 Provider 卡片

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 8.1.1 | 卡片显示内容 | 图标 + 名称 + 备注（如有） |
| 8.1.2 | 无备注时不显示备注行 | 只有名称，无空白区域 |
| 8.1.3 | 备注过长时截断 | 显示省略号，title 属性显示完整内容 |
| 8.1.4 | 激活的卡片样式 | 琥珀金边框 + 选中背景 |
| 8.1.5 | 激活的卡片操作区 | 显示"使用中"标记，不显示"使用"按钮 |
| 8.1.6 | 未激活卡片 hover | 显示操作按钮（使用/编辑/测试/删除） |
| 8.1.7 | 拖拽 handle 默认状态 | 半透明（opacity: 0.3） |
| 8.1.8 | 拖拽 handle hover 状态 | 更明显（opacity: 0.7） |

### 8.2 编辑面板

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 8.2.1 | 打开编辑面板 | 全屏覆盖主面板（z-index: 100） |
| 8.2.2 | 编辑面板内容分区 | 基本信息、环境变量、高级配置三个区域 |
| 8.2.3 | 必需环境变量标记 | 含 TOKEN/KEY/SECRET 的变量显示 `*` 标记和密码切换按钮 |
| 8.2.4 | 自定义变量默认折叠 | 只显示折叠/展开按钮和数量 |
| 8.2.5 | 展开自定义变量 | 显示所有自定义变量，每个有删除按钮 |
| 8.2.6 | 添加自定义变量 | 底部出现内联输入行（变量名 + 变量值），无原生弹窗 |
| 8.2.7 | 按 Enter 确认添加新变量 | 变量添加到列表，输入行消失 |
| 8.2.8 | 按 Escape 取消添加 | 输入行消失，不添加变量 |
| 8.2.9 | 变量名重复时不添加 | 确认按钮无效果 |
| 8.2.10 | 点击"编辑通用配置" | 通用配置面板覆盖编辑面板（z-index: 200） |

### 8.3 面板层级

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 8.3.1 | 主面板 → 编辑面板 | 编辑面板覆盖主面板 |
| 8.3.2 | 编辑面板 → 通用配置面板 | 通用配置面板覆盖编辑面板 |
| 8.3.3 | 通用配置面板返回 | 回到编辑面板 |
| 8.3.4 | 编辑面板返回 | 回到主面板 |
| 8.3.5 | 确认对话框在最上层 | z-index: 300，覆盖所有面板 |
| 8.3.6 | 点击确认对话框遮罩层 | 对话框关闭（@click.self） |

### 8.4 JSON 编辑器

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 8.4.1 | 编辑器语法高亮 | JSON 关键字、字符串、数字有颜色区分 |
| 8.4.2 | 输入非法 JSON | 实时显示错误位置和原因 |
| 8.4.3 | 错误状态下保存 | 保存按钮 disabled |
| 8.4.4 | CommonConfigPanel JSON 滚动 | 编辑器自适应内容高度，panel-body 整体可滚动 |
| 8.4.5 | ProviderEditPanel JSON | 编辑器自适应内容高度，面板内可滚动 |

## 9. 边界与异常

### 9.1 数据边界

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 9.1.1 | settingsConfig 为空对象 `{}` | 正常激活，写入空对象到 settings.json |
| 9.1.2 | settingsConfig.env 为空（无 API Key） | 可激活，但 Claude CLI 启动会报错（非 CC-Box 职责） |
| 9.1.3 | Provider 数量很多（50+） | 列表渲染正常，滚动流畅 |
| 9.1.4 | settingsConfig 包含深层嵌套 JSON | 合并和写入正确 |
| 9.1.5 | name 包含 HTML 特殊字符（`<script>`） | 不执行脚本，纯文本显示 |

### 9.2 并发操作

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 9.2.1 | 快速连续激活不同 Provider | 最后一次激活的结果生效，settings.json 为最后一次的配置 |
| 9.2.2 | 激活同时删除该 Provider | 返回错误（Provider not found） |
| 9.2.3 | 编辑面板打开时删除该 Provider | 关闭编辑面板，Provider 已删除 |

### 9.3 兼容性

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 9.3.1 | 从 cc-switch 导入的数据可直接激活 | settings.json 正确写入 |
| 9.3.2 | cc-switch 导入的 commonConfigEnabled 生效 | 合并/不合并行为正确 |
| 9.3.3 | 未知 category 值的 Provider | category 解析为 null，不影响其他功能 |
| 9.3.4 | meta 字段含 CC-Box 不识别的额外 key | 忽略额外 key，不报错 |

## 10. 预设同步与图标（2026-10 cc-switch 全量对接后新增）

### 10.1 图标渲染

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 10.1.1 | 预设面板浏览各厂商（DeepSeek/Ollama/腾讯等） | 有资产的 74 家显示厂商图标（svg/png），无资产的（therouter/groq/本机推理等）回退首字母头像，无空白 |
| 10.1.2 | Provider 列表（已添加的 Provider） | 图标渲染与预设面板一致 |
| 10.1.3 | 远程模式（手机浏览器）打开预设面板 | 图标正常加载（打包产物 URL） |
| 10.1.4 | `local` 分类筛选 | 显示 Ollama/LM Studio/llama.cpp/vLLM 四个本机推理预设，标签「本机」/「Local」 |

### 10.2 预设同步流程

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 10.2.1 | 下载 cc-switch zip 后 `npm run sync:presets -- --zip <zip>` | 133 个预设生成、图标复制、meta.json 更新、守卫测试通过 |
| 10.2.2 | 生成的预设 URL 抽查（Kimi/火山/智谱） | 无 track_id/aff/utm 等推广参数残留 |

### 10.3 UI 回归

| # | 测试项 | 预期结果 |
|---|--------|----------|
| 10.3.1 | Provider 卡片按钮组（使用/编辑/测试/删除） | 按钮间有 8px 间距；手机断点换行后右对齐、间距保留 |
| 10.3.2 | 选择新预设（如 Ollama 本机）创建 Provider | 字段带入正确（BASE_URL localhost:11434），可正常保存 |
