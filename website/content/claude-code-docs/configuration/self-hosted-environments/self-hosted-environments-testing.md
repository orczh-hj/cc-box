> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 端到端测试自托管环境

> 从 CI 验证自托管运行器镜像：使用 CLI 分派会话，通过 Stop hook 读取 Claude 的回复，并编写完整循环脚本。

> [!NOTE]
>
> 自托管环境在 Team 和 Enterprise 计划上处于公开测试阶段；[可用性和限制](https://code.claude.com/docs/zh-CN/self-hosted-environments#availability-and-limitations)涵盖启用路径。本页面是 CI 测试方案；有关设置，请参阅[快速入门](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md)，有关群组方案，请参阅[部署到生产环境](https://code.claude.com/docs/zh-CN/self-hosted-environments-deploy.md)。

在[自托管环境](https://code.claude.com/docs/zh-CN/self-hosted-environments)中，Claude Code [云会话](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md)在您构建和维护的运行器镜像上运行。在将新镜像推送到生产环境之前，从脚本对测试环境驱动完整会话：创建会话、读取 Claude 的回复、发送后续问题，并读取该回复。这是 CI 烟雾测试的形式，用于验证您的运行器镜像、git 访问和任何自定义工具，然后再推广更改。

此方案假设您已经[设置了环境和运行器](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-an-environment-and-runner)，并且您的 CI 作业在与测试脚本相同的主机上启动运行器进程，这是测试新运行器镜像的自然设置。您在运行器上安装的 Stop hook 将每个回合的最终回复写入本地文件，脚本从那里读取它，因此对 Anthropic API 的唯一调用是两个分派本身。如果您的测试运行器在单独的基础设施上，请参阅[远程测试运行器](#remote-test-runners)。

## 在测试运行器上安装捕获 hook

读回通过 Claude Code [Stop hook](https://code.claude.com/docs/zh-CN/hooks.md#stop)工作：当 Claude 完成一个回合时，hook 在其 stdin JSON 中接收最终助手消息作为 `last_assistant_message`，并将其附加到 `$E2E_REPLY_DIR/<session_id>.txt`。以与[commit-nudge Stop hook](https://code.claude.com/docs/zh-CN/self-hosted-environments-configuration.md#prompt-sessions-to-push-their-work)相同的方式安装它，在运行器主机的 `~/.claude/` 上，运行器将其种子化到每个会话中。

### 保存 hook 文件

在运行器主机上保存以下两个文件：

* 设置块：合并到运行器主机上的 `~/.claude/settings.json`
* 脚本：保存为运行器主机上的 `~/.claude/hooks/e2e-stop-hook-capture.sh` 并使其可执行

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook for testing a self-hosted environment end to end: writes each
# turn's final assistant reply to $E2E_REPLY_DIR/<session_id>.txt so a
# co-located test driver can read it without calling the Anthropic API.
# Install on the TEST runner only. Requires jq.

# No-op unless the driver is listening. Never fail the turn.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID is exported in cse_... form; the session
# id the dispatch CLI prints is in session_... form. Same id, different
# prefix.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message is absent when the final assistant turn had no
# text, such as a tool-use-only turn. The `// empty` filter makes that a
# zero-byte write rather than the literal string "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

### 启动运行器之前

hook 有以下要求：

* 在启动运行器之前安装它。运行器在启动时对 `~/.claude/` 进行快照，因此添加到运行中的运行器的 hook 仅在重新启动后才生效。
* 将 `E2E_REPLY_DIR` 导出到运行器进程。当变量未设置或目录不存在时，hook 是无操作的，因此在启动运行器的任何地方设置它，例如 systemd 单元、pod 规范或 CI 步骤。下面的测试脚本也需要它。

仅在为测试环境提供服务的运行器上安装此 hook。每当 `E2E_REPLY_DIR` 存在时，它会将每个会话的最终回复写入磁盘，这在一次性 CI 运行器上是无害的，但不应该进入生产环境运行器镜像，其中变量可能会被意外设置。

## 运行测试循环

`--environment` 和 `--ref` 分派标志需要在运行脚本的机器上使用 Claude Code v2.1.224 或更高版本，这与运行器本身的下限相同。安装 hook 并在此主机上启动运行器后，测试脚本：

1. 使用 `claude -p "<prompt>" --environment <environment-id> --output-format json` 在测试环境上创建会话。请从 git 检出中运行该命令，以便 CLI 可以从 `origin` 远程自动检测仓库。可选的 `--ref <branch>` 将会话的检出基于命名的 ref 而不是本地 HEAD。该命令退出时不会等待 Claude 的回复。其输出会告知脚本结果：
   * **会话已创建**：一行 JSON，例如 `{"ok":true,"session_id":"session_...","title":"...","url":"...","pool_id":"..."}`
   * **会话创建失败**：输出 `{"ok":false,"error":"..."}` 这一行，并且命令以状态 1 退出
   * **某些较早发生的错误**，例如您的组织无法使用云端会话或缺少提示词：错误输出到 stderr，不输出 JSON 行，并且命令以状态 1 退出
2. 等待回复出现在 `$E2E_REPLY_DIR/<session_id>.txt` 中，由运行器上的 Stop hook 在回合完成后写入。
3. 使用 `claude -p "<message>" --cloud <session_id> --output-format json` 发送后续消息（请参阅[向运行中的会话发送后续消息](https://code.claude.com/docs/zh-CN/claude-code-on-the-web.md#send-follow-ups-from-the-cli)），它将用户事件发布到现有会话并退出。
4. 以与步骤 2 相同的方式等待后续回复。

### `--environment` 分派行为

Claude Code 创建会话，打印会话 ID 和指向它的链接，然后退出。

该标志优先于 [`remote.defaultEnvironmentId`](https://code.claude.com/docs/zh-CN/settings-reference#remote-defaultenvironmentid) 设置。它不支持 `--output-format stream-json`，不能与恢复、附加到或预配置会话的标志组合，例如 `--resume`、`--continue`、`--teleport`、`--session-id` 或 `--init-only`。`--cloud` 在使用会话 ID 或 URL 时被拒绝，在非交互式运行中当它带有描述时也被拒绝。裸 `--cloud` 被视为不存在。从终端，您可以将任务作为 `--cloud` 描述而不是位置提示传递。

## 示例脚本

示例脚本与测试运行器在同一台机器上运行。运行之前，请先准备好该机器：

* **仓库检出**：从您希望会话在其中工作的仓库的 git 检出运行该脚本。
* **运行器**：在此主机上启动运行器，安装捕获 hook 并导出 `E2E_REPLY_DIR`。
* **登录**：按照[从 CI 进行身份验证](#authenticate-from-ci)中的说明，在运行该脚本的机器上使用 claude.ai 账户登录。
* **环境 ID**：将 `CLAUDE_TEST_ENVIRONMENT_ID` 设置为您的测试环境的 `ccpool_...` ID，该 ID 显示在管理页面上的环境详细信息对话框中，或由[创建环境调用](#create-a-dedicated-test-environment)返回。

下面的脚本针对 `$CLAUDE_TEST_ENVIRONMENT_ID` 运行完整循环，并对每个回复中的哨兵短语进行断言。

```bash theme={null}
#!/usr/bin/env bash
# End-to-end test against a self-hosted environment, using Stop-hook read-back.
# Prereqs: `claude auth login` has been run on this machine (see "Authenticate
# from CI" below); jq is installed; CLAUDE_TEST_ENVIRONMENT_ID names an
# environment whose runner is the one on this host, with the capture hook
# installed and E2E_REPLY_DIR in its environment.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID is the legacy spelling
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Waits until $E2E_REPLY_DIR/<session_id>.txt contains $2, or fails after
# 90 seconds. Tune the timeout to your environment's cold-start time. The
# file is written by the Stop hook on the runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Create the session on the test environment. Run from a git checkout
# so the CLI can auto-detect the repo. --ref pins the checkout to a named
# ref regardless of local HEAD.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json < /dev/null)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Wait for the turn-1 reply.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Post a follow-up via the CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json < /dev/null)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Wait for the turn-2 reply.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

将 `TURN1`/`TURN2` 提示和 `EXPECT1`/`EXPECT2` 哨兵替换为任何练习您的设置的内容，例如要求 Claude 运行您的一个自定义 MCP 工具并对其输出进行断言。

## 远程测试运行器

如果您的测试运行器在单独的基础设施上，例如您的 CI 作业无法共享文件系统的持久 Kubernetes 集群，请将 Stop hook 中的文件写入交换为 POST 到您的驱动程序侦听的端点：

```sh theme={null}
#!/bin/sh
# Variant of the capture hook for runners on separate infrastructure.
# Set E2E_REPLY_URL on the runner to an endpoint the driver controls.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

在驱动程序端，运行任何接受 POST 并保持回复直到测试要求它的内容，例如 CI 作业内的小型 HTTP 侦听器或您已经运行的 webhook 接收器。hook 在您的基础设施上运行，因此端点只需要从您的运行器可达。

## 从 CI 进行身份验证

`claude -p ... --environment` 和 `claude -p ... --cloud` 都使用 claude.ai OAuth 令牌进行身份验证；API 密钥（例如 `sk-ant-xxxxx`）对于任何一个调用都不被接受。两种方法使令牌在 CI 中可用。

### 长期 CI 主机

在执行脚本的机器上使用专用自动化用户帐户交互式运行一次 `claude auth login`。Claude Code 在 macOS 上将令牌存储在 OS 密钥链中，或在 Linux 和 Windows 上存储在 `~/.claude/.credentials.json` 中。在 macOS 主机上，其密钥链无法写入（如 SSH 会话中的典型情况，其中登录密钥链保持锁定），Claude Code 也将令牌存储在 `~/.claude/.credentials.json` 中。请参阅[凭证管理](https://code.claude.com/docs/zh-CN/authentication.md#credential-management)。

CLI 在每次调用时自动刷新短期访问令牌，但基础刷新令牌授予从初始登录起限制为 30 天，因此每 30 天在该主机上交互式重新运行一次 `claude auth login`。

### 临时 CI 运行器

目前没有针对此的长期 CI 令牌。授予云会话控制的范围 `user:sessions:claude_code` 在服务器端限制为 30 天，因此 `claude setup-token`（它铸造一年推理令牌）不涵盖它。[环境秘密](https://code.claude.com/docs/zh-CN/self-hosted-environments-quickstart.md#set-up-an-environment-and-runner)也不被接受，因为它仅授权运行器向环境注册，而不是创建会话。

要在临时运行器上配置存储的登录，请设置 [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` 和 `CLAUDE_CODE_OAUTH_SCOPES`](https://code.claude.com/docs/zh-CN/env-vars#variables)，以便 `claude auth login` 交换令牌而不需要浏览器；相同的 30 天上限适用于刷新授予。如果您需要不受人类帐户约束的机器身份路径，请联系您的 Anthropic 帐户团队。

## 创建专用测试环境

以编程方式创建和删除环境，以便每个 CI 运行都获得一个干净的环境；您的 CI 作业启动的运行器注册到新环境中。下面的创建和删除调用是 claude.ai 上的**云环境**管理页面使用的相同端点，它们需要 `anthropic-beta: ccr-byoc-2025-07-29` 标头。

### 铸造管理令牌

`$ADMIN_TOKEN` 是持有 Owner 角色的帐户的 claude.ai OAuth 访问令牌，以与[从 CI 进行身份验证](#authenticate-from-ci)相同的方式铸造：

* **铸造它**：使用持有 Owner 角色的帐户运行 `claude auth login`，然后从[长期 CI 主机](#long-lived-ci-host)说 Claude Code 存储它的任何地方读取当前访问令牌。
* **每次运行时读取新鲜的**：CLI 轮换访问令牌，相同的 30 天刷新授予上限适用，因此不要存储副本。
* **通过 stdin 传递它**：如示例所示，以便令牌永远不会进入 curl 的参数列表或您的构建日志。

### 创建环境

捕获响应而不回显它：`pool_secret` 是一个长期凭证，可以将运行器注册到环境中，因此将其存储为掩蔽 CI 秘密并仅打印环境 ID。保持令牌不在进程列表中的 `-H @-` 形式需要 curl 7.55 或更高版本；较旧的 curl 将 `@-` 视为文字标头并在没有授权的情况下发送请求。

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

在[所有者为组织启用**允许自托管环境**](https://code.claude.com/docs/zh-CN/self-hosted-environments#availability-and-limitations)之前，调用失败，出现 `403` `permission_error`，读取 `self-hosted runners are disabled by your organization's policy`。

在此主机上启动运行器，使用 `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`，加上捕获 hook 和 `E2E_REPLY_DIR`，根据[在测试运行器上安装捕获 hook](#install-the-capture-hook-on-your-test-runner)，然后运行测试脚本。

### 删除环境

运行完成后删除环境，以便每个 CI 运行都从干净状态开始：

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
