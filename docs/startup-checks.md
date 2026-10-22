# 启动先决条件检查

应用启动时自动检测 Claude CLI 和 Git Bash（仅 Windows）的可用性。检测是**只读**的：结果只作为运行时状态返回，不写回持久化配置。

涉及文件：`src-tauri/src/checks.rs`、`src-tauri/src/platform.rs`、`src-tauri/src/pty.rs`、`src-tauri/src/store.rs`、`src/App.vue`

## 检查流程

```
应用启动
  │
  ├─ lib.rs: CHECK_RESULTS (LazyLock)
  │   → checks::run_checks()
  │     ├─ platform::refresh_path()      ← PATH 来源对齐执行 shell
  │     ├─ read_config_paths()           ← 读取用户配置的 claudePath / gitBashPath
  │     ├─ check_claude_cli()            ← cli_target() + probe_cli()，不写配置
  │     └─ check_git_bash()              ← 检查 Git Bash (Windows only)
  │
  ├─ 前端 App.vue onMounted()
  │   → appStore.runChecks()
  │   ├─ 全部通过 → initAfterChecks() 进入主界面
  │   └─ 有失败   → 显示检查失败面板
```

## 配置写入边界

- **只有用户操作会写 `claudePath` / `gitBashPath`**：设置里的自定义路径、安装器安装完成。
- `run_checks()` / `pty::spawn_claude()` 都是只读；自动检测到的路径只出现在 `CheckResult`（`source = "path"`）与运行时 `cliHealth` 中。
- 这样做消除了“自动检测覆盖用户配置”“检查与执行争抢写入”的隐式耦合——配置来源始终是用户意图。

## 自动安装功能

检查失败时提供自动安装选项：

### 安装路径

| 软件 | 安装目录 | PATH 添加项 |
|------|----------|-------------|
| Claude CLI | `%LOCALAPPDATA%\Claude\claude.exe` | `%LOCALAPPDATA%\Claude` |
| Git 便携版 | `%LOCALAPPDATA%\PortableGit\bin\bash.exe` | `%LOCALAPPDATA%\PortableGit\bin` |

### 安装流程

1. 用户点击 "Auto Install" 按钮
2. 从 OSS 获取 latest.json 元数据
3. 下载对应平台的二进制文件
4. 放置到 `%LOCALAPPDATA%` 目录（Windows）
5. 动态添加到进程 PATH（立即生效）
6. 重新运行检查

### OSS 资源

```
cc-box/
├── deps/
│   ├── claude/
│   │   ├── latest.json              # 元数据（版本号、checksum、各平台下载链接）
│   │   └── v1.0.33/
│   │       ├── win32-x64/claude.exe
│   │       ├── darwin-arm64/claude
│   │       └── ...
│   └── git/
│       ├── latest.json              # Git 元数据
│       └── PortableGit-2.47.0-64-bit.7z.exe
```

### 下载脚本

使用 `npm run download-deps` 从官方源下载并上传到 OSS：

```bash
set HTTP_PROXY=http://127.0.0.1:33210
npm run download-deps
```

### PATH 动态配置

`platform::refresh_path()`（`src-tauri/src/platform.rs`）是 PATH 对齐的唯一入口：

- **Unix**：从登录 shell（`$SHELL -l -c 'printenv PATH'`）读取——launchd 启动的 GUI 不继承 shell PATH，这是裸命令 `claude` 可用的前提。
- **Windows**：Git Bash 会把继承的 Windows PATH 转成 POSIX 形式，因此只需保证父进程 PATH 完整（系统 + 用户环境），并补充 CC-Box 管理的安装目录（`%USERPROFILE%\.local\bin`、PortableGit\bin）。

原则：**PATH 的来源必须与执行命令的 shell 一致**，否则检查结论与执行结果会分叉。

## Claude CLI 检查

与执行层共用 `platform::cli_target()`：

| 步骤 | 来源 | 判定 |
|------|------|------|
| 1 | config.claudePath | 必须通过 `is_runnable_cli`（Unix：普通文件 + 执行位；Windows：PATHEXT 扩展名）→ `source = "configured"` |
| 2 | `where`/`which`（`probe_cli()`） | 仅用于检查/展示 → `source = "path"` |

两者都不可用时返回失败，并在文案里区分「配置路径不可运行」与「PATH 未找到」。

注意：检查只回答“当前能否找到可运行的 CLI”，执行层仍会自行做 PATH 解析（Unix：`find_executable` → 完整路径；Windows：交给 shell）——**展示结果绝不直接作为执行目标**。

## Git Bash 检查（仅 Windows）

| 步骤 | 来源 | 说明 |
|------|------|------|
| 1 | config.gitBashPath | 配置文件保存的路径 |
| 2 | 环境变量 `CLAUDE_CODE_GIT_BASH_PATH` | Claude Code 原生支持 |
| 3 | `where git.exe` → 推导安装目录 | 找到 git.exe 后拼接 `bin/bash.exe` |

## 运行态状态反馈

`appStore.cliHealth` 是 CLI 可用性的唯一状态源，三路更新：

| 事件 | 状态 |
|------|------|
| `runChecks()` 返回 | `ok`（含 path/source）或 `missing` |
| `pty-error`（stage = detection/spawn/startup） | `failed`（含原因与阶段） |
| spawn 成功 | 清除 `failed` |

TerminalView 的状态条消费该状态，展示原因并提供「重试」（重新跑 `runChecks(true)`）入口；启动检查失败仍走 App.vue 的全屏遮罩（填写路径 + 安装引导）。
