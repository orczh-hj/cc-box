> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 应用网关支出限制

> 通过 Claude 应用网关为每个开发者按天、周或月设置支出上限。使用 Admin API 设置限制，网关在每个请求上实时执行这些限制。

支出限制限制了每个开发者在给定的一天、一周或一个月内通过你的 [Claude 应用网关](https://code.claude.com/docs/zh-CN/claude-apps-gateway) 可以花费的金额。当开发者超过他们的上限时，网关在他们的下一个请求上返回 `429`，并阻止他们直到该周期重置或管理员提高上限。使用支出限制为每个开发者、团队或整个组织设置一个共享凭证的上限。

默认情况下，Claude 应用网关通过一个共享的上游凭证转发所有推理，因此你的提供商的账单将所有内容归属于该凭证，而不是单个开发者。在 Amazon Bedrock 上，[按开发者的 AWS 成本归属](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#per-developer-aws-cost-attribution) 改变了这一点。没有按开发者的限制，一个失控的代理群可能会花费组织的整个承诺。支出限制是网关在该共享账单之上的按开发者视图和断路器。

## 设置上限

配置了 [`admin:`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin) 块在 `gateway.yaml` 中后，网关在 `/v1/organizations/spend_limits` 处提供一个 admin API，并在每个推理请求上实时执行上限。上限本身通过该 API 设置，而不是在 `gateway.yaml` 中；每个 `POST /v1/organizations/spend_limits` 请求从 `{scope, amount, period}` 创建或替换一个上限。该 API 镜像了 Anthropic 的公共 [Admin API](https://platform.claude.com/docs/en/manage-claude/admin-api) 支出限制端点的线路形状，因此针对该契约编写的 HTTP 客户端可以通过更改其基础 URL 来针对网关。

此请求为每个开发者设置了一个组织范围的默认值，每月 \$500：

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

此请求在 `contractors` 组的每个成员上分层了一个更严格的每天 \$100 的上限：

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| 字段 | 值 | 描述 |
| - | - | - |
| `scope.type` | `user`, `rbac_group`, `organization` | `user` 通过其 OpenID Connect (OIDC) `sub`（你的身份提供商分配的稳定用户 ID）针对一个开发者；将其作为 `scope.user_id` 传递。`rbac_group` 通过名称针对一个 [IdP 组](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config.md#managed)；将其作为 `scope.rbac_group_id` 传递。`organization` 是组织范围的默认值。网关接受所有三个；Anthropic 的公共 `POST` 目前仅限用户。 |
| `amount` | USD 美分的整数字符串，或 `null` | `null` 是无限制的。`"0"` 是零上限，它阻止每个请求。 |
| `period` | `daily`, `weekly`, `monthly` | 一个作用域可以为每个时期保持一个上限，每个都独立执行：如果开发者超过其中任何一个，他们就会被阻止。 |

组或组织上限是每个成员继承的按座位默认值，而不是共享池。每个时期，开发者的有效上限按以下顺序解决：按用户覆盖，然后是其组上限中最严格的，然后是组织默认值，然后是无限制。[`admin.group_limit_mode: max`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin) 将多组平局打破翻转为最不严格的。

### 向 admin API 进行身份验证

发送以下之一：

* 一个 `x-api-key` 标头，匹配 [`admin.write_keys`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin) 中的一个密钥以获得完全访问权限，或 `admin.read_keys` 以获得仅 `GET` 访问权限。每个密钥都有一个 `id`，在审计日志中显示为 `admin-key:<id>`，因此为 Terraform、CI 和每个自动化提供自己的密钥。
* 一个网关承载令牌，其 `groups` 声明包括 [`admin.admin_groups`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin) 中的一个。这是完全访问权限，审计为 `oidc:<sub>`，因此对人类管理员更好。

## 执行如何工作

在每个 `/v1/messages` 请求上，网关在一个 Postgres 查询中查找开发者的上限和期间至今的支出。超过任何上限的开发者会获得 `429` 和 `error.type: billing_error` 以及标头 `x-should-retry: false`。

消息命名该期间和重置时间，例如 `spend limit reached (daily; resets 2026-08-08 00:00 UTC)`，后跟你的 [`admin.blocked_message`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin)（如果设置）。当开发者同时超过多个上限时，消息命名最后重置的上限。响应还包含一个 `retry-after` 标头，其中包含直到该重置的剩余秒数。在网关服务器上的 v2.1.225 之前，消息是 `spend limit reached`，没有期间、重置时间或 `retry-after` 标头。

在 v2.1.227 或更高版本上，`<public_url>/protocol` 处的协议参考也列出了确切的使用限制响应标头和 `429` 正文。

上限在 UTC 日历边界重置：每天在 00:00 UTC、周一和每月的第一天。网关从不阻止 `/v1/messages/count_tokens`，因为令牌计数是免费的。

### 请求如何定价

在每个响应之后，使用计量器读取令牌计数并将成本添加到每日、每周和每月计数器。它从不接触发送给客户端的字节，因此计量失败无法破坏响应。这些金额是美元估计值，是断路器而不是发票；对于计费，请根据你的提供商的使用报告进行协调。

计量器按以下顺序为每个请求选择费率：

1. 为提供请求的上游匹配的 [`pricing.overrides`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#pricing) 行。需要 v2.1.227 或更高版本。
2. 上游模型 ID 的列表价格，即网关发送给提供商的字符串，当 Claude Code 成本表识别它时。该表接受 Anthropic、Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry ID 形式。
3. 你映射到该上游 ID 的 [`models[].id`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#models) 的列表价格，对于不包含模型名称的上游字符串，例如 Amazon Bedrock 应用推理配置文件 ARN 或 Microsoft Foundry 部署名称。需要 v2.1.218 或更高版本。
4. 未知模型层级 \$5/\$25 每百万输入/输出令牌，因此计量器无法识别的 ID 永远不会免费。网关在启动时和运行时每个 ID 一次警告何时使用此层级。

无论应用哪种费率，计量器然后将金额乘以 [`pricing.multiplier`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#pricing)，默认为 `1`。

客户端中止也被计费。当流在没有上游最终使用帧的情况下结束时，计量器为已发送给客户端的文本计费约每输出令牌四个字符的下限估计，因此提前中止请求不会规避上限。

#### 长提示词费率

[Claude Haiku 5.5](https://platform.claude.com/docs/en/models/haiku-5-5/overview) 的列表价格对[超过 token 阈值的提示词采用更高费率](https://code.claude.com/docs/zh-CN/model-config.md#haiku-5-5-context-window-and-pricing)。当 Haiku 5.5 处理提示词超过该阈值的请求时，运行 Claude Code v2.1.293 或更高版本的网关服务器会按该费率为整个请求定价。计量器将输入 token、缓存读取和缓存写入计入该阈值。

如果您的某个 `pricing.overrides` 行与请求匹配，则无论提示词长度如何，都适用该行的费率。

### Postgres 可用性

预检查使用两秒超时查询 Postgres。如果存储无法访问或超时，执行默认情况下失败打开：请求继续，网关记录警告，响应不包含 `anthropic-ratelimit-unified-*` 标头。设置 [`enforcement.fail_closed_on_error: true`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#enforcement) 改为失败关闭，它返回相同的 `429 billing_error`，但消息为 `spend limit unavailable`，没有期间、重置时间或 `retry-after` 标头。失败打开防止存储中断成为推理中断；失败关闭保证没有无计量支出。

失败打开仅在你的负载均衡器或编排器仍然将流量路由到网关时有帮助。有关 `store.readiness_grace_seconds` 的信息，请参阅 [Outage behavior](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#outage-behavior)，它使副本在短暂中断期间通过其就绪检查。

### Claude Code 中的使用警告

Claude Code 在开发者接近其上限时向其发出警告：一旦利用率超过 75% 时警告一次，超过其消耗最多的上限的 95% 时再次警告。当网关阻止请求时，Claude Code 按原样显示网关的 `429` 消息，包括您的 `admin.blocked_message`。它还会在 `/usage` 中显示该上限，并将其传递给开发者的[状态栏](https://code.claude.com/docs/zh-CN/statusline.md#spend-limit-fields)脚本。

每种显示都需要开发者机器和网关服务器上具备最低 Claude Code 版本：

| 开发者看到的内容 | 开发者的机器 | 网关服务器 |
| :- | :- | :- |
| 75% 和 95% 时的用量警告 | v2.1.225 或更高版本 | v2.1.225 或更高版本 |
| `/usage` 中的 **Spend limit** 栏，显示其上限已使用的百分比和重置时间，以及状态栏输入中的 `rate_limits.spend_limit` 对象 | v2.1.251 或更高版本 | v2.1.225 或更高版本 |
| **Spend limit** 栏中以美元表示的估计支出和上限，例如"\$271.40 / \$500.00 spent this month"，以及状态栏输入中的相同金额和上限期间 | v2.1.284 或更高版本 | v2.1.284 或更高版本 |

警告和百分比来自 `anthropic-ratelimit-unified-*` 标头，网关会将这些标头添加到具有上限的开发者的每个成功 `/v1/messages` 响应中。标头始终描述开发者自己的上限：网关会剥离上游提供商的速率限制标头（这些标头描述您的共享配额），从不转发它们。

开发者看到的支出是网关自身的[估计值](#how-requests-are-priced)，即网关用于执行上限的同一数值，而不是来自您的提供商账单的金额。Claude Code 通过向网关发送单独的请求来读取美元金额，即使您在开发者的机器上设置了 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`，它也会发送该请求。在 v2.1.295 之前，设置了该变量时 Claude Code 会跳过该请求，`/usage` 和状态栏仅显示百分比和重置日期，不显示美元金额。

## Admin API 参考

下面的端点在 `/v1/organizations/spend_limits` 下提供。

| 方法和路径 | 描述 |
| - | - |
| `GET /v1/organizations/spend_limits` | 列出配置的上限，可选地过滤到 `organization`、`rbac_group` 或 `user` 的一个 `scope_type`。查询：`?limit=&after_id=&before_id=&scope_type=`。 |
| `POST /v1/organizations/spend_limits` | 为 `{scope, period}` 创建或替换上限。 |
| `GET /v1/organizations/spend_limits/{id}` | 通过其 `spl_` 前缀 ID 获取一个上限。 |
| `DELETE /v1/organizations/spend_limits/{id}` | 删除一个上限。返回 `{type: "spend_limit_deleted", id}`。 |
| `GET /v1/organizations/spend_limits/effective` | 每个主体每个时期的已解决上限和至今支出。 |
| `GET /v1/organizations/spend_limits/audit` | 管理员变更跟踪，最新优先。查询：`?limit=&after_id=`。 |

约定镜像 Anthropic 的 Admin API：

* 每个对象上的 `type`
* `spl_` 前缀 ID
* 金额为 USD 美分的整数字符串；`POST` 拒绝任何其他 `currency`，返回 `400`
* `{type: "error", error: {type, message}, request_id}` 错误信封
* 每个管理员响应上的 `request-id` 响应标头，成功或错误；错误正文也将其作为 `request_id` 携带

每个变更在同一事务中向 `admin_audit` 写入前/后行，归属于 `admin-key:<id>` 或 `oidc:<sub>`。

网关仅提供支出限制端点。其他 Admin API 表面，例如 `spend_limit_increase_requests` 队列，不是网关 Admin API 的一部分。

### `/effective`

`GET /v1/organizations/spend_limits/effective` 返回 Anthropic 的 `SpendSummary` 模式：每行是一个主体的一个时期，具有已解决的上限、期间至今的支出和一个 `actor` 对象。网关特定的差异：

* `user_id` 是 OIDC `sub`。
* `actor.name` 和 `actor.email_address` 是 `null`，直到主体通过网关的第一个推理请求。网关没有用户目录；它从每个用户自己的会话 JWT 记录最后看到的值。
* 每行还携带一个 `groups` 数组，主体的最后看到的 IdP 组。这是一个网关扩展，因此管理员 UI 可以显示应用的每个上限层；Anthropic 形状的客户端忽略它。
* 没有 `user_ids[]` 过滤器，它列出有记录支出的主体，因为网关无法枚举所有组织成员。

组源上限根据那些最后看到的组解决，具有与执行使用的相同 `group_limit_mode` 平局打破，因此查看器显示实际应用的上限。

| 查询参数 | 描述 |
| - | - |
| `user_ids[]` | 可重复。按 OIDC `sub` 过滤到特定主体。 |
| `period[]` | 可重复。过滤到 `daily`、`weekly` 或 `monthly` 行。 |
| `sort` | `spend_desc` 首先列出最高支出者。需要恰好一个 `period[]`。 |
| `q` | 对 OIDC `sub`、最后看到的电子邮件和最后看到的显示名称的不区分大小写的子字符串过滤。 |
| `limit` / `page` | 页面大小（1–1000，默认 20）和前一个响应的 `next_page` 中的不透明游标。 |

> [!WARNING]
>
> `q=` 和 `user_ids[]=` 乘坐 GET 查询字符串，因此任何前置代理或负载均衡器在其访问日志中捕获它们。如果你的 PII 日志策略很严格，请在那里清理这些参数。

### `/audit`

返回支出限制变更跟踪：谁更改了哪个上限，具有前/后快照，最新优先。`has_more` 是精确的。此端点遵循本地 Admin API 约定，而不是第一方线路形状。

### 分页

原始列表按 `after_id` 和 `before_id` 分页，它们是互斥的 `spl_…` ID；结果按创建排序，`has_more` 反映遍历方向。`/effective` 按传回的不透明 `next_page` 令牌分页为 `?page=`，主体按升序排序，因此在记录支出时页面保持稳定。`limit` 在两者上都是 1–1000，默认 20，在 `/audit` 上按 `after_id`（前一页上最后一个事件的数字 `id`）分页，其 `limit` 默认为 100。

## 数据生命周期

网关保持四个支出相关的表；每小时扫描执行保留窗口：

| 表 | 内容 | 保留 |
| - | - | - |
| `spend` | 按主体期间至今的计数器（美分） | [`admin.spend_retention_months`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin)，默认 13 |
| `spend_limits` | 配置的上限 | 直到通过 API 删除 |
| `admin_audit` | 变更跟踪 | [`admin.audit_retention_days`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin)，默认 365 |
| `principal_emails` | 每个主体的最后看到的电子邮件、显示名称和 IdP 组。包含 PII。 | [`admin.identity_retention_days`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin) 自上次活动以来，默认 90 |

当开发者离开时，通过 `DELETE /v1/organizations/spend_limits/{id}` 删除任何按用户上限；他们的支出和身份行按上面的保留窗口老化。要立即擦除一个人，用于离职或数据主体访问请求 (DSAR)，直接针对网关数据库运行 `DELETE FROM principal_emails WHERE principal = '<sub>'`。这删除了唯一保存其电子邮件、名称和组的表。`spend` 和 `admin_audit` 行仅引用伪匿名 OIDC `sub`，并按其自己的窗口老化。

## 相关

* [`admin` 和 `enforcement` 配置](https://code.claude.com/docs/zh-CN/claude-apps-gateway-config#admin)：启用 admin API 和调整保留
* [部署指南](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#postgres)：Postgres 模式和备份指导
