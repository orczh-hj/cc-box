> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 应用网关配置

> 每个 gateway.yaml 选项的参考：监听器和 TLS、OIDC、会话、Postgres 存储、Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上游、模型路由、托管策略和遥测。

Claude 应用网关部署由一个 YAML 文件配置，按惯例命名为 `gateway.yaml`。该文件定义网关所做的一切：它在哪里监听、开发者如何登录、推理去往何处，以及应用哪些策略和遥测。本页是该文件中每个选项的参考。

要编写你的第一个配置，请从[快速入门](https://code.claude.com/docs/zh-CN/claude-apps-gateway#quickstart)开始，它构建一个最小的工作配置并运行它。一旦你有了满意的配置，[部署指南](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md)涵盖了在 Kubernetes、Cloud Run 或你自己的平台上容器化和托管它。

网关在启动时使用 `claude gateway --config /path/to/gateway.yaml` 读取该文件一次。每个选项都在启动时根据模式进行验证，因此格式错误的配置在启动时失败并显示字段级错误，而不是在首次使用时失败。

本页末尾的[完整示例](#complete-example)演示了每个部分。

## 文件结构

五个部分是[必需的](#required-sections)。其他所有部分都是[可选的](#optional-sections)，省略的部分采用其默认值。未知的键会导致启动失败，因此拼写错误会显示为命名错误，而不是被静默忽略的设置。

**必需部分：**

* [`listen`](#listen)：绑定地址、公共 URL、TLS 终止
* [`oidc`](#oidc)：您的身份提供商 (IdP)，包括颁发者、客户端、声明映射以及谁可以登录
* [`session`](#session)：网关铸造的持有者令牌，包括密钥和生命周期
* [`store`](#store)：PostgreSQL，用于设备授权和速率限制计数器
* [`upstreams`](#upstreams)：推理的去向，无论是 Anthropic、Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 还是 Microsoft Foundry

**可选部分：**

* [`admin`](#admin)：Admin API 身份验证和支出限制的保留
* [`enforcement`](#enforcement)：支出限制故障开放或故障关闭行为
* [`pricing`](#pricing)：合同费率，以及适用于支出计量器和开发人员所见成本数字的乘数
* [`models`](#models) 和 `auto_include_builtin_models`：管理员策划的模型列表和每个上游 ID
* [`managed`](#managed)：按 IdP 组的托管设置策略
* [`telemetry`](#telemetry)：OTLP 转发到您的可观测性堆栈
* [`access_control`、`limits`、`timeouts`、`rate_limits`](#http-tuning)：IP 允许/拒绝、请求大小上限、上游首字节时间和每 IP 登录限制
* [`load_test_mode`](#load_test_mode)：在不调用模型提供商的情况下对网关进行负载测试

## 密钥扩展

不要直接在 `gateway.yaml` 中写入密钥，如 `client_secret`、`jwt_secret` 或 `postgres_url`。使用下面的一种形式引用它们，网关在启动时从环境变量或文件解析该值：

| 形式 | 解析为 | 用于 |
| - | - | - |
| `${VAR}` | 环境变量 `VAR`。如果未定义，启动失败。 | 容器环境变量、通过环境注入的 AWS Secrets Manager |
| `${file:/path}` | 该绝对路径处的文件内容，已修剪。该引用必须是字段的整个值：与 `${VAR}` 不同，它不会在较长的字符串内展开，因此对于数据库密码，请设置 `store.password` 而不是将其嵌入 `postgres_url`。 | Kubernetes Secret 卷挂载、Vault Agent、SOPS |

## 必需的部分

### `listen`

`listen` 块控制网关的服务位置：绑定地址和端口、外部可见的源以及可选的 TLS 终止。

| 字段 | 必需 | 描述 |
| - | - | - |
| `host` | 否 | 绑定地址。默认 `0.0.0.0`。 |
| `port` | 否 | 绑定端口。默认 `8080`。 |
| `public_url` | 除非 `host` 是环回地址 | 外部可见的 `https://` 源，用于构建 IdP `redirect_uri` 和发现元数据。当 `host` 不是环回地址时需要，无论 TLS 是在代理（如 ALB、Ingress 或 Cloud Run）还是通过 `tls` 在网关本身终止，因为网关永远不会从 `X-Forwarded-*` 标头派生自己的源；这些标头可被客户端欺骗。没有它启动会失败。下面的 `trusted_proxies` 仅控制客户端 IP 解析。还需要启用[遥测](#telemetry)，因为网关从此 URL 构建它推送给客户端的 OTLP 端点。 |
| `tls.cert` / `tls.key` | 否 | 如果网关自己终止 TLS，则为 PEM 路径 |
| `trusted_proxies` | 否 | 网关前面的负载均衡器的 CIDR 或 IP。设置后，网关仅从这些对等方信任 `X-Forwarded-For`，并记录真实客户端 IP 用于按 IP 速率限制和审计。等同于 nginx `set_real_ip_from`。`X-Forwarded-For` 条目写成 `ipv4:port` 或 `[ipv6]:port`（如某些负载均衡器所做），读取时端口被丢弃。带有端口附加且无括号的 IPv6 地址可能被读取为不同的地址或根本不被读取，因此在任何写入该形式的代理上关闭端口选项。 |

### `oidc`

`oidc` 块将网关连接到您的身份提供商，并决定谁可以登录。它命名发行者和 OAuth 客户端，映射携带电子邮件和组的声明，并按电子邮件域或组限制登录。

OpenID Connect (OIDC) 是网关与您的身份提供商一起使用的 SSO 协议；有关在 IdP 端注册的内容，请参阅[身份提供商设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#identity-provider-setup)。

| 字段 | 必需 | 描述 |
| - | - | - |
| `issuer` | 是 | OIDC 发现基础。必须在 `/.well-known/openid-configuration` 提供发现。在生产中使用 HTTPS；网关接受 `http://` 发行者。环回发行者（如 `http://localhost:8081`）被[SSRF 防护](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#threat-model-summary)拒绝，除非在网关的环境中设置了 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`。 |
| `client_id` | 是 | 来自您的 OAuth 客户端注册 |
| `client_secret` | 除非 `token_endpoint_auth_method` 为 `private_key_jwt` | 来自您的 OAuth 客户端注册。使用[证书客户端身份验证](#certificate-client-authentication)时省略此项。 |
| `allowed_email_domains` | 否 | 拒绝 `email` 声明不在这些域之一中的 id\_tokens，不区分大小写。针对多租户 IdP 配置错误的深度防御。独立于此设置，`email_verified` 声明明确为 `false` 的 id\_token 总是被拒绝。 |
| `allowed_groups` | 否 | 限制登录仅限于这些 IdP 组的成员，与 `groups_claim` 匹配。处于允许的电子邮件域但不在这些组中的用户被拒绝。需要 IdP 发出组声明。匹配是对该声明中的值的精确、区分大小写的字符串比较，网关不展开嵌套组：要允许子组的成员，在此列出子组或配置 IdP 发出扁平化成员资格。 |
| `groups_claim` | 否 | 哪个 id\_token 声明携带组成员资格。默认 `groups`。Microsoft Entra 在 `roles` 下发出应用角色。接受平面键或 RFC 6901 JSON 指针（如 `/resource_access/gateway/roles`）用于嵌套声明。 |
| `google_groups` | 否 | 通过 Google Workspace Admin SDK Directory API 查找已登录用户的组，因为 Google 的 id\_token 不携带组声明。将 `service_account_json_path` 设置为具有 `https://www.googleapis.com/auth/admin.directory.group.readonly` 范围的域范围委派的服务帐户密钥文件，并将 `admin_email` 设置为服务帐户模拟的 Workspace 管理员；Directory API 需要真实的管理员主体。每个用户的组电子邮件地址成为他们的组声明，因此 `allowed_groups` 和 `managed.policies.match.groups` 匹配组电子邮件。 |
| `email_claim` | 否 | 哪个 id\_token 声明携带用户的电子邮件。默认 `email`。某些 IdP（如 ADFS 和 Entra B2C）改为发出 `upn` 或 `preferred_username`。接受平面键、JSON 指针或回退键列表，其中使用第一个存在的键。 |
| `scopes` | 否 | 网关请求的 OIDC 范围的完全覆盖。默认 `[openid, profile, email, offline_access]`。当您的 IdP 拒绝它不识别的范围或需要自定义范围来发出组或电子邮件时设置。必须包括 `openid`。删除 `offline_access` 会禁用刷新令牌，因此开发人员每 `session.ttl_hours` 重新运行浏览器登录。有关每个 IdP 范围配方（如 Google 的刷新令牌流），请参阅[身份提供商设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#identity-provider-setup)。 |
| `scope_on_refresh` | 否 | 当网关交换刷新令牌时，也发送 `scope`，与登录请求相同的列表。默认 `false`：刷新请求省略 `scope`。大多数 IdP 在每次刷新时返回 id\_token，不需要这个。当您的 IdP 仅在再次请求 `openid` 时才在刷新时返回 id\_token 时设置 `true`，Okta 为其刷新授权记录了这一点。没有 id\_token，每次刷新都取决于 IdP 的 userinfo 端点接受刷新的访问令牌。如果您在登录或匹配策略上设置了组，并且您的 IdP 的刷新时间 id\_token 省略了它们，也设置 `userinfo_fallback: true` 以便网关从 userinfo 端点填充它们。授予的范围少于请求的 IdP 可以用 `invalid_scope` 拒绝刷新，包括如果您在此打开时向 `scopes` 添加条目的现有会话。如果在设置后刷新开始在 `token_endpoint` 失败，请取消设置该键。需要网关服务器上的 Claude Code v2.1.260 或更高版本。 |
| `extra_auth_params` | 否 | 附加到 IdP 授权请求的额外查询参数，逐字。这是 IdP 特定行为的覆盖机制，如 Google 刷新令牌的 `access_type: offline`、某些 Entra 租户的 `domain_hint` 或分步流的 `acr_values`。无法覆盖网关管理的协议参数：`state`、`nonce`、`redirect_uri`、PKCE、`scope`、`response_type`、`response_mode` 和 `client_id`。 |
| `userinfo_fallback` | 否 | 当 id\_token 省略电子邮件或组时，从 `/userinfo` 获取它们。Keycloak 轻量级访问令牌、Okta 组织服务器和 ADFS 最小令牌需要。id\_token 保持权威；userinfo 仅填补空白。默认 `false`。 |
| `use_pkce` | 否 | 在授权请求上发送 PKCE (S256) 质询。默认 `true`。仅当您的 IdP 为此机密客户端拒绝 PKCE 时设置 `false`。 |
| `clock_skew_seconds` | 否 | 验证 id\_token 时间声明时容忍时钟漂移。默认 `0`，严格。如果您在登录后立即看到"令牌已过期/尚未有效"错误，请提高以应对主机/IdP 时钟偏差。 |
| `token_endpoint_auth_method` | 否 | 网关向 IdP 令牌端点进行身份验证的方式：`client_secret_basic`、`client_secret_post`，或用于[证书客户端身份验证](#certificate-client-authentication)的 `private_key_jwt`。默认情况下，网关根据 IdP 公布的内容从两种 `client_secret` 方法中选择一种。 |
| `client_assertion` | 使用 `private_key_jwt` 时 | 包含 `private_key_pem` 和 `certificate_pem` 的块：用于[证书客户端身份验证](#certificate-client-authentication)的私钥和证书。需要 v2.1.284 或更高版本。 |
| `id_token_signed_response_alg` | 否 | 预期的 id\_token 签名算法。默认 `RS256`。为使用 ES256、PS256 或 EdDSA 签名的 IdP 设置。 |
| `additional_authorized_parties` | 否 | 除 `client_id` 外要接受的额外 `azp` 值，用于 Keycloak 代理和令牌交换流 |
| `discovery_url` | 否 | 从此 URL 获取发现文档而不是从 `issuer` 派生，用于代理后面重写发行者主机的 IdP。路径必须包含 `/.well-known/`。 |
| `use_proxy` | 否 | 通过 `HTTPS_PROXY` 或 `HTTP_PROXY` 中的前向代理发送网关自己的 IdP 请求，遵守 `NO_PROXY`。`false` 保持这些请求直接。需要 v2.1.227 或更高版本；请参阅下面的[通过前向代理的 IdP 请求](#idp-requests-through-a-forward-proxy)。 |
| `form_action_origins` | 否 | `/device` 页面的 `Content-Security-Policy: form-action` 指令的其他源。网关已允许 `'self'` 和发现的 `authorization_endpoint` 源，但 Chrome 对整个重定向链强制执行 `form-action`。如果您的 IdP 通过第二个主机重定向，如 Azure AD 联合到 ADFS、中心辐射 Okta 或公司 SSO 拦截器，列出授权请求可能重定向通过的每个源。 |
| `ca_cert_pem` | 否 | PEM 编码的 CA 证书本身，不是文件的路径。它仅替换 IdP 请求的系统信任存储。要加载挂载的文件，请写 `${file:/etc/gateway/idp-ca.pem}`。用于公司 PKI 后面的 Keycloak 或 Dex。 |

#### 证书客户端身份验证

如果您的身份提供商使用证书而不是客户端密钥对 OAuth 客户端进行身份验证（如 Microsoft Entra 使用证书凭据），请设置 `token_endpoint_auth_method: private_key_jwt`。需要网关服务器上的 Claude Code v2.1.284 或更高版本。

使用此配置时，网关不发送任何密钥。当开发人员登录时以及网关每次刷新其会话时，网关使用由证书私钥签名的短期 JWT 向 IdP 的令牌端点进行身份验证。该 JWT 使用 RS256 签名，并通过 `x5t` 和 `x5t#S256` 指纹标头而不是 `kid` 来标识证书。您的 IdP 必须能够按指纹找到已注册的证书。

**1. 创建密钥和证书**

创建一个至少 2048 位的未加密 RSA 私钥（PKCS#8 或 PKCS#1 PEM 格式），并为其创建证书。对于任何不满足这些条件的密钥，网关都会拒绝启动。以下 `openssl` 命令会创建这样的密钥以及有效期为一年的自签名证书：

```bash theme={null}
openssl req -x509 -newkey rsa:2048 -nodes -keyout idp-client.key -out idp-client.crt -days 365 -subj "/CN=claude-gateway"
```

它会将 `idp-client.key` 和 `idp-client.crt` 写入当前目录。将这两个文件复制或挂载到网关可以读取的位置。第 3 步中的示例使用 `/etc/gateway/`。

**2. 将证书上传到 IdP**

将证书（而不是私钥）上传到 IdP 上网关的应用注册。

**3. 将密钥和证书添加到 gateway.yaml**

在 `client_assertion` 块中向网关提供私钥和证书。省略 `client_secret`，因为当它与 `private_key_jwt` 一起设置时，网关会拒绝启动。以下 `oidc` 块使用证书向 Microsoft Entra 租户对网关进行身份验证：

```yaml theme={null}
oidc:
  issuer: https://login.microsoftonline.com/<tenant-id>/v2.0
  client_id: <application-id>
  token_endpoint_auth_method: private_key_jwt
  client_assertion:
    private_key_pem: ${file:/etc/gateway/idp-client.key}
    certificate_pem: ${file:/etc/gateway/idp-client.crt}
```

这两个值都是 PEM 内容，而不是文件路径，因此请像示例那样使用 `${file:/path}` 加载挂载的文件。除非 `certificate_pem` 是单个 PEM 证书（不含证书链的其余部分）且其公钥与 `private_key_pem` 匹配，否则网关会拒绝启动。

**4. 重启网关并检查启动日志**

重启网关，并在启动日志中找到以下行：

```text theme={null}
[gateway] 2026-10-01T23:07:40.512Z info oidc: client authentication private_key_jwt; certificate CN=claude-gateway, SHA-1 thumbprint DE92821854EE8BAA1D98C758FAA04AABE80B9F57, expires Oct  1 23:07:31 2027 GMT
```

将该 SHA-1 指纹与 IdP 为您上传的证书显示的指纹进行比较。如果证书已过期或尚未生效，网关仍会启动，但会记录一条警告，说明在您替换证书之前登录和刷新都将失败。要确认 IdP 接受该证书，请让一位开发人员通过网关登录。

#### 轮换客户端证书

网关在启动时读取一次密钥和证书，因此更改后的文件仅在重启后生效。请按以下顺序轮换，以确保任何令牌请求都不会出示 IdP 中不存在的证书：

1. 将新证书与旧证书一起上传到 IdP。
2. 替换 `gateway.yaml` 加载的密钥和证书文件，然后重启网关。如果您运行多个副本，可以使用[滚动重启](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#upgrades)，因为在您删除旧证书之前，IdP 同时拥有这两个证书。
3. 在每个副本都重启之后，从 IdP 中删除旧证书。

#### 通过前向代理的 IdP 请求

推理上游在每个版本上都遵守 `HTTPS_PROXY` 和 `HTTP_PROXY`。网关自己对 IdP、发现、JWKS、令牌和 userinfo 的请求直接进行，除非您设置 `oidc.use_proxy: true`，这需要 v2.1.227 或更高版本。设置代理变量、`use_proxy` 未设置且发行者不被 `NO_PROXY` 覆盖时，网关保持这些请求直接并在启动时记录通知，要求您选择；`use_proxy: false` 保持它们直接并沉默通知。

使用 `use_proxy: true`，pod 自己解析每个 IdP 端点的主机名，并要求代理 `CONNECT` 到解析的 IP 地址，因此代理必须接受 `CONNECT` 到发现文档命名的每个主机的 IP 地址，不仅仅是发行者。使用 `http://` 代理 URL。`ca_cert_pem` 和[SSRF 防护](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#threat-model-summary)也适用于代理路径。

[仅代理出口](#proxy-only-egress)改变这两者：当它活跃时，IdP 请求遵循代理，除非您设置 `use_proxy: false`，网关将每个 IdP 主机名交给代理而不首先解析它。

#### 仅代理出口

在网关的环境中设置 `CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1`，在 `HTTPS_PROXY` 旁边，当 pod 仅通过该前向代理到达其他主机且无法自己解析公共 DNS 名称时，或当代理拒绝 `CONNECT` 到 IP 地址时。需要 v2.1.277 或更高版本。它是环境变量而不是 `gateway.yaml` 键，以便配置文件中的任何内容都无法放松网关的地址检查。

```bash theme={null}
export HTTPS_PROXY=http://proxy.corp.example.com:3128
export NO_PROXY=
export no_proxy=
export CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1
```

网关在仅代理出口活跃时在启动时记录一条 `network:` 行。

下面的每一行是具有 `HTTPS_PROXY` 设置的网关上的一类出站请求，默认情况下和仅代理出口活跃时。

| 出站请求 | 默认 | 仅代理出口活跃 |
| - | - | - |
| `provider: anthropic` 上游、Workload Identity Federation 令牌交换、`telemetry.forward_to` 导出 | 在本地解析和检查，然后通过代理 `CONNECT` 到检查的 IP 地址。`NO_PROXY` 中列出的遥测收集器改为直接到达 | 主机名交给代理 |
| IdP 发现、JWKS、令牌和 userinfo | 直接，除非[`oidc.use_proxy: true`](#idp-requests-through-a-forward-proxy)，然后 `CONNECT` 到检查的 IP 地址 | 主机名交给代理，除非 `oidc.use_proxy: false` 保持内部 IdP 直接 |
| Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上游；Google 组查找 | 主机名交给代理 | 不变 |

仅代理出口保持关闭，除非网关的环境满足所有这三个条件：

* `HTTPS_PROXY` 或 `HTTP_PROXY` 已设置。
* `NO_PROXY` 和 `no_proxy` 为空。如果您的平台将任一个注入到 pod，在网关容器上将两者设置为空值。在 `NO_PROXY` 中列出遥测收集器保持仅代理出口关闭。
* `CLAUDE_GATEWAY_ALLOW_LOOPBACK` 未打开。pod 自己环回上的收集器或 IdP 无法与仅代理出口结合，因为交给代理的环回地址将是代理主机自己的，因此给这些服务一个代理可以到达的地址。出于同样的原因，网关在仅代理出口活跃时完全拒绝 `localhost` 风格的名称。

当这些条件之一未满足时，网关在启动时记录警告，命名停止它的变量，并保持默认行为。

一旦仅代理出口活跃，允许代理中的每个目的地，包括内部收集器和任何由 IP 地址配置的主机。您仍然可以使用[`oidc.use_proxy: false`](#idp-requests-through-a-forward-proxy)保持内部 IdP 直接。

> [!WARNING]
>
> 仅当代理的允许列表至少与网关自己的检查一样严格时才打开此功能。代理必须拒绝云元数据端点（如 `169.254.169.254` 和 `metadata.google.internal`）、链路本地地址和代理主机自己的环回，并且必须按名称解析到的地址拒绝它们，而不仅仅按名称，因为网关不再捕获解析到其中之一的主机名。连接到任何要求的地方的代理会移除网关对这些请求的[SSRF 防护](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#threat-model-summary)。

### `session`

`session` 块塑造网关在登录后铸造的持有者令牌：签署它们的秘密和它们的生存时间。

| 字段 | 必需 | 描述 |
| - | - | - |
| `jwt_secret` | 是 | 至少 32 字节的熵，例如来自 `openssl rand -base64 32`。签署网关的 HS256 持有者令牌。接受单个字符串或用于轮换的数组：索引 0 签署，所有条目验证。要轮换，前置新秘密，等待 `ttl_hours`，然后删除旧秘密。 |
| `ttl_hours` | 否 | 网关持有者令牌生存期。默认 `1`。当 IdP 发出刷新令牌时，CLI 在过期前静默刷新。较短的生存期更快地取消配置；较长的生存期减少 IdP 往返。如果您的 IdP 因为 `offline_access` 不可用而无法发出刷新令牌，则没有静默刷新，因此将其提高到 `8` 或 `12` 以避免每小时将开发人员发送回浏览器登录。 |

### `store`

`store` 块将网关指向其 PostgreSQL 数据库，该数据库保存设备授权和速率限制计数器。

| 字段 | 必需 | 描述 |
| - | - | - |
| `postgres_url` | 是 | `postgres://` 或 `postgresql://` URL，只能包含一个主机，不能是逗号分隔的列表。网关在启动和升级时运行自己的 schema 迁移，因此角色需要在目标 schema 上创建和更改表的权限。请参阅[升级](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#upgrades)和 [Postgres](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#postgres)。 |
| `username` | 否 | 覆盖 `postgres_url` 中的用户 |
| `password` | 否 | 数据库凭据。在此设置而不是在 `postgres_url` 中，以便凭据保持在 URL 之外。接受任何字符并优先于 URL 凭据。 |
| `max_connections` | 否 | 每个副本的 Postgres 连接池大小。默认 `5`，保守且对共享数据库友好。启用[支出限制](#admin)后，网关每个推理请求会执行几次数据库操作，因此在负载下为专用数据库提高它，但不要超出[调整连接池大小](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#size-the-connection-pool)所允许的范围。 |
| `connect_timeout_seconds` | 否 | 网关打开 Postgres 连接时等待的秒数。从 `1` 到 `60` 的整数，默认 `5`。如果新网关实例启动时连接尝试超时，请提高它。需要网关服务器上的 Claude Code v2.1.274 或更高版本。早期版本在设置键时拒绝启动。 |
| `readiness_grace_seconds` | 否 | Postgres 停止应答后 `/readyz` 继续报告就绪的秒数。从 `0` 到 `3600` 的整数，默认 `0`。请参阅[中断行为](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#outage-behavior)了解如何选择值。需要网关服务器上的 Claude Code v2.1.282 或更高版本。早期版本在设置键时拒绝启动。 |

对于本地开发，将 `postgres_url` 指向一次性 Postgres 容器，例如 `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`。

### `upstreams`

`upstreams` 是一个有序列表。网关将推理转发到解析请求的模型的第一个上游。

在 `5xx`、`429`、`401`、`403`、`404` 或超时时，网关故障转移到下一个上游；其他 `4xx` 不会，因为这些错误可归因于请求而不是上游。`401` 或 `403` 意味着上游拒绝了网关使用的凭据，或拒绝其访问，例如对所请求模型的访问。`404` 意味着该上游不提供请求的模型，因此列表中的后续上游仍然可以。

如果您在上游上设置 `forward_user_identity: true`，它返回给携带开发人员电子邮件的请求的 `429` 不会故障转移。请参阅[每用户限制拒绝如何到达开发人员](#per-user-identity-headers-for-a-proxy-you-run)。

`404` 上的故障转移需要网关 v2.1.198 或更高版本。早期版本即使列表中的后续上游提供模型，也将第一个 `404` 返回给客户端。

相同提供商的多个上游必须设置不同的 `name:`。

Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 和 Microsoft Foundry 客户端在启动时构建一次，其 SDK 在内部刷新凭据，因此轮换云凭据不需要重启。静态 Anthropic API 密钥和持有者在启动时读取；请参阅 [Anthropic API](#anthropic-api)。

#### 上游错误消息

网关返回一个上游的错误响应或其自己的 `502`，取决于上游如何应答：

* **上游返回网关不[故障转移](#multiple-upstreams)的状态**：该上游的响应。网关不尝试进一步的上游。
* **网关尝试的每个上游都以网关[故障转移](#multiple-upstreams)的方式失败**：最后一个 `429`。当没有返回 `429` 时，网关按顺序优先选择最后一个 `401` 或 `403`、最后一个 `404` 和最后一个 `501`。当没有返回任何这些时，网关自己的 `502`，`all upstreams failed (N attempted)`，其中 N 计算 [`upstreams`](#upstreams) 中的每个条目，包括网关跳过的条目，因为它们不提供请求的模型。

当网关返回上游的响应时，它保持上游的状态代码。它是否保持上游的消息取决于提供商。Anthropic API 上游的错误正文不变地到达开发人员。

Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上游可以在其错误文本中命名您的帐户 ID、角色 ARN 和项目 ID。网关在[操作日志](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#logs)中记录该完整文本。开发人员从这些上游看到的内容取决于拒绝：

* Anthropic 标准错误信封中的 `400` 或 `413`：上游自己的消息，如 `prompt is too long`。AWS 上的 Claude Platform、Agent Platform 和 Microsoft Foundry 为模型 API 拒绝返回此信封。
* 提供商自己形状中的 `400` 或 `413`：`capability_rejected:` 令牌。当网关无法分类拒绝时，`400` 上的 `upstream rejected the request` 或 `413` 上的 `request too large for this upstream`。
* 任何其他状态：通用的按状态副本，如 `429` 上的 `upstream rate limit exceeded`。

例如，网关将 Amazon Bedrock 的 `Input is too long for requested model.` 替换为 `capability_rejected: prompt_too_long`。Claude Code [自动压缩](https://code.claude.com/docs/zh-CN/errors.md#prompt-is-too-long)该令牌，就像它对 `prompt is too long` 所做的那样。

保持云上游的 `400` 或 `413` 消息或将其替换为 `capability_rejected:` 令牌需要网关 v2.1.233 或更高版本。

#### Anthropic API

最小的 Anthropic 上游是来自 [Claude Console](https://platform.claude.com) 的 API 密钥：

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # 或 OAuth 持有者（例如 Workload-Identity-Federation 交换的令牌）：
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # 默认；为前向代理覆盖
```

两种凭据形式在它们发送的标头中有所不同：

* **`api_key`**：发送 `x-api-key`。在 Claude Console 中轮换它并更新环境变量。
* **`oauth_token`**：发送 `Authorization: Bearer`。当您的组织发出短期令牌而不是长期 API 密钥时使用持有者形式。持有者在启动时读取一次，因此通过重新挂载秘密和重启来刷新。

您可以使用 Workload Identity Federation 而不是静态密钥或持有者。按照 [Workload Identity Federation 指南](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)创建联合规则，然后将您的工作负载的 OIDC JWT 挂载为文件，如 Kubernetes 投影服务帐户令牌或 CI 平台的 id-token。网关将 JWT 交换为短期持有者并自动刷新它。令牌文件在每次交换时重新读取，因此轮换的投影令牌被拾取而无需重启。

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # 如果规则覆盖 >1 个工作区则需要
      # service_account_id: svac_...   # 可选的预期目标检查
```

##### 为您运行的代理的每用户身份标头

您可以将 `provider: anthropic` 上游的 `base_url` 指向您运行的代理而不是 Anthropic API。要告诉该代理哪个开发人员发送了每个请求，在该上游上设置 `forward_user_identity: true`。代理然后可以按开发人员属性支出。需要网关运行 Claude Code v2.1.233 或更高版本。

例如，对于 `upstream-gateway.internal.example.com` 上的代理：

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # 默认 false
```

网关将这些标头添加到它转发到该上游的每个请求。

| 标头 | 值 |
| - | - |
| `x-litellm-end-user-id` | 开发人员的电子邮件，当 IdP 提供时。 |
| `x-claude-gateway-user-id` | 开发人员的 IdP 主体，来自令牌的 `sub` 声明。 |
| `x-claude-gateway-user-email` | 开发人员的电子邮件，当 IdP 提供时。 |

当 IdP 令牌不携带电子邮件时，网关仅发送 `x-claude-gateway-user-id` 并省略两个电子邮件标头。如果您的 IdP 将电子邮件放在不同的声明中，将 [`oidc.email_claim`](#oidc) 设置为该声明。

您的代理不会收到 Claude Code 发送的 [`x-claude-code-*` 标头](https://code.claude.com/docs/zh-CN/llm-gateway-protocol#request-headers)，例如 `x-claude-code-session-id`。网关不会将它们传递给任何上游，因此请使用上表中的标头按开发人员对请求进行分组。

当您的代理对携带开发人员电子邮件的请求应答 `429` 时，网关按原样将该响应返回给开发人员而不是故障转移到下一个上游，因此您的代理的每用户预算或速率限制保持。代理的其他响应遵循普通[故障转移规则](#upstreams)。如果开发人员的 IdP 令牌不携带电子邮件，网关转发其请求而不带电子邮件标头，因此对其中一个请求的 `429` 计为上游容量并故障转移。在网关服务器上的 v2.1.267 之前，每个 `429` 都故障转移。

仅在 `base_url` 是您操作的代理的上游上设置 `forward_user_identity`。网关将开发人员电子邮件发送到该 `base_url` 命名的任何服务器。如果 `base_url` 是 Anthropic API（默认），网关拒绝启动。

> [!NOTE]
>
> 如果您在 Amazon Bedrock 上游上设置 `forward_user_identity`，网关会在启动时退出并显示 `Unrecognized key(s) in object: 'forward_user_identity'`。如果您的代理位于 Bedrock 前面，请改为将其列为 `provider: anthropic` 上游。网关向该上游发送 Anthropic Messages 格式，因此您的代理必须[为 Bedrock 转换请求](https://code.claude.com/docs/zh-CN/llm-gateway-protocol.md#format-mismatch-with-the-upstream)。若要改为向 AWS 标明每个开发人员，请参阅[每开发人员 AWS 成本属性](#per-developer-aws-cost-attribution)。

#### Amazon Bedrock

对于网关替换或前置的客户端 Amazon Bedrock 部署，请参阅 [Amazon Bedrock 上的 Claude Code](https://code.claude.com/docs/zh-CN/amazon-bedrock.md)。网关端上游：

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # 首选：AWS 默认凭据链
    # 或显式凭据：
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # 或 Bedrock API 持有者令牌：
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # 为 FIPS 或 VPC 端点部署覆盖 bedrock-runtime 端点：
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

空 `auth` 块使用 AWS SDK 的默认凭据链：环境变量、`~/.aws/credentials`、ECS 任务角色、EC2 实例元数据或 EKS 上的 IRSA。在生产中，给网关 pod 一个 IAM 角色而不是在容器镜像中嵌入静态密钥。

显式凭据必须完整：当 `aws_access_key_id` 和 `aws_secret_access_key` 未一起设置时，或当 `aws_session_token` 在没有它们的情况下设置时，网关在启动时失败。在 v2.1.207 之前，部分 `auth:` 块通过验证。

| 设置 | 如何 |
| - | - |
| IAM 权限 | 授予网关的主体 `bedrock:InvokeModel` 和 `bedrock:InvokeModelWithResponseStream` 在推理配置文件 ARN 和底层基础模型 ARN 上。对于美国地区的内置目录：`arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` 和 `arn:aws:bedrock:*::foundation-model/anthropic.*`。也在基础模型 ARN 上授予 `bedrock:CountTokens`。网关使用它（免费）来计算客户端放弃的请求的输入 token，因此[支出限制](#admin)保持准确。没有它，网关回退到针对该计数的单 token Bedrock 请求。 |
| 模型访问 | Amazon Bedrock 在商业地区默认启用模型访问。剩余的帐户级门是 Anthropic 的一次性用例表：如果您的 AWS 帐户中没有人提交过，请打开 Amazon Bedrock 控制台，从模型目录中选择 Anthropic 模型，并完成表单。有关 AWS Organizations 表和提交者需要的权限，请参阅[提交用例详情](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#1-submit-use-case-details)。 |
| EKS (IRSA) | 创建具有上述策略和您的集群 OIDC 提供商的信任策略的 IAM 角色，范围限于网关的服务帐户。使用 `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway` 注释服务帐户。`auth: {}` 拾取它。 |
| ECS / EC2 | 将 IAM 角色附加到任务定义或实例配置文件。`auth: {}` 拾取它。 |
| 其他任何地方 | 通过 `AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY` 和 `AWS_SESSION_TOKEN` 环境变量传递凭据，或在 `auth:` 中使用 `${VAR}` 扩展显式设置它们 |
| 区域 | `region:` 是 API 端点区域。跨区域推理配置文件跨地理位置（美国、欧盟、亚太）路由，无论您选择哪一个。对于非美国地区或预配吞吐量 ARN，添加带有正确的按上游 ID 的 [`models:`](#models) 块。 |

##### 应用 Amazon Bedrock 防护栏

要将 Amazon Bedrock 防护栏应用于网关通过 Bedrock 上游发送的每个推理请求，将 `guardrail` 块添加到该上游。需要网关服务器上的 Claude Code v2.1.281 或更高版本。

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}
    guardrail:
      id: gr-abc123                    # 防护栏 ID 或完整 ARN
      version: "1"                     # 已发布的版本号或 DRAFT
 # 保留引号：裸 1 在启动时失败
```

> [!WARNING]
>
> 网关不支持防护栏输入标签。它不向提示词添加防护内容标签，因此 Amazon Bedrock 仅应用于标记输入的防护栏过滤器不在通过网关的流量上运行。对于哪些过滤器依赖输入标签，请参阅 Amazon Bedrock 文档中的[输入标签](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-tagging.html)。

也在防护栏上授予 `bedrock:ApplyGuardrail` 给签署此上游请求的主体：网关的 AWS 主体，或使用 [`assume_role`](#bedrock-in-another-aws-account) 的 `role_arn` 中命名的角色。

在每个 `bedrock` 上游或不在任何上游上设置 `guardrail`。网关拒绝在混合上启动，因为[故障转移](#multiple-upstreams)可能会将请求发送到没有防护栏的 Bedrock 上游。

防护栏仅覆盖 Bedrock 上游。如果您在 `upstreams` 中列出另一个提供商，网关将请求发送到该提供商而不带防护栏；如果该提供商是 [`mantle`](#amazon-bedrock-mantle-endpoint)，则网关拒绝启动。

当 `/v1/messages` 请求的正文携带 `amazon-bedrock-*` 字段（如 `amazon-bedrock-guardrailConfig`）到达设置了 `guardrail` 的 Bedrock 上游时，网关应答 400 而不是转发它。

##### 另一个 AWS 帐户中的 Bedrock

在 Bedrock 上游上设置 `assume_role`，网关仅使用其自己的 AWS 身份来调用您命名的角色上的 `sts:AssumeRole`，该角色可以在与网关不同的 AWS 帐户中。该上游的每个 Bedrock 请求都使用 STS 返回的一小时凭据签署，因此没有长期访问密钥跨帐户。

需要网关运行 Claude Code v2.1.281 或更高版本。早期网关在找到键时拒绝启动。

```yaml theme={null}
upstreams:
  - name: bedrock-isolated
    provider: bedrock
    region: us-east-1
    auth: {}                           # 网关自己的角色：它仅调用 STS
    assume_role:
      role_arn: arn:aws:iam::222222222222:role/claude-gateway-bedrock
      # external_id: ${BEDROCK_ROLE_EXTERNAL_ID}   # 当角色的信任策略需要时
```

`assume_role` 块采用三个键：

| 键 | 含义 |
| - | - |
| `role_arn` | 网关假设的 IAM 角色，作为 `arn:aws:iam::` 或 `arn:aws-us-gov:iam::` ARN。给它这个上游需要的 [Bedrock 权限](#amazon-bedrock)，包括 `bedrock:CountTokens`，加上当上游设置 `guardrail` 时的 `bedrock:ApplyGuardrail`。 |
| `external_id` | 可选。在每个 `sts:AssumeRole` 调用上作为外部 ID 发送。当角色的信任策略需要时设置它，如果它全是数字则引用它。 |
| `session_name` | 可选。`email` 或 `sub` 给每个开发人员他们自己的会话：请参阅[每开发人员 AWS 成本属性](#per-developer-aws-cost-attribution)。未设置，每个请求使用一个名为 `claude-apps-gateway` 的会话。 |

角色的信任策略命名网关自己的主体，如其 IRSA 或 ECS 任务角色。该主体需要在角色上的 `sts:AssumeRole` 且没有 Bedrock 权限。如果您设置没有 `external_id`，删除 `Condition`。

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [{
    "Effect": "Allow",
    "Principal": { "AWS": "arn:aws:iam::111111111111:role/claude-gateway" },
    "Action": "sts:AssumeRole",
    "Condition": { "StringEquals": { "sts:ExternalId": "your-external-id" } }
  }]
}
```

* 如果 STS 拒绝或无法到达，网关不使用上游自己的凭据发送请求。它记录 STS 错误和要检查的内容，然后尝试您列出的下一个上游。[上游错误消息](#upstream-error-messages)覆盖当没有上游成功时客户端接收的内容。没有 `assume_role` 的后续上游将使用其自己的凭据提供请求，因此仅在这是您想要的情况下列出一个。
* 网关调用区域 STS 端点 `sts.<region>.amazonaws.com`，其网络必须到达。对于 FIPS 端点，在网关的环境中设置 `AWS_USE_FIPS_ENDPOINT=true` 而不是在 AWS 配置文件中的 `use_fips_endpoint`。
* `assume_role` 仅适用于 `provider: bedrock` 并需要 SigV4 源凭据：当它在 `aws_bearer_token` 旁边设置时，网关拒绝启动。
* 网关允许的每个开发人员都可以使用此上游；[`managed`](#managed) 控制哪些开发人员可能使用哪些模型。要保持通过角色提供的模型也不从另一个帐户提供，给它一个自定义 id，其 `upstream_model` 映射仅具有此上游的名称。对于这样的 id，网关跳过每个其他上游，因此请求和放弃请求的 token 计数都无法故障转移到另一个帐户。对内置模型名称的请求仍可能[到达此上游](#multiple-upstreams)，网关会使用同一角色对其签名。除非其帐户也应该提供这些模型，否则请将此上游列在最后。

此示例给一个模型一个自定义 id，仅隔离上游提供：

```yaml theme={null}
models:
  - id: restricted-claude-opus-4-8      # 自定义 id，不是内置模型名称
    upstream_model:
      bedrock-isolated: us.anthropic.claude-opus-4-8   # 唯一提供它的上游
```

##### 每开发人员 AWS 成本属性

默认情况下，网关使用一个凭据签署每个 Bedrock 请求，因此 AWS 在单个 IAM 主体下看到所有开发人员的请求。将 `session_name: email` 添加到 [`assume_role`](#bedrock-in-another-aws-account)，网关每个开发人员每小时调用一次 `sts:AssumeRole`，会话名称设置为该开发人员的电子邮件，并使用返回的凭据签署其请求，因此每个开发人员的请求在 AWS 下以其自己的假设角色会话到达。角色可以在网关自己的帐户中。

需要网关运行 Claude Code v2.1.281 或更高版本。[AWS 上的成本属性](https://code.claude.com/docs/zh-CN/claude-apps-gateway-on-aws.md#cost-attribution)覆盖 IAM 角色和 AWS 计费显示会话的位置。

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # 网关自己的角色：它仅调用 STS
    assume_role:
      role_arn: arn:aws:iam::123456789012:role/claude-gateway-bedrock-user
      session_name: email              # 或 sub
```

`session_name` 选择哪个已验证的声明成为 AWS `RoleSessionName`：`email` 或 `sub`。网关将除 ASCII 字母、数字和 `_+,.@-` 之外的任何字符写成 `=XX` 十六进制（按 UTF-8 字节），并将长于 64 字符的结果缩短为前缀加哈希，因此每个开发人员的会话名称保持有效且唯一。来自其令牌缺少声明的开发人员的请求不通过此上游发送，操作员日志说切换到 `sub` 或设置 [`oidc.email_claim`](#oidc)。

活跃开发人员每小时每个网关副本成本一个 STS 调用，并发首次请求共享一个调用。

网关也在此角色上进行一个调用：客户端放弃的请求的 token 计数，因此[支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)保持准确。该计数及其[单 token 回退请求](#amazon-bedrock)由共享 `claude-apps-gateway` 会话签署，因此 AWS 将回退属性到 `claude-apps-gateway` 而不是开发人员。

对于严格的每开发人员属性，在您列出的每个 Bedrock 上游上设置 `assume_role` 与 `session_name`。没有它的上游使用其自己的凭据签署它提供的请求。

#### Amazon Bedrock Mantle 端点

`mantle` 提供商将推理发送到 Amazon Bedrock 的 [Mantle 端点](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#use-the-mantle-endpoint)。它需要网关服务器上的 Claude Code v2.1.283 或更高版本。早期网关版本在启动时拒绝它，因此请在添加之前升级每个副本。

下面的示例将 Mantle 放在首位，其后是一个 Amazon Bedrock 上游，用于提供 `models` 字段未列出的每个模型：

```yaml theme={null}
upstreams:
  - provider: mantle
    region: us-east-1
    models: [claude-opus-4-7, claude-haiku-4-5]   # 必需
    auth: {}                           # AWS 默认凭据链
  - provider: bedrock
    region: us-east-1
    auth: {}
```

下表列出 `mantle` 上游特有的字段。

| 字段 | 必需 | 描述 |
| - | - | - |
| `region` | 是 | AWS 区域。网关从它派生端点为 `https://bedrock-mantle.<region>.api.aws/anthropic`。 |
| `models` | 是 | 您的 AWS 帐户在 Mantle 上获授权的模型，按客户端发送的名称命名，如 `claude-haiku-4-5`。只有这些模型会发送到此上游，其他每个模型都跳到下一个上游。 |
| `auth` | 否 | 接受与 [Amazon Bedrock](#amazon-bedrock) 上游的 `auth` 块相同的键，遵循相同的规则。 |
| `base_url` | 否 | 覆盖派生的端点。在末尾保留 `/anthropic` 路径。 |

为上游的 AWS 身份授予 Mantle 自己用于推理和 token 计数的 IAM 操作，[使用 Mantle 端点](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#use-the-mantle-endpoint)中列出了这些操作。

对于网关不知道的 Mantle 模型 ID，在顶层 [`models:`](#models) 块中添加一个条目，其 `upstream_model` 将此上游的名称映射到该 ID。然后也将该条目的 `id` 放入此上游的 `models` 字段。

`bedrock` 上游的 `guardrail` 和 `assume_role` 设置不会扩展到 Mantle 提供的请求：

* **`guardrail`**：网关不对发送到 Mantle 的请求应用 [Bedrock 防护栏](#apply-an-amazon-bedrock-guardrail)，因此当列出 `mantle` 上游而任何 `bedrock` 上游设置了 `guardrail` 时，网关拒绝启动。
* **`assume_role`**：`mantle` 上游不接受 [`assume_role`](#bedrock-in-another-aws-account)。Mantle 提供的请求使用 `mantle` 上游自己的 `auth` 凭据发送，且不会[按开发人员归属](#per-developer-aws-cost-attribution)。

有关 Mantle 自身错误响应的含义，请参阅 [Mantle 端点错误](https://code.claude.com/docs/zh-CN/amazon-bedrock.md#mantle-endpoint-errors)。

#### AWS 上的 Claude Platform

AWS 上的 Claude Platform 在 AWS 基础设施上提供第一方 Anthropic API，位于 `aws-external-anthropic.<region>.api.aws`。它使用第一方模型 ID，按原样遵守 `anthropic-beta` 标头，并提供 `count_tokens`，因此没有 Bedrock 特定的转换适用。`anthropicAws` 提供商需要 Claude Code v2.1.198 或更高版本；早期网关版本在启动时拒绝它。

对于相同平台的客户端部署，请参阅 [AWS 上的 Claude Platform 上的 Claude Code](https://code.claude.com/docs/zh-CN/claude-platform-on-aws.md)。网关端上游：

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # 作为 x-api-key 发送
    # 或通过 AWS 默认凭据链的 SigV4：
    # auth: {}
    # 或显式 SigV4 凭据：
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # 覆盖派生的端点：
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

平台在与 Amazon Bedrock 不同的 AWS 帐户中运行，并为其自己的服务名称 `aws-external-anthropic` 签署 SigV4 请求，因此 Bedrock 范围的 IAM 角色不授权它。`auth.api_key` 中的 API 密钥在同时设置 SigV4 凭据时优先。空 `auth` 块使用 AWS SDK 的默认凭据链，与 [Amazon Bedrock](#amazon-bedrock) 上游使用的链相同。

| 字段 | 必需 | 描述 |
| - | - | - |
| `region` | 是 | AWS 区域，小写字母、数字和连字符。网关从它派生端点为 `https://aws-external-anthropic.<region>.api.aws`。 |
| `workspace_id` | 是 | 在每个请求上作为标头发送；平台需要它 |
| `auth.api_key` | 否 | 平台的 API 密钥，作为 `x-api-key` 发送。不是持有者令牌：两种身份验证模式是 API 密钥或 SigV4。 |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | 否 | 显式 SigV4 凭据。在没有另一个的情况下设置一个在启动时失败。`auth.aws_session_token` 在它们旁边被接受。 |
| `base_url` | 否 | 覆盖派生的端点 |

因为平台解析第一方模型 ID，内置目录路由到它而不带 [`models:`](#models) 块。当您策划 `models:` 列表时，使用第一方 ID 作为 `anthropicAws:` 条目的值。

#### Google Cloud Agent Platform

对于等效的客户端设置，请参阅 [Google Cloud 上的 Claude Code](https://code.claude.com/docs/zh-CN/google-vertex-ai.md)。网关端上游：

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # 首选：应用默认凭据
    # 或服务帐户密钥文件：
    # auth: { service_account_json: /secrets/sa.json }
    # 为私有服务连接覆盖 aiplatform 端点：
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

空 `auth` 块使用应用默认凭据：`GOOGLE_APPLICATION_CREDENTIALS`、GCE 元数据或 GKE Workload Identity。服务帐户 JSON 密钥文件被支持但不鼓励；使用 Workload Identity 或将服务帐户附加到 GCE 或 Cloud Run 实例。

设置 `region: global` 以使用 [Google Cloud Agent Platform 的全局端点](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)而不是区域端点。Google 然后将每个请求路由到可用区域，因此您不跟踪按区域模型可用性。设置特定区域将每个请求固定到它。

| 设置 | 如何 |
| - | - |
| IAM 权限 | 授予网关的服务帐户项目上的 `roles/aiplatform.user`，或具有 `aiplatform.endpoints.predict` 的自定义角色。启用 Google Cloud Agent Platform API (`aiplatform.googleapis.com`)。 |
| 模型访问 | 在 Model Garden 中，为您的项目启用 Claude 模型。它们发布到特定区域；检查模型卡以了解支持的区域。 |
| GKE (Workload Identity) | 将 GCP 服务帐户绑定到网关的 Kubernetes 服务帐户，并使用 `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com` 注释 KSA。`auth: {}` 拾取它。 |
| Cloud Run / GCE | 将服务的服务帐户设置为具有 `roles/aiplatform.user` 的服务帐户。`auth: {}` 拾取它。 |
| 其他任何地方 | `auth: { service_account_json: /secrets/sa.json }`，JSON 密钥文件的路径挂载为秘密。该字段采用文件路径，而不是密钥内容，因此不涉及 `${file:…}` 扩展。 |

#### Microsoft Foundry

对于客户端 Microsoft Foundry 部署，请参阅 [Microsoft Foundry 上的 Claude Code](https://code.claude.com/docs/zh-CN/microsoft-foundry.md)。网关端上游：

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # 首选：DefaultAzureCredential / 托管身份
    # 或 API 密钥：
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` 通过 `DefaultAzureCredential` 解析：AKS、ACI 或 App Service 上的托管身份；Azure CLI；或环境凭据。API 密钥有效但是项目范围的，不自动轮换。Microsoft Foundry 的端点从 `resource:` 派生；为主权云（如 Azure Government）设置可选 `base_url` 以覆盖它。

| 设置 | 如何 |
| - | - |
| RBAC | 授予网关的身份 Microsoft Foundry 资源上的 `Azure AI User` 或 `Cognitive Services User` |
| 部署 | Microsoft Foundry 使用管理员选择的部署名称，而不是规范模型 ID。添加 [`models:`](#models) 块将每个规范 ID 映射到您的部署名称。 |
| AKS (workload identity) | 将用户分配的托管身份与集群的 OIDC 发行者联合，并将其绑定到网关的服务帐户。`use_azure_ad: true` 通过 `WorkloadIdentityCredential` 拾取它。 |
| ACI / App Service | 在资源上启用系统分配或用户分配的托管身份。`use_azure_ad: true` 拾取它。 |
| 其他任何地方 | `auth: { api_key: "${FOUNDRY_API_KEY}" }`。在 `{ }` 内引用 `${…}`。 |

#### 上游请求上的静态标头

要将固定标头添加到网关发送到一个上游的请求，在该上游上设置 `headers:`。当您运行的代理通过标头路由或属性流量时使用它。

`headers:` 需要网关服务器上的 Claude Code v2.1.277 或更高版本。早期网关在找到键时拒绝启动。在添加键之前升级每个副本，并在回滚到早期版本之前删除键。

标头转到 `base_url` 命名的服务器，或当 `base_url` 未设置时转到提供商自己的端点。提供商也接收它们，除非您的代理删除它们。

此示例通过 `upstream-proxy.internal.example.com` 上的代理到达 `provider: vertex` 上游。它设置代理读取的 `x-source` 标头，并从 `PROXY_TOKEN` 环境变量发送令牌作为 `x-proxy-token`：

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    base_url: https://upstream-proxy.internal.example.com
    auth: {}
    headers:
      x-source: claude-apps-gateway
      x-proxy-token: ${PROXY_TOKEN}
```

值是可打印的 ASCII 文本，两端没有空格。引用数字、`true` 或 `false` 以便 YAML 将其读取为文本。

要将秘密保持在配置文件之外，使用[秘密扩展](#secret-expansion)从环境变量使用 `${VAR}` 或从文件使用 `${file:/path}` 加载值。解析为空值的 `${VAR}` 停止网关启动。

`headers:` 适用于每个提供商，每个上游仅发送自己的。

并非网关发送到上游的每个请求都携带它们：

| 网关发送到此上游的请求 | 携带 `headers:` |
| - | - |
| `/v1/messages`，流式或非流式，和 `/v1/messages/count_tokens` | 是 |
| 从另一个上游故障转移的请求 | 是，仅此上游的 `headers:` |
| Amazon Bedrock 的 `CountTokens` 调用用于客户端放弃的请求 | 否 |
| Workload Identity Federation 令牌交换 | 否 |

在使用 AWS SigV4 签署请求的 Amazon Bedrock 或 AWS 上的 Claude Platform 上游上，这些标头是签名的一部分，因此您的代理必须原样通过它们。

如果您使用网关保留的名称，它拒绝启动，启动错误命名标头。保留名称包括：

* `authorization` 和 `x-api-key`
* `host`、`content-type` 和 `user-agent`
* 任何以 `anthropic-`、`x-goog-`、`x-amz-` 或 `x-amzn-` 开头的名称

#### 多个上游

相同提供商可以出现多次，具有不同的 `name:`。这涵盖不同的区域、通过不同凭据链的不同帐户、预配吞吐量与按需，以及跨提供商故障转移。

网关按顺序尝试上游。`5xx`、`429`、`401`、`403`、`404`、超时和缺失端点 (`501`) 故障转移；其他 `4xx` 不会。

`429` 是按上游容量，因此预配吞吐量 (PT) 耗尽故障转移到按需。如果您在上游上设置 [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run)，对携带开发人员电子邮件的请求的 `429` 是按用户拒绝而不是故障转移。

每个请求从第一个上游开始。请求仅在它前面的每个上游都失败或不提供请求的模型时才到达后续上游。

网关不保持失败上游的记录，因此当上游关闭时，到达它的每个请求仍然尝试它并等待它失败后再继续。

网关等待响应开始的时长如下：

* **`provider: anthropic` 上游**：[`timeouts.upstream_ttfb_ms`](#http-tuning)，默认 120 秒
* **其他提供商**：最多一小时。要缩短流式请求的等待时间，请在网关 v2.1.295 或更高版本上设置 `timeouts.upstream_ttfb_ms`。非流式模型请求无论如何都最多等待一小时

`404` 是按上游模型可用性，因此未启用模型的上游不阻止提供它的后续上游。无法解析请求的模型的上游被跳过而不进行网络往返。

此示例首先路由预配吞吐量 Amazon Bedrock 分配，溢出到按需和第二个帐户，并最后回退到 Anthropic API：

```yaml theme={null}
upstreams:
  # 主要：您主区域中的预配吞吐量。
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # 溢出：按需跨区域。
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # 不同帐户：通过静态密钥的单独 Bedrock 分配。
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # 最后的手段：直接 Anthropic API。
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# 每个上游的模型 ID 以上游的 `name:` 为键。
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| 杠杆 | 如何 |
| - | - |
| 不同区域 | 每个区域一个 Amazon Bedrock 上游，每个都有自己的 `region:`。使用 [`auto_include_builtin_models: true`](#models) 跨区域推理配置文件自动路由；对于区域固定部署，使用 `models:` 块。 |
| 不同帐户 | 每个帐户一个 Amazon Bedrock 上游。默认链 (`auth: {}`) 使用 pod 的身份；对于第二个帐户，添加 [`assume_role`](#bedrock-in-another-aws-account) 以使用短期凭据到达它，或在 `auth:` 中设置显式凭据或持有者令牌。 |
| 预配吞吐量 | 将模型映射到该上游名称的 `models:` 中的预配吞吐量 ARN。其他上游保持按需 ID，因此 PT 容量在故障转移前耗尽。 |
| VPC / FIPS 端点 | 在上游上设置 `base_url:` 到您的 VPC 端点或 FIPS 端点 URL |
| 模型范围路由 | 仅自定义模型 `id`（即不是内置 Claude 模型的 id）会跳过其 `upstream_model:` 映射中不存在的上游。`mantle` 上游仅针对其 [`models` 字段](#amazon-bedrock-mantle-endpoint)中列出的模型被尝试。在其他每个上游上，网关按顺序尝试内置模型，并在映射没有条目时使用提供商的默认 ID，因此对于内置模型，映射改变上游接收的 ID 而不是它是否被尝试；拒绝 ID 的上游遵循与任何其他上游错误相同的[故障转移规则](#upstreams)。 |

在云提供商之间或直接 Anthropic API 之间故障转移改变哪个协议、地理位置和其他条款控制请求。

CLI 对网关应用相同的功能门控，无论哪个上游提供给定请求，因此默认情况下故障转移不会发送上游会拒绝的正文字段。如果您在策略中设置了[一小时缓存 TTL](#one-hour-prompt-cache)，它会发送到列表中的每个上游，因此仅当所有上游都接受它时才设置。

## 可选部分

### `admin`

可选。启用 `/v1/organizations/spend_limits`（它与 Anthropic 的公开 Admin API 一致），并在 `/v1/messages` 上按开发者执行支出限制。有关上限的设置和执行方式，请参阅[支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)；本节介绍用于启用和调整该功能的 `gateway.yaml` 键。

```yaml theme={null}
admin:
  # Named static API keys for the admin endpoints, sent as x-api-key.
  # The id appears in the audit log as admin-key:<id> so each key is
  # attributable. Array for rotation: add the new key, roll clients,
  # remove the old.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # IdP groups granted full admin via the normal gateway JWT (no API key).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| 字段 | 必需 | 描述 |
| - | - | - |
| `write_keys` | 否 | `{id, key}` 数组。与其中之一匹配的 `x-api-key` 可以列出、设置和删除支出限制。密钥值长度至少为 32 个字符；`id` 在 `read_keys` 和 `write_keys` 中必须唯一。 |
| `read_keys` | 否 | `{id, key}` 数组。只读：可访问所有 `GET` 端点，包括列出上限、按 ID 获取单个上限，以及读取 [`/effective`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits#%2Feffective) 和 [`/audit`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits#%2Faudit)。 |
| `admin_groups` | 否 | IdP 组名。`groups` 声明包含其中之一的网关 JWT 拥有完整的管理员访问权限（读和写），并以 `oidc:<sub>` 记入审计。人工管理员使用此方式；机器使用 API 密钥。此列表中的空条目会导致网关在启动时停止。请参阅[会导致网关在启动时停止的匹配器值](#matcher-values-that-stop-the-gateway-at-boot)。 |
| `blocked_message` | 否 | 原样附加到被阻止的开发者看到的 `429 billing_error` 中。请写出完整的说明，例如 URL 或 Slack 频道。未设置时，网关仅发送默认消息。请参阅[执行方式](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md#how-enforcement-works)。 |
| `audit_retention_days` | 否 | 默认值 `365`。更早的 `admin_audit` 行会被清除。 |
| `spend_retention_months` | 否 | 默认值 `13`。早于此时间的 `spend` 计数器行会被清除。默认值保留完整一年加上当前未满的月份，以便进行同比报告。 |
| `identity_retention_days` | 否 | 默认值 `90`。`principal_emails` 行的最后活跃 TTL，这些行保存每位开发者的电子邮件、显示名称和组（PII）。有意设置得比支出保留期短，以便已取消配置的身份过期，而其匿名支出计数器仍然保留。 |
| `group_limit_mode` | 否 | `min`（默认）或 `max`。当开发者属于多个设有上限的组时，`min` 执行最严格的上限，`max` 执行最宽松的上限。执行和 `/effective` 都使用此设置。 |

### `enforcement`

`enforcement` 块控制存储不可用时支出限制检查的行为。

| 字段 | 必需 | 描述 |
| - | - | - |
| `fail_closed_on_error` | 否 | 默认值 `false`。Postgres 中断时，支出执行以失败开放方式处理，因此推理保持可用。设置为 `true` 可改为失败关闭：超出上限的开发者会被阻止，但如果存储不可达，其他所有人也会被阻止。需要 [`admin:`](#admin) 块：支出执行仅在配置了 `admin` 时运行，如果您在没有该块的情况下将此项设为 `true`，网关将拒绝启动。 |

### `pricing`

`pricing` 块告诉支出计量器按什么价格计费，而不是使用美元标价，因此上限和 [`/effective`](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits#%2Feffective) 会反映您的合同价格。金额仍以美元计，并且仍然是估算值，而不是发票。两个前提条件：

* 网关服务器上的 Claude Code 为 v2.1.227 或更高版本。更早的版本会在启动时拒绝这个未知键。
* 存在 [`admin:`](#admin) 块，或者在 v2.1.268 或更高版本中，存在至少包含一个策略的 [`managed:`](#managed) 块。如果设置了 `pricing` 但两个块都不存在，网关将拒绝启动，因为没有任何内容会读取它。

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| 字段 | 必需 | 描述 |
| - | - | - |
| `multiplier` | 否 | 默认值 `1`。计量器将每笔计量金额（无论是标价还是覆盖价格）乘以此值，因此 `0.85` 按价格的 85% 计费。必须大于 0 且不超过 10，大于 1 的值表示[加价](#mark-prices-up)。 |
| `overrides` | 否 | `{upstream, model, input, output, cache_read, cache_write}` 行，单位为每百万 token 的美元价格。四个费率都是必需的。每个费率必须大于 0 且不超过 10000。 |

计量器匹配覆盖行的方式：

* 对于 `upstream`（一个 [`upstreams[].name`](#upstreams)）为 `model` 提供服务的请求，该行会替换标价。这包括较高的[快速模式](https://code.claude.com/docs/zh-CN/fast-mode.md#understand-the-cost-tradeoff)费率，因此快速请求和标准请求按相同的四个费率计量。
* 内置 ID（例如 `claude-sonnet-4-6`）的匹配方式与 [`models[].id`](#models) 相同，涵盖计量器按该模型计价的所有带日期形式、区域性 Amazon Bedrock 形式或 Google Cloud Agent Platform 形式。其他任何字符串（例如别名或推理配置文件 ARN）会不区分大小写地匹配客户端发送的 ID 或发送到上游的字符串。
* 当多行重叠时，计量器选择最具体的行，而不是第一行：首先是 `model` 为发送到上游的确切模型字符串的行，其次是与客户端发送的确切 ID 匹配的行，最后是指定内置模型的行。
* 未知的上游名称会导致启动失败，同一上游的两行指定相同模型（包括同一内置模型的两种写法）也会导致启动失败。对于任何可请求模型都无法使用的行，网关会在启动时发出警告。
* 网页搜索请求保持 \$0.01 的标价；乘数仍然适用于它们。

要使用按区域的费率，请为每个区域提供各自的命名上游，并为每个上游提供一行。

#### 加价

在网关服务器上使用 v2.1.271 或更高版本时，您可以将 `multiplier` 设置为大于 1（最高 10），以高于提供商收费的价格进行计量，例如用于内部成本分摊费率。此示例将每个请求按价格的 120% 计量：

```yaml theme={null}
pricing:
  multiplier: 1.2
```

存在 [`admin:`](#admin) 块时，加价也适用于支出限制。计量器按价格的 120% 计数，因此开发者会更早达到上限。网关会在启动时记录一条说明此情况的警告。

乘数不会改变上游提供商对这些请求的收费。

如果网关还[将费率发送到已登录的客户端](#send-the-rates-to-signed-in-clients)，开发者需要 Claude Code v2.1.271 或更高版本才能看到加价。更早的客户端会忽略大于 1 的 `multiplier`，并显示不含加价的成本。

如果您将 `multiplier` 设置为大于 1，早于 v2.1.271 的网关服务器将拒绝启动。

#### 将费率发送到已登录的客户端

在网关服务器上使用 v2.1.268 或更高版本时，网关还会将 `pricing` 中的费率作为 [`modelPricing`](https://code.claude.com/docs/zh-CN/settings-reference#modelpricing) 托管设置放入其提供的 [`managed`](#managed) 策略中。随后，被策略匹配的开发者会在 `/usage`、状态栏和 OpenTelemetry 中看到为每个模型 ID 提供服务的第一个上游的 `pricing` 费率。未匹配任何策略的开发者不会收到托管设置，因此其数据保持标价。客户端在 Claude Code v2.1.242 或更高版本中应用该设置。

* 网关添加的内容：除非策略的 `cli` 块已设置 `modelPricing`，否则网关会添加 `multiplier`，并针对客户端可以请求的每个模型 ID，添加为该 ID 提供服务的第一个上游的覆盖行。仅由故障转移上游收取的费率保留在网关上。
* 让某个策略退出：在该策略的 `cli` 块中将 `modelPricing` 设置为 `{}`，其开发者将保持标价。
* 保留策略自身的费率：如果策略的 `cli` 块使用自己的 `multiplier` 或 `overrides` 设置了 `modelPricing`，则该 `modelPricing` 会完整保留，网关不会向其中添加自己的任何费率。

### `models`

`models` 块是一个可选的、由管理员维护的模型列表，在 `/v1/models` 提供，并用于按上游转换模型 ID。对于非美国 Amazon Bedrock 区域、Amazon Bedrock 预置吞吐量 ARN 和 Microsoft Foundry 部署名称，此块是必需的。

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

`upstream_model` 下的每个键必须与已配置上游的 `name` 匹配，该名称默认为提供商名称。不匹配任何上游的键会导致启动失败，因此请省略您不使用的提供商对应的行。

### `managed`

`managed` 块定义基于 IdP 组或电子邮件域的基于角色的访问策略。策略按顺序评估；选中第一个匹配项，然后将其合并到 `match: {}` 兜底基础层上。这些策略按用户在 `GET /managed/settings` 提供，并使用 ETag/304 缓存。

```yaml theme={null}
managed:
  policies:
    # Specific groups first.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Default catch-all last: matches everyone who authenticated.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        # Make the Default option in /model resolve inside each policy's
        # list. The eng-contractors policy inherits enforceAvailableModels.
        enforceAvailableModels: true
```

`match: {}` 兜底策略按惯例放在最后，被视为基础层。其他每个策略都会从兜底策略继承自己未设置的任何键，因此按角色的条目只需列出与组织默认值不同的内容。合并规则取决于键的类型：

* **允许列表**：`availableModels` 和 `permissions.allow`。特定策略的列表会完全替换基础层的列表。
* **拒绝列表和 hook 数组**：`permissions.deny`、`permissions.ask`、`disabledMcpjsonServers`、`deniedMcpServers`、`blockedMarketplaces`、`deniedModels` 以及每个 `hooks` 事件类型数组。这些取基础层与策略的并集，因此组织范围的拒绝规则或审计 hook 不会被按角色的覆盖意外丢弃。
* **记录类型的键**：`env`、`modelOverrides` 和 `skillOverrides`。这些进行浅合并，因此按角色的 `env` 块会覆盖它设置的键，并从基础层继承其余的键。

`availableModels` 还会在 `/v1/messages` 的服务器端执行，因此无论客户端发送什么，被拒绝的模型都会返回 `400`。空列表会拒绝所有模型。该检查还涵盖开发者选择模型之前会话启动时使用的模型，因此请[让会话在策略允许的模型上启动](#start-sessions-on-a-model-the-policy-allows)。

网关在转发请求之前会自行验证 `model` 值，因此格式错误的值永远不会到达上游。在以下两种情况下，它会以 `400` 拒绝请求：

* 当该值缺失或为空时，网关会以消息 `model is required` 拒绝请求。该检查要求网关运行 Claude Code v2.1.228 或更高版本。
* 当该值存在但不是字符串时，网关会以消息 `model must be a string` 拒绝请求。要求网关运行 Claude Code v2.1.221 或更高版本。

| 匹配器 | 行为 |
| - | - |
| `match: {}` | 匹配每个已通过身份验证的用户。先从这样一个策略开始，之后再在其上方添加按组范围的策略。 |
| `match: { groups: [a, b] }` | 如果 JWT 的 `groups` 声明包含所列的任一组，则匹配。区分大小写：组名必须与 IdP 中的确切大小写一致。 |
| `match: { email_domain: example.com }` | 匹配 JWT 的 `email` 声明中最后一个 `@` 之后的部分，不区分大小写。每个策略只接受一个域。 |
| `match: { groups: [a], email_domain: example.com }` | 两个条件都必须匹配 |

未匹配任何策略的已通过身份验证的用户将获得网关的默认值，即目录中的所有模型且没有托管设置。如果您希望有一个确定的默认策略，请在最后添加一个 `match: {}` 兜底策略。

> [!NOTE]
>
> 网关本身不保存用户目录。它根据用户的 IdP 令牌对每个请求进行授权，从令牌的 `groups` 声明中读取组成员身份，并据此评估策略。没有可枚举的名单，也没有需要预先创建的账户，因此也没有 SCIM 端点，因为没有任何内容可供 SCIM 同步。
>
> 请在事实来源处进行用户和组的生命周期管理，即您的 IdP 的原生 SCIM 预配或专用的身份治理平台。在那里管理的成员身份和取消预配会通过令牌自动传递到网关。如果您希望对 Claude 账户本身进行 SCIM 预配，那是 [Claude for Enterprise](https://code.claude.com/docs/zh-CN/admin-setup.md) 的功能。
>
> 有两个传播时钟：
>
> * **策略内容**：编辑策略并重新部署后，已连接的 Claude Code 客户端会在下一次托管设置轮询时（一小时内）收到更改，但[仅在下次启动时生效的更改](https://code.claude.com/docs/zh-CN/server-managed-settings.md#fetch-and-caching-behavior)除外
> * **组成员身份**：更改用户的组成员身份会改变与其匹配的策略。这在下一次会话重新签发时生效，即下一次静默刷新，最长不超过 `session.ttl_hours`。
>
> Claude Desktop 遵循[其自身的时间安排](#when-a-policy-change-reaches-claude-desktop)。

#### 让会话在策略允许的模型上启动

如果 `availableModels` 未包含 Claude Code 的默认模型，会话将收到 `400` 响应，直到开发者选择一个列出的模型（例如使用 `/model`）。在网关会话中，默认模型是 `opus` 别名解析到的 Opus 模型，而单独使用 `availableModels` 不会改变它。

要解决此问题，请在同一 `cli` 块中设置 [`enforceAvailableModels: true`](https://code.claude.com/docs/zh-CN/model-config#enforce-the-allowlist-for-the-default-model)，然后检查列表中包含哪种条目：

* **别名（例如 `sonnet`）或内置 ID（例如 `claude-sonnet-4-6`）**：会话会在其中一个模型上启动，`/model` 中的 Default 选项会解析为该模型
* **列表中没有别名或内置 ID**：会话可能仍在内置默认模型上启动，因此还需在该策略的 `cli` 块中将 [`model`](https://code.claude.com/docs/zh-CN/model-config#control-the-model-users-run-on) 设置为列出的 ID 之一

此策略列出了一个由 [`models`](#models) 定义的自定义 ID，并让会话在该 ID 上启动：

```yaml theme={null}
managed:
  policies:
    - match: { groups: [restricted-projects] }
      cli:
        availableModels: [restricted-claude-opus-4-8]
        enforceAvailableModels: true
        model: restricted-claude-opus-4-8
        env:
          ANTHROPIC_DEFAULT_OPUS_MODEL: restricted-claude-opus-4-8   # opus and Default resolve to the ID too
```

#### 会导致网关在启动时停止的匹配器值

启动时，网关会检查每个策略的 `match` 块以及 [`admin_groups`](#admin) 列表。以下任一值都会使网关停止，并显示一条指明该字段的错误：

* 空的 `groups` 列表
* `groups` 或 `admin_groups` 中的空条目
* 空的 `email_domain`
* 包含 `@`、空白或逗号的 `email_domain`。网关在此检查之前会修剪该值并去除一个前导 `@`。请写一个不带修饰的域，例如 `example.com`。

在 v2.1.232 之前，网关会带着这些值启动。每个值的影响如下：

* 空的 `email_domain`：网关跳过域检查，因此 `email_domain` 为空且没有 `groups` 列表的策略会匹配每个已通过身份验证的用户
* 空的 `groups` 列表：该策略不匹配任何人
* 包含 `@`、空白或逗号的 `email_domain`：该策略不匹配任何人
* `groups` 或 `admin_groups` 中的空条目：仅当用户的 IdP `groups` 声明也包含空条目时，该条目才会匹配该用户。在 `admin_groups` 中，这种匹配会授予管理员访问权限。如果您的 `admin_groups` 列表从未包含空条目，则没有人会通过这种方式获得管理员访问权限。

#### 选择 `cli` 或 `code`

> [!WARNING]
>
> 当策略使用 `code` 键时，以下每种情况都会阻止网关启动：
>
> * **网关版本**：`code` 要求网关服务器上的 Claude Code 为 v2.1.296 或更高版本。更早的网关发现该键时会拒绝启动。请在添加该键之前升级每个副本，并在回滚到更早版本之前将 `code` 改回 `cli`。
> * **混用键**：同时包含 `code` 和 `cli`（或其早期写法 `settings`）的文件会导致网关在启动时停止。请在一次编辑中将所有块放在同一个键下。

策略的 Claude Code 设置（例如拒绝读取 `.env` 文件的规则）放在 `cli` 或 `code` 键下的块中。`code` 是推荐的键，`cli` 是旧版键。两个键接受相同的内容。键决定了设置在何处执行：

* **`cli`**：终端、VS Code 和 JetBrains 扩展以及 Agent SDK。在 `cli` 下，Claude Desktop 的 Code 选项卡获得的是[派生设置](#claude-desktop-overlay)，因此像 `Read(./.env)` 这样的限定范围规则不会在那里阻止用户。
* **`code`**：相同的位置，并且也可以覆盖 Claude Desktop 的 Code 选项卡。

使用 `cli` 的文件照常工作；如果网关在带有 [`desktop`](#claude-desktop-overlay) 键的策略中发现 `cli`，会在启动时发出警告，但仍会启动。请切换到 `code`，以便设置也能覆盖 Code 选项卡。

切换之前，请阅读[在 Code 选项卡中应用 `code` 设置](#apply-code-settings-in-the-code-tab)。策略需要 `desktop` 键，并且用户的机器需要完成设置后这些设置才会在那里生效，此外 Claude Desktop 中的网页搜索会被关闭。

此策略将拒绝规则放在 `code` 下，并带有一个空的 `desktop` 键：

```yaml theme={null}
managed:
  policies:
    - match: {}
      code:
        permissions: { deny: ["Read(./.env)"] }
      desktop: {}
```

切换时，请在重命名块的同一次编辑中删除所有 `serve_to_desktop` 行。在说明中提到 `cli` 块的地方，请使用您的 `code` 块。

`serve_to_desktop: true` 是向 Code 标签页提供策略 `cli` 设置的旧方式。当 Claude Desktop 通过 [`bootstrapUrl`](#claude-desktop-overlay) 连接时，它不起作用。当您的策略使用 `cli` 时，提到此键的启动警告意味着这些设置未到达 Code 标签页：

* **要覆盖 Code 标签页**：在用户的计算机满足[在 Code 标签页中应用 `code` 设置](#apply-code-settings-in-the-code-tab)中的条件后，切换到 `code`
* **要继续使用 `cli`**：在策略中的 `cli` 旁设置 `serve_to_desktop: false` 以消除警告

#### `cli` 中包含什么

每个 `code` 或 `cli` 值都是一个完整的 Claude Code `managed-settings.json` 文档，与您通过 MDM 或 `/etc/claude-code/managed-settings.json` 部署的 schema 相同，只是此处以 YAML 表示。CLI 在托管层级（高于用户和项目设置）应用所交付的文档，以取代服务器托管设置。因此，它会忽略[仅限于操作系统级策略来源](https://code.claude.com/docs/zh-CN/server-managed-settings.md#current-limitations)的设置，例如 `policyHelper` 和 `wslInheritsWindowsSettings`。

网关在启动时根据 CLI 的设置 schema 验证每个文档，因此无法识别的顶层键会导致启动失败，并显示指明每个违规键的错误。schema 中有意开放的部分仍接受任意值，因为较新的客户端可能识别网关 schema 无法识别的条目。这些开放的键包括 `env`、`pluginConfigs` 以及嵌套在 `permissions` 下的键。

由于验证使用的是网关已安装版本中捆绑的 schema，要在托管配置中放入较新 Claude Code 版本引入的顶层设置键，需要先升级网关。在推广新策略之前，请先在一个客户端上进行冒烟测试。

完整的键参考请参阅 [Claude Code 设置](https://code.claude.com/docs/zh-CN/settings-reference.md#all-settings)。运维人员最常首先使用的键：

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Model access (also enforced server-side at /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        enforceAvailableModels: true              # Default resolves inside the list

        # Permission policy
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Environment pushed into the CLI process. DISABLE_UPDATES blocks
        # background and manual updates; DISABLE_AUTOUPDATER stops only
        # background updates.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Org-wide hooks. Hook commands run on developer machines, not the
        # gateway, so the path must exist on every client OS in the policy.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| 键 | 执行方 | 效果 |
| - | - | - |
| `availableModels` | 网关 + CLI | 模型允许列表。还会在 `/v1/messages` 进行检查，因此打过补丁的客户端无法绕过它。 |
| [`deniedModels`](https://code.claude.com/docs/zh-CN/settings-reference#deniedmodels)、[`availableModelsMatch`](https://code.claude.com/docs/zh-CN/settings-reference#availablemodelsmatch) | 网关 + CLI | [阻止特定模型或版本](https://code.claude.com/docs/zh-CN/model-config.md#block-specific-models-or-versions)，即使 `availableModels` 原本会允许它们。被阻止的模型会在 `/v1/messages` 以 `400` 被拒绝，并且不会出现在 `/v1/models` 中。网关自身的检查要求网关服务器上的 Claude Code 为 v2.1.299 或更高版本。CLI 的检查要求客户端为 v2.1.283 或更高版本，更早的客户端会忽略这两个键。 |
| `permissions.allow` / `.deny` | CLI | 工具和命令规则。请参阅[权限](https://code.claude.com/docs/zh-CN/permissions.md)和 [Bash 规则不匹配的情况](https://code.claude.com/docs/zh-CN/permissions.md#bash-rule-limits)。 |
| `permissions.disableBypassPermissionsMode` | CLI | 设置为 `disable` 可阻止 [`bypassPermissions`](https://code.claude.com/docs/zh-CN/permission-modes#skip-all-checks-with-bypasspermissions-mode)（跳过权限提示的模式）以及 `--dangerously-skip-permissions` 标志 |
| `allowManagedPermissionRulesOnly` | CLI | 为 `true` 时，托管设置成为权限规则的唯一设置来源。[`allowManagedPermissionRulesOnly`](https://code.claude.com/docs/zh-CN/settings-reference#allowmanagedpermissionrulesonly) 条目列出了 Claude Code 随后会忽略的每个来源。 |
| `env` | CLI | 合并到 CLI 进程中的环境变量。用于遥测、自动更新和模型名称覆盖。 |
| `hooks` | CLI | 组织范围的 [hook](https://code.claude.com/docs/zh-CN/hooks.md) |
| `managedMcpServers` | CLI | [提供给每位匹配开发者](https://code.claude.com/docs/zh-CN/managed-mcp.md#provide-servers-through-managed-settings)的远程 MCP 服务器，与开发者自行添加的服务器并存，仅限 `http` 和 `sse`。请参阅[策略中的 MCP 服务器](#mcp-servers-in-a-policy)。要求网关服务器和客户端上的 Claude Code 为 v2.1.259 或更高版本。更早的客户端会忽略该键。 |

由于这些设置通过网络到达，CLI 在应用以下设置之前会向每位开发者显示安全批准对话框：

* `hooks`
* 需要开发者批准的 `env` 变量，例如代理和基础 URL 变量
* shell 执行设置，例如 `apiKeyHelper` 和 `statusLine`
* 沙箱二进制设置 `sandbox.bwrapPath`、`sandbox.socatPath` 和 `sandbox.ripgrep`
* 拦截流量、注入凭据或削弱隔离的沙箱设置，例如 `sandbox.network.tlsTerminate` 和代理端口设置。[安全批准对话框](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-approval-dialogs)列出了全部此类设置。

[批准记忆](https://code.claude.com/docs/zh-CN/server-managed-settings.md#approval-memory)介绍了批准的有效期以及对话框何时会再次出现。

Claude Code 应用某些交付的 `env` 变量时不会向开发者显示批准对话框，例如模型选择设置和数值限制。其他交付的变量可能需要开发者批准；非空的代理、base-URL 或 `OTEL_EXPORTER_OTLP_ENDPOINT` 值始终需要。当交付的变量需要批准时，对话框会列出它的名称。[环境变量与批准对话框](https://code.claude.com/docs/zh-CN/server-managed-settings.md#environment-variables-and-the-approval-dialog)提供了详细信息，包括四个隐私开关，其交付值决定它们是否需要批准。

网关的[遥测](#telemetry)配置会推送 `OTEL_EXPORTER_OTLP_ENDPOINT`，因此设置 `telemetry.forward_to` 会在每个交互式客户端上触发对话框。该对话框保护的是开发者的机器免受被入侵或恶意网关的影响，而不是保护组织免受开发者的影响。

[非交互式运行](https://code.claude.com/docs/zh-CN/server-managed-settings.md#security-approval-dialogs)（例如 `claude -p`、Agent SDK 会话或 Claude Desktop 的 Code 选项卡中的会话）无法显示对话框。它仅在该次运行中应用推送的设置，并且不会将其记录为已批准，因此开发者的下一个交互式会话仍会显示对话框。

如果开发者拒绝，Claude Code 会退出该会话，而不是应用策略。因此，当您向一个广泛的策略推送新的 hook 或任何会触发对话框的环境变量时，每位匹配的开发者都会在其交互式会话中看到该对话框。正在运行的交互式会话会在下一次每小时轮询时显示它，否则它会在开发者下次交互式启动时出现。

`cli` 键在早期版本中名为 `settings`，网关仍接受该写法作为别名。对于新部署，请使用 [`code`](#choose-cli-or-code)。

#### 在部署前测试策略

要测试策略更改，请让网关连接一个临时的 Postgres 数据库启动。网关会先连接 Postgres 并应用其 schema 迁移，然后才检查 `cli`、`code` 和 `desktop` 块的内容。如果无法连接数据库，启动会在 `could not connect to Postgres` 处停止，且不会报告这些块内容的任何信息。

#### 终端会话中的上下文窗口

通过 `/login` 登录的终端会话对 Opus 4.7 及更高版本、Sonnet 5 及更高版本以及 Fable 模型使用 1M 上下文窗口。模型 ID 不需要 `[1m]` 后缀，会话在约 967K token 时压缩。在开发者机器上的 Claude Code v2.1.287 之前，除非模型 ID 以 `[1m]` 结尾，否则 Claude Code 将 Opus 和 Fable 模型视为具有 200K 窗口。

要让终端会话改为在 200K 边界处压缩，请在策略的 `env` 中设置[自动压缩窗口](https://code.claude.com/docs/zh-CN/model-config.md#set-the-auto-compact-window)：

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        env:
          CLAUDE_CODE_AUTO_COMPACT_WINDOW: "200000"
```

Claude Code 应用此变量时不会向开发者显示批准对话框。该变量适用于所有模型，包括以 `[1m]` 结尾的模型 ID。

要改为关闭 1M 上下文，请在同一 `env` 块中设置 [`CLAUDE_CODE_DISABLE_1M_CONTEXT: "1"`](https://code.claude.com/docs/zh-CN/model-config#extended-context)。Claude Code 随后将每个模型视为具有 200K 窗口。在交互式会话中，每位开发者需要在[批准对话框](#what-goes-in-cli)中批准此变量后它才会生效。

#### 一小时提示缓存

要请求一小时的[提示缓存 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime) 而非默认的五分钟，请将以下键添加到策略现有的 [`code`、`cli` 或 `settings` 块](#choose-cli-or-code)中，然后重启网关。这些键要求网关服务器和客户端上运行 Claude Code v2.1.242 或更高版本。

`promptCacheTtl` 涵盖主对话。`subagentPromptCacheTtl` 是可选的，涵盖子代理、工作流和后台请求。此示例将两者都添加到以 `cli` 为键的 `match: {}` 策略中：

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        promptCacheTtl: 1h
        subagentPromptCacheTtl: 1h
```

> [!WARNING]
>
> 仅当您的上游为网关提供的每个模型都支持一小时 TTL 时才设置这些键。如果上游拒绝该 TTL，请求会失败，网关和 Claude Code 都不会使用五分钟 TTL 重试。对于 Amazon Bedrock，请查看[支持的模型、区域和限制](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models)。

要确认 `promptCacheTtl`，请以策略匹配的用户身份登录网关，然后[检查您的缓存写入使用了哪个 TTL](https://code.claude.com/docs/zh-CN/prompt-caching.md#choose-the-ttl-yourself)。传入 `--model` 以对网关提供的每个模型重复此检查。

一小时缓存写入的[费用高于](https://code.claude.com/docs/zh-CN/prompt-caching.md#cache-lifetime)五分钟写入。要让[支出限额](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md#how-requests-are-priced)计入这一较高费率，网关服务器上需要 v2.1.286 或更高版本。

#### 策略中的 MCP 服务器

要向策略匹配的 Claude Code 客户端提供 MCP 服务器，请在该策略的 `cli` 块中设置 [`managedMcpServers`](https://code.claude.com/docs/zh-CN/managed-mcp#provide-servers-through-managed-settings)。网关服务器和客户端上需要 Claude Code v2.1.259 或更高版本。

网关在启动时使用[与 Claude Code 在客户端上应用的相同规则](https://code.claude.com/docs/zh-CN/managed-mcp.md#what-an-entry-can-contain)检查每个条目；如果某个条目未通过检查，网关将拒绝启动并指明该条目。

如果您在 `gateway.yaml` 中写入 `${VAR}` 引用，网关会在运行条目检查之前，在启动时通过[密钥展开](#secret-expansion)从其环境中解析它，因此每个匹配的客户端都会收到字面值并可以读取它。[提供服务器的标头指南](https://code.claude.com/docs/zh-CN/managed-mcp.md#provide-servers-through-managed-settings)适用于展开后的值。

网关会拒绝 `cli` 块中的 `.mcp.json` 写法 `mcpServers`，其启动错误会指明应使用的键为 `managedMcpServers`。在 v2.1.259 之前，网关会拒绝 `cli` 块中的任何 MCP 服务器定义。

#### Claude Desktop 叠加层

如果您的组织还部署了 [Claude Desktop](https://code.claude.com/docs/zh-CN/desktop.md)，同一个网关可以同时为两个客户端提供服务。在 Claude Desktop 的[托管配置](https://claude.com/docs/third-party/claude-desktop/configuration)中，将 `bootstrapUrl` 指向 `<listen.public_url>/user/bootstrap`。Claude Desktop 从该 URL 派生 OAuth 颁发者，针对此网关运行相同的设备代码登录，并从响应中获取其配置。

> [!NOTE]
>
> 要求网关服务器上的 Claude Code 为 v2.1.203 或更高版本，并且需要显式选择加入：除非与用户匹配的策略带有 `desktop` 键，否则 `/user/bootstrap` 返回 404。空的 `desktop: {}` 即可让策略选择加入，而 `match: {}` 基础层上的 `desktop` 键会让所有继承它的策略选择加入。审计日志会将每个请求记录为 `desktop_bootstrap.serve` 或 `desktop_bootstrap.denied`。

如果您不部署 Claude Desktop，请在策略中完全省略 `desktop`；此时网关会对每个用户从 `/user/bootstrap` 返回 404。

##### 网关为 Claude Desktop 派生的设置

网关从匹配策略的 `cli` 或 `code` 块以及顶层网关配置派生出 bootstrap 响应的大部分内容：

* 模型列表，来自 `availableModels`。该列表不包含策略通过 [`deniedModels` 或 `availableModelsMatch: exact`](#what-goes-in-cli) 阻止的模型。此筛选要求网关服务器上的 Claude Code 为 v2.1.299 或更高版本。[Claude Desktop 中的扩展上下文](#extended-context-in-claude-desktop)介绍了每个模型的 1M 上下文选项
* 禁用的工具，来自仅包含工具名称的 `permissions.deny` 条目。如果您在策略的 `desktop` 块中设置了 `disabledBuiltinTools`，网关会提供您的值与派生列表的并集，因此您可以通过这种方式禁用更多工具，但无法重新启用通过 `permissions.deny` 禁用的工具
* 出站允许列表，来自 `sandbox.network.allowedDomains`。如果您在策略的 `desktop` 块中设置了 `coworkEgressAllowedHosts`，网关会使用该值代替派生列表。如果网关提供的允许列表包含 `*`，且网关未同时提供 Claude Desktop 的 [`allowedWorkspaceFolders`](https://claude.com/docs/third-party/claude-desktop/code#applied-as-managed-policy)，Claude Desktop 不会在 Code 选项卡中启用 [Bash 沙箱](https://code.claude.com/docs/zh-CN/sandboxing.md)
* 一个指向网关自身的 OTLP 端点，以及已登录用户的身份属性。网关会将其在该端点收到的导出数据转发到您的 `forward_to` 目标。当您同时设置 [`telemetry.forward_to`](#telemetry) 和 `listen.public_url` 时，网关会包含该端点和这些属性。

  Claude Desktop 使用一种编码导出所有信号：`http/protobuf`；如果您在策略的 `env` 中将 `OTEL_EXPORTER_OTLP_PROTOCOL` 或其按信号的变体之一设置为 `http/json`，则使用 `http/json`。在网关服务器上的 Claude Code v2.1.261 之前，响应始终设置为 `http/json`，因此仅接受 protobuf 的收集器会拒绝 Claude Desktop 的导出

要在策略的 `desktop` 块中设置 `disabledBuiltinTools`、`coworkEgressAllowedHosts` 或 Claude Desktop 自己的 `managedMcpServers` 设置，网关服务器上需要 Claude Code v2.1.232 或更高版本。Claude Desktop 的 `managedMcpServers` 接受数组值而不是对象。

网关会从 bootstrap 响应中省略没有 Claude Desktop 对应项的键，例如 `hooks` 以及像 `Bash(npm *)` 这样的限定范围权限规则。

##### 在 Code 选项卡中应用 `code` 设置

当以下所有条件都满足时，策略在其 [`code`](#choose-cli-or-code) 键下保存的 Claude Code 设置会在 Claude Desktop 的 Code 选项卡中生效：

* 策略还带有 `desktop` 键，空的 `desktop: {}` 也算
* Claude Desktop 为 2.9939.2 或更高版本
* 机器上的 Claude Code 托管设置[指定了此网关](#client-side-managed-settings)
* 该机器通过私有网络上的 HTTPS 访问网关

后两个条件适用于为会话运行 Claude Code 的那台机器。对于本地会话，那是用户自己的计算机。对于通过 SSH 的 Code 选项卡会话，那是远程主机。

在将 `cli` 重命名为 `code` 之前，请先将[客户端托管设置](#client-side-managed-settings)部署到这些机器。当前两个条件满足而后两个条件之一不满足时，策略匹配的用户会得到以下结果之一：

* **通过 HTTPS，且 Claude Desktop 捆绑了 Claude Code v2.1.296 或更高版本**：Code 选项卡不会启动，对用户提示词的回复会说明原因
* **通过 HTTPS，且 Claude Desktop 捆绑了更早的版本**：Code 选项卡启动但不带您的 `code` 设置，会话中也没有任何提示
* **通过普通 HTTP**：无论 Claude Desktop 捆绑哪个版本，Code 选项卡都会启动但不带您的 `code` 设置

当网关前面的代理用自己的 404 响应 `/managed/settings` 时，Code 选项卡同样会启动但不带您的 `code` 设置，即使在满足全部四个条件的机器上也是如此。

在带有 `desktop` 键的策略中，包含设置的 `code` 块会在 Cowork、Chat 和 Code 选项卡中关闭 WebSearch 工具。关于 MCP 服务器，请参阅 [`managedMcpServers` 的适用范围](https://code.claude.com/docs/zh-CN/managed-mcp#where-managedmcpservers-applies)。

##### 直接设置 Claude Desktop 设置

在 `cli` 旁边添加可选的 `desktop` 块，即可直接设置 Claude Desktop 设置。以扁平键名写入 Claude Desktop [托管配置参考](https://claude.com/docs/third-party/claude-desktop/configuration)中的设置。请省略 Claude Desktop 仅从 MDM 或本地文件读取的键，例如 `bootstrapUrl`；网关会在启动时拒绝它们。

此示例为 `eng-contractors` 组在其 `cli` 设置旁设置了三个 Claude Desktop 键：

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        enforceAvailableModels: true
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

每个键都是可选的；对于您省略的任何键，Claude Desktop 会应用自己的默认值。

##### 网关在启动时拒绝的内容

网关在启动时根据 Claude Desktop 自身使用的配置 schema 验证每个 `desktop` 块，因此错误会在网关启动时以指明该键的错误形式暴露出来，而不会传递到每个已连接的桌面客户端。当块中包含以下内容时，网关会在启动时失败：

* 未知的键
* 已识别的键，但其值会被 Claude Desktop 拒绝或静默丢弃，例如空值或嵌套条目中拼写错误的子键。在 v2.1.260 之前，对于 `managedMcpServers` 或 `orgPluginSettings` 条目的嵌套对象中拼写错误的字段，网关会静默丢弃，而不是在启动时失败。
* 由网关自行计算的键：推理连接、模型列表和 OTLP 中继。请通过 [`upstreams`](#upstreams)、[`models`](#models) 以及 [`telemetry`](#telemetry) 部分的 `forward_to` 配置这些内容。
* 当前键的旧版别名。在启动错误中，网关会指明应写入的规范键。

如果您使用已弃用的值或条目形式，例如不带 `transport` 的 `managedMcpServers` 条目，网关会启动并记录一条指明替代项的警告。

在 v2.1.232 之前，网关只接受由 11 个功能开关键组成的固定列表（例如 `chatTabEnabled` 和 `disableAutoUpdates`），并在启动时拒绝其他所有键。在 v2.1.227 之前，网关还会在启动时拒绝 `chatTabEnabled` 和 `chatAdvancedFileAnalysisEnabled`。

##### 需要更新的网关或 Claude Desktop 版本的键

网关根据其已安装版本捆绑的 schema 验证 `desktop` 块，与验证 `cli` 块的方式相同。要交付由较新 Claude Desktop 版本引入的设置，请先升级网关。例如，`userPluginMarketplacesEnabled` 和 `userPluginUploadsEnabled` 需要网关服务器上的 Claude Code v2.1.260 或更高版本，以及成员机器上的 Claude Desktop 1.37937.0 或更高版本。

`blockReadsOutsideWorkingDirectories`、`disableBypassPermissionsMode`、`configRecheckIntervalMinutes` 和 `sshClientPath` 需要网关服务器上的 Claude Code v2.1.281 或更高版本。`microsoftAuthBroker` 的 `required` 值以及 Microsoft 365 `managedMcpServers` 条目的 `continuousAccessEvaluation` 字段也是如此。早于 `required` 值的 Claude Desktop 版本会将其读取为 `disabled`，因此请仅在每位成员的 Claude Desktop 都支持后再设置 `required`。Claude Desktop 的[托管配置参考](https://claude.com/docs/third-party/claude-desktop/configuration)列出了首次读取每个键的版本。

> [!NOTE]
>
> [`sshTransport`](https://claude.com/docs/third-party/claude-desktop/configuration#sshtransport) 需要网关服务器上为 Claude Code v2.1.293 或更高版本。

如果您在策略的 `desktop` 块中设置了 `orgPluginSettings`，网关会以 Claude Desktop 1.15200.0 及更高版本读取的数组形式提供它。较旧的桌面客户端会忽略该数组且不执行任何插件工具策略，因此在依赖它之前，请将成员更新到 1.15200.0 或更高版本。

##### 角色策略如何继承基础 `desktop` 块

对于策略的 `desktop` 块未设置的键，网关会从 `match: {}` 兜底策略的 `desktop` 块中补充，方式与从基础层补充策略的 `cli` 块相同。如果您在基础层和角色策略中都设置了 `disabledBuiltinTools` 或 `builtinToolPolicy`，网关会保留基础层的限制：

* `disabledBuiltinTools`：网关使用基础层列表与策略列表的并集
* `builtinToolPolicy`：如果您在基础层中将某个工具设置为 `allow` 以外的值，即使您在角色策略中为同一工具设置了 `allow`，网关也会保留该值

对于其他所有键，如果您在角色策略中设置了它，网关会使用角色策略的值。网关会整体替换数组或嵌套对象（例如 `banner`），因此如果您在角色策略中设置了 `banner.text`，网关会丢弃基础层的 `banner.backgroundColor`。

##### 策略更改何时到达 Claude Desktop

使用更改后的策略重新部署网关后，Claude Desktop 大多数设置仅在下次启动时应用：

* **已关闭**：Claude Desktop 在启动时获取 bootstrap 响应，因此更改从下次启动开始生效
* **已打开**：Claude Desktop 默认每 10 分钟检查一次响应是否有更改，并在无需重启的情况下应用少数设置。对于其余设置，例如 [`skillCreationEnabled`](https://claude.com/docs/third-party/claude-desktop/configuration#skillcreationenabled)，用户会在侧边栏中看到 **Relaunch Claude Desktop** 卡片，并在重启应用之前保留之前的配置。默认 24 小时后，Claude Desktop 会显示重启对话框，并在 2 分钟无操作后自行重启

要缩短这 24 小时，请在策略的 `desktop` 块中设置 [`relaunchEnforcementHours`](https://claude.com/docs/third-party/claude-desktop/configuration#relaunchenforcementhours)。网关服务器上需要 Claude Code v2.1.260 或更高版本，成员机器上需要 Claude Desktop 1.40609.0 或更高版本。设置为 `0` 时，Claude Desktop 一发现更改就会显示对话框。

#### Claude Desktop 中的扩展上下文

如果您通过网关提供 [Claude Desktop](#claude-desktop-overlay)，其模型选择器会为每个可以使用 1M 上下文窗口运行的已列出模型提供 1M 上下文选项。这些模型包括 Claude Opus 4.6 及更高版本、Claude Sonnet 4.6 及更高版本以及 Fable 模型。该选项即模型的 `[1m]` 变体，[扩展上下文](https://code.claude.com/docs/zh-CN/model-config.md#extended-context)对此有介绍。网关服务器上需要 Claude Code v2.1.284 或更高版本。

在以下情况下，[`models`](#models) 条目不会获得 1M 选项：

* 可以为该条目提供服务的某个上游将其映射到不支持 1M 的模型，包括网关仅在故障转移时才访问的上游
* 其 `id` 和任何 `upstream_model` 值都没有指定 Claude 模型，例如路由到应用程序推理配置文件 ARN 的自定义别名

要更改选择器提供的内容，请使用以下方式之一：

* **让用户从 1M 选项开始**：在策略的 `desktop` 块中设置 `modelPrefer1mContext: true`。当第一个列出的模型具有 1M 选项时，尚未选择模型的用户会从该 1M 选项开始。已经选择模型的用户保留其选择。
* **手动提供该选项**：如果您的网关服务器运行的版本早于 v2.1.284，或某个条目未指定 Claude 模型，请使用此方式。在 `models` 中将该模型列出两次，一次使用其普通 ID，一次附加 `[1m]`，两者使用相同的 `upstream_model` 映射。Claude Desktop 会将这一对显示为一个带有 1M 选项的模型。网关提供 `[1m]` 条目时不会对其进行检查，因此请仅为上游以 1M 提供服务的模型添加此类条目。

此示例为路由到应用程序推理配置文件的自定义别名手动提供该选项，并让新用户从该选项开始：

```yaml theme={null}
models:
  - id: corp-sonnet
    upstream_model:
      bedrock: arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-5-prod
  - id: corp-sonnet[1m]
    upstream_model:
      bedrock: arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-5-prod

managed:
  policies:
    - match: {}
      desktop:
        modelPrefer1mContext: true
```

##### 移除 1M 选项

要从选择器中移除该选项，请在策略 `cli` 键下的 `env` 块中设置 `CLAUDE_CODE_DISABLE_1M_CONTEXT: "1"`。如果您还列出了 `id` 以 `[1m]` 结尾的条目，网关仍会提供它，因此也请删除该条目。

该变量也会传递到策略匹配的开发者的终端会话。有关它在那里的作用，请参阅[扩展上下文](https://code.claude.com/docs/zh-CN/model-config.md#extended-context)。

#### 与其他托管来源的优先级

如果设备还有通过 MDM 交付的策略或本地 `managed-settings.json`，则网关交付的设置优先级最高。托管设置页面上的[托管层级内的优先级](https://code.claude.com/docs/zh-CN/managed-settings.md#precedence-within-the-managed-tier)说明了本地来源何时生效，并列出了 [Claude Code 从每个管理员来源读取的键](https://code.claude.com/docs/zh-CN/managed-settings.md#keys-read-from-every-admin-source)（无论它选择了哪个来源），例如沙箱锁定键、`forceRemoteSettingsRefresh` 以及按变量的 `env` 合并。在 MDM 配置文件或托管设置文件中配置的 [`policyHelper`](https://code.claude.com/docs/zh-CN/settings-reference#policyhelper) 仅在网关未交付任何设置时运行；该条目说明了其输出会替换哪些内容。

[Claude Desktop](https://code.claude.com/docs/zh-CN/desktop.md) 等嵌入宿主可以通过 SDK 的 `managedSettings` 选项提供策略。[来自嵌入宿主的父级设置](https://code.claude.com/docs/zh-CN/managed-settings.md#parent-settings-from-embedding-hosts)说明了 Claude Code 何时应用它，[限制父级设置](https://code.claude.com/docs/zh-CN/claude-apps-gateway#restrict-parent-settings)列出了在没有 `allowManaged*Only` 锁定时仍然适用的放宽方向设置。

网关策略适用于机器上的每次 Claude Code 调用，包括非交互式 `claude -p` 运行以及由 Agent SDK 生成的会话。如果启动时网关不可达，已登录的会话会以错误退出，而不是在没有其策略的情况下运行。

### `telemetry`

CLI 将指标、日志以及（启用时的）追踪发送到网关，网关将它们原样转发到每个已配置的目标。导出使用基于 HTTP 的 OpenTelemetry Protocol（OTLP）。要跳过中继并让会话直接导出到您的收集器，请[在策略中指定收集器](#export-directly-to-your-collector)。有关 CLI 发出的指标和事件，请参阅[监控使用情况](https://code.claude.com/docs/zh-CN/monitoring-usage.md)。

在通过 `/login` 登录的会话中，CLI 会在每次导出中标注已通过身份验证的用户身份，该身份从网关签发的 JWT 中读取：`user.id`、`user.email` 和 `user.groups` 属性。因此，按开发者的成本和使用情况归因无需开发者端进行任何配置。Claude Code 在开发者登录之前记录的事件[不带有此身份](https://code.claude.com/docs/zh-CN/monitoring-usage.md#standard-attributes)。要了解开发者的组发生变化时哪个属性会随之变化，请参阅[会话打开期间的组变化](#group-changes-during-an-open-session)。

通过网关登录的 [Claude Desktop](#claude-desktop-overlay) 和 Cowork 会话会在其遥测中同时标注 `user.email`、`user.groups` 和 `enduser.id`，因此您可以通过对 `user.email` 或 `user.groups` 进行一次查询来涵盖终端、Desktop 和 Cowork 的使用情况。`user.groups` 是以逗号分隔的 IdP 组列表。

Desktop 和 Cowork 遥测还带有 `enduser.sub`，即您的身份提供商为该用户签发的 `sub` 声明，它在用户电子邮件更改时保持不变。终端会话在 `user.id` 下标注相同的值，因此将 `enduser.sub` 与终端 `user.id` 匹配的查询可以同时涵盖一个用户的终端、Desktop 和 Cowork 使用情况。在 Desktop 和 Cowork 导出中，`user.id` 是一个匿名标识符，而不是主体。

与 Claude Code 的所有 OpenTelemetry 数据一样，这些属性只会发送到您的组织配置的目标，绝不会发送给 Anthropic。

如果用户的组列表经百分比编码后超过 255 个字符，或者某个组名包含逗号或等号，网关会在该用户的 Desktop 和 Cowork 遥测中省略 `user.groups`，而不是将其截断。该用户的终端会话仍带有完整列表。

当主体经百分比编码后超过 255 个字符，或者包含空格、可打印 ASCII 之外的字符，或 `,` `;` `=` `\` `"` `%` 之一时，网关会省略 `enduser.sub`。该用户的 Desktop 和 Cowork 遥测会保留其其他属性。

要在 Desktop 和 Cowork 遥测中包含 `user.email` 和 `user.groups`，网关服务器上需要 Claude Code v2.1.265 或更高版本；要包含 `user.groups`，每位开发者的机器上还需要 Claude Desktop 1.24012 或更高版本。

要包含 `enduser.sub`，网关服务器上需要 Claude Code v2.1.274 或更高版本。

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Per-signal opt-in. Default: metrics only.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

> [!WARNING]
>
> 每个目标独立选择加入 `metrics`、`logs` 和 `traces`，默认仅为指标。这些信号的敏感程度不同：
>
> * **指标**：聚合计数器，例如 token 计数、请求计数和延迟
> * **日志和追踪**：可能包含完整的 Bash 命令、工具输入和文件路径，涵盖 Claude Code 在开发者机器上所做的任何事情
>
> 请仅在具备与这些数据相称的访问控制和保留策略的目标上启用日志和追踪。

每个 `forward_to` URL 必须使用 `https://`，但位于网关自身回环接口上的收集器是一个例外：

* `http://localhost:<port>` 可以通过配置验证，但除非您在网关的环境中设置了 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`，否则 [SSRF 防护](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#threat-model-summary)会以 `ECONNREFUSED_SSRF` 阻止每次导出
* 除非设置了该变量，否则 `http://127.0.0.1:<port>` 或 `http://[::1]:<port>` 会导致启动失败

对于集群内的收集器，请在其自己的内部地址上通过 HTTPS 公开它，或者将其作为 sidecar 运行并设置该变量。

设置了 `HTTPS_PROXY` 时，网关会通过该代理发送导出数据。

要直接访问内部收集器，请将其按主机名或以带前导点的域（例如 `.internal.example.com`）添加到 `NO_PROXY`，这要求网关服务器上的 Claude Code 为 v2.1.277 或更高版本。请确保网关可以不经代理访问该收集器。不带前导点的条目只匹配该确切名称，不匹配其下级名称。CIDR 范围不会匹配。

启用[仅代理出站](#proxy-only-egress)时，请改为在代理中允许该收集器，因为任何 `NO_PROXY` 条目都会使仅代理出站保持关闭。

CLI 中的遥测默认处于关闭状态。当您同时设置 `telemetry.forward_to` 和 `listen.public_url` 时，网关会通过 `/managed/settings` 推送六个环境变量，为已连接的客户端启用遥测：

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`、`OTEL_LOGS_EXPORTER` 和 `OTEL_TRACES_EXPORTER`，如果至少一个 `forward_to` 目标启用了该信号，则各自设置为 `otlp`，否则设置为 `none`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

当您[添加自己的标签](#add-your-own-labels)时，网关还会推送 `OTEL_RESOURCE_ATTRIBUTES`。

在网关服务器上的 Claude Code v2.1.265 之前，网关会将全部三个导出器选择器都推送为 `otlp`，包括没有任何目标选择加入的信号。

推送的端点由公共 URL 构建，因此指标和日志不需要开发者或策略进行任何 OTEL 配置。

通过 `/login` 登录的开发者无法使用自己的 OTEL 配置重定向导出：

* **本地设置的变量**：Claude Code 在托管层级应用推送的变量，因此每个变量都会覆盖开发者在本地为其设置的值。
* **本地配置的端点**：启用 OTLP/HTTP 导出后，无论网关是否推送了遥测变量，CLI 都会忽略任何本地配置的端点。除非策略[将您的收集器指定为端点](#export-directly-to-your-collector)，否则其导出会发送到网关。

如果某个信号没有 `forward_to` 目标，网关会接受并丢弃它。如果开发者已经将 Claude Code 遥测导出到您的某个收集器，请将其添加为 `forward_to` 目标（如果他们导出日志或追踪，则启用相应信号），以便在他们登录后该收集器继续接收其数据。要改为跳过中继，请[在策略中指定收集器](#export-directly-to-your-collector)。

[追踪](https://code.claude.com/docs/zh-CN/monitoring-usage.md#traces-beta)还要求每个客户端上设置 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`。请在托管策略的 `env` 块中设置它，因为网关不会推送它。开发者会在推送的端点已触发的同一[安全批准对话框](#managed)中批准它。

请仅在您希望追踪其组的策略中将其设置为 `1`。根据[合并规则](#managed)，未设置它的策略会从您的 `match: {}` 兜底策略继承该值（如果兜底策略设置了它）。要让某个组的客户端即使在开发者本地设置了该变量时也不发送追踪，请在该组的策略中将其设置为 `0`。

protobuf 和 JSON 两种 OTLP 编码都会被转发，任何兼容 OpenTelemetry 的后端都可以用作目标。

#### 发送累积指标

如果您的收集器只接受累积指标，请将 [`OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE`](https://code.claude.com/docs/zh-CN/monitoring-usage#common-configuration-variables) 设置为 `cumulative`。Claude Code 默认发送增量指标，网关转发它们时不会进行转换。请在每种会话读取该变量的位置设置它：

* **终端会话**：在策略的 [`cli` 或 `code` 键](#choose-cli-or-code)下的 `env` 块中
* **[在用户机器上运行](https://code.claude.com/docs/zh-CN/managed-settings.md#where-and-when-a-policy-applies)的 Cowork 会话**：在该机器上[客户端托管设置](#client-side-managed-settings)的 `env` 块中
* **`code` 策略下的 Code 选项卡会话**：在满足[`code` 设置生效的条件](#apply-code-settings-in-the-code-tab)后，在策略的 `env` 块中
* **`cli` 策略下的 Code 选项卡会话**：在运行该会话的机器上客户端托管设置的 `env` 块中。通过 SSH 时，那是远程主机。

如果您将该变量添加到机器的托管设置中，还需在该机器上优先级最高的来源中设置 [`parentSettingsBehavior: "merge"`](https://code.claude.com/docs/zh-CN/claude-apps-gateway#set-the-opt-in)。否则，Cowork 会话和 `cli` 策略下的 Code 选项卡会话在运行时将不带有 Claude Desktop 作为父级设置传递给它们的出站允许列表和其他策略。

#### 添加自己的标签

要在通过网关登录的会话的遥测上添加固定标签，例如 `service.namespace` 或 `deployment.environment.name`，请设置 `telemetry.resource_attributes`。每个标签都是一个 OpenTelemetry 资源属性，每个目标都会收到相同的标签。

仅当您同时设置 `telemetry.forward_to` 和 `listen.public_url` 时，会话才会获得这些标签。此示例添加了两个标签：

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
  resource_attributes:
    service.namespace: claude
    deployment.environment.name: prod
```

当标签违反以下规则之一时，网关会拒绝启动，启动错误会指明该标签：

* 名称只能使用字母、数字、`.`、`_` 和 `-`
* 名称不能是保留名称。保留名称不区分大小写进行比较，包括所有以 `user.`、`enduser.` 或 `identity.` 开头的名称，以及 `service.name`、`service.version`、`claude.deployment_mode`、`host.arch`、`os.type`、`os.version` 和 `wsl.version`
* 值为非空的可打印 ASCII，不含空格，也不含 `, ; = \ " %` 中的任何字符
* 值在网关按百分比编码后计数时最多 255 个字符，因此 `/`、`:` 和 `@` 各计为三个字符
* 值为文本，因此请为数字、`true` 或 `false` 加引号

要设置 `telemetry.resource_attributes`，网关服务器上需要 Claude Code v2.1.281 或更高版本。更早的网关发现该键时会拒绝启动。请在添加该键之前升级每个副本，并在回滚到更早版本之前移除该键。

通过 `/login` 登录的终端会话会以 `OTEL_RESOURCE_ATTRIBUTES` 的形式接收标签，与其他[遥测变量](#telemetry)一起推送。如果您在策略的 `env` 块中设置了 `OTEL_RESOURCE_ATTRIBUTES`，该策略匹配的终端会话会获得该值而不是这些标签。Claude Desktop 从网关接收标签，同时接收 `user.email` 和其他身份属性。

Claude Code 还会将每个标签复制到每个指标数据点上，因此您可以在不索引资源属性的后端中按标签筛选指标。要关闭此复制，请参阅[指标基数控制](https://code.claude.com/docs/zh-CN/monitoring-usage.md#metrics-cardinality-control)。

#### 会话打开期间的组变化

终端会话将 `user.groups` 放在 OTLP 资源上，并再次放在每个指标数据点和事件上。如果开发者的组在会话打开期间发生变化，下一次[静默刷新](#session)之后的使用情况所对应的数据点和事件会带有新的组。资源会保留旧的组，直到开发者重启 Claude Code，因此请按数据点或事件上的属性进行分组。

如果您在 OpenTelemetry Collector 的 Prometheus remote write 导出器中启用了 `resource_to_telemetry_conversion`，该导出器会用资源的 `user.groups` 替换每个数据点的 `user.groups`，因此每个数据点都会显示旧的组。要保留数据点的值，请在该导出器之前从资源中删除 `user.groups`。

此 OpenTelemetry Collector `resource` 处理器会在列出它的管道中删除该属性：

```yaml theme={null}
processors:
  resource/drop-user-groups:
    attributes:
      - key: user.groups
        action: delete
```

将 `resource/drop-user-groups` 添加到指标管道的 `processors` 之后，每个序列都会带有来自其自身数据点的 `user_groups` 标签。

#### 直接导出到您的收集器

要让通过 `/login` 登录的会话将遥测直接发送到您的收集器而不经过中继，请在[托管策略](#managed)的 `env` 块中将 `OTEL_EXPORTER_OTLP_ENDPOINT` 设置为收集器的 `https://` 基础 URL。Claude Code 会在您设置的 URL（例如 `https://otel-collector.example.com:4318`）后附加 `/v1/metrics`、`/v1/logs` 或 `/v1/traces`，并通过 OTLP/HTTP 将每个信号导出到那里。要求每位开发者机器上的 Claude Code 为 v2.1.265 或更高版本。更早的客户端通过中继导出。

要向收集器进行身份验证，请在同一 `env` 块中设置 `OTEL_EXPORTER_OTLP_HEADERS`。会话永远不会将开发者的网关会话令牌发送到以这种方式指定的收集器。

当您在策略中添加或更改此端点时，Claude Code 会在交互式会话中应用它之前，要求每位开发者在[安全批准对话框](#managed)中批准它。

Claude Code 在直接导出信号之前会检查端点，检查失败时该信号会保留在中继上。检查包括：

* 端点来自网关本身。如果您在 MDM 配置文件或本地 `managed-settings.json` 中设置了同一变量，导出会保留在中继上。
* URL 使用 `https://`，或使用指向回环地址的 `http://`
* URL 解析为以 `/v1/<signal>` 结尾的路径，且不带查询或片段。Claude Code 会根据通用变量自行构建该路径。对于按信号的变量（例如 `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`），它会按原样使用，因此请在其中包含完整路径。
* URL 不是网关自己的主机。指向网关的端点会保留中继路径及其会话令牌。
* 您和开发者都未在任何设置来源中配置 [`otelHeadersHelper`](https://code.claude.com/docs/zh-CN/settings-reference#otelheadershelper)。配置了该辅助程序时，每个信号都会保留在中继上。

您指定的端点只改变导出的去向。您仍然需要通过 `OTEL_*_EXPORTER` 选择器决定哪些信号要导出。

仅设置端点并不会启用导出，因此还需设置启用导出的变量，除非网关已经推送了它们：

* 如果网关已经[推送了遥测变量](#telemetry)，它们涵盖启用、选择器和协议，您显式设置的端点会覆盖推送的 `<public_url>` 值。仅对没有任何 `forward_to` 目标启用的信号，才需要您自行将 `OTEL_*_EXPORTER` 选择器设置为 `otlp`。
* 如果网关没有推送，还需设置 `CLAUDE_CODE_ENABLE_TELEMETRY=1`、`OTEL_*_EXPORTER` 选择器和 `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`。

当开发者注销或登录到其他网关时，到收集器的导出会停止，Claude Code 会丢弃每个剩余批次而不是发送它们。

#### 目标失败时

网关不会缓冲、重试或存储遥测，因此对于未能到达目标的导出，它会将其丢弃，而不是延迟交付。每个目标独立成功或失败，并且无论哪种情况，导出的客户端都会收到成功响应，因此交付失败只会出现在网关的日志中。

向某个目标连续五次交付失败后，网关会以 30 秒为间隔暂停向其转发，并记录每次暂停，直到某次交付成功。任何错误响应、超时或连接错误都计为交付失败，但 `400`、`413`、`415`、`422` 和 `431` 除外，这些表示收集器因负载格式错误或过大而拒绝了该次导出。

被拒绝的负载既不会增加也不会重置失败计数：网关会继续向该目标转发，并在该目标第一次拒绝时以及此后每第一百次拒绝时，记录一条指明该目标和状态的警告。

### HTTP 调优

四个可选的顶层块 `access_control`、`limits`、`timeouts` 和 `rate_limits` 用于调优 HTTP 层面的行为。默认值适用于大多数部署。

| 块 | 键 | 默认值 | 说明 |
| - | - | - | - |
| `access_control` | `allow_cidrs` / `deny_cidrs` | 空 | 在完成 `trusted_proxies` 解析后，按客户端地址对入站 IP 进行允许/拒绝。会先检查 `deny_cidrs`；与其匹配的客户端会被拒绝，即使 `allow_cidrs` 也匹配。如果 `allow_cidrs` 非空，网关默认拒绝。`/healthz` 和 `/readyz` 不受 `allow_cidrs` 约束。当受信任的代理发送的 `X-Forwarded-For` 条目不是 IP 地址时，真实客户端未知，网关会记录一次警告，指明需要检查的内容。如果任一列表适用于该请求，网关将以 `403` 拒绝该请求，审计原因为 `xff_unparseable`。如果两个列表都不适用，网关会处理该请求，并使用代理自身的地址作为客户端 IP，用于按 IP 的速率限制和审计。 |
| `limits` | `max_request_bytes` | 32 MiB | 入站请求体的最大大小；超出大小的请求会在缓冲请求体之前收到 `413`。对于大型文件或图片请求，请调高此值。 |
| `limits` | `max_request_header_bytes` | 未设置 | 降低网关对请求头总大小的 256 KiB 限制。超出限制的请求返回 `431`，大于 256 KiB 的值不起作用。如果开发者在登录后收到 `431`，请参阅[登录后请求头过大](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#request-headers-too-large-after-sign-in)。 |
| `limits` | `max_url_length` | 未设置 | 设置后，过长的 URL 返回 `414` |
| `timeouts` | `upstream_ttfb_ms` | 120000 | 等待上游响应标头的最长时间（首字节时间）。之后响应体以流式传输，没有总时长上限。如果未设置此键，默认值适用于 `provider: anthropic` 上游；对于其他所有提供商，网关最多等待一小时让响应开始。如需缩短此等待时间，请参阅[多个上游](#multiple-upstreams)。 |
| `rate_limits` | `device_authorization.max` / `.window_seconds` | 30 / 600 | 对未经身份验证的设备授权端点按 IP 施加的速率限制。对于位于共享出口 IP 或 NAT 之后的大型组织，请调高此值。[大规模推广](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#large-rollouts)说明了如何确定其大小。这些限制仅适用于设备授权登录流程，不适用于 `/v1/messages` 推理。请参阅[用户码暴力破解防护](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#user-code-brute-force-resistance)。 |
| `rate_limits` | `device_verify.max` / `.window_seconds` | 10 / 600 | 对 `/device` 处 `user_code` 提交按 IP 施加的速率限制。它用于阻止他人猜测其他开发者的码。[大规模推广](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#large-rollouts)说明了可以将其调高到什么程度。 |

如果两个 `access_control` 列表都留空（即默认情况），网关会为任何客户端地址提供服务，因此只有您的网络能限制谁可以访问它。这一点很重要，因为网关可以推送在开发者机器上运行命令的[托管设置](#managed)。

当 `allow_cidrs` 为空时，网关会在两个地方发出警告，但不会改变其响应任何请求的方式：

* **启动时**：运行日志中的一条警告建议仅允许私有地址段 `10.0.0.0/8`、`172.16.0.0/12`、`192.168.0.0/16`、`100.64.0.0/10`、`127.0.0.0/8`、`::1/128` 和 `fc00::/7`，以及开发者连接所用的其他内部地址段。如果将网关绑定到环回地址，并且既未设置 `trusted_proxies` 也未设置 `public_url`（如本地开发时），则不会出现该警告。
* **运行时**：当首次有请求来自这些私有地址段之外的地址时，网关会记录一条警告，并发出一个携带客户端 IP 的 [`access.public_client` 审计事件](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy#logs)。两者在每个进程中各触发一次。链路本地地址 `169.254.0.0/16` 和 `fe80::/10` 不算作公网地址。网关会在此检查运行之前响应 `/healthz` 和 `/readyz`，因此来自公网地址段的健康探测不会触发它。

这两种信号都使用网关解析得到的客户端地址。如果负载均衡器、端口转发或隧道中继了流量，但未列在 `listen.trusted_proxies` 中，网关看到的将是中继的地址（通常是私有地址），因此运行时警告和私有允许列表都无法捕获经由它中继的流量。

在此类前端之后，请先设置 [`listen.trusted_proxies`](#listen)，使网关能看到真实的客户端地址，并且无论如何都要确保网关及其前面的所有组件无法从公网访问。

### `load_test_mode`

`load_test_mode` 块可让您在不调用模型提供商的情况下对网关进行负载测试。启用后，网关会照常构建并签名每个提供商请求，但将其丢弃而不发送，并通过其正常的响应路径以流式方式返回一条预设回复。该回复是填充文本，开头有一句话说明它是预设的。

要求网关服务器上运行 Claude Code v2.1.282 或更高版本。更早版本的网关在发现该键时会拒绝启动。请在添加该块之前升级每个副本，并在回滚之前移除该块。

以下示例使用默认值启用该模式，即约 750 个 token 的文本回复，在约 10 秒内以流式传输：

```yaml theme={null}
load_test_mode:
  enabled: true
  reply_tokens: 750     # roughly how many tokens of text each canned reply carries
  reply_seconds: 9.5    # how long a streamed reply takes
```

| 字段 | 必需 | 说明 |
| - | - | - |
| `enabled` | 是 | `true` 启用该模式。`false` 会在关闭该模式的同时将您的数值保留在文件中。如果存在该块但缺少此字段，网关会拒绝启动。 |
| `reply_tokens` | 否 | 默认 `750`。每条预设回复大约包含多少个 token 的文本，取值为 1 到 100000 之间的整数。 |
| `reply_seconds` | 否 | 默认 `9.5`。一条流式回复所需的时间，取值为 0 到 600。`0` 会一次性发送整条回复。对非流式请求的回复始终一次性返回。 |

此模式下的负载测试涵盖网关、您的 Postgres 以及网关前面的所有组件。它不涵盖提供商的限制、速度或网络路径。

由于不会向提供商发送模型请求，副本每个请求的 CPU 用量只是估算值，且读数低于生产环境，因为生产环境还需要加密发往提供商的流量。请针对真实提供商进行小规模试点，以确认副本数量。在 v2.1.283 之前，该估算值读数会低得多。

启用该模式时，请求可以携带一个 `x-load-test-user` 请求头，其值为最多七位数的整数。网关会将每个数字计为一名独立的开发者，并使用随请求附带的令牌所属开发者的电子邮件和组。

请为负载测试部署提供单独的空数据库，因为如果数据库中已有任何开发者产生过花费，网关在启用该模式时会拒绝启动。

> [!WARNING]
>
> 切勿在开发者使用的网关上启用此模式。每个请求都会收到预设回复，不会调用任何模型。启用该模式时，网关会在启动时记录一条 `load_test_mode is on` 警告，并在每个 `inference` [审计事件](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md#logs)上标记 `load_test: true`。

## 完整示例

此完整参考配置演示了每个核心部分；[HTTP 调整块](#http-tuning)保持其默认值。复制它，删除您不需要的，并填入您的值。[快速入门](https://code.claude.com/docs/zh-CN/claude-apps-gateway#quickstart)中的配置是此的最小版本。

```yaml gateway.yaml theme={null}
# 运行方式：
#   claude gateway --config gateway.yaml
#
# 操作日志详细程度由 CLAUDE_GATEWAY_LOG_LEVEL
# 环境变量控制（debug | info | warn | error；默认 info）。debug
# 也会记录每个 id_token 中的声明名称，用于 groups_claim 诊断。
# 它不影响审计事件，总是发出。

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # 在 TLS 终止入口后面运行时省略 tls 块。
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # 当发行者是 Okta 组织服务器时需要，其 id_tokens
  # 可以省略电子邮件和组；网关从 /userinfo 填充它们。
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta 仅在请求 `groups` 范围且
  # 应用的组声明过滤允许它们时发出组。下面的承包商策略
  # 匹配组，因此范围在此处请求。
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Entra 应用角色：使用 `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5
  # connect_timeout_seconds: 5
  # readiness_grace_seconds: 300   # 在数据库故障转移期间保持通过就绪检查

# 启用 /v1/organizations/spend_limits（镜像 Anthropic Admin API）
# 和 /v1/messages 上的每开发者支出强制。省略以禁用。
# 上限本身通过 admin API 设置，而不是此处。
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

# 在不调用模型提供商的情况下对此部署进行负载测试。永远不要在
# 开发人员使用的网关上：每个请求都会获得一个预设回复。
# load_test_mode:
#   enabled: true
#   # reply_tokens: 750
#   # reply_seconds: 9.5

# 按合同费率而不是美元列表价格计费。需要 admin: 或
# managed: 策略。使用 managed:，相同的费率也会发送到已登录的客户端。
# 下面的费率是占位符，不是真实合同价格。
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: mantle
  #   region: us-east-1
  #   models: [claude-opus-4-8, claude-opus-4-7, claude-haiku-4-5]
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # mantle: anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # allow 自动批准这些工具；它不阻止其余的。
        # 添加拒绝规则以限制工具。
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        # 将默认选择器选项限制为每个策略的 availableModels，
        # 而不是内置默认值，因此任何角色都不会在默认上获得 400。
        # 承包商策略继承此键。
        enforceAvailableModels: true
        permissions:
          allow: [Read, Grep, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

## 客户端托管设置

上面的所有内容配置网关服务器。将开发者机器指向网关是在每个设备上单独配置的，通过 Claude Code 的[托管设置](https://code.claude.com/docs/zh-CN/managed-settings.md)。网关无法自己推送登录密钥，因为这些密钥是告诉客户端网关在哪里的内容。

对于 CLI，在每个 OS 的 `managed-settings.json` 中设置这些密钥。这两个登录密钥将每个开发者的 `/login` 路由到你的网关：

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` 保持 Claude Desktop 向其嵌入式 Claude Code 会话传递出站允许列表的功能；[向 Claude Desktop 会话传递策略](https://code.claude.com/docs/zh-CN/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions)解释了该机制以及选择加入必须位于的位置。

为了防止开发者通过云提供商变量或他们自己的 `ANTHROPIC_BASE_URL` 绕过网关，请在同一文件中添加 `"allowedProviders": ["gateway"]`。Claude Code 随后会拒绝机器上未设置为云网关的每个会话，并仅允许网关在它是 `forceLoginGatewayUrl` 命名的网关或文件的 `env` 块将其 URL 设置为 `ANTHROPIC_BASE_URL` 的网关时。`claude gateway` 拒绝在设置该列表的机器上运行，因此请在网关主机上保持该密钥关闭。请参阅设置参考中的 [`allowedProviders`](https://code.claude.com/docs/zh-CN/settings-reference#allowedproviders) 条目。需要 Claude Code v2.1.285 或更高版本。

将 `managed-settings.json` 文件部署到每个设备，通常通过你的 MDM 平台。文件路径因平台而异。请参阅[每个机制存储策略的位置](https://code.claude.com/docs/zh-CN/managed-settings.md#where-each-mechanism-stores-the-policy)。

默认情况下，Windows 上的注册表策略或 macOS 上的托管首选项 plist 会替换 `managed-settings.json` 文件而不是与其合并，除了[上面的例外密钥和跨源检查](#precedence-with-other-managed-sources)。此代码片段中的所有三个密钥都遵循最高优先级源规则，因此通过组策略或配置文件传递策略的团队必须改为将所有三个密钥放在该机制中。

对于 Claude Desktop，在 Claude Desktop 自己的[托管配置](https://claude.com/docs/third-party/claude-desktop/configuration)中设置 `bootstrapUrl` 密钥为 `<listen.public_url>/user/bootstrap`。登录流程和每组策略随后与 CLI 的匹配，一旦策略通过 `desktop` 密钥在服务器端选择加入；没有选择加入，`/user/bootstrap` 返回 404。有关服务器端部分，请参阅 [Claude Desktop 覆盖层](#claude-desktop-overlay)。

Claude Code 从机器上的托管源读取并遵循 [`forceLoginGatewayUrl`](https://code.claude.com/docs/zh-CN/settings-reference#forcelogingatewayurl)、[`gatewayInternalNetworks`](https://code.claude.com/docs/zh-CN/settings-reference#gatewayinternalnetworks) 和 [`forceLoginMethod`](https://code.claude.com/docs/zh-CN/settings-reference#forceloginmethod) 的 `"gateway"` 值：`managed-settings.json`、macOS plist 或 Windows HKLM 注册表，或策略助手。在网关负载中设置它们不会配置网关登录。关于开发者自己的 `~/.claude/settings.json`，请参阅[在用户设置中设置网关 URL](https://code.claude.com/docs/zh-CN/claude-apps-gateway#set-the-gateway-url-in-user-settings)。

不要在有效负载中包含 `forceLoginMethod` 和 `forceLoginOrgUUID`。Claude Code 仍然从有效负载中读取这两个密钥以进行启动凭证检查，因此在机器上保留 Anthropic 颁发的凭证的开发者会获得[管理员策略需要云网关登录](https://code.claude.com/docs/zh-CN/errors.md#administrator-policy-requires-a-cloud-gateway-sign-in)下描述的启动退出，即使他们已经登录。

## 相关

* [Claude 应用网关概述](https://code.claude.com/docs/zh-CN/claude-apps-gateway)：快速入门和开发者连接
* [部署指南](https://code.claude.com/docs/zh-CN/claude-apps-gateway-deploy.md)：IdP 设置、容器镜像、Kubernetes 和 Cloud Run，以及操作
* [支出限制](https://code.claude.com/docs/zh-CN/claude-apps-gateway-spend-limits.md)：每开发者上限和 Admin API
