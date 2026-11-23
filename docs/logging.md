# 日志系统

应用日志按日期保存到 `~/.cc-box/logs/`，前端和后端统一写入同一套文件。

## 文件结构

```
~/.cc-box/logs/
├── 2026-04-28.log           # 当日全部日志（保存级别见下表）
├── 2026-04-28.error.log     # 当日错误日志（WARN 及以上）
└── ...
```

## 日志级别

| 模式 | 默认保存级别 | stderr 输出 |
|------|---------|------------|
| debug | Debug 及以上 | 有 |
| release | Info 及以上 | 无 |

**运行时覆盖**：环境变量 `CC_BOX_LOG=error|warn|info|debug|trace`（大小写不敏感）优先于编译默认——release 排障时设系统级 `CC_BOX_LOG=debug` 可临时放开过程日志（GUI 启动时读一次，需在启动前设好）。注意日志量会明显放大，排障后记得关闭；非法取值被忽略、回退编译默认。

## 级别规则（写日志前先对号）

| 级别 | 语义 | 例 |
|------|------|----|
| ERROR | 失败且影响用户 | spawn 失败、下载失败、移动文件重试耗尽 |
| WARN | 降级/重试/可疑 | 直连失败转代理、杀软锁文件重试中 |
| INFO | 用户可感知的状态变迁 | 会话创建/退出、连接建立/断开、安装完成 |
| DEBUG | 过程流水 | resize、cache hit、下载进度、探测结果 |

**上下文前缀统一 `key=value`**（如 `pty=<id> cmd=<name>`）。模块信息已由 target 承载，不再写 `[MCP]`/`[Installer]` 类方括号前缀（target 本身就是模块名）。

**错误链必须展开**：持有 `std::error::Error` 的错误用 `crate::logger::error_chain(&e)`（Display 起头逐层 ` -> ` 连接，深度上限 8 防环）；anyhow 带 context 的错误用 `format!("{e:#}")`。禁止只打印最外层 `format!("{}", e)` 丢 source（npm shim spawn 排障踩过坑）。

## 文件命名与轮转

- 文件名格式：`{YYYY-MM-DD}.log` / `{YYYY-MM-DD}.error.log`
- **跨零点运行中滚动**：每条日志写入前比对当前日期，跨天自动重开新文件（重开失败保留旧句柄继续写、下条再试，不丢日志）
- **init 打开失败**：启动时任一文件打开失败则不记日期前缀（current_date 留空），首条日志触发与滚动失败路径相同的重试——否则日期已记录会让重试被跳过、整日静默丢弃
- 启动时后台清理超过 **7 天** 的旧日志文件

## 日志格式

```
[2026-04-28 14:30:00.123][INFO] [cc_box::pty] pty=abc Claude CLI spawned successfully
[2026-04-28 14:30:01.456][ERROR] [cc_box::installer] Failed to create temp dir ...: NotFound -> ...
[2026-04-28 14:30:02.789][WARN] [Frontend] startTab: blocked by isPtyStarting
```

格式：`[时间戳][级别] [模块路径] 消息内容`

前端日志的模块路径统一为 `[Frontend]`。

## 前端日志接口

```typescript
import { logMessage } from '@/api/tauri'

logMessage('error', `startTab failed: ${err}`)
logMessage('warn', 'startNewSession: blocked')
logMessage('info', 'user clicked new session')
logMessage('debug', 'debug info')
```

参数：`level: 'error' | 'warn' | 'info' | 'debug'`，`message: string`

## 代码结构

```
src-tauri/src/logger.rs      # FileLogger 实现、跨零点滚动、级别解析、error_chain、旧日志清理
src-tauri/src/pty.rs         # spawn_fail 单点（log + emit pty-error + 返回 Err）
src-tauri/src/commands.rs    # log_message command（前端→后端桥接）
src-tauri/src/lib.rs         # setup 阶段调用 logger::init()
src-tauri/src/tests/logger.rs # 级别解析/优先级、错误链、滚动判断、行格式测试
src/api/tauri.ts             # logMessage 前端封装
```

## 可靠性

- 每条日志写入后立即 `flush`
- 日志文件以追加模式打开，多次启动不覆盖
