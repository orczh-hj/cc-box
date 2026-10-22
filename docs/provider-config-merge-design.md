# Provider 配置合并统一设计（通用配置 / Provider / 用户 settings.json）

> 状态：**已实施（schema v2）**。当前实现正本见 `docs/provider-management.md`；
> 本文件保留设计动机、取舍与迁移说明。
>
> 实施时的决策：
> - 通用配置允许包含核心 `ANTHROPIC_*` env（Provider 显式值优先）；cc-switch 导入仍剥离核心 env。
> - 数组保持整体替换；新建空白 Provider 模板不再注入 `permissions` 空数组。
> - 编辑面板以 effective 视图 + 提示展示继承关系，保存时自动差分成覆盖层；
>   未做“继承路径可展开列表”与“取消勾选二次确认”两项 UI 增强。
> - `settings.json` 写入前备份 + 原子写；坏 JSON 直接报错拒绝覆盖。

## 1. 背景

当前实现（`docs/provider-management.md`、`src-tauri/src/providers.rs`、`ProviderEditPanel.vue`）：

- 通用配置在保存/勾选时被 **deepMerge 物化** 进每个 Provider 的 `settingsConfig`，且**通用配置赢**；
- 激活时再把 Provider 配置 deepMerge 进用户 `~/.claude/settings.json`；
- 最终优先级：`通用配置 > Provider > 用户 settings.json`（用户独有键保留）；
- 取消勾选不回滚、删除通用项不传播、Provider 专属值会被通用配置覆盖、编辑中的 JSON 会被外部通用配置更新覆盖；
- 激活只增不删，切换 Provider 后旧键残留；`providers.rs` 还忽略了 `CLAUDE_CONFIG_DIR`。

## 2. 设计目标

1. **可逆**：勾选/取消通用配置、删除通用项，行为可预测且可撤销。
2. **单一优先级**：`Provider 覆盖 > 通用默认 > 用户现状`，用户独有键始终保留。
3. **单一合成入口**：Rust 提供唯一权威实现，前端只做同一算法的预览/差异计算，双方用共享 fixture 契约测试防漂移。
4. **轻量透明**：仍然只有 JSON 文件，不引入数据库/状态机；新增 schema 版本用于一次性迁移。
5. **安全写入**：尊重 `CLAUDE_CONFIG_DIR`；写入前备份；坏 JSON 不静默覆盖；只清理 cc-box 自己写过的键。

## 3. 数据模型（providers.json schema v2）

```jsonc
{
  "schemaVersion": 2,
  "commonConfig": {
    "enabled": true,
    "settings": {
      "env": { "CLAUDE_CODE_SCROLL_SPEED": "5" },
      "effortLevel": "high"
    }
  },
  "providers": [
    {
      "id": "uuid",
      "name": "Provider A",
      // 只存「覆盖层」：相对通用配置的差异 + Provider 自有字段
      "settingsConfig": {
        "env": { "ANTHROPIC_AUTH_TOKEN": "sk-xxx" },
        "effortLevel": "low"
      },
      "meta": {
        "commonConfigEnabled": true,
        // 可选：显式屏蔽的继承路径（JSON Pointer）
        "commonConfigRemoved": ["/env/FOO"]
      }
    }
  ],
  "activeProviderId": "uuid",
  // 上次激活由 cc-box 写入 settings.json 的托管路径（用于删除残留键）
  "appliedSettings": {
    "providerId": "uuid",
    "paths": ["/env/ANTHROPIC_AUTH_TOKEN", "/env/ANTHROPIC_MODEL", "/effortLevel"]
  }
}
```

字段约定：

| 字段 | 缺省 | 说明 |
|---|---|---|
| `schemaVersion` | 1（缺失） | 2 = 分层语义；用于一次性迁移 |
| `meta.commonConfigEnabled` | `true` | 前端/后端统一按 true 缺省；迁移时显式写入 |
| `meta.commonConfigRemoved` | `[]` | JSON Pointer，仅在该 Provider 启用通用配置时生效 |
| `appliedSettings.paths` | `[]` | 上次激活写入的叶子路径；旧数据缺失时不做清理 |

> 不建议用 `null` 作为删除哨兵：`settings.json` 中 null 是合法值。

## 4. 合并语义

### 4.1 层级与优先级

```
第 1 层（最低）  用户 ~/.claude/settings.json 现状（用户独有键全部保留）
第 2 层          通用配置 commonConfig.settings（默认值）
第 3 层（最高）  Provider 覆盖层 settingsConfig（provider 显式值）
始终最高          Claude Code 自管状态键（不参与合并，用户现状保留）
```

- 通用配置的启用条件：`commonConfig.enabled && (meta.commonConfigEnabled ?? true)`。
- 全局 `commonConfig.enabled=false` 时忽略第 2 层，Provider 覆盖层仍生效。
- 项目级 `.claude/settings.json` / 本地级 `.claude/settings.local.json` 完全不碰；
  Claude Code 运行时仍按自身层级让项目/本地配置优先。

### 4.2 effective 解析（纯函数）

```
resolve(common, provider):
  overrides = clone(provider.settingsConfig)
  if !common.enabled or !(provider.meta.commonConfigEnabled ?? true):
      return overrides

  base = clone(common.settings)
  for p in provider.meta.commonConfigRemoved:
      delete_at_json_pointer(base, p)     // 不存在则忽略

  return deep_merge(base, overrides)      // overrides 赢
```

- `deep_merge`：对象递归；数组/标量整体覆盖。
- 删除路径只抑制「继承」，不会删除 Provider 显式覆盖的值。
- 空对象 `{}` 视为「无意见」：`env: {}` 不会清空通用配置的 env。

### 4.3 与用户 settings.json 的合并（激活）

```
activate(providerId):
  cfg       = load_providers_config()
  provider  = find(providerId)
  effective = strip_cc_state_fields(resolve(cfg.commonConfig, provider))

  dir  = claude_config_dir()            // 尊重 CLAUDE_CONFIG_DIR（复用 store.rs 实现）
  file = dir/settings.json
  current = read_json(file)             // 不存在 → {}
                                        // 解析失败 → Err，不写入（保留原文件）

  backup(file)                          // settings.json.cc-box.bak（覆盖前）

  stale = cfg.appliedSettings.paths - leaf_paths(effective)
  for p in stale:
      delete_at_json_pointer(current, p)   // 只删 cc-box 上次写过、本次不该存在的键

  merged = deep_merge(current, effective)
  write_atomic(file, merged)

  cfg.appliedSettings = { providerId, paths: leaf_paths(effective) }
  cfg.activeProviderId = providerId
  save_providers_config(cfg)
```

关键点：

- **状态键保护**：`enabledPlugins`、`disabledPlugins`、`extraKnownMarketplaces`、`hooks`
  从 effective 中剥离（resolve 后、激活前各剥一次作为防御）。
- **只清理自己写过的路径**：`appliedSettings.paths` 只包含上次 effective 的叶子路径；
  用户自己加的键不在其中，不会被删。
- **用户现状始终是底座**：Provider 没有的键保留；Provider 有的键覆盖。
- **数组**：视为叶子整体替换（与 JSON Merge Patch 一致）。路径也按数组路径记录。
- **类型变化**：以新 effective 的类型为准；旧路径若不再存在则按 stale 删除。
- **原子写**：同目录临时文件 + rename，避免半截文件。
- **坏 JSON**：返回可操作错误（提示修复或手动恢复备份），绝不静默当成 `{}` 覆盖。

### 4.4 数组与空值规则

| 场景 | 规则 |
|---|---|
| 普通数组 | 整体替换（不是并集） |
| `permissions.allow` / `deny` | 按整体替换处理；**新建空白 Provider 的默认模板不再注入空数组**，避免误清用户权限 |
| `null` | 合法值，正常覆盖；不作为删除标记 |
| 空对象 | 无意见，递归后底座保留 |
| 类型不一致（对象 vs 标量） | 高层整体替换 |

> `permissions` 是否改为并集是独立产品决策，本设计先保持「整体替换 + 默认模板不注入空数组」，
> 避免引入「无法删除权限项」的新问题。

## 5. 编辑与保存（Provider 面板）

### 5.1 展示

- 打开面板时展示 `effective`（与当前 UX 一致），并标注：
  - 顶部提示「已合并通用配置（N 项）」，可展开继承路径列表；
  - 继承值只作为展示；用户修改后即成为覆盖值。
- 实时编辑仍在本地 ref 中进行，不自动保存。

### 5.2 保存：effective → 覆盖层

```
newEffective = 编辑器 JSON
if !commonActive:
    overrides = newEffective
    removed   = []
else:
    overrides = diff_against_common(newEffective, common.settings)
    removed   = update_removed_paths(oldEffective, newEffective, oldRemoved, common.settings)
```

`diff_against_common`：

- 递归比较；与通用配置**值相等**的叶子/数组 → 不写入覆盖层（自动继承）；
- 值不同、通用配置没有的路径 → 写入覆盖层；
- 对象类型不一致 → 保留整个子树。

`update_removed_paths`：

- 旧 effective 中存在、新 effective 中不存在、且来自通用配置的路径 → 加入 removed；
- removed 中已在新 effective 出现的路径 → 移除；
- removed 中路径已被覆盖层包含或父路径被覆盖 → 归一化移除。

### 5.3 勾选/取消

- 勾选：重新按 `effective` 渲染（继承值出现），不写盘。
- 取消：只改 `meta.commonConfigEnabled=false`；编辑器切换为「仅覆盖层」视图。
  由于通用值从未物化，取消是**完全可逆**的；再次勾选即恢复。
- 取消时若通用配置贡献了字段，给出提示（「将隐藏来自通用配置的 N 个字段」）。

### 5.4 外部更新的处理

- 通用配置在别处被保存时，若编辑面板有未保存修改（dirty）：
  **不覆盖编辑内容**，只更新「通用配置已变化」提示；用户可手动重新应用。
- 非 dirty 状态下按新 common 重新计算 effective。

## 6. 通用配置保存

```
update_common_config(enabled, settings):
  validate settings is object
  cfg.commonConfig = { enabled, settings: strip_cc_state_fields(settings) }
  save_providers_config(cfg)           // 不再遍历/改写任何 Provider
```

- 前端保存后：重载配置；若存在 active provider，重新 `activate(active)` 让新默认生效。
- 不再批量物化，因此不会再出现「保存通用配置覆盖 Provider 专属值」。
- 通用配置允许包含 `env` 等字段（作为默认值），Provider 显式值始终优先。
  这解决了导入路径与更新路径不一致（导入剥离核心 env、更新不剥离）的历史问题。

## 7. 迁移（schema v1 → v2）

时机：应用启动时调用 `ensure_providers_schema()` 落盘迁移。
读命令保持纯读（legacy 文件仅在内存中按 v2 解释）；写命令保存时自然持久化迁移结果。
若启动迁移失败，日志记录错误，后续写操作仍以 v2 语义保存（可能没有 `providers.json.cc-box.bak`）。

算法（仅对 `commonConfig.enabled && meta.commonConfigEnabled !== false` 的 Provider）：

```
overrides = remove_equal_leaves(provider.settingsConfig, common.settings)
```

- 与通用配置当前值相等的叶子/数组递归移除（这些值本来就是上次合并写入的）；
- 不同或通用配置没有的键保留为显式覆盖；
- 对已物化的值，迁移前后 **effective 一致**；但旧版在 common 保存之后创建的 Provider
  可能缺少 common 叶子，升级后会开始继承——这是分层语义的预期变化，不是数据丢失；
- Provider 显式值恰好等于 common 时会被归一化为「继承」，common 日后变化会影响该值；
- `meta.commonConfigEnabled` 显式写为布尔值，消除缺省歧义；
- `meta.commonConfigRemoved` 初始化为 `[]`；
- `schemaVersion = 2`；
- 迁移前备份 `providers.json.cc-box.bak`，原子写入；
- 迁移幂等：只按 schemaVersion 执行一次（不能反复跑，否则 common 后续变化会误删后来形成的显式覆盖）。

已知不可恢复项：旧实现里被通用配置覆盖掉的 Provider 原值已经丢失，迁移后只能保持当前值；
用户需要重新填写。此点写入发布说明。

cc-switch 导入：

- 保持现有读取方式（providers + common），导入后对每个 Provider 执行同一个
  `remove_equal_leaves` 归一化，再落盘 schema v2；
- 保留未知的 `meta` 字段，保持兼容。

## 8. 前端/后端一致性

- Rust：`resolve_effective()`、`subtract_equal()`（diff）、`migrate_config()`、
  `leaf_paths()`、JSON Pointer 工具，作为激活/迁移的权威实现。
- 前端：同一算法的 TS 版本（`resolveEffective` / `diffAgainstCommon` / `computeRemovedPaths`），
  放在 `src/utils/provider-config.ts`；`computeRemovedPaths` 目前仅前端使用（后端只校验根节点），
  设计上由编辑面板负责选择「哪些继承项被显式屏蔽」。
- 防漂移：`tests/fixtures/provider-merge/*.json` 共享样例，Rust 与 Vitest 各跑一遍，
  断言 effective / overrides / removed 结果一致（与 remote protocol 契约测试同思路）。
- 前端不得自行拼一套不同的优先级；任何新规则先进 fixture，再改两端。

## 9. 测试计划

Rust（`src-tauri/src/tests/providers.rs`）：

1. resolve：Provider 优先、common 补缺、两层开关、removed 路径、类型冲突、数组替换。
2. 迁移：effective 前后一致、幂等、只迁移一次、缺省 meta 显式化、禁用 common 的 Provider 不动。
3. 激活：用户独有键保留、4 个状态键保留、stale 路径删除、用户新增键不删、
   数组路径清理、`CLAUDE_CONFIG_DIR` 生效、坏 JSON 拒绝写入、备份与原子写。
4. 通用配置保存：不改写 Provider；重新激活后新默认生效。
5. 导入：归一化后 effective 与导入前一致。

前端（`tests/`）：

1. 编辑面板：effective 展示、保存 diff 成覆盖层、与 common 相等的值不落盘。
2. 勾选/取消可逆；取消时只改 meta。
3. 删除继承项 → removed 路径；恢复值 → removed 移除。
4. dirty 状态下 common 外部更新不覆盖编辑内容。
5. 共享 fixture 契约测试。

## 10. 实施阶段

| 阶段 | 内容 | 主要文件 |
|---|---|---|
| P1 后端语义 + 迁移 | schema v2、resolve、migration、update_common_config 去物化、CLAUDE_CONFIG_DIR、备份/原子写、坏 JSON 报错 | `src-tauri/src/providers.rs`、`store.rs`、`lib.rs`、`tests/providers.rs` |
| P2 激活清理 | `appliedSettings` 记录与 stale 路径删除、测试 | `providers.rs`、`tests/providers.rs` |
| P3 前端编辑 | effective 展示、diff 保存、removed 管理、dirty 保护、共享 fixture | `ProviderEditPanel.vue`、`CommonConfigPanel.vue`、`stores/providers.ts`、`utils/provider-config.ts`、`tests/` |
| P4 兼容与文档 | cc-switch 导入归一化、协议/i18n、更新 `docs/provider-management.md`、发布说明 | 文档与测试 |

P1+P2 可先上线（不改 UI 也能保证激活正确、通用配置不再物化），P3 完成可逆编辑体验。

## 11. 风险与回滚

- **迁移是一次性写盘**：必须先备份。迁移后降级到旧版本时，旧版会把 `settingsConfig`
  当作完整配置激活——通用默认层不会再写入 `settings.json`，需要手工处理或恢复备份；
  升级前请先复制 `providers.json`。
- **优先级变化**：旧用户中「依赖通用配置覆盖 Provider」的场景会改为 Provider 优先；
  这是本次要修的语义，发布说明需明确。
- **appliedSettings 删除**：只删自己上次写过的叶子路径；若用户手工改过同一键，会随 Provider 管理被覆盖/清理，
  与「cc-box 管理这些键」的定位一致。
- **数组清理**：数组按路径整段管理；如果用户在同一数组里手工追加过内容，切换 Provider 时可能被整段替换。
  文档需注明；后续可评估 permissions 并集方案。
- 回滚：保留 `providers.json.cc-box.bak` 与 `settings.json.cc-box.bak`；迁移前可手工恢复。

## 12. 待确认决策

1. 通用配置是否允许包含核心 6 个 env（`ANTHROPIC_*`）？
   建议：允许（Provider 显式值优先，缺省时作为兜底）；若不希望共享 API Key，可在 UI 保存时剥离。
2. `permissions.allow/deny` 是否改为并集？
   建议：本期不做，先移除空白模板中的空数组，避免误清；后续单独立项。
3. 取消勾选时是否提示「隐藏 N 个继承字段」？
   建议：提示一次，不阻塞。
4. 是否需要「导出/复制 effective 为独立配置」的显式冻结操作？
   建议：不做，保持分层可逆。
