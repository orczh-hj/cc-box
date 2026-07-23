# Provider 管理

API Provider 配置管理，支持多厂商预设、自定义配置、通用参数合并、一键激活切换。

数据结构兼容 [cc-switch](https://github.com/area44/cc-switch)，支持从 cc-switch 数据库导入。

## 响应逻辑原则

### 一、JSON 是唯一真相源

ProviderEditPanel 中 `jsonContent` ref 是 settingsConfig 的唯一持有者。Env 表单只是 `env` 字段的结构化视图/编辑器，不持有独立状态。修改 JSON 只影响 env 展示，保存时直接写入全部内容。

### 二、JSON ↔ Env 双向实时同步

| 方向 | 机制 |
|------|------|
| JSON → Env | `parsedConfig` computed 缓存 → `getEnvValue` 读取 |
| Env → JSON | `setEnvValue` → 修改 parsedConfig → 序列化回 `jsonContent` |

所有读取走 `parsedConfig` computed 缓存，禁止在渲染路径中重复 `JSON.parse`。

### 三、勾选"应用通用配置" → 即时合并

勾选时立即执行 `deepMerge(parsedConfig, commonConfig.settings)`，将合并结果写入 `jsonContent`。用户可在 JSON 编辑器中看到并编辑合并后的完整内容。**合并发生在编辑时，不是激活时。**

### 四、取消勾选"应用通用配置" → 无操作

Provider 保留当前配置（含之前已合并的通用配置字段）。只更新 `meta.commonConfigEnabled = false`，不回退已合并的内容。

### 五、通用配置修改 → 批量同步所有已勾选 Provider

保存通用配置时，后端将新内容 `deepMerge` 到 **所有** `meta.commonConfigEnabled === true` 的 Provider 的 `settingsConfig` 中，一并持久化到 `providers.json`，并重新激活当前活跃的 Provider。

### 六、激活 = Deep Merge 写入 + 剥离状态字段

`activate_provider` 读取当前 `~/.claude/settings.json`，对 Provider 的 `settingsConfig` 执行以下处理后再 merge 写回：

1. **剥离 Claude Code 状态字段**（`strip_cc_state_fields`）：移除 `enabledPlugins`、`disabledPlugins`、`extraKnownMarketplaces`、`hooks` 四个键，避免覆盖用户后续通过 CLI（如 `claude plugin enable/disable`）做的修改。
2. **Deep Merge**（`deep_merge_json`）：Provider 配置作为 source 覆盖 target（磁盘现状）的同名叶值，target 独有的键保留。

合并已在编辑时完成（通用配置 → Provider），activate 阶段不再做通用配置合并。

### 七、不自动保存

所有修改仅存于本地 ref，持久化只在用户点击按钮时发生。

### 响应矩阵

| 用户操作 | 系统响应 | 持久化 |
|---------|---------|--------|
| 修改 JSON | Env 表单刷新 | 否 |
| 修改 Env 字段 | JSON 更新 | 否 |
| **勾选**"应用通用配置" | `deepMerge(provider, common)` → 写入 `jsonContent` | 否（本地） |
| **取消勾选**"应用通用配置" | 无操作，仅标记 `meta.commonConfigEnabled = false` | 否（本地） |
| 保存通用配置 | 合并到所有已勾选 Provider → 保存 `providers.json` → 重新激活 | **是** |
| 保存 | Provider 配置写入 `providers.json` | 是 |
| 保存并激活 | 写入 `providers.json` → 直接写入 `settings.json` | 是 |
| 激活（卡片"使用"按钮） | 直接写入 `settings.json` | 是 |

## 数据结构

### 文件存储

| 文件 | 用途 | 读写 |
|------|------|------|
| `~/.cc-box/providers.json` | Provider 列表 + 通用配置 + 激活状态 | 读写 |
| `~/.claude/settings.json` | Claude Code 运行时配置（激活时写入） | 写入 |
| `~/.cc-switch/cc-switch.db` | cc-switch SQLite 数据库（导入时读取） | 只读 |

### Provider 配置结构 (`providers.json`)

```jsonc
{
  "providers": [
    {
      "id": "uuid-123",
      "name": "我的 Anthropic",
      "settingsConfig": {
        "env": {
          "ANTHROPIC_AUTH_TOKEN": "sk-ant-xxx",
          "ANTHROPIC_BASE_URL": "https://api.anthropic.com",
          "ANTHROPIC_MODEL": "claude-sonnet-4-6",
          "ANTHROPIC_DEFAULT_HAIKU_MODEL": "claude-haiku-4-5-20251001",
          "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-6",
          "ANTHROPIC_DEFAULT_OPUS_MODEL": "claude-opus-4-7"
        },
        "model": "claude-sonnet-4-6",
        "permissions": { "allow": [], "deny": [] },
        "attribution": { "commit": "", "pr": "" },
        "includeCoAuthoredBy": false,
        "effortLevel": "high"
      },
      "websiteUrl": "https://anthropic.com",
      "category": "official",
      "createdAt": 1715555555000,
      "sortIndex": 0,
      "notes": "我的主账号",
      "isPartner": false,
      "meta": {
        "commonConfigEnabled": true,
        "apiFormat": "anthropic",
        "apiKeyField": "ANTHROPIC_AUTH_TOKEN"
      },
      "icon": "anthropic",
      "iconColor": "#D4915D",
      "inFailoverQueue": false
    }
  ],
  "commonConfig": {
    "enabled": true,
    "settings": {
      "env": {
        "CLAUDE_CODE_SCROLL_SPEED": "5"
      },
      "attribution": { "commit": "", "pr": "" },
      "includeCoAuthoredBy": false
    }
  },
  "activeProviderId": "uuid-123"
}
```

### Provider 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | UUID，唯一标识 |
| `name` | string | 显示名称 |
| `settingsConfig` | object | 完整配置（含 env、model、permissions 等） |
| `websiteUrl` | string? | 厂商官网 |
| `category` | string? | 分类（official / cn_official / aggregator / cloud_provider / third_party / custom / omo / omo-slim） |
| `createdAt` | number? | 创建时间戳（毫秒） |
| `sortIndex` | number? | 排序索引 |
| `notes` | string? | 用户备注（空字符串存储为 null） |
| `meta` | ProviderMeta? | 元数据（不写入 settings.json） |
| `icon` | string? | 图标名称（anthropic / deepseek / openrouter 等） |
| `iconColor` | string? | 图标颜色（十六进制） |
| `inFailoverQueue` | bool | 故障转移队列标记（默认 false） |

### settingsConfig 结构

`settingsConfig` 直接对应 `~/.claude/settings.json` 的完整结构。核心字段：

| 字段 | 类型 | 说明 |
|------|------|------|
| `env` | object | 环境变量（API Key、Base URL、Model 等） |
| `model` | string | 默认模型 |
| `permissions` | { allow: [], deny: [] } | 权限配置 |
| `attribution` | { commit: "", pr: "" } | 归属信息 |
| `includeCoAuthoredBy` | bool | 是否包含 co-authored-by |
| `effortLevel` | string | 推理强度 |

**6 个必需环境变量**（在编辑面板中不可删除）：

| 变量名 | 说明 |
|--------|------|
| `ANTHROPIC_AUTH_TOKEN` | API 密钥 |
| `ANTHROPIC_BASE_URL` | API 端点 |
| `ANTHROPIC_MODEL` | 默认模型 |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL` | Haiku 模型 |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Sonnet 模型 |
| `ANTHROPIC_DEFAULT_OPUS_MODEL` | Opus 模型 |

### ProviderMeta 字段（完全兼容 cc-switch，22 个字段）

| 字段 | 类型 | 说明 |
|------|------|------|
| `commonConfigEnabled` | bool? | 是否应用通用配置（默认 true） |
| `endpointAutoSelect` | bool? | 自动选择最佳端点 |
| `apiFormat` | string? | API 格式：anthropic / openai_chat / openai_responses / gemini_native |
| `apiKeyField` | string? | API Key 字段名 |
| `isFullUrl` | bool? | 是否完整 URL |
| `providerType` | string? | 供应商类型（如 github_copilot） |
| `costMultiplier` | string? | 成本倍率 |
| `pricingModelSource` | string? | 计价来源（request / response） |
| `limitDailyUsd` | string? | 日限额 |
| `limitMonthlyUsd` | string? | 月限额 |
| `isPartner` | bool? | 合作商 |
| `partnerPromotionKey` | string? | 促销 key |
| `testConfig` | object? | 连接测试配置 |
| `usageScript` | object? | 用量查询脚本配置 |
| `authBinding` | object? | 认证绑定配置 |
| `promptCacheKey` | string? | Prompt Cache key |
| `codexFastMode` | bool? | Codex OAuth 快速模式 |
| `liveConfigManaged` | bool? | 是否由管理器控制 live 配置 |
| `githubAccountId` | string? | GitHub Copilot 账号 ID |
| `customEndpoints` | object? | 自定义端点列表 |

## 激活流程

用户点击"使用"按钮激活 Provider，流程如下：

```
前端 providersStore.activate(id)
  → API activateProvider(id)
    → Rust activate_provider(provider_id)
        ├── 1) 读取 providers.json
        ├── 2) 查找 Provider
        ├── 3) 读取当前 ~/.claude/settings.json（不存在视为 {}）
        ├── 4) strip_cc_state_fields(provider.settings_config) 剥离状态字段
        ├── 5) merge_provider_into_settings(current, stripped) deep merge
        ├── 6) 写回 ~/.claude/settings.json
        └── 7) 更新 activeProviderId，保存 providers.json
```

**关键规则**：
- 激活 = Deep Merge 写入（保留 settings.json 现有字段，Provider 字段覆盖同名叶值）
- 状态字段（`enabledPlugins` 等）不会被 Provider 快照覆盖，避免回滚 CLI 修改
- 新启动的终端会话使用新配置
- 已运行的终端不受影响

### 被剥离的字段（CC_STATE_KEYS）

| 字段 | 剥离原因 |
|------|---------|
| `enabledPlugins` | Claude Code 通过 `claude plugin enable/disable` 维护，Provider 快照不得覆盖 |
| `disabledPlugins` | 同上（防御性纳入） |
| `extraKnownMarketplaces` | 由 `claude plugin marketplace add/remove` 维护 |
| `hooks` | Claude Code 运行时 hook 配置，独立于 Provider |

剥离发生在三个位置：
- `activate_provider`：写 settings.json 前剥离 Provider settingsConfig（运行时净化）
- `update_common_config`：保存通用配置前剥离（防止用户在 commonConfig 误填污染所有 Provider）
- `import_from_cc_switch`：导入 cc-switch 通用配置时串联剥离（与 `strip_core_env` 配合）

`create_provider` / `update_provider` 不剥离，保留用户编辑器中的原始输入。

## 通用配置合并

### 合并策略

`deep_merge_json(target, source)` — source 覆盖 target：

| 类型组合 | 行为 |
|----------|------|
| Object + Object | 递归深度合并 |
| Object + 其他 | source 替换 target |
| 其他 + 其他 | source 替换 target |

**示例**：

```jsonc
// Provider settingsConfig
{ "env": { "A": "1", "B": "2" }, "model": "claude-sonnet-4-6", "permissions": { "allow": ["tool1"] } }

// 通用配置
{ "env": { "B": "99", "C": "3" }, "attribution": { "commit": "", "pr": "" } }

// 合并结果（通用配置覆盖同名字段）
{ "env": { "A": "1", "B": "99", "C": "3" }, "model": "claude-sonnet-4-6", "permissions": { "allow": ["tool1"] }, "attribution": { "commit": "", "pr": "" } }
```

### 合并时机

通用配置的合并不在激活时发生，而在以下时机：

| 操作 | 是否触发合并 | 说明 |
|------|:---:|------|
| 勾选"应用通用配置" | 是（前端） | 即时合并到 JSON 编辑器内容，用户可编辑 |
| 取消勾选"应用通用配置" | 否 | 保留当前配置不回退 |
| 修改通用配置（保存） | 是（后端） | 批量合并到所有 `commonConfigEnabled === true` 的 Provider |
| 编辑 Provider（保存） | 否 | 只保存到 providers.json |
| 创建 Provider | 否 | 只保存到 providers.json |
| 删除 Provider | 否 | 如果删除的是当前激活的，清除 activeProviderId |
| 激活 Provider | 否 | 直接写入 settings.json，不执行合并 |

### commonConfigEnabled 决策

```
批量合并条件 = meta.commonConfigEnabled === Some(true)
```

- `commonConfigEnabled: true`：参与通用配置的批量合并
- `commonConfigEnabled: false` 或 `undefined`：不参与批量合并
- 前端编辑面板中勾选框默认值：`provider.meta?.commonConfigEnabled ?? true`

### 通用配置修改流程

```
用户修改通用配置并保存
  → providersStore.updateCommon(settings)
    → Rust update_common_config(enabled, settings)
        ├── 1) strip_cc_state_fields(settings) 剥离状态字段
        ├── 2) 保存剥离后的通用配置到 providers.json
        ├── 3) 遍历所有 providers
        ├── 4) 若 meta.commonConfigEnabled === true
        │     → deep_merge_json(provider.settings_config, stripped_settings)
        │     → 更新 provider.settings_config
        ├── 5) 保存 providers.json（含所有已更新的 Provider）
        └── 6) 返回
  → Store 重载 providers（获取后端批量合并后的最新数据）
  → Store 重新激活当前活跃的 Provider（写入 settings.json）
  → ProviderEditPanel watch props.commonConfig → 重复合并到编辑内容
```

## CRUD 操作

### 创建 Provider

```
用户选择预设模板 → ProviderPresetPanel
  → providersStore.createFromPreset(preset)
    → 处理 templateValues（替换占位符）
    → API createProvider(name, settingsConfig, ...)
      → Rust create_provider()
        → 生成 UUID + 时间戳
        → 追加到 providers.json
```

### 编辑 Provider

```
用户点击编辑 → ProviderEditPanel（全屏面板，非 Modal）
  编辑内容：
    - 基本信息：名称、备注
    - 环境变量：6 个必需 + 自定义变量（可折叠），双向同步 JSON
    - 配置 JSON 编辑器：完整 settingsConfig（含 env），可编辑
    - 通用配置：勾选时即时合并到 JSON，取消勾选无操作

  保存 → providersStore.update(id, { name, settingsConfig, notes, meta })
    → API updateProvider(id, ...)
      → Rust update_provider()
        → 更新 providers.json（不触发激活）

  保存并激活 → providersStore.update + providersStore.activate
    → 更新 providers.json → 直接写入 settings.json
```

### 删除 Provider

```
用户点击删除 → 自定义确认对话框
  → providersStore.remove(id)
    → API deleteProvider(id)
      → Rust delete_provider()
        → 从 providers.json 移除
        → 如果是当前激活的，清除 activeProviderId
```

### 排序

```
拖拽卡片 → useDraggable (vue-draggable-plus)
  → handle=".drag-handle", forceFallback=true
  → onUpdate → providersStore.reorder(ids)
    → API updateProviderSortOrder(ids)
      → Rust update_provider_sort_order()
        → 更新 sortIndex，保存 providers.json
```

## cc-switch 数据导入

### 导入条件

- 检测 `~/.cc-switch/cc-switch.db` 是否存在
- 仅在"从 cc-switch 导入"按钮可见时（hasCcSwitchDb = true）

### 导入流程

```
providersStore.importCcSwitch()
  → API importFromCcSwitch()
    → Rust import_from_cc_switch()
        ├── 1) 打开 SQLite 数据库
        ├── 2) SELECT providers WHERE app_type = 'claude'
        │     字段：id, name, settings_config, website_url, category,
        │           created_at, sort_index, notes, icon, icon_color,
        │           meta, in_failover_queue, is_current
        ├── 3) 解析 meta JSON（含全部 22 个字段）
        ├── 4) 识别 is_current = true 的 Provider → 激活状态
        ├── 5) 按 ID 去重，追加到现有 providers
        ├── 6) 读取 settings 表 common_config_claude → 通用配置
        ├── 7) 如果 CC-Box 无激活的 Provider，设置为 cc-switch 当前的
        └── 8) 保存 providers.json，返回导入结果
```

### 导入结果

```typescript
interface ImportResult {
  count: number                    // 导入的 Provider 数量
  importedCommonConfig: boolean    // 是否导入了通用配置
  activeProviderName: string | null // cc-switch 中当前激活的 Provider 名称
}
```

### cc-switch 数据库表结构

**providers 表**（按 `app_type = 'claude'` 过滤）：

| 列 | 类型 | 映射到 CC-Box |
|----|------|-------------|
| id | TEXT | id |
| name | TEXT | name |
| settings_config | TEXT (JSON) | settingsConfig |
| website_url | TEXT? | websiteUrl |
| category | TEXT? | category |
| created_at | INTEGER? | createdAt |
| sort_index | INTEGER? | sortIndex |
| notes | TEXT? | notes |
| icon | TEXT? | icon |
| icon_color | TEXT? | iconColor |
| meta | TEXT (JSON) | meta（含 ProviderMeta 全部 22 字段） |
| in_failover_queue | BOOLEAN | inFailoverQueue |
| is_current | BOOLEAN | 用于识别 cc-switch 当前激活的 Provider |

**settings 表**（通用配置）：

| key | 说明 |
|-----|------|
| `common_config_claude` | Claude 通用配置片段（JSON） |

## 前端组件

### 组件树

```
ProvidersSection.vue          # 主面板容器
├── ProviderList.vue          # Provider 列表（useDraggable 排序）
│   └── ProviderCard.vue      # 单个卡片（图标 + 名称 + 备注 + 操作按钮）
├── ProviderPresetPanel.vue   # 预设选择面板（分类筛选 + 网格布局）
├── ProviderEditPanel.vue     # 全屏编辑面板
│   ├── 基本信息表单           # 名称、备注、官网
│   ├── 环境变量编辑器         # 6 必需 + 自定义（折叠）
│   └── JSON 编辑器           # CodeMirror (vue-codemirror)
├── CommonConfigPanel.vue     # 通用配置编辑面板（z-index: 200）
│   └── JSON 编辑器           # CodeMirror
└── 确认对话框                # 删除二次确认（z-index: 300）
```

### 面板层级

| 面板 | z-index | 说明 |
|------|---------|------|
| ProvidersSection | 默认 | 主面板 |
| ProviderEditPanel | 100 | 编辑面板，覆盖主面板 |
| CommonConfigPanel | 200 | 通用配置，覆盖编辑面板 |
| 确认对话框 | 300 | fixed 定位，覆盖所有 |

### Store 状态

```typescript
// src/stores/providers.ts
{
  providers: Provider[]           // 所有 Provider 列表
  commonConfig: CommonConfig      // 通用配置 { enabled, settings }
  activeProviderId: string | null // 当前激活的 Provider ID
  isLoading: boolean              // 加载状态
  hasCcSwitchDb: boolean          // cc-switch 数据库是否存在
  presets: ProviderPreset[]       // 预设模板列表
}
```

## Tauri IPC 命令

| 命令 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `get_providers_config` | — | ProvidersConfig | 读取 providers.json |
| `save_providers_config` | config: ProvidersConfig | void | 保存 providers.json |
| `activate_provider` | providerId: string | void | 激活 Provider（直接写入 settings.json，不合并） |
| `create_provider` | name, settingsConfig, ... | Provider | 创建 Provider |
| `update_provider` | id, name?, settingsConfig?, notes?, meta? | Provider | 更新 Provider |
| `delete_provider` | id: string | void | 删除 Provider |
| `update_provider_sort_order` | providerIds: string[] | void | 更新排序 |
| `update_common_config` | enabled, settings | void | 更新通用配置 + 批量合并到所有 commonConfigEnabled 的 Provider |
| `check_cc_switch_db_exists` | — | bool | 检测 cc-switch 数据库 |
| `import_from_cc_switch` | — | ImportResult | 从 cc-switch 导入 |

## 预设配置

预设定义在 `src/config/providerPresets.ts`，包含 50+ 厂商模板。

### 分类

| 分类 | 厂商示例 |
|------|---------|
| `official` | Claude Official |
| `cn_official` | DeepSeek、智谱 GLM、阿里云百炼、Kimi、MiniMax、StepFun、豆包、百度千帆 |
| `aggregator` | OpenRouter、SiliconFlow、ModelScope、DMXAPI、TheRouter、AIHubMix |
| `cloud_provider` | AWS Bedrock、Google Vertex AI |
| `third_party` | GitHub Copilot、PackyCode、Cubence、AIGoCode |

### 模板变量（templateValues）

部分预设包含模板变量（如 AWS Bedrock 需填 AWS_REGION、AWS_ACCESS_KEY_ID），创建时占位符自动替换。

## 关键文件

| 文件 | 职责 |
|------|------|
| `src/types/provider.ts` | TypeScript 类型定义 |
| `src/config/providerPresets.ts` | 预设模板配置 |
| `src/api/provider.ts` | Tauri invoke 封装 |
| `src/stores/providers.ts` | Pinia Store（状态 + 操作） |
| `src-tauri/src/providers.rs` | Rust 后端（存储 + 合并 + 导入） |
| `src/components/settings/sections/ProvidersSection.vue` | 主面板 |
| `src/components/settings/providers/ProviderList.vue` | 列表（useDraggable） |
| `src/components/settings/providers/ProviderCard.vue` | 卡片组件 |
| `src/components/settings/providers/ProviderEditPanel.vue` | 编辑面板 |
| `src/components/settings/providers/ProviderPresetPanel.vue` | 预设选择 |
| `src/components/settings/providers/CommonConfigPanel.vue` | 通用配置编辑 |
