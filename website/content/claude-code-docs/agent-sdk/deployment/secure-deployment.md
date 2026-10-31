> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 安全部署 AI 代理

> 关于使用隔离、凭证管理和网络控制来保护 Claude Code 和 Agent SDK 部署的指南

Claude Code 和 Agent SDK 可以代表您执行代码、访问文件并与外部服务交互。

与遵循预定代码路径的传统软件不同，这些工具根据上下文和目标动态生成其操作。这种灵活性使它们很有用，但这也意味着它们的行为可能受到它们处理的内容的影响：文件、网页或用户输入。这有时被称为提示注入。例如，如果存储库的 README 包含不寻常的指令，Claude Code 可能会以操作员未预料到的方式将这些指令纳入其操作中。本指南涵盖了减少这种风险的实用方法。

并非每个部署都需要最大安全性。在笔记本电脑上运行 Claude Code 的开发人员的要求与在多租户环境中处理客户数据的公司不同。本指南提供了从 Claude Code 的内置安全功能到加固生产架构的各种选项，因此您可以选择适合您情况的选项。

## 威胁模型

代理可能由于提示注入（嵌入在它们处理的内容中的指令）或模型错误而采取意外操作。Claude 模型旨在抵抗这种情况；有关评估详情，请参阅[模型概览](https://platform.claude.com/docs/zh-CN/about-claude/models/overview)和您部署的模型的系统卡。

不过，纵深防御仍然是很好的做法。例如，如果代理处理一个恶意文件，该文件指示它将客户数据发送到外部服务器，网络控制可以完全阻止该请求。

## 内置安全功能

Claude Code 包含多个安全功能来解决常见问题。有关完整详情，请参阅[安全文档](https://code.claude.com/docs/zh-CN/security.md)。

* **权限系统**：每个工具和 bash 命令都可以配置为允许、阻止或提示用户批准。使用 glob 模式创建规则，如"允许所有 npm 命令"或"阻止任何带有 sudo 的命令"。组织可以设置适用于所有用户的策略。请参阅[权限](https://code.claude.com/docs/zh-CN/permissions.md)。
* **权限的命令解析**：在执行 bash 命令之前，Claude Code 将其解析为 AST 并将结果与您的权限规则进行匹配。无法干净解析或不符合允许规则的命令需要明确批准。一小组构造（如 `eval`）无论允许规则如何都始终需要批准。这是一个权限门，而不是沙箱；除了内置的安全检查（如对 `rm` 和 `rmdir` 的[关键路径检查](https://code.claude.com/docs/zh-CN/permission-modes.md#critical-paths)和[受保护路径](https://code.claude.com/docs/zh-CN/permission-modes.md#protected-paths)列表）外，它不会根据其目标路径或效果推断命令是否危险。
* **Web 搜索摘要**：搜索结果被摘要化，而不是将原始内容直接传递到上下文中，从而降低了来自恶意网络内容的提示注入风险。
* **Sandbox 模式**：Bash 命令可以在限制文件系统和网络访问的沙箱环境中运行。有关详情，请参阅[沙箱文档](https://code.claude.com/docs/zh-CN/sandboxing.md)。

## 安全原则

对于需要超越 Claude Code 默认值进行额外加固的部署，这些原则指导可用选项。

### 安全边界

安全边界将具有不同信任级别的组件分开。对于高安全部署，您可以将敏感资源（如凭证）放在包含代理的边界之外。如果代理环境中出现问题，该边界外的资源仍然受到保护。

例如，与其直接给予代理对 API 密钥的访问权限，您可以在代理环境外运行一个代理，将密钥注入到请求中。代理可以进行 API 调用，但它永远看不到凭证本身。这种模式对于多租户部署或处理不受信任的内容时很有用。

### 最小权限

在需要时，您可以将代理限制为仅执行其特定任务所需的功能：

| 资源 | 限制选项 |
| - | - |
| 文件系统 | 仅挂载所需目录，优先选择只读 |
| 网络 | 通过代理限制到特定端点 |
| 凭证 | 通过代理注入而不是直接公开 |
| 系统功能 | 在容器中删除 Linux 功能 |

### 纵深防御

对于高安全环境，分层多个控制提供额外保护。选项包括：

* 容器隔离
* 网络限制
* 文件系统控制
* 代理处的请求验证

正确的组合取决于您的威胁模型和操作要求。

## 隔离技术

不同的隔离技术在安全强度、性能和操作复杂性之间提供不同的权衡。

> [!NOTE]
>
> 在所有这些配置中，Claude Code（或您的 Agent SDK 应用程序）在隔离边界内运行（沙箱、容器或 VM）。下面描述的安全控制限制了代理可以从该边界内访问的内容。

| 技术 | 隔离强度 | 性能开销 | 复杂性 |
| - | - | - | - |
| Sandbox 运行时 | 良好（安全默认值） | 非常低 | 低 |
| 容器 (Docker) | 取决于设置 | 低 | 中等 |
| gVisor | 优秀（正确设置） | 中等/高 | 中等 |
| VM (Firecracker, QEMU) | 优秀（正确设置） | 高 | 中等/高 |

### Sandbox 运行时

对于无需容器的轻量级隔离，[sandbox-runtime](https://github.com/anthropics/sandbox-runtime) 在操作系统级别强制执行文件系统和网络限制。

主要优势是简单性：不需要 Docker 配置、容器镜像或网络设置。代理和文件系统限制是内置的。

**工作原理：**

* **文件系统**：使用操作系统原语（Linux 上的 `bubblewrap`、macOS 上的 `sandbox-exec`）来限制对配置路径的读/写访问
* **网络**：删除网络命名空间（Linux）或使用 Seatbelt 配置文件（macOS）通过内置代理路由网络流量
* **配置**：域和文件系统路径的基于 JSON 的允许列表

**设置：**

```bash theme={null}
npm install @anthropic-ai/sandbox-runtime
```

然后创建一个配置文件，指定允许的路径和域。

**安全考虑：**

1. **同主机内核**：与 VM 不同，沙箱进程共享主机内核。理论上，内核漏洞可能导致逃逸。对于某些威胁模型，这是可以接受的，但如果您需要内核级隔离，请使用 gVisor 或单独的 VM。

2. **无 TLS 检查**：代理基于客户端提供的主机名将域列入允许列表，不会终止或检查加密流量。在沙箱内运行的代码可能会使用[域前置](https://en.wikipedia.org/wiki/Domain_fronting)或类似技术来访问允许列表外的主机。如果您的威胁模型需要更强的保证，请配置[TLS 终止代理](#traffic-forwarding)。有关更多详情，请参阅[沙箱安全限制](https://code.claude.com/docs/zh-CN/sandboxing.md#security-limitations)。另外，如果代理对允许的域具有宽松凭证，请确保它无法使用该域来触发其他网络请求或泄露数据。

对于许多单开发人员和 CI/CD 用例，sandbox-runtime 以最少的设置显著提高了标准。下面的部分涵盖了需要更强隔离的部署的容器和 VM。

### 容器

容器通过 Linux 命名空间提供隔离。每个容器都有自己的文件系统、进程树和网络堆栈视图，同时共享主机内核。

安全加固的容器配置可能如下所示：

```bash theme={null}
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

以下是每个选项的作用：

| 选项 | 目的 |
| - | - |
| `--cap-drop ALL` | 删除 Linux 功能，如 `NET_ADMIN` 和 `SYS_ADMIN`，这些功能可能导致权限提升 |
| `--security-opt no-new-privileges` | 防止进程通过 setuid 二进制文件获得权限 |
| `--security-opt seccomp=...` | 限制可用的系统调用；Docker 的默认值阻止约 44 个，自定义配置文件可以阻止更多 |
| `--read-only` | 使容器的根文件系统不可变，防止代理持久化更改 |
| `--tmpfs /tmp:...` | 提供一个可写的临时目录，在容器停止时清除 |
| `--network none` | 删除所有网络接口；代理通过下面挂载的 Unix 套接字进行通信 |
| `--memory 2g` | 将内存使用限制为 2GB 以防止资源耗尽 |
| `--pids-limit 100` | 限制进程数以防止 fork 炸弹 |
| `--user 1000:1000` | 以非 root 用户身份运行 |
| `-v ...:/workspace:ro` | 以只读方式挂载代码，以便代理可以分析但不能修改。**避免挂载敏感的主机目录，如 `~/.ssh`、`~/.aws` 或 `~/.config`** |
| `-v .../proxy.sock:...` | 挂载连接到在容器外运行的代理的 Unix 套接字（见下文） |

**Unix 套接字架构：**

使用 `--network none`，容器根本没有网络接口。代理到达外部世界的唯一方式是通过挂载的 Unix 套接字，该套接字连接到在主机上运行的代理。此代理可以强制执行域允许列表、注入凭证并记录所有流量。

这与 [sandbox-runtime](https://github.com/anthropics/sandbox-runtime) 使用的架构相同。即使 Agent 通过提示词注入被破坏，它也无法将数据泄露到任意服务器。它只能通过代理进行通信，代理控制哪些域可以访问。有关更多详情，请参阅 [Claude Code 沙箱隔离博客文章](https://www.anthropic.com/engineering/claude-code-sandboxing)。

**额外加固选项：**

| 选项 | 目的 |
| - | - |
| `--userns-remap` | 将容器 root 映射到非特权主机用户；需要守护程序配置，但限制容器逃逸造成的损害 |
| `--ipc private` | 隔离进程间通信以防止跨容器攻击 |

### gVisor

标准容器共享主机内核：当容器内的代码进行系统调用时，它直接进入运行主机的同一内核。这意味着内核漏洞可能允许容器逃逸。gVisor 通过在用户空间中拦截系统调用来解决这个问题，然后才到达主机内核，实现自己的兼容性层来处理大多数系统调用，而无需涉及真实内核。

如果代理运行恶意代码（可能由于提示注入），该代码在容器中运行，可能会尝试内核漏洞利用。使用 gVisor，攻击面要小得多：恶意代码首先需要利用 gVisor 的用户空间实现，并且对真实内核的访问有限。

要将 gVisor 与 Docker 一起使用，请安装 `runsc` 运行时并配置守护程序：

```json /etc/docker/daemon.json theme={null}
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

然后使用以下命令运行容器：

```bash theme={null}
docker run --runtime=runsc agent-image
```

**性能考虑：**

| 工作负载 | 开销 |
| - | - |
| CPU 密集型计算 | \~0%（无系统调用拦截） |
| 简单系统调用 | \~2 倍慢 |
| 文件 I/O 密集型 | 对于繁重的打开/关闭模式，最多慢 10-200 倍 |

对于多租户环境或处理不受信任的内容时，额外的隔离通常值得开销。

### 虚拟机

VM 通过 CPU 虚拟化扩展提供硬件级隔离。每个 VM 运行自己的内核，创建强大的边界。客户内核中的漏洞不会直接危害主机。但是，VM 不一定比 gVisor 等替代方案"更安全"。VM 安全在很大程度上取决于虚拟机管理程序和设备仿真代码。

Firecracker 专为轻量级 microVM 隔离而设计。它可以在 125 毫秒内启动 VM，内存开销不到 5 MiB，去除不必要的设备仿真以减少攻击面。

使用这种方法，代理 VM 没有外部网络接口。相反，它通过 `vsock`（虚拟套接字）进行通信。所有流量通过 vsock 路由到主机上的代理，该代理强制执行允许列表并在转发请求之前注入凭证。

### 云部署

对于云部署，您可以将上述任何隔离技术与云原生网络控制相结合：

1. 在没有互联网网关的私有子网中运行代理容器
2. 配置云防火墙规则（AWS 安全组、GCP VPC 防火墙）以阻止除代理外的所有出站流量
3. 运行代理（如带有 `credential_injector` 过滤器的 [Envoy](https://www.envoyproxy.io/)），验证请求、强制执行域允许列表、注入凭证并转发到外部 API
4. 为代理的服务账户分配最小 IAM 权限，尽可能通过代理路由敏感访问
5. 在代理处记录所有流量以供审计

## 凭证管理

代理通常需要凭证来调用 API、访问存储库或与云服务交互。挑战是提供此访问权限而不公开凭证本身。

### 代理模式

推荐的方法是在代理的安全边界外运行一个代理，将凭证注入到传出请求中。代理发送没有凭证的请求，代理添加凭证，并将请求转发到其目的地。

这种模式有几个好处：

1. 代理永远看不到实际凭证
2. 代理可以强制执行允许的端点的允许列表
3. 代理可以记录所有请求以供审计
4. 凭证存储在一个安全位置，而不是分布到每个代理

### 配置 Claude Code 使用代理

Claude Code 支持两种方法来通过代理路由采样请求：

**选项 1：ANTHROPIC\_BASE\_URL（简单，但仅适用于采样 API 请求）**

```bash theme={null}
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

这告诉 Claude Code 和 Agent SDK 将采样请求发送到您的代理，而不是直接发送到 Claude API。您的代理接收纯文本 HTTP 请求，可以检查和修改它们（包括注入凭证），然后转发到真实 API。

**选项 2：HTTP\_PROXY / HTTPS\_PROXY（系统范围）**

```bash theme={null}
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code 和 Agent SDK 尊重这些标准环境变量，通过代理路由所有 HTTP 流量。对于 HTTPS，代理创建加密的 CONNECT 隧道：它无法在没有 TLS 拦截的情况下看到或修改请求内容。

### 实现代理

您可以构建自己的代理或使用现有的：

* [Envoy Proxy](https://www.envoyproxy.io/)：生产级代理，带有 `credential_injector` 过滤器用于添加身份验证标头
* [mitmproxy](https://mitmproxy.org/)：TLS 终止代理，用于检查和修改 HTTPS 流量
* [Squid](http://www.squid-cache.org/)：具有访问控制列表的缓存代理
* [LiteLLM](https://github.com/BerriAI/litellm)：具有凭证注入和速率限制的 LLM 网关

### 其他服务的凭证

除了从 Claude API 采样外，代理通常需要对其他服务的身份验证访问，如 git 存储库、数据库和内部 API。有两种主要方法：

#### 自定义工具

通过 MCP 服务器或自定义工具提供访问权限，该工具将请求路由到在代理的安全边界外运行的服务。代理调用工具，但实际的身份验证请求发生在外部。工具调用代理，代理注入凭证。

例如，git MCP 服务器可以接受来自代理的命令，但将它们转发到在主机上运行的 git 代理，该代理在联系远程存储库之前添加身份验证。代理永远看不到凭证。

优势：

* **无 TLS 拦截**：外部服务直接进行身份验证请求
* **凭证保持在外**：代理只看到工具接口，而不是底层凭证

#### 流量转发

对于 Claude API 调用，`ANTHROPIC_BASE_URL` 允许您将请求路由到可以以纯文本方式检查和修改它们的代理。但对于其他 HTTPS 服务（GitHub、npm 注册表、内部 API），流量通常是端到端加密的。即使您通过 `HTTP_PROXY` 通过代理路由它，代理也只看到不透明的 TLS 隧道，无法注入凭证。

要修改任意服务的 HTTPS 流量，而不使用自定义工具，您需要一个 TLS 终止代理，该代理解密流量、检查或修改它，然后在转发之前重新加密它。这需要：

1. 在代理的容器外运行代理
2. 在代理的信任存储中安装代理的 CA 证书（以便代理信任代理的证书）
3. 配置 `HTTP_PROXY`/`HTTPS_PROXY` 通过代理路由流量

这种方法处理任何基于 HTTP 的服务，而无需编写自定义工具，但增加了围绕证书管理的复杂性。

请注意，并非所有程序都遵循 `HTTP_PROXY`/`HTTPS_PROXY`。大多数工具（curl、pip、npm、git）都遵循，但有些可能绕过这些变量并直接连接。例如，Node.js `fetch()` 默认忽略这些变量；在 Node 24+ 中，您可以设置 `NODE_USE_ENV_PROXY=1` 来启用支持。为了覆盖忽略这些变量的工具，您可以使用 [proxychains](https://github.com/haad/proxychains) 来拦截网络调用，或配置 iptables 将出站流量重定向到透明代理。

> [!NOTE]
>
> **透明代理**在网络级别拦截流量，因此客户端不需要配置为使用它。常规代理要求客户端显式连接并使用 HTTP CONNECT 或 SOCKS。透明代理（如 Squid 或透明模式下的 mitmproxy）可以处理原始重定向的 TCP 连接。

两种方法仍然需要 TLS 终止代理和受信任的 CA 证书。它们只是确保流量实际到达代理。

## 文件系统配置

文件系统控制确定代理可以读取和写入的文件。

### 只读代码挂载

当代理需要分析代码但不修改它时，以只读方式挂载目录：

```bash theme={null}
docker run -v /path/to/code:/workspace:ro agent-image
```

> [!WARNING]
>
> 即使对代码目录的只读访问也可能暴露凭证。挂载前要排除或清理的常见文件：
>
> | 文件 | 风险 |
> | - | - |
> | `.env`, `.env.local` | API 密钥、数据库密码、机密 |
> | `~/.git-credentials` | Git 密码/令牌（纯文本） |
> | `~/.aws/credentials` | AWS 访问密钥 |
> | `~/.config/gcloud/application_default_credentials.json` | Google Cloud ADC 令牌 |
> | `~/.azure/` | Azure CLI 凭证 |
> | `~/.docker/config.json` | Docker 注册表身份验证令牌 |
> | `~/.kube/config` | Kubernetes 集群凭证 |
> | `.npmrc`, `.pypirc` | 包注册表令牌 |
> | `*-service-account.json` | GCP 服务账户密钥 |
> | `*.pem`, `*.key` | 私钥 |
>
> 考虑仅复制所需的源文件，或使用 `.dockerignore` 风格的过滤。

### 可写位置

如果代理需要写入文件，您有几个选项，具体取决于您是否希望更改持久化：

对于容器中的临时工作区，使用仅存在于内存中的 `tmpfs` 挂载，在容器停止时清除：

```bash theme={null}
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

如果您想在持久化更改之前查看更改，覆盖文件系统允许代理写入而不修改底层文件。更改存储在单独的层中，您可以检查、应用或丢弃。对于完全持久的输出，挂载专用卷，但将其与敏感目录分开。

## 进一步阅读

* [Claude Code 安全文档](https://code.claude.com/docs/zh-CN/security.md)
* [托管 Agent SDK](https://code.claude.com/docs/zh-CN/agent-sdk/hosting.md)
* [处理权限](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md)
* [沙箱运行时](https://github.com/anthropics/sandbox-runtime)
* [AI 代理的致命三角](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
* [OWASP 大型语言模型应用程序前 10 名](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
* [Docker 安全最佳实践](https://docs.docker.com/engine/security/)
* [gVisor 文档](https://gvisor.dev/docs/)
* [Firecracker 文档](https://firecracker-microvm.github.io/)
