> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 跟踪待办事项

> 在 Agent SDK 会话中跟踪待办事项，并从结构化工具调用中呈现 Claude 的进度

Claude Code 默认仅在[模型可用性](#model-availability)下列出的模型上提供[任务跟踪工具](https://code.claude.com/docs/zh-CN/tools-reference.md#task-tool-availability)。较新的模型无需书面待办事项列表即可跟踪多步骤工作，因此在这些模型上，您不需要本页面上的任何内容即可让 Claude 完成多步骤任务。

在具有任务跟踪工具的会话中，Claude 保持书面待办事项列表，在工作时更新每个项目的状态。您在消息流中看到每个更改作为结构化工具调用。仅当您的应用程序读取这些工具调用时才选择加入会话，无论是记录任务活动还是呈现自己的进度显示。

## 模型可用性

> [!NOTE]
>
> 以下工具仅在 Claude Opus 4.7、Claude Sonnet 4.6、Claude Haiku 4.5 及更早的模型上默认可用。在所有其他模型上，包括 Claude Code 无法识别的模型 ID，除非您选择加入，否则它们不可用：
>
> * `TodoWrite`
> * `TaskCreate`
> * `TaskGet`
> * `TaskUpdate`
> * `TaskList`
>
> 无论工具在何处可用，Claude Code 都提供四个 Task 工具，或者当你设置 `CLAUDE_CODE_ENABLE_TASKS=0` 时改为提供 `TodoWrite`。
>
> 此默认集合适用于 Claude Code v2.1.268 及更高版本，TypeScript Agent SDK 从 v0.3.268 开始捆绑此版本。

在默认情况下没有这些工具的模型上，除非您选择加入会话，否则您在消息流中看不到这些工具的 `tool_use` 块。Agent SDK 通过它捆绑的 Claude Code 二进制文件应用这些默认值。如果您将 `pathToClaudeCodeExecutable`（TypeScript）或 `cli_path`（Python）指向您自己的 Claude Code 安装，您将获得该安装提供的任何工具，在其自己的默认值下。要查看运行中会话中的确切集合，请[检查哪些工具可用](https://code.claude.com/docs/zh-CN/tools-reference.md#check-which-tools-are-available)。要选择加入会话，请执行以下操作之一：

* 在 [`allowedTools`](https://code.claude.com/docs/zh-CN/agent-sdk/permissions#allow-and-deny-rules)（TypeScript）或 `allowed_tools`（Python）选项中命名其中一个工具
* 在 `tools` 选项中列出工具，该选项将会话的内置工具限制为它命名的工具。将您想要的工具与您使用的其他内置工具一起包括
* 在 `env` 选项中设置 `CLAUDE_CODE_ENABLE_TODO_TOOLS=1`，如本页面上的示例所做的那样。在 TypeScript 中，`env` 替换子进程环境，因此展开 `...process.env` 以保持继承的变量。在 Python 中，`env` 合并在继承的环境之上

## 待办事项生命周期

Claude 将每个待办事项移动通过可预测的生命周期：

1. **创建**：当 Claude 识别任务时，将待办事项添加为 `pending`
2. **激活**：当 Claude 开始工作时，将待办事项设置为 `in_progress`
3. **完成**：当任务成功完成时，Claude 将其标记为已完成
4. **移除**：Claude 通过在 `TaskUpdate` 调用中设置 `status: "deleted"` 来删除不再需要的待办事项

## 何时 Claude 创建待办事项

在[具有任务跟踪工具的会话](#model-availability)中，Claude 为大多数多步骤工作创建待办事项，例如：

* **复杂的多步骤任务**需要三个或更多不同的操作
* **用户提供的任务列表**当提到多个项目时
* **较长的操作**受益于进度跟踪
* **明确的请求**当用户要求待办事项组织时

Claude 可能会跳过非常短或单步骤请求的待办事项。

## 示例

在运行这些示例之前，请按照[快速入门](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md)安装 Claude Agent SDK。本页面上的每个示例都共享相同的权限设置和退出行为：

* **权限模式**：示例提示要求 Claude 对项目进行真实工作，因此每个示例都设置 `permissionMode: "acceptEdits"`（TypeScript）或 `permission_mode="acceptEdits"`（Python）以自动批准工作产生的文件编辑。有关替代方案，请参阅[权限模式](https://code.claude.com/docs/zh-CN/agent-sdk/permissions.md#permission-modes)。
* **轮次限制**：每个示例运行直到代理完成并产生其最终结果消息。如果会话首先达到其轮次限制，该结果消息具有 `error_max_turns` 子类型。检查 `subtype` 以检测该结束。
* **错误处理**：这些示例使用单次 `query()` 调用。在产生 `error_max_turns` 结果后，`query()` 会抛出一个包含 `Reached maximum number of turns` 的错误。每个示例都将其循环包装在 try 块中，以便在发生这种情况时干净地退出。有关结果子类型，请参阅[处理结果](https://code.claude.com/docs/zh-CN/agent-sdk/agent-loop.md#handle-the-result)。

> [!NOTE]
>
> 任务系统消息，[`SDKTaskNotificationMessage`](https://code.claude.com/docs/zh-CN/agent-sdk/typescript#sdktasknotificationmessage)（TypeScript）或 [`TaskNotificationMessage`](https://code.claude.com/docs/zh-CN/agent-sdk/python#tasknotificationmessage)（Python）等，报告后台任务，例如后台命令和子代理。在消息流中，您看到待办事项活动作为助手消息中的 `tool_use` 块。

### 监控待办事项变化

以下示例监视助手流中的 `TaskCreate` 和 `TaskUpdate` `tool_use` 块，并为每个新任务的主题打印一条 `+` 行，为每个状态更改的任务 ID 和新状态打印一条更新行。当您想要任务活动的日志而不是呈现的显示时，请使用此形状。`+` 行不包括分配的 ID，因此此日志无法将更新与其创建相匹配。要保持该关联，请按照[实时显示进度](#display-progress-in-real-time)所做的那样捕获 ID。

流式传输的 `tool_use` 输入是模型发出的原始形状。Claude Code 在执行前修复一些接近但不正确的键名，将 `id` 或 `task_id` 映射到 `taskId`，将 `active_form` 映射到 `activeForm`，但该修复不会反映在流中。防御性地读取 `TaskUpdate` 输入字段，如本页面上的两个示例所做的那样，而不是假设规范名称始终存在。

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
    // Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
    options: { maxTurns: 15, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
  })) {
    if (message.type !== "assistant") continue;
    for (const block of message.message.content) {
      if (block.type !== "tool_use") continue;
      if (block.name === "TaskCreate") {
        const input = block.input as { subject: string };
        console.log(`+ ${input.subject}`);
      } else if (block.name === "TaskUpdate") {
        const input = block.input as {
          taskId?: string;
          id?: string;
          task_id?: string;
          status?: string;
        };
        const taskId = input.taskId ?? input.id ?? input.task_id;
        if (taskId && input.status) console.log(`  ${taskId} -> ${input.status}`);
      }
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result.
  console.log(`Session ended with an error: ${error}`);
}
```

```python Python theme={null}
import asyncio

from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock

async def main():
    try:
        async for message in query(
            prompt="Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
            # Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
            options=ClaudeAgentOptions(max_turns=15, permission_mode="acceptEdits", env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"}),
        ):
            if not isinstance(message, AssistantMessage):
                continue
            for block in message.content:
                if not isinstance(block, ToolUseBlock):
                    continue
                if block.name == "TaskCreate":
                    print(f"+ {block.input.get('subject', '')}")
                elif block.name == "TaskUpdate" and block.input.get("status"):
                    task_id = (
                        block.input.get("taskId")
                        or block.input.get("id")
                        or block.input.get("task_id")
                    )
                    if task_id:
                        print(f"  {task_id} -> {block.input['status']}")
    except Exception as error:
        # A single-shot query() raises after yielding an error result.
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

### 实时显示进度

以下示例监视助手流中的 `TaskCreate` 和 `TaskUpdate` `tool_use` 块，并在 `TaskTracker` 类中保持以任务 ID 为键的任务映射，在每次更改时重新呈现进度摘要。摘要计算已完成和进行中的任务，并显示每个活跃项目的 `activeForm` 标签代替其 `subject`。当您的应用程序维护进度显示而不是记录每个事件时，请使用此形状。

分配的任务 ID 不在 `TaskCreate` 输入中。Claude Code 在携带其 `tool_result` 块的用户消息上传递每个工具的结构化输出，在 `tool_use_result` 字段中。对于 `TaskCreate`，该对象在 TypeScript 中记录为[工具输出类型](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#tool-output-types)下的 `TaskCreateOutput`，在 Python 中该字段是相同形状的普通字典。跟踪器通过 `tool_use_id` 将每个 `tool_result` 块与其 `tool_use` 调用配对，并从配对消息的 `tool_use_result` 中读取 `task.id`。Claude 可以使用 `TaskList` 读取列表，使用 `TaskGet` 读取一个任务的完整详细信息。

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

type Task = { subject: string; activeForm?: string; status: string };

class TaskTracker {
  private tasks = new Map<string, Task>();
  private pendingCreates = new Map<string, { subject: string; activeForm?: string }>();

  displayProgress() {
    if (this.tasks.size === 0) {
      console.log("\nProgress: no open tasks\n");
      return;
    }

    const items = [...this.tasks.values()];
    const completed = items.filter((t) => t.status === "completed").length;
    const inProgress = items.filter((t) => t.status === "in_progress").length;

    console.log(`\nProgress: ${completed}/${this.tasks.size} completed`);
    console.log(`Currently working on: ${inProgress} task(s)\n`);

    for (const [id, task] of this.tasks) {
      const icon =
        task.status === "completed" ? "✅" : task.status === "in_progress" ? "🔧" : "❌";
      const text = task.status === "in_progress" && task.activeForm ? task.activeForm : task.subject;
      console.log(`${id}. ${icon} ${text}`);
    }
  }

  handleToolUse(block: { id: string; name: string; input: unknown }) {
    if (block.name === "TaskCreate") {
      const input = block.input as { subject: string; activeForm?: string; active_form?: string };
      this.pendingCreates.set(block.id, {
        subject: input.subject,
        activeForm: input.activeForm ?? input.active_form,
      });
    } else if (block.name === "TaskUpdate") {
      const input = block.input as {
        taskId?: string;
        id?: string;
        task_id?: string;
        status?: string;
        activeForm?: string;
        active_form?: string;
      };
      const taskId = input.taskId ?? input.id ?? input.task_id;
      if (!taskId) return;
      if (input.status === "deleted") {
        this.tasks.delete(taskId);
        this.displayProgress();
        return;
      }
      const task = this.tasks.get(taskId);
      if (!task) return;
      if (input.status) task.status = input.status;
      const active = input.activeForm ?? input.active_form;
      if (active) task.activeForm = active;
      this.displayProgress();
    }
  }

  handleToolResult(block: { tool_use_id: string; is_error?: boolean }, result: unknown) {
    const create = this.pendingCreates.get(block.tool_use_id);
    if (!create) return;
    this.pendingCreates.delete(block.tool_use_id);
    if (block.is_error) return;
    // The result's user message carries the tool's structured output as
    // tool_use_result; for TaskCreate that's TaskCreateOutput,
    // { task: { id, subject } }.
    const out = result as { task?: { id: string } };
    if (!out?.task?.id) return;
    this.tasks.set(out.task.id, { ...create, status: "pending" });
    this.displayProgress();
  }

  async trackQuery(prompt: string) {
    try {
      for await (const message of query({
        prompt,
        options: { maxTurns: 20, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
      })) {
        if (message.type === "assistant") {
          for (const block of message.message.content) {
            if (block.type === "tool_use") this.handleToolUse(block);
          }
        }
        if (message.type === "user" && Array.isArray(message.message.content)) {
          for (const block of message.message.content) {
            if (block.type === "tool_result") this.handleToolResult(block, message.tool_use_result);
          }
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result,
      // such as when the maxTurns limit is hit.
      console.log(`Session ended with an error: ${error}`);
    }
  }
}

// Usage
const tracker = new TaskTracker();
await tracker.trackQuery("Build a complete authentication system with todos");
```

```python Python theme={null}
import asyncio

from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    AssistantMessage,
    UserMessage,
    ToolUseBlock,
    ToolResultBlock,
)


class TaskTracker:
    def __init__(self):
        self.tasks: dict[str, dict] = {}
        self.pending_creates: dict[str, dict] = {}

    def display_progress(self):
        if not self.tasks:
            print("\nProgress: no open tasks\n")
            return

        completed = len([t for t in self.tasks.values() if t["status"] == "completed"])
        in_progress = len([t for t in self.tasks.values() if t["status"] == "in_progress"])

        print(f"\nProgress: {completed}/{len(self.tasks)} completed")
        print(f"Currently working on: {in_progress} task(s)\n")

        for task_id, task in self.tasks.items():
            icon = (
                "✅"
                if task["status"] == "completed"
                else "🔧"
                if task["status"] == "in_progress"
                else "❌"
            )
            text = (
                task["activeForm"]
                if task["status"] == "in_progress" and task.get("activeForm")
                else task["subject"]
            )
            print(f"{task_id}. {icon} {text}")

    def handle_tool_use(self, block: ToolUseBlock):
        if block.name == "TaskCreate":
            self.pending_creates[block.id] = {
                "subject": block.input.get("subject", ""),
                "activeForm": block.input.get("activeForm") or block.input.get("active_form"),
            }
        elif block.name == "TaskUpdate":
            task_id = (
                block.input.get("taskId")
                or block.input.get("id")
                or block.input.get("task_id")
            )
            if not task_id:
                return
            if block.input.get("status") == "deleted":
                self.tasks.pop(task_id, None)
                self.display_progress()
                return
            task = self.tasks.get(task_id)
            if not task:
                return
            if block.input.get("status"):
                task["status"] = block.input["status"]
            active = block.input.get("activeForm") or block.input.get("active_form")
            if active:
                task["activeForm"] = active
            self.display_progress()

    def handle_tool_result(self, block: ToolResultBlock, tool_use_result):
        create = self.pending_creates.pop(block.tool_use_id, None)
        if create is None or block.is_error:
            return
        # The result's user message carries the tool's structured output as
        # tool_use_result; for TaskCreate that's {"task": {"id": ..., "subject": ...}}.
        task = (tool_use_result or {}).get("task") or {}
        if not task.get("id"):
            return
        self.tasks[task["id"]] = {**create, "status": "pending"}
        self.display_progress()

    async def track_query(self, prompt: str):
        try:
            async for message in query(
                prompt=prompt,
                options=ClaudeAgentOptions(
                    max_turns=20,
                    permission_mode="acceptEdits",
                    env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"},
                ),
            ):
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, ToolUseBlock):
                            self.handle_tool_use(block)
                if isinstance(message, UserMessage) and isinstance(message.content, list):
                    for block in message.content:
                        if isinstance(block, ToolResultBlock):
                            self.handle_tool_result(block, message.tool_use_result)
        except Exception as error:
            # A single-shot query() raises after yielding an error result,
            # such as when the max_turns limit is hit.
            print(f"Session ended with an error: {error}")


# Usage
async def main():
    tracker = TaskTracker()
    await tracker.track_query("Build a complete authentication system with todos")


asyncio.run(main())
```

## 相关文档

* [Agent SDK 参考 - TypeScript](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md)：TypeScript SDK 的选项、类型和工具架构，包括 Task 工具输入和输出类型
* [Agent SDK 参考 - Python](https://code.claude.com/docs/zh-CN/agent-sdk/python.md)：Python SDK 的选项、类型和工具文档
* [流式输入](https://code.claude.com/docs/zh-CN/agent-sdk/streaming-vs-single-mode.md)：两种输入模式，以及何时使用流式输入而不是这些示例使用的单次调用
* [为 Claude 提供自定义工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)：使用 SDK 的进程内 MCP 服务器定义您自己的工具
