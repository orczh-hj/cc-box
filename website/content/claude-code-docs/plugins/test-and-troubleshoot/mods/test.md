> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 测试 mod

> 为 Claude Code mod 编写自动化测试，该测试可以触发事件、存根 Claude Code 的答案、按下按钮，无需会话、登录或网络。

您可以为 mod 编写自动化测试，并使用 [`claude plugin test`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#commands) 从您的 shell 运行它们。测试会触发您的 hook 处理的事件并检查 hook 做了什么，这样您可以在问题到达会话之前捕获它。第一个示例测试来自 [创建 mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md) 的 mod。

## 编写测试

测试加载您的 mod，通过其 hooks 发送事件，就像 Claude Code 会做的那样，并检查 hooks 做了什么，无需会话、登录或网络。您可以使用 `claude plugin test` 从 shell 运行测试，每个测试文件都导入测试工具包，这是 `claude-code/testing` 模块中的测试库。

给每个测试文件一个以 `.test.ts` 结尾的名称，例如 `first-mod.test.ts`，并将其保存在插件目录中的任何位置。每个测试文件至少需要一个 `test()`，否则运行会失败并显示 `declares no test(): nothing ran`。测试文件可以导入您的 mod 自己的文件和兄弟 `.ts` 帮助程序，因此您可以对纯函数（例如游戏的规则）进行单元测试，而无需使用工具包。

此测试触发两个工具调用，从 [Create a mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md) 运行 `/tally` 命令，并检查回复是否计算了两者。其第一行是一个 [stub](#stub-what-claude-code-would-answer)，它代替 Claude Code 回答工具调用。将其保存为 `first-mod/tests/first-mod.test.ts`：

```typescript first-mod/tests/first-mod.test.ts theme={null}
import { expect, test } from 'claude-code/testing'

test('/tally reports the tool calls the mod has seen', async ($, on) => {
  // Answer each tool call in Claude Code's place, so no tool runs
  on('tool.call', () => ({ result: 'ok' }))

  // Fire two tool calls, which the mod's tool.call hook counts
  await $.tool.call({ tool: 'Bash', command: 'ls' })
  await $.tool.call({ tool: 'Read', file_path: 'README.md' })

  // Run /tally and check the text its hook returns
  const answer = await $.command.run({ command: 'tally', args: '' })
  expect(answer.text).toBe('Claude has made 2 tool calls since this mod loaded')
})
```

在您的 shell 中，从 `first-mod` 目录运行测试：

```bash theme={null}
claude plugin test
```

输出列出每个测试及其是否通过，时间从一次运行到另一次运行会有所不同：

```text theme={null}
tests/first-mod.test.ts:
(pass) /tally reports the tool calls the mod has seen [22.87ms]

 1 pass
 0 fail
Ran 1 test across 1 file. [0.19s]
```

每个 `$.tool.call` 都通过了 mod 的 [`tool.call`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#tools) hook，该 hook 将其计数加一并将调用传递给 stub。没有 `ls` 运行，也没有文件被读取。然后 `$.command.run` 进入 mod 的 [`command.run`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#commands-and-configuration) hook，`answer` 是该 hook 返回的对象。

当测试失败时，命令以状态 1 退出，因此它在 CI 中有效。如果您自己的 mods 无法在运行它的 shell 中加载，它会打印一行以 `claude plugin test: hooks modules are turned off` 开头的行，说明原因，并以状态 1 退出。

### Stub Claude Code 会回答的内容

在测试中没有模型、存储或工具运行，因此无论您的 mod 期望 Claude Code 回答什么，测试都会使用 stub 提供答案。测试函数为此接收两个参数：

* **`$`**：测试自己的 `$`，它代替 Claude Code。它不是 hook 接收的 [mods API](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#mods-api-methods)。它的每个方法都会触发同名事件，通过您的 mod 的 hooks 发送它，并解析为结果：`$.tool.call({ tool: 'Bash', command: 'ls' })` 触发 `tool.call`。`$.command.run`、`$.prompt.submit`、`$.session.start` 和 `$.turn.complete` 的工作方式相同，`$.classic.Stop` 和其他 `$.classic` 方法触发 [settings hook 事件](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#hook-the-settings-hook-events)。测试无法直接触发 mods API 调用，例如 `ui.close`。通过您的 mod 触发它，例如按下关闭窗格的按钮。
* **`on`**：调用它来注册 stubs，这些是代替 Claude Code 回答的 hooks。为 mods API 调用命名 stub 时不要使用 `$.`，因此注册为 `store.get` 的 stub 会回答您的 mod 的 `$.store.get`。当您的 mod 调用 [`$.model.complete`](https://code.claude.com/docs/zh-CN/plugins/mods/api#call-a-model) 或 [`$.store.get`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#keep-state) 时，stub 会提供答案。

此示例 stubs 一个模型调用。hook 属于一个名为 `grader` 的 mod，处理一个 `/grade` 命令，该命令将句子发送到模型并报告回复是否以 `PASS` 开头。该文件仅包含被测试的 hook，因此 mod 还需要 `plugin.json` 和 `hooks.json`，如 [Create a mod](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#write-a-mod-yourself) 中所示。要在会话中输入 `/grade`，mod 还必须 [注册命令](https://code.claude.com/docs/zh-CN/plugins/mods/api.md#add-a-command)：

```javascript grader/hooks/register.js theme={null}
export function register(on) {
  on('command.run', { command: 'grade' }, async ($, e) => {
    // e.args is the text typed after /grade
    const reply = await $.model.complete({
      model: 'haiku',
      system: 'Grade the sentence. Start your reply with PASS or FAIL.',
      prompt: e.args,
    })
    const passed = reply.isAnswered && reply.text.startsWith('PASS')
    return { text: passed ? 'Passed' : 'Try again' }
  })
}
```

此测试 stubs 模型调用以检查 hook 对通过回复的处理：

```typescript grader/tests/grader.test.ts theme={null}
import { expect, test } from 'claude-code/testing'

test('a passing grade is reported', async ($, on) => {
  // Answer the mod's $.model.complete call with a fixed reply, so no model runs
  on('model.complete', () => ({
    value: {
      isAnswered: true,
      text: 'PASS\nNice sentence.',
      usage: { input_tokens: 10, output_tokens: 5, cache_read_input_tokens: 0, cache_creation_input_tokens: 0 },
    },
  }))

  // Run /grade, which makes the mod call the model
  const answer = await $.command.run({ command: 'grade', args: 'The cat sat on the mat.' })
  expect(answer.text).toBe('Passed')
})
```

测试通过是因为 hook 的 `reply` 是 `value` 下的对象，其 `text` 以 `PASS` 开头。要检查另一个分支，添加第二个测试，其 stub 返回以 `FAIL` 开头的 `text`，并期望 `Try again`。

mods API 调用的 stub 返回一个带有 `value` 字段的对象，该字段保存调用在您的 mod 中解析的内容：`{ value: 7 }` 使 `$.store.get` 解析为 `7`。Claude Code 事件（例如 [`turn.step`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#turns) 或 `tool.call`）的 stub 返回该事件自己的结果，例如 `{ result: 'ok' }`。`$.session.send` 和 `$.prompt.fill` 也采用其事件的结果，如表所示。[查看 stub 返回的内容](#look-up-what-a-stub-returns) 显示每个常见名称采用的形式。以下错误意味着 stub 是错误的或缺失的。失败的测试的输出包括一个以 `the engine reported:` 开头的块，每个错误都出现在那里：

* `returned neither { value } nor { deny }`：mods API 调用的 stub 返回了一个裸值，这会导致测试失败
* `no implementation for` 后跟一个名称：您的 mod 进行了该调用，没有 stub 回答它

工具包还导出现成的 mocks，用于时钟、存储、环境变量以及追加到对话中的行：

* **`mock.clock(on)`**：回答 [`$.clock`](https://code.claude.com/docs/zh-CN/plugins/mods/api#run-work-in-the-background)，并返回一个您的测试可以推进的 mock 时钟，因此计时器的测试不会等待。
* **`mock.store(on, { count: 7 })`**：从以这些条目开始的存储中回答 `$.store`。它不返回任何内容，因此要检查您的 mod 保存了什么，请自己编写两个 `store` stubs，如 [drawing test](#test-a-drawing) 所做的那样。
* **`mock.env(on, { CI: 'true' })`**：从这些变量中回答 `$.env.get`。
* **`mock.session(on)`**：返回一个 mock 会话，其 `appended()` 方法按从旧到新的顺序列出您的 mod 通过 [`$.session.append`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session) 添加的行；需要 Claude Code v2.1.293 或更高版本。

### 遵循测试工具包的规则

测试工具包有一些自己的规则，违反其中一个会产生新测试作者首先遇到的错误：

* **在测试对 `$` 的第一次调用之前注册每个 stub。** 在那之后调用 `on` 会抛出一个错误，例如 `on("ui.render") after the test first called $`。

* **[`session.start`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#session) 不会自己运行。** 每个测试都以您的模块新加载开始，其 hooks 都没有被调用，因此模块级变量保持其初始值。如果 hook 依赖于 `session.start` 设置的内容，请先触发它：

  ```typescript theme={null}
  // Answer the event after your hook passes it on with next(e)
  on('session.start', () => ({ cwd: '/work' }))
  // Answer the $.command.register call your hook makes
  on('command.register', () => ({ value: undefined }))
  // Fire the event, which runs your session.start hook
  await $.session.start({ surface: 'terminal', isInteractive: true, cwd: '/work' })
  ```

  第二个 stub 回答 `session.start` hook（例如 [tutorial](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#write-a-mod-yourself) 中的那个）进行的 `$.command.register` 调用。没有它，该调用会以 `no implementation for command.register` 拒绝，工具包会跳过您的 hook，因此 hook 中调用后的任何内容都不会运行。测试在那一点不会失败。仅当稍后的检查失败时，跳过的 hook 才会在 `the engine reported:` 下列出。

* **返回 `next(e)` 的 hook 需要一个 stub 来回答。** 例如，当您的 [`ui.render`](https://code.claude.com/docs/zh-CN/plugins/mods/reference#interface) hook 返回 `next(e)` 时，为了在 Claude 空闲时不绘制任何内容，[mounting it](#test-a-drawing) 会失败并显示 `no implementation for ui.render`。注册一个返回元素作为纯数据的 stub：

  ```typescript theme={null}
  // Stands for what Claude Code would draw at the site
  on('ui.render', () => ({ type: 'Text', props: {}, children: ['drawn by Claude Code'] }))
  ```

  注册 stub 后，mount 成功，`ui.find({ type: 'Text' })` 在您的 hook 返回 `next(e)` 时返回该元素。

* **`turn.step` 的 stub 是一个异步生成器**，测试读取流到其末尾以获得结果：

  ```typescript theme={null}
  on('turn.step', async function* ($, e) {
    // Each yield is one piece of the model's streamed reply
    yield { kind: 'text', index: 0, text: 'ok' }
    // The return value is the result of the whole request
    return { turnId: e.turnId, index: e.index, answer: 'ok', toolUses: [], stopReason: 'end_turn', usage: null }
  })

  // Fire one request to the model, which runs your turn.step hook
  const stream = $.turn.step({ turnId: 't', index: 0, model: 'claude-test', messageCount: 1 })
  // Read every piece until the stream says it's done
  let step = await stream.next()
  while (step.done !== true) step = await stream.next()
  const result = step.value
  ```

  当循环结束时，`result` 是 stub 返回的对象，在您的 `turn.step` hook 有机会更改它之后。这里 `result.answer` 是 `'ok'`。

* **使用工具的名称和参数作为字段触发工具调用**，例如 `await $.tool.call({ tool: 'Bash', command: 'ls' })`，并注册一个返回 `{ result }` 的 `tool.call` stub。

### 查看 stub 返回的内容

您的 mod 在测试中进行的每个 mods API 调用都需要一个 stub 来回答，除了工具包自己回答的少数几个：[`$.ui.invalidate`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#redraw-when-something-changes)、[`$.state`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#keep-state) 和 `$.session.append` 调用。对于 `$.clock` 调用，使用 `mock.clock(on)`，否则您的 mod 的 `$.clock.now()` 会失败并显示 `no implementation for clock.now`。

此表列出了 mods 最常使用的。第一列是您的 mod 进行的调用或它使用 `next(e)` 传递的事件。第二列是传递给 `on` 的函数，使用该名称，因此 `$.store.get` 行变成 `on('store.get', ($, e) => ({ value: saved.get(e.key) }))`。stub 中的 `'...'` 标记您需要填写的文本：

| 您的 mod 调用或传递 | Stub |
| :- | :- |
| `$.command.register`、`$.tool.register`、`$.ui.toast`、`$.ui.log`、`$.ui.status`、`$.ui.close`、`$.store.set` | `() => ({ value: undefined })`。对于 `ui.toast` 和 `ui.log`，文本是 `e.text`。 |
| `$.store.get` | `($, e) => ({ value: saved.get(e.key) })` |
| `$.fs.read` | `($, e) => ({ value: e.path.endsWith('notes.md') ? '# Notes' : '' })`。`e.path` 作为绝对路径到达，因此与 `endsWith` 比较。 |
| `$.ui.open` | `() => ({ value: { isPlaced: true } })` |
| `$.ui.ask` | 一个 `tool.call` stub，因为问题作为对 `AskUserQuestion` 工具的调用到达它：`($, e) => ({ result: { answers: { [e.questions[0].question]: 'Run it' } } })`。如果您的 mod 传递其他工具调用，请先检查 `e.tool`。 |
| `$.model.complete` | `() => ({ value: { isAnswered: true, text: '...', usage } })` |
| `$.process.run` | `($, e) => ({ value: { exitCode: 0, stdout: '...', stderr: '' } })`。`e.argv` 是参数列表，`e.init` 保存 `cwd` 和 `timeoutMs`。 |
| 任何应该失败的 mods API 调用 | `() => ({ deny: 'the reason' })`，这使调用在您的 mod 中拒绝。抛出的 stub 会被跳过。 |
| `session.start` | `() => ({ cwd: '/work' })` |
| `turn.start` | `($, e) => ({ turnId: e.turnId })` |
| `tool.call` | `() => ({ result: '...' })` |
| `turn.complete` | `() => ({ text: '' })`。使用 `$.turn.complete({ turnId, answer, durationMs, isAborted: false, usage: null })` 触发它。 |
| `prompt.submit` | `($, e) => ({ text: e.text })` |
| `prompt.fill` | `() => ({ isFilled: true })` |
| `$.prompt.read` | `() => ({ value: { text: '...', cursor: 0 } })` |
| `$.ui.copy` | `() => ({ value: { isCopied: true } })` |
| `$.ui.notify` | `() => ({ value: { isSent: true, channel: 'terminal_bell' } })`。文本是 `e.text`，标题是 `e.title`。 |
| `$.session.messages` | `() => ({ value: [{ role: 'assistant', text: '...', toolUses: [] }] })` |
| `$.session.id`、`$.agent.list` | `() => ({ value: 'abc123' })`、`() => ({ value: [] })` |
| `session.send` | `() => ({ isDelivered: true })`。`e.to` 即使您的 mod 传递 `{ sessionId }`，也作为字符串到达。 |
| `session.receive` | `($, e) => ({ text: e.text })`。使用 `$.session.receive({ origin: { kind: 'peer-send-message' }, text })` 触发它。 |
| `ui.render` | `() => ({ type: 'Text', props: {}, children: ['...'] })` |

`expect` 有断言 `toBe`、`toEqual`、`toMatch`、`toMatchObject`、`toContain`、`toBeDefined`、`toBeUndefined` 和 `toThrow`，以及任何之前的 `.not`。

当 `expect` 在您以普通函数（而非异步生成器）形式传给 `on` 的 stub 或 hook 中失败时，测试会失败。引擎会跳过该 hook，失败输出会指出其名称，例如 `in the test's store.set hook`。

## 测试计时器

在计时器上运行工作的 mod 需要测试控制的时钟，因此测试可以向前移动时间而不是等待。`const clock = mock.clock(on)` 返回一个从 `0` 开始的 mock 时钟，仅在您的测试移动它时才移动。要在另一个时间开始，请以毫秒为单位传递它，如 `mock.clock(on, { now: 5000 })`。时钟有这些方法：

| 方法 | 它做什么 |
| :- | :- |
| `await clock.advance(1000)` | 将时间向前移动该毫秒数并运行每个到期的计时器 |
| `await clock.set(5000)` | 将时间向前移动到该值，如 `advance` 会做的那样 |
| `clock.now()` | 返回时间，这是您的 mod 的 `$.clock.now()` 解析的内容 |
| `await clock.settle()` | 运行已经到期的计时器，例如一系列零延迟 `$.clock.after` 调用，而不移动时间 |
| `await clock.sleep(2000)` | 在 stub 内，使该 stub 仅在测试推进到那么远时才回答，这是您模拟缓慢模型或进程的方式 |

此 hook 属于一个名为 `countdown` 的 mod，处理一个 `/countdown` 命令，该命令接受秒数，启动一个一秒的 `$.clock.every` 计时器，并在零时显示 toast。与 `grader` 一样，该文件仅包含被测试的 hook，不注册命令：

```javascript countdown/hooks/register.js theme={null}
export function register(on) {
  on('command.run', { command: 'countdown' }, async ($, e) => {
    // e.args is the text typed after /countdown
    let left = Number(e.args)
    const timer = $.clock.every(1000, () => {
      left -= 1
      if (left === 0) {
        timer.cancel()
        $.ui.toast('Time is up')
      }
    })
    // Print nothing in the transcript
    return {}
  })
}
```

此测试运行 `/countdown 3` 并移动 mock 时钟，因此它检查三秒的行为而不等待三秒：

```typescript countdown/tests/countdown.test.ts theme={null}
import { expect, mock, test } from 'claude-code/testing'

test('the countdown ends with a toast', async ($, on) => {
  // Answer every $.clock call from a clock the test controls
  const clock = mock.clock(on)
  // Collect the text of each toast the mod shows
  const toasts: string[] = []
  on('ui.toast', ($, e) => {
    toasts.push(e.text)
    return { value: undefined }
  })

  await $.command.run({ command: 'countdown', args: '3' })
  // After two seconds the timer has fired twice, and no toast is due
  await clock.advance(2000)
  expect(toasts).toEqual([])
  // The third second brings the count to zero
  await clock.advance(1000)
  expect(toasts).toEqual(['Time is up'])
})
```

第一个 `expect` 显示 toast 不会提前出现，第二个显示它出现一次。每个 `advance` 在到期的计时器运行后解析，因此下一行的检查会看到它们的效果。

## 测试绘图

测试可以绘制您的 mod 的 [render sites](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#render-sites) 之一，然后按下、输入和查找它绘制的元素。`$.ui.mount` 通过您的 mod 的 `ui.render` hook 绘制站点，并返回一个带有每个方法的句柄。要在一个测试中覆盖多个应用，请将 `surface` 设置为要绘制的应用。此测试打开来自 [Build a pane with tabs](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#build-a-pane-with-tabs) 的窗格，切换选项卡，按下按钮，并检查终端和桌面应用中的计数：

```typescript hello-tabs/tests/hello-tabs.test.ts theme={null}
import { expect, test } from 'claude-code/testing'

// What Claude Code passes to a ui.render hook for this pane, apart from the app
const PANE = {
  plugin: 'hello-tabs',
  component: 'Pane',
  requestId: 'hello-tabs',
  viewport: { columns: 100, rows: 30 },
  props: {
    title: 'Hello tabs',
    isFocused: true,
    bodyColumns: 60,
    placement: 'inline',
    scroll: { offset: 0, bodyRows: 10 },
    view: {},
  },
} as const

test('the second tab counts presses and saves the count', async ($, on) => {
  // Stub $.store with a Map, so the test can read what the mod saved
  const saved = new Map<string, unknown>()
  on('store.get', ($, e) => ({ value: saved.get(e.key) }))
  on('store.set', ($, e) => {
    saved.set(e.key, e.value)
    return { value: undefined }
  })

  // Draw the pane once for each app
  for (const surface of ['terminal', 'desktop'] as const) {
    const ui = await $.ui.mount({ ...PANE, surface })
    // Press the buttons by the key the mod gave them
    await ui.press({ key: 'tab-two' })
    await ui.press({ key: 'more' })
    // The second tab's count line is in the drawing
    expect(await ui.find({ type: 'Text', text: /^Count: \d+$/ })).toBeDefined()
    await ui.unmount()
  }

  // One press in each app makes two
  expect(saved.get('count')).toBe(2)
})
```

在您的 shell 中，从 `hello-tabs` 目录运行 `claude plugin test`。当两个应用都绘制计数行且 mod 已保存 `2` 时，测试通过。计数从第一个应用转移到第二个应用，因为两个 mounts 都使用相同的加载模块。

`$.ui.mount` 返回的句柄有这些方法，它们通过您给它们的 `key` 来寻址元素：

| 方法 | 它做什么 |
| :- | :- |
| `press({ key: 'more' })` | 按下具有该 key 的 `Button` |
| `input({ key: 'new-note', text: 'buy milk' })` | 将文本输入到具有该 key 的 `Input` 中并按 Enter。添加 `kind: 'change'` 以输入而不提交。对于带有 `rows` 的字段，文本中可以包含 `\n`。 |
| `select({ key: 'size', value: 'large' })` | 在具有该 key 的 `Select` 中选择具有该值的选项 |
| `find({ key: 'more' })` 或 `find({ type: 'Text', text: 'Count: 2' })` | 返回第一个匹配的元素作为 `{ type, props, children }`，或 `undefined`。`text` 可以是字符串或正则表达式。 |
| `unmount()` | 移除绘图 |

每个方法在您的处理程序完成后解析，因此您可以在下一行检查结果。将 `props` 设置为 Claude Code 为该站点传递的内容。[render sites table](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md#render-sites) 列出每个站点的 props，[您的构建的类型](https://code.claude.com/docs/zh-CN/plugins/mods/create.md#get-the-types-for-your-build) 有它们的类型。

要捕获对您在 `surface` 中指定的应用无效的[元素树](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#build-a-tree-from-elements)，请使用 `find` 检查您的 hook 添加的某个元素。测试不检查应用如何绘制树，因此也请在真实会话中查看新布局。

### 在 `/clear` 后测试绘图

每个测试都以每个 `$.state` 值在其默认值开始，这是 `/clear` 留下它们的方式。要测试您的 mod 接下来做什么，跳过 `session.start`，使用 `source: 'clear'` 触发 `classic.SessionStart`，并检查您的 mod 绘制的内容。

此测试检查来自 [Load a saved value again after `/clear`](https://code.claude.com/docs/zh-CN/plugins/mods/interface#load-a-saved-value-again-after-clear) 的模块。将其添加到来自 [Test a drawing](#test-a-drawing) 的文件中，其中定义了 `PANE`。该文件的第一个测试期望按钮保存计数，如 [Save from more than one session](https://code.claude.com/docs/zh-CN/plugins/mods/interface.md#save-from-more-than-one-session) 中的按钮所做的那样：

```typescript hello-tabs/tests/hello-tabs.test.ts theme={null}
test('the saved count comes back after /clear', async ($, on) => {
  // The store already holds a count of 7
  on('store.get', () => ({ value: 7 }))
  // Answer the event after your hook passes it on with next(e)
  on('classic.SessionStart', () => ({}))

  // Fire the event that follows /clear, which runs your hook
  await $.classic.SessionStart({ source: 'clear' })

  const ui = await $.ui.mount({ ...PANE, surface: 'terminal' })
  await ui.press({ key: 'tab-two' })
  // The pane shows the stored count, not the default of 0
  expect(await ui.find({ type: 'Text', text: 'Count: 7' })).toBeDefined()
})
```

当您的 `classic.SessionStart` hook 在窗格绘制之前将存储的 `7` 复制到 `$.state` 时，测试通过。如果您的模块中没有该 hook，窗格绘制 `Count: 0`，`find` 返回 `undefined`，测试在 `toBeDefined` 处失败。

## 测试 policy mod

您的组织在 [`prependPlugins`](https://code.claude.com/docs/zh-CN/plugins/mods/admin) 中列出的 mod 可以在另一个 mod 加载之前拒绝它。要测试一个，请设置您的 mod 的层级并给测试第二个 mod，供您的 mod 接受或拒绝：

* **`tier`**：在测试文件的顶部调用它一次，如 `tier('prepend')`，以将您的 mod 加载为 `prepend`、`append` 或 `builtin`，其在 [mods 运行的顺序](https://code.claude.com/docs/zh-CN/plugins/mods/events.md#the-order-mods-run-in) 中的位置。没有它，您的 mod 加载为 `user`。
* **`plugins`**：在测试体之前将 `test` 传递一个选项对象。其 `plugins` 数组保存您内联编写的 mods，每个都有一个 `name` 和一个 `register` 函数。要在 `user` 以外的地方加载一个，请将 `tier` 添加到它。

此测试文件首先加载 [admin page 中的 policy mod](https://code.claude.com/docs/zh-CN/plugins/mods/admin.md#enforce-a-policy-with-a-mod-of-your-own)。它检查 policy mod 是否拒绝启动进程的 mod 并接受不启动的 mod：

```typescript acme-guard/tests/guard.test.ts theme={null}
import { expect, test, tier } from 'claude-code/testing'

// Load the mod under test ahead of every other mod
tier('prepend')

// A second mod whose code calls $.process.run, which the policy blocks
const runner = {
  name: 'runner',
  register(on) {
    on('tool.call', async ($, e, next) => {
      await $.process.run(['ls'])
      return { result: 'runner answered' }
    })
  },
}

// A second mod that calls nothing the policy blocks
const reader = {
  name: 'reader',
  register(on) {
    on('tool.call', async ($, e, next) => {
      return { result: 'reader answered' }
    })
  },
}

test('refuses a mod that starts a process', { plugins: [runner] }, async ($, on) => {
  on('tool.call', () => ({ result: 'claude code answered' }))
  let message = ''
  try {
    // The first call on $ loads the mods, so the refusal is thrown here
    await $.tool.call({ tool: 'Bash', command: 'ls' })
  } catch (error) {
    message = error.message
  }
  expect(message).toBe('runner: refused by acme-guard: Acme policy: mods may not call process.run')
})

test('admits a mod that starts no process', { plugins: [reader] }, async ($, on) => {
  on('tool.call', () => ({ result: 'claude code answered' }))
  const out = await $.tool.call({ tool: 'Bash', command: 'ls' })
  // The answer comes from reader, which shows that it loaded
  expect(out).toEqual({ result: 'reader answered' })
})
```

在您的 shell 中，从 `acme-guard` 目录运行 `claude plugin test`。当 policy mod 如 admin page 所示时，两个测试都通过。

工具包在测试对 `$` 的第一次调用时加载每个 mod。当您的 mod 拒绝一个时，该调用会抛出，消息会命名被拒绝的 mod、拒绝它的 mod 和您的原因。在第二个测试中，没有任何东西被拒绝，因此 `reader` 在到达 stub 之前回答工具调用。

## 后续步骤

* [Troubleshoot a mod](https://code.claude.com/docs/zh-CN/plugins/mods/troubleshoot.md)：找出为什么 mod 在会话中不做任何事情
* [Mods reference](https://code.claude.com/docs/zh-CN/plugins/mods/reference.md)：每个事件的输入和结果，用于编写 stubs
