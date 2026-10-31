> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在自托管环境中验证会话身份

> 验证 CLAUDE_CODE_SESSION_ACCESS_TOKEN JWT，以便网络上的服务可以信任来自自托管环境中会话的请求。

> [!NOTE]
>
> 自托管环境在 Team 和 Enterprise 计划上处于公开测试阶段；[所有者](https://code.claude.com/docs/zh-CN/cloud-environments.md#organization-shared-environments)可以通过在[**云环境**管理页面](https://claude.ai/admin-settings/cloud-environments)上打开**允许自托管环境**来启用它们。本页面涵盖会话身份验证；有关设置，请参阅[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md)，有关舰队配方，请参阅[部署到生产](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)。

[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)让 Claude Code [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)在您运营的基础设施上运行，而不是在 Anthropic 的基础设施上运行。由于会话在您的网络内运行，Claude 可以直接调用您的内部服务。这些服务需要一种方式来确认请求来自您环境中的 Claude Code 会话，并识别创建该会话的用户或服务身份。

自托管环境中的每个会话都会在 `CLAUDE_CODE_SESSION_ACCESS_TOKEN` 环境变量中收到一个签名的 JSON Web Token (JWT)。会话像任何持有者凭证一样呈现令牌；例如，Claude 运行的脚本可以使用 `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"` 调用您的服务。Anthropic 对令牌进行签名，并在公开 JWKS 端点发布验证密钥。您的服务获取这些密钥，验证签名，并读取声明以决定授予什么访问权限。

## 会话令牌

在编写验证代码之前，了解令牌建立的内容以及 JWT 库将看到的形状。

### 令牌证明的内容

有效的令牌建立了一些事实，但故意不建立其他事实：

* **证明**：Anthropic 为特定环境中的特定会话发布了令牌，以及会话的创建方式：由您组织中的用户创建，或由您组织的服务身份创建，这是 [Claude Tag 频道会话](https://claude.com/docs/claude-tag/concepts/agent-identity)的启动方式
* **不证明**：运行程序主机上的哪个进程呈现它。令牌位于会话内的环境变量中，因此 Claude 运行的任何代码以及会话启动的任何工具或 MCP 服务器都可以读取并呈现它。

对您的服务的两个后果：

* 根据您的环境 ID（在[**云环境**管理页面](https://claude.ai/admin-settings/cloud-environments)上与您的环境一起显示的 `ccpool_...` 值）验证 `aud` 声明，以拒绝发布给任何其他组织环境的令牌。
* 将从令牌派生的凭证范围限制在单个编码会话应该能够做的事情，而不是会话创建者能够做的一切。请参阅[范围派生凭证](#scope-derived-credentials)。

### 令牌格式

`CLAUDE_CODE_SESSION_ACCESS_TOKEN` 的值具有 `sk-ant-cc-` 前缀，后跟标准的三部分 JWT：

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

在将值传递给 JWT 库之前，请删除前缀。发布给 Anthropic 托管云会话的令牌改为携带 `sk-ant-si-` 前缀，并由不同的密钥集签名，因此拒绝任何不以 `sk-ant-cc-` 开头的值。

签名算法是 `ES256`，这是 P-256 曲线上的 ECDSA，带有 SHA-256。令牌头部携带一个 `kid`，用于标识 JWKS 中的哪个密钥对其进行了签名。

## 验证令牌

验证可以在两个位置之一进行。您网络中的服务可以使用 Anthropic 发布的密钥对令牌进行加密验证，而会话内的包装脚本则可以改用运行器二进制文件内置的解码器。

### 从您的服务验证令牌

Anthropic 在一个公开、无需身份验证的端点上发布验证密钥：

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

响应是标准的 [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517)。Anthropic 会定期轮换签名密钥，轮换前的密钥会在密钥集中保留足够长的时间，使其签发的令牌仍能通过验证，因此请勿固定使用单个密钥。该端点设置了 `Cache-Control: public, max-age=300`，因此缓存密钥集并每五分钟重新获取一次是安全的。

请按以下检查项验证每个传入的令牌：

**1. 检查前缀**

如果值不以 `sk-ant-cc-` 开头，则拒绝该值，否则移除该前缀。剩余部分是一个标准的紧凑格式 JWT。

**2. 验证签名**

获取 JWKS，选择 `kid` 与令牌头部匹配的密钥，并验证 `ES256` 签名。拒绝 `alg` 头部不是 `ES256` 的令牌。如果传入令牌的 `kid` 不在您缓存的密钥集中，请在拒绝之前重新获取一次 JWKS：密钥轮换后，新令牌会使用您缓存的密钥集中尚未包含的密钥进行签名。

**3. 验证签发者**

如果 `iss` 不完全等于 `ccr`，则拒绝该令牌。

**4. 根据您的环境验证受众**

`aud` 声明是一个数组。除非其中包含您的环境 ID（格式为 `ccpool_...`），否则拒绝该令牌。环境 ID 显示在 [**Cloud environments** 管理页面](https://claude.ai/admin-settings/cloud-environments)中您环境的详情对话框里，也会作为 `ccr:pool_id` 声明出现在该环境的任意会话令牌中。正是这项检查将令牌限定在您的环境内，并拒绝签发给其他组织的令牌。

**5. 验证角色**

如果 `ccr:role` 不完全等于 `session_worker`，则拒绝该令牌。为自托管环境签发的其他令牌（例如环境密钥、运行器令牌和工单）由同一密钥集签名，但携带不同的角色。

**6. 验证过期时间**

如果 `exp` 已经过去，则拒绝该令牌。Anthropic 签发的会话令牌默认有效期为四小时，最长为八小时。运行器会在令牌过期前刷新令牌并将新值推送到会话中，因此 Claude 在刷新后启动的子进程会继承新令牌。因此，一个会话在其生命周期内可能会向您的服务出示多个不同的有效令牌。

**7. 读取身份**

创建者的用户身份位于 `act` 声明中。`act.sub` 是其 Anthropic 用户 ID，采用带前缀的形式 `user:<id>`；`act.email`（当创建会话的使用入口记录了该信息时）是其电子邮件地址。由您组织的服务身份创建的会话则携带 `agent:` 主体。仅当 `act.sub` 带有 `user:` 前缀时才将会话视为用户创建，而不是通过检测身份声明是否缺失来判断。

有关完整结构和扁平的重复声明，请参阅[声明参考](#claims-reference)。

这些检查可以直接对应到标准 JWT 库。以下示例分别在 Node.js 中使用 [`jose`](https://www.npmjs.com/package/jose)（它负责 JWKS 的获取、缓存和 `kid` 选择），以及在 Python 中使用 [`PyJWT`](https://pyjwt.readthedocs.io/) 及其内置的 JWKS 客户端，实现了完整的检查流程。

> [tab] Node.js (jose)
>
> ```typescript theme={null}
> import { createRemoteJWKSet, jwtVerify } from "jose";
>
> const JWKS = createRemoteJWKSet(
>   new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
> );
>
> const PREFIX = "sk-ant-cc-";
> const EXPECTED_POOL_ID = "ccpool_...";
>
> export async function verifySessionToken(raw: string) {
>   if (!raw.startsWith(PREFIX)) {
>     throw new Error("not a self-hosted runner session token");
>   }
>   const jwt = raw.slice(PREFIX.length);
>
>   const { payload } = await jwtVerify(jwt, JWKS, {
>     issuer: "ccr",
>     audience: EXPECTED_POOL_ID,
>     algorithms: ["ES256"],
>   });
>
>   if (payload["ccr:role"] !== "session_worker") {
>     throw new Error("token is not a session_worker token");
>   }
>
>   const act = payload.act as { email?: string; sub?: string };
>   return {
>     sessionId: payload["ccr:session_id"] as string,
>     poolId: payload["ccr:pool_id"] as string,
>     orgId: payload["ccr:org_id"] as string,
>     creatorEmail: act?.email,
>     creatorSub: act?.sub,
>   };
> }
> ```

> [tab] Python (PyJWT)
>
> ```python theme={null}
> import jwt
> from jwt import PyJWKClient
>
> JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
> PREFIX = "sk-ant-cc-"
> EXPECTED_POOL_ID = "ccpool_..."
>
> jwks = PyJWKClient(JWKS_URL)
>
>
> def verify_session_token(raw: str) -> dict:
>     if not raw.startswith(PREFIX):
>         raise ValueError("not a self-hosted runner session token")
>     token = raw.removeprefix(PREFIX)
>
>     signing_key = jwks.get_signing_key_from_jwt(token)
>     payload = jwt.decode(
>         token,
>         signing_key.key,
>         algorithms=["ES256"],
>         issuer="ccr",
>         audience=EXPECTED_POOL_ID,
>     )
>
>     if payload.get("ccr:role") != "session_worker":
>         raise ValueError("token is not a session_worker token")
>
>     act = payload.get("act") or {}
>     return {
>         "session_id": payload["ccr:session_id"],
>         "pool_id": payload["ccr:pool_id"],
>         "org_id": payload["ccr:org_id"],
>         "creator_email": act.get("email"),
>         "creator_sub": act.get("sub"),
>     }
> ```

### 在会话内验证令牌

[包装脚本](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#wrapper-scripts)在会话内、Claude 启动之前运行。它们无需调用 JWT 库，而是可以运行运行器二进制文件的 `self-hosted-runner decode-token` 子命令。该子命令依次从位置参数、`CLAUDE_CODE_SESSION_ACCESS_TOKEN` 或通过管道传入的 stdin 读取令牌，然后去除前缀，根据 JWKS 端点验证签名，检查过期时间，并以 JSON 格式打印声明。该子命令仅执行签名和过期检查；它不会检查 `iss`、`aud` 或 `ccr:role`。当您的包装脚本的授权决策依赖于这些声明时，请从打印的 JSON 中读取它们并进行显式比较。

以下命令提取创建者身份，优先使用电子邮件地址，其次使用创建者的 `act.sub` 主体（`user:<id>` 或 `agent:<id>`）：

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.email // .act.sub'
```

包装脚本会通过 `CLAUDE_RUNNER_CLAUDE_BIN` 获得运行器自身二进制文件的绝对路径；请使用该路径，而不是通过 PATH 解析的 `claude`，以便解码操作在运行器本身所使用的同一个二进制文件上运行。

请使用 `jq -re` 而不是 `jq -r`，这样缺失的声明会导致非零退出码。如果仅使用 `-r`，缺失的声明会打印字面字符串 `null` 并以零退出码退出，从而悄无声息地将错误值传递到下游。

如果 `decode-token` 无法从 JWKS 端点获取密钥或无法验证令牌，它会将原因打印到 stderr，不打印任何声明，并以退出码 1 退出。仅在 JWKS 端点无法访问、需要进行离线检查时，才向 `decode-token` 传递 `--no-verify`。

## 声明参考

从 `ccr:*` 命名空间和 `act` 链读取身份；平面 `account_email`、`organization_uuid` 和 `account_uuid` 声明是可能被删除的向后兼容性重复项。您组织的服务身份创建的会话在 `act.sub` 中携带 `agent:` 主题，并省略 `act.email`、`ccr:account_id`、`account_email` 和 `account_uuid`。

两个电子邮件声明对于用户创建的会话也是可选的。Anthropic 仅在创建请求的凭据携带电子邮件时才会在会话创建时记录它们，从 CLI 分派的会话可能两者都缺少。请根据 `act.sub` 或 `ccr:account_id` 而不是电子邮件来确定身份。令牌也可以携带此表之外的其他声明；忽略您不认识的声明。下表列出了与验证相关的会话令牌声明。

| 声明 | 类型 | 描述 |
| :- | :- | :- |
| `iss` | 字符串 | 始终为 `ccr`。 |
| `sub` | 字符串 | `ccr:session:<session_id>`。 |
| `aud` | 字符串数组 | 始终包含 `anthropic-api`。对于自托管环境中的会话，数组还包含您的环境 ID，例如 `ccpool_...`。验证环境 ID，而不是 `anthropic-api`。 |
| `exp` | 数字 | 过期时间为 Unix 时间戳。四小时默认生命周期，八小时最大值。 |
| `iat` | 数字 | 发布时间为 Unix 时间戳。 |
| `jti` | 字符串 | 唯一令牌标识符。 |
| `ccr:role` | 字符串 | 对于会话令牌始终为 `session_worker`。 |
| `ccr:session_id` | 字符串 | `cse_...` 形式的会话 ID。与 `sub` 的后缀相同的值，也与 [`CLAUDE_CODE_REMOTE_SESSION_ID`](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#wrapper-scripts) 是相同的字符串。要将其与[生命周期 hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#lifecycle-hooks) 接收的 `CLAUDE_RUNNER_SESSION_ID` 匹配，请将 `cse_` 前缀替换为 `session_`。 |
| `ccr:pool_id` | 字符串 | 您的环境 ID。与出现在 `aud` 中的值相同。 |
| `ccr:org_id` | 字符串 | 您的 Anthropic 组织 ID。 |
| `ccr:account_id` | 字符串 | 创建用户的 Anthropic 账户 ID：`act.sub` 的值去掉 `user:` 前缀，一个标记的 `user_...` ID。与[spawn-runner hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#the-spawn-runner-hook) 的 `CLAUDE_RUNNER_ACCOUNT_ID` 携带的值相同，以及 [`--lock-to-account`](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference#runner-cli-flags) 接受的值，因此三者作为相等的字符串进行比较。 |
| `account_email` | 字符串 | `act.email` 的重复；每当 `act.email` 不存在时就不存在。 |
| `organization_uuid` | 字符串 | 您的 Anthropic 组织 UUID。 |
| `account_uuid` | 字符串 | 创建用户的 Anthropic 账户 UUID。 |
| `act` | 对象 | [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) 委托链。请参阅[`act` 链](#the-act-chain)。 |

### `act` 链

`act` 声明记录从创建会话的用户或服务身份到[环境](https://code.claude.com/docs/zh-CN/self-hosted-environments#key-concepts)（其机密允许运行程序）以及创建该机密的身份的完整委托路径。创建者是最外层的参与者，因此 `act.sub` 直接标识他们。

| 路径 | 描述 |
| :- | :- |
| `act.sub` | 创建用户的 Anthropic 用户 ID，形式为 `user:<id>`，或当您组织的服务身份创建会话时为 `agent:<id>`。 |
| `act.email` | 创建用户的电子邮件地址，当在会话创建时记录了一个时。不要求它；根据 `act.sub` 确定身份。 |
| `act.attested_by` | 保留用于上游身份提供程序对创建用户的证明。预期它不存在，请勿依赖它。根据 `act.sub` 确定身份。如果您需要地址，请在 `act.email` 存在时读取它。 |
| `act.act` | 生成会话的运行程序。`act.act.sub` 是 `ccr:runner:<runner_id>`。 |
| `act.act.act` | 环境。`act.act.act.sub` 是 `ccr:pool:<pool_id>`。 |
| `act.act.act.act` | 创建运行程序注册的环境机密的身份。链在此处结束。 |

## 范围派生凭证

会话令牌标识创建会话的用户或服务身份，但不要将其视为等同于该创建者直接登录。令牌位于会话内的环境变量中，因此 Claude 运行的任何代码以及会话启动的任何工具或 MCP 服务器都可以读取并呈现它。

验证也是离线的：根据 JWKS 验证的令牌在其 `exp` 之前保持有效，无论自那以后会话发生了什么，Anthropic 不为会话令牌发布撤销源。相应地绑定您从令牌派生的任何内容。

当您的服务将令牌交换为内部凭证时，发布范围限制在一个编码会话应该能够到达的凭证：

* **限制功能**：授予对会话编码任务所需资源的读写访问权限，而不是创建者在其他地方持有的管理功能。
* **限制生命周期**：将派生凭证绑定到令牌的 `exp` 或更短。
* **作为会话审计**：记录 `ccr:session_id` 和 `jti` 以及创建者身份，以便您可以将操作追踪回特定会话。

## 相关环境变量

创建者身份也以纯环境变量的形式出现在两个从不验证令牌的表面上：

* **[`spawn-runner` hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration#the-spawn-runner-hook)，在编排器上**：hook 在任何运行程序存在于排队会话之前运行，并在 `CLAUDE_RUNNER_ACCOUNT_EMAIL` 和 `CLAUDE_RUNNER_ACCOUNT_ID` 等变量中接收创建者身份。编排器从工作订单（授权生成一个运行程序的签名单次使用令牌）读取它们，而不验证工作订单的签名本身；声明是受信任的，因为工作订单通过编排器与 Anthropic 的连接到达，环境机密对其进行身份验证。
* **[包装脚本](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#wrapper-scripts)，在会话内**：包装脚本接收 `CCR_SESSION_ACCOUNT_EMAIL`，创建者的电子邮件从令牌中预提取，无需签名验证。该变量适合用于标记，例如提交预告片，而不是用于身份验证决定。

使用纯变量进行编排器端决定，例如选择机器映像。当下游服务需要独立的加密证明而不是信任运行程序的环境时，使用 `CLAUDE_CODE_SESSION_ACCESS_TOKEN`。

## 接下来

* [自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)：环境、运行程序和会话模型；[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md)和[部署到生产](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)包含设置和操作
* [自定义会话](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md)：使用令牌的包装脚本和 `spawn-runner` hook
* [参考](https://code.claude.com/docs/zh-CN/self-hosted-environments-reference.md)：CLI 标志、环境变量和指标
