> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 第37周 · 2026年9月7日–11日

> 使用 claude plugin eval 测试您的插件，并将 Claude Code Desktop 窗格弹出到各自的窗口中。

发布版本 [v2.1.263 → v2.1.269](https://code.claude.com/docs/en/changelog.md#2-1-263)
2 项功能 · 9月7日–11日

使用 claude plugin eval 测试插件
v2.1.269

`claude plugin eval` 针对一套测试用例运行您的插件，对结果进行评分，默认情况下每个用例都会再次运行一次（不使用插件），以便您可以看到它的贡献。`claude plugin eval init` 询问您什么是好的结果，然后提议测试用例和对其进行评分的检查，尝试一次该套件，并写入文件。每次运行，以及每个有第二个模型判断回复的检查，都是您账户上的真实模型调用。

![claude plugin eval 的终端输出：一个包含七个用例的表格，每个用例都显示有插件和无插件的分数、两者之间的差值、运行次数和成本，后面是一个汇总行，显示平均差值、总持续时间和总成本](https://mintcdn.com/claude-code/f9HTZGyMtxIFOUgt/images/whats-new/plugin-eval.jpg?fit=max\&auto=format\&n=f9HTZGyMtxIFOUgt\&q=85\&s=913066f6d4a2a15426e98a627802f47f)

从您的插件根目录，让 Claude 起草该套件：

```bash terminal theme={null}
claude plugin eval init
```

当 Claude 告诉您该套件已准备好时，退出 `claude plugin eval init` 打开的会话，并运行 `claude plugin eval .` 来对每个用例进行评分。汇总表在您的终端中打印，`evals/results/` 下的 `report.html` 包含每次运行的详细信息。

[使用 evals 测试插件](https://code.claude.com/docs/zh-CN/plugin-evals.md)

将 Desktop 窗格弹出到各自的窗口中
Desktop

在 Claude Code Desktop 应用中，您可以将任何窗格弹出到其自己的窗口中。将 diff 或终端拖到第二个屏幕，同时 Claude 在主窗口中继续工作，然后在完成后将窗格停靠回去。

<video autoPlay muted loop playsInline src="https://mintcdn.com/claude-code/f9HTZGyMtxIFOUgt/images/whats-new/desktop-pop-out-panes.mp4?fit=max&auto=format&n=f9HTZGyMtxIFOUgt&q=85&s=ff3770dd09bb15ed9cf17a460f3d1e23" />

[整理您的工作区](https://code.claude.com/docs/zh-CN/desktop.md#arrange-your-workspace)

其他改进

在顶级或 `modelSettings` 下按模型设置 [`maxEffortLevel`](https://code.claude.com/docs/zh-CN/settings-reference#maxeffortlevel) 以限制每个提供商（包括 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry）上的努力级别；任何更高的级别都以上限运行将 `--plugin-dir` 指向一个插件文件夹，以 [加载每个具有清单的直接子文件夹](https://code.claude.com/docs/zh-CN/plugins/create.md#load-a-directory-or-archive-for-one-session)如果 WebFetch 在五分钟内未完成下载页面，[获取失败并显示截止时间错误](https://code.claude.com/docs/zh-CN/tools-reference.md#webfetch-tool-behavior)，而不是挂起；设置 `CLAUDE_CODE_WEBFETCH_DEADLINE_MS` 以更改截止时间，或设置为 `0` 以移除限制将 `--json` 传递给 `claude plugin install`、`uninstall`、`update`、`enable` 或 `disable` 以将结果打印为 [stdout 最后一行的一个 JSON 对象](https://code.claude.com/docs/zh-CN/plugins/cli-reference.md#plugin-json-result)当自动模式分类器阻止一个操作时，Claude 收到的原因 [通常会命名匹配的规则](https://code.claude.com/docs/zh-CN/auto-mode-config.md#fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry)，例如 `[Data Exfiltration]`当您在提示中途键入 `/` 时，您现在可以从 [匹配命令的列表](https://code.claude.com/docs/zh-CN/interactive-mode.md#complete-a-command-mid-prompt)中选择，而不是单个建议。该列表在全屏渲染中键入时打开。插件技能也可以按其名称（不带插件前缀）进行匹配在 VS Code 扩展中，单击提示框底部的代理计数以打开 [代理地图](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)，您可以在其中打开子代理的只读记录或停止它在 VS Code 扩展中，在命令菜单的"自定义"部分中选择 **Hooks** 或 **Permissions** 以 [在您的用户、项目和本地设置中添加或删除 hooks 和权限规则](https://code.claude.com/docs/zh-CN/vs-code.md#use-the-prompt-box)Claude 可以选择一个 [浏览器标签图标](https://code.claude.com/docs/zh-CN/artifacts.md#create-an-artifact)来匹配它发布的每个工件在 Claude Code 网页版中，在 Claude 读取之前在云会话中取回一条排队的消息：从队列中删除它，或按 `Esc` 或 `Up`，文本返回到消息框

[v2.1.263–v2.1.269 的完整更新日志 →](https://code.claude.com/docs/en/changelog.md#2-1-263)
