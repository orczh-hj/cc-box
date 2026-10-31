> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 从代理获取结构化输出

> 使用 JSON Schema、Zod 或 Pydantic 从代理工作流返回验证的 JSON。在多轮工具使用后获取类型安全的结构化数据。

结构化输出让你定义从代理返回的数据的确切形状。代理可以使用任何需要的工具来完成任务，最后你仍然会获得与你的 schema 匹配的验证 JSON。定义一个 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 来描述你需要的结构，SDK 会根据它验证输出，在不匹配时重新提示。如果验证在重试限制内没有成功，结果将是一个错误而不是结构化数据；请参阅 [错误处理](#error-handling)。

为了获得完整的类型安全，使用 [Zod](#type-safe-schemas-with-zod-and-pydantic)（TypeScript）或 [Pydantic](#type-safe-schemas-with-zod-and-pydantic)（Python）来定义您的 schema 并获取强类型对象。

## 为什么使用结构化输出？

代理默认返回自由格式的文本，这适用于聊天但不适用于需要以编程方式使用输出的情况。结构化输出为你提供类型化数据，你可以直接传递给应用逻辑、数据库或 UI 组件。

考虑一个食谱应用，其中代理搜索网络并返回食谱。没有结构化输出，你会得到需要自己解析的自由格式文本。有了结构化输出，你定义你想要的形状并获得可以直接在应用中使用的类型化数据。

> [accordion] 没有结构化输出
>
> ```text theme={null}
> 这是一个经典的巧克力芯片饼干食谱！
>
> **巧克力芯片饼干**
> 准备时间：15 分钟 | 烹饪时间：10 分钟
>
> 材料：
> - 2 1/4 杯通用面粉
> - 1 杯黄油，软化
> ...
> ```
>
> 要在你的应用中使用这个，你需要解析出标题，将"15 分钟"转换为数字，将材料与说明分开，并处理响应中的不一致格式。

> [accordion] 有结构化输出
>
> ```jsonc theme={null}
> {
>   "name": "Chocolate Chip Cookies",
>   "prep_time_minutes": 15,
>   "cook_time_minutes": 10,
>   "ingredients": [
>     { "item": "all-purpose flour", "amount": 2.25, "unit": "cups" },
>     { "item": "butter, softened", "amount": 1, "unit": "cup" }
>     // ...
>   ],
>   "steps": ["Preheat oven to 375°F", "Cream butter and sugar" /* ... */]
> }
> ```
>
> 你可以直接在 UI 中使用的类型化数据。

## 快速开始

要使用结构化输出，定义一个 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 来描述你想要的数据形状，然后通过 `outputFormat` 选项（TypeScript）或 `output_format` 选项（Python）将其传递给 `query()`。当代理完成时，结果消息包含一个 `structured_output` 字段，其中包含与你的 schema 匹配的验证数据。

在运行本页上的示例之前，请按照[快速入门](https://code.claude.com/docs/zh-CN/agent-sdk/quickstart.md#setup)安装 Claude Agent SDK。下面的示例要求 Agent 研究 Anthropic 并返回公司名称、成立年份和总部作为结构化输出。

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// 定义你想要返回的数据形状
const schema = {
  type: "object",
  properties: {
    company_name: { type: "string" },
    founded_year: { type: "number" },
    headquarters: { type: "string" }
  },
  required: ["company_name"]
};

try {
  for await (const message of query({
    prompt: "Research Anthropic and provide key company information",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // 结果消息包含带有验证数据的 structured_output
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
} catch (error) {
  // 单次 query() 在产生错误结果后抛出异常，例如
  // error_max_structured_output_retries；请参阅错误处理部分。
  console.error(`Session ended with an error: ${error}`);
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

# 定义你想要返回的数据形状
schema = {
    "type": "object",
    "properties": {
        "company_name": {"type": "string"},
        "founded_year": {"type": "number"},
        "headquarters": {"type": "string"},
    },
    "required": ["company_name"],
}


async def main():
    try:
        async for message in query(
            prompt="Research Anthropic and provide key company information",
            options=ClaudeAgentOptions(
                output_format={"type": "json_schema", "schema": schema}
            ),
        ):
            # 结果消息包含带有验证数据的 structured_output
            if isinstance(message, ResultMessage) and message.structured_output:
                print(message.structured_output)
                # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}
    except Exception as error:
        # 单次 query() 在产生错误结果后抛出异常，例如
        # error_max_structured_output_retries；请参阅错误处理部分。
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

## 使用 Zod 和 Pydantic 的类型安全 schema

与其手动编写 JSON Schema，你可以使用 [Zod](https://zod.dev/)（TypeScript）或 [Pydantic](https://docs.pydantic.dev/latest/)（Python）来定义你的 schema。这些库为你生成 JSON Schema，并让你将响应解析为完全类型化的对象，你可以在整个代码库中使用，具有自动完成和类型检查。

下面的示例定义了一个功能实现计划的 schema，包括摘要、步骤列表（每个步骤都有复杂度级别）和潜在风险。代理规划功能并返回一个类型化的 `FeaturePlan` 对象。然后你可以访问 `plan.summary` 等属性，并使用完整的类型安全遍历 `plan.steps`。

SDK 使用 JSON Schema draft-07 验证 schema，因此声明更新版本的 schema 会被拒绝。Zod 默认针对 draft 2020-12，所以在转换你的 schema 时传递 `target: "draft-7"`。

```typescript TypeScript theme={null}
import { z } from "zod";
import { query } from "@anthropic-ai/claude-agent-sdk";

// 使用 Zod 定义 schema
const FeaturePlan = z.object({
  feature_name: z.string(),
  summary: z.string(),
  steps: z.array(
    z.object({
      step_number: z.number(),
      description: z.string(),
      estimated_complexity: z.enum(["low", "medium", "high"])
    })
  ),
  risks: z.array(z.string())
});

type FeaturePlan = z.infer<typeof FeaturePlan>;

// 使用 SDK 期望的 draft-07 目标转换为 JSON Schema
const schema = z.toJSONSchema(FeaturePlan, { target: "draft-7" });

// 在查询中使用
try {
  for await (const message of query({
    prompt:
      "Plan how to add dark mode support to a React app. Break it into implementation steps.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // 验证并获取完全类型化的结果
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`Feature: ${plan.feature_name}`);
        console.log(`Summary: ${plan.summary}`);
        plan.steps.forEach((step) => {
          console.log(`${step.step_number}. [${step.estimated_complexity}] ${step.description}`);
        });
      }
    }
  }
} catch (error) {
  // 单次 query() 在产生错误结果后抛出，例如
  // error_max_structured_output_retries；请参阅错误处理部分。
  console.error(`Session ended with an error: ${error}`);
}
```

```python Python theme={null}
import asyncio
from pydantic import BaseModel
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


class Step(BaseModel):
    step_number: int
    description: str
    estimated_complexity: str  # 'low', 'medium', 'high'


class FeaturePlan(BaseModel):
    feature_name: str
    summary: str
    steps: list[Step]
    risks: list[str]


async def main():
    try:
        async for message in query(
            prompt="Plan how to add dark mode support to a React app. Break it into implementation steps.",
            options=ClaudeAgentOptions(
                output_format={
                    "type": "json_schema",
                    "schema": FeaturePlan.model_json_schema(),
                }
            ),
        ):
            if isinstance(message, ResultMessage) and message.structured_output:
                # 验证并获取完全类型化的结果
                plan = FeaturePlan.model_validate(message.structured_output)
                print(f"Feature: {plan.feature_name}")
                print(f"Summary: {plan.summary}")
                for step in plan.steps:
                    print(
                        f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                    )
    except Exception as error:
        # 单次 query() 在产生错误结果后抛出，例如
        # error_max_structured_output_retries；请参阅错误处理部分。
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

## 输出格式配置

`outputFormat`（TypeScript）或 `output_format`（Python）选项接受一个对象，包含：

* `type`：设置为 `"json_schema"` 以获得结构化输出
* `schema`：一个 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 对象，定义你的输出结构。你可以使用 `z.toJSONSchema(schema, { target: "draft-7" })` 从 Zod schema 生成它，或使用 `.model_json_schema()` 从 Pydantic 模型生成它

SDK 支持标准 JSON Schema 功能，包括所有基本类型（object、array、string、number、boolean、null）、`enum`、`const`、`required`、嵌套对象和 `$ref` 定义。有关支持的功能和限制的完整列表，请参阅 [JSON Schema 限制](https://platform.claude.com/docs/zh-CN/build-with-claude/structured-outputs#json-schema-limitations)。

不是有效 JSON Schema 的 schema 在启动时会导致运行失败，并显示一条错误消息，说明问题所在。在 v2.1.205 之前，无效的 schema 会被静默忽略，代理会返回非结构化文本。

`format` 关键字，例如 `"format": "email"`，被接受为注释，SDK 的验证器不会强制执行它。在 v2.1.205 之前，任何包含 `format` 的 schema 都被视为无效。

## 示例：TODO 跟踪代理

此示例演示了结构化输出如何与多步工具使用配合工作。代理需要在代码库中查找 TODO 注释，然后为每个注释查找 git blame 信息。它自主决定使用哪些工具（Grep 搜索、Bash 运行 git 命令）并将结果合并为单个结构化响应。

schema 包括可选字段（`author` 和 `date`），因为 git blame 信息可能不适用于所有文件。代理填充它能找到的内容并省略其余部分。

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// 定义 TODO 提取的结构
const todoSchema = {
  type: "object",
  properties: {
    todos: {
      type: "array",
      items: {
        type: "object",
        properties: {
          text: { type: "string" },
          file: { type: "string" },
          line: { type: "number" },
          author: { type: "string" },
          date: { type: "string" }
        },
        required: ["text", "file", "line"]
      }
    },
    total_count: { type: "number" }
  },
  required: ["todos", "total_count"]
};

// 代理使用 Grep 查找 TODO，使用 Bash 获取 git blame 信息
try {
  for await (const message of query({
    prompt: "Find all TODO comments in this codebase and identify who added them",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`Found ${data.total_count} TODOs`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  Added by ${todo.author} on ${todo.date}`);
        }
      });
    }
  }
} catch (error) {
  // 单次 query() 在产生错误结果后抛出异常，例如
  // error_max_structured_output_retries；请参阅错误处理部分。
  console.error(`Session ended with an error: ${error}`);
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

# 定义 TODO 提取的结构
todo_schema = {
    "type": "object",
    "properties": {
        "todos": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "text": {"type": "string"},
                    "file": {"type": "string"},
                    "line": {"type": "number"},
                    "author": {"type": "string"},
                    "date": {"type": "string"},
                },
                "required": ["text", "file", "line"],
            },
        },
        "total_count": {"type": "number"},
    },
    "required": ["todos", "total_count"],
}


async def main():
    # 代理使用 Grep 查找 TODO，使用 Bash 获取 git blame 信息
    try:
        async for message in query(
            prompt="Find all TODO comments in this codebase and identify who added them",
            options=ClaudeAgentOptions(
                output_format={"type": "json_schema", "schema": todo_schema}
            ),
        ):
            if isinstance(message, ResultMessage) and message.structured_output:
                data = message.structured_output
                print(f"Found {data['total_count']} TODOs")
                for todo in data["todos"]:
                    print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                    if "author" in todo:
                        print(f"  Added by {todo['author']} on {todo['date']}")
    except Exception as error:
        # 单次 query() 在产生错误结果后抛出异常，例如
        # error_max_structured_output_retries；请参阅错误处理部分。
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

## 识别 StructuredOutput 工具调用

当您在 TypeScript 中设置 `outputFormat` 或在 Python 中设置 `output_format` 时，消息流中会出现一个名为 `StructuredOutput` 的工具调用，而您从未定义过该工具。Claude Code 会将此工具添加到会话中，Agent 通过调用它来返回答案。[系统 init 消息](https://code.claude.com/docs/zh-CN/agent-sdk/typescript.md#sdksystemmessage)会在 `tools` 数组中列出该工具，每次调用都以两个块的形式出现：

* 一个名为 `StructuredOutput` 的 `tool_use` 块，其输入是 Agent 根据您的 schema 生成的对象
* 一个内容为 `Structured output provided successfully` 的 `tool_result` 块

当该对象与您的 schema 不匹配时，`tool_result` 会改为带有 `is_error: true`，Agent 会再次调用 `StructuredOutput`，因此一次查询的消息流中可能包含多次调用。请从结果消息的 `structured_output` 中读取经过验证的数据，而不是从 `tool_use` 块中读取；关于所有尝试均未成功时得到的结果，请参阅[错误处理](#error-handling)。

如果您的应用程序会显示工具调用或将其记录到日志中，请跳过名为 `StructuredOutput` 的调用，以免这一机制显示为 Agent 自行选择的步骤。`tool_result` 块不包含工具名称，只包含其所响应调用的 `tool_use_id`，因此请将其与您跳过的每个调用的 `id` 进行匹配。

## 错误处理

结构化输出生成可能会失败，当 Agent 无法生成与您的 schema 匹配的有效 JSON 时。这通常发生在 schema 对于任务来说太复杂、任务本身不明确或 Agent 在尝试修复验证错误时达到重试限制时。它也可能在没有任何验证失败的情况下发生：[模型回退](https://code.claude.com/docs/zh-CN/model-config.md#automatic-model-fallback)可以在流中途收回已完成的输出，如果没有重试替换它，运行将以相同的错误结束。在调试您的 schema 之前，检查错误结果消息上的 `errors` 列表以区分这两个原因。

发生错误时，结果消息有一个 `subtype` 指示出了什么问题：

| Subtype | 含义 |
| - | - |
| `success` | 输出已成功生成并验证 |
| `error_max_structured_output_retries` | 多次尝试后没有有效输出存活（验证失败，或模型回退收回且没有成功重试） |

结果也可以以 `success` 的 subtype 结束，但没有 `structured_output` 值，例如当运行完成而代理没有生成结构化输出时。将该情况也视为失败。故障排除条目[structured\_output 为 None 但结果显示 success](https://code.claude.com/docs/zh-CN/agent-sdk/troubleshooting.md#structured_output-is-none-but-the-result-says-success)涵盖了这种情况。下面的示例仅当 `subtype` 为 `success` 且 `structured_output` 存在时才将结果视为成功，并将所有其他结果作为失败处理：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const contactSchema = {
  type: "object",
  properties: {
    name: { type: "string" },
    email: { type: "string" }
  },
  required: ["name"]
};

try {
  for await (const msg of query({
    prompt: "Extract contact info from the document",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: contactSchema
      }
    }
  })) {
    if (msg.type === "result") {
      if (msg.subtype === "success" && msg.structured_output) {
        // 使用验证的输出
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        console.error("Could not produce valid output");
      } else {
        console.error("Run ended without a structured output");
      }
    }
  }
} catch (error) {
  // 单次 query() 在产生错误结果后抛出。如果失败是错误结果，
  // 上面的错误 subtype 分支已经运行；连接或进程失败不会产生结果消息。
  console.log(`Session ended with an error: ${error}`);
}
```

```python Python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

contact_schema = {
    "type": "object",
    "properties": {
        "name": {"type": "string"},
        "email": {"type": "string"},
    },
    "required": ["name"],
}


async def main():
    try:
        async for message in query(
            prompt="Extract contact info from the document",
            options=ClaudeAgentOptions(
                output_format={"type": "json_schema", "schema": contact_schema}
            ),
        ):
            if isinstance(message, ResultMessage):
                if message.subtype == "success" and message.structured_output:
                    # 使用验证的输出
                    print(message.structured_output)
                elif message.subtype == "error_max_structured_output_retries":
                    print("Could not produce valid output")
                else:
                    print("Run ended without a structured output")
    except Exception as error:
        # 单次 query() 在产生错误结果后抛出。如果失败是错误结果，
        # 上面的错误 subtype 分支已经运行；连接或进程失败不会产生结果消息。
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

**避免错误的提示：**

* **保持 schema 专注。** 具有许多必需字段的深层嵌套 schema 更难满足。从简单开始，根据需要添加复杂性。
* **匹配 schema 到任务。** 如果任务可能没有你的 schema 要求的所有信息，请将这些字段设为可选。
* **使用清晰的提示。** 模糊的提示使代理更难知道要生成什么输出。

## 相关资源

* [JSON Schema 文档](https://json-schema.org/)：学习 JSON Schema 语法以定义具有嵌套对象、数组、枚举和验证约束的复杂 schema
* [API 结构化输出](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)：直接使用 Claude API 的结构化输出进行单轮请求，无需工具使用
* [自定义工具](https://code.claude.com/docs/zh-CN/agent-sdk/custom-tools.md)：在返回结构化输出之前，在执行期间给你的代理自定义工具来调用
