# 手动测试条目

自动化测试无法覆盖的 UI 交互与端到端测试。按功能模块分类。

---

## i18n 多语言支持

### I18n_LanguageSwitch_001 — 设置中切换语言立即生效

**目标**：验证在设置 > 外观中切换语言后，所有 UI 文本立即更新

**前置条件**：应用已启动，语言为 English

**操作步骤**：
1. 打开 Settings（`Ctrl+,` 或点击图标栏齿轮图标）
2. 在 Appearance 区域找到 Language 选项
3. 点击「中文」卡片

**预期结果**：
- 设置导航标签立即变为中文：外观、启动、接口、快捷键、更新、关于
- 页面标题「Appearance」变为「外观」
- Font Size 标签变为「字体大小」
- Theme 标签变为「主题」
- Light/Dark 标签变为「浅色」/「深色」
- Language 标签变为「语言」

### I18n_LanguageSwitch_002 — 切换到英文后各页面文本正确

**目标**：切换到英文后，验证所有主要页面的文本都为英文

**前置条件**：语言已切换为中文

**操作步骤**：
1. 在设置中切换回「English」
2. 依次点击设置导航中的每个 section（Appearance、Startup、Providers、Shortcuts、Update、About）
3. 关闭设置，查看图标栏 tooltip
4. 打开项目，查看侧边栏面板（Sessions、Skills、Agents、MCP Servers、Plugins）

**预期结果**：
- 所有页面标题、描述、按钮文本为英文
- 无残留中文字符
- 图标栏 tooltip 为英文

### I18n_LanguageSwitch_003 — 切换到中文后各页面文本正确

**目标**：切换到中文后，验证所有主要页面的文本都为中文

**前置条件**：应用已启动

**操作步骤**：
1. 设置 > 外观中切换为「中文」
2. 依次点击设置导航中的每个 section
3. 关闭设置，查看图标栏 tooltip
4. 打开侧边栏面板
5. 查看 Provider 设置中的按钮和对话框

**预期结果**：
- 所有页面标题、描述、按钮文本为中文
- Provider 卡片的按钮：使用、编辑、测试、删除 为中文
- 删除确认对话框为中文
- 技术术语（Claude Code、MCP、Provider 等）保持英文

### I18n_LanguageSwitch_004 — 重启后语言设置保持

**目标**：切换语言后重启应用，语言设置持久化

**前置条件**：应用已启动

**操作步骤**：
1. 设置 > 外观中切换为「中文」
2. 关闭并重新启动应用
3. 打开设置查看语言选项

**预期结果**：
- 重启后 UI 文本全部为中文
- 语言选择器中「中文」卡片为选中状态
- `~/.cc-box/config.json` 中 `language` 字段为 `"zh"`

### I18n_LanguageSwitch_005 — 欢迎页和项目选择页文本随语言切换

**目标**：欢迎页和项目选择页的文本也随语言变化

**前置条件**：应用已启动，语言为 English

**操作步骤**：
1. 设置中切换为「中文」
2. 回到欢迎页（关闭所有项目）
3. 查看标题、副标题、按钮文本
4. 打开项目选择页，查看搜索框、按钮、分组标题

**预期结果**：
- 欢迎页标题保持「Claude Code」（品牌名不翻译）
- 副标题变为中文
- 按钮文本变为中文
- 项目选择页搜索框 placeholder 为中文
- 「近期会话」「项目」「启动选项」等标签为中文

### I18n_LanguageSwitch_006 — 环境检查页面文本随语言切换

**目标**：首次安装时的环境检查界面也支持多语言

**前置条件**：删除 Claude CLI 或 Git（或模拟环境检查失败）

**操作步骤**：
1. 设置语言为中文
2. 重启应用触发环境检查

**预期结果**：
- 环境检查标题为中文
- 「自动安装」按钮为中文
- 「重试」按钮为中文
- 安装状态消息为中文

### I18n_KeyboardShortcuts_007 — 快捷键面板文本随语言切换

**目标**：Shortcuts 设置页和快捷键弹窗文本随语言切换

**前置条件**：应用已启动

**操作步骤**：
1. 设置语言为中文
2. 打开设置 > 快捷键页面
3. 按 `Ctrl+Shift+/` 打开快捷键弹窗

**预期结果**：
- 快捷键分组标题为中文（应用快捷键、会话管理等）
- 快捷键描述为中文
- 弹窗标题为中文

---

## PTY 环境变量注入

环境变量从 PTY 启动时注入进程，不写入 `~/.claude/settings.json`。以下测试验证注入行为、设置面板交互、与 Provider 的兼容性。

### EnvVars_PtyInject_001 — 新建 Claude 终端默认环境变量已注入

**目标**：验证默认的 6 个环境变量在新建 Claude 终端时正确注入

**前置条件**：应用已启动，环境变量为默认值（未修改过）

**操作步骤**：
1. 选择一个项目，新建 Claude 终端
2. 等待终端出现输入提示符
3. 在终端中逐个输入以下命令查看值：
   - `echo $LANG`
   - `echo $LC_ALL`
   - `echo $PYTHONUTF8`
   - `echo $CLAUDE_CODE_SCROLL_SPEED`
   - `echo $PYTHONIOENCODING`
   - `echo $CLAUDE_CODE_NO_FLICKER`

**预期结果**：
- `LANG` → `en_US.UTF-8`
- `LC_ALL` → `en_US.UTF-8`
- `PYTHONUTF8` → `1`
- `CLAUDE_CODE_SCROLL_SPEED` → `5`
- `PYTHONIOENCODING` → `utf-8`
- `CLAUDE_CODE_NO_FLICKER` → `1`

### EnvVars_PtyInject_002 — 新建 Shell 终端环境变量已注入

**目标**：验证环境变量在 Shell 终端（非 Claude）中同样注入

**前置条件**：应用已启动

**操作步骤**：
1. 选择一个项目，新建 Shell 终端（通过终端标题栏的 `+` 按钮下拉选择 Shell）
2. 在终端中输入 `echo $CLAUDE_CODE_NO_FLICKER`
3. 输入 `echo $PYTHONUTF8`

**预期结果**：
- `CLAUDE_CODE_NO_FLICKER` → `1`
- `PYTHONUTF8` → `1`

### EnvVars_SettingsEdit_003 — 设置面板修改值后新终端生效

**目标**：在设置面板修改环境变量的值，新建终端后生效

**前置条件**：应用已启动

**操作步骤**：
1. 打开 Settings → Startup → Environment Variables
2. 将 `PYTHONUTF8` 的值从 `1` 改为 `0`
3. 关闭设置，回到终端
4. 新建一个 Claude 终端
5. 输入 `echo $PYTHONUTF8`

**预期结果**：
- 新终端输出 `0`
- 已运行的终端不受影响（仍为旧值）

### EnvVars_SettingsAddRemove_004 — 设置面板新增/删除环境变量

**目标**：验证新增和删除环境变量后，新终端正确反映变更

**前置条件**：应用已启动

**操作步骤**：
1. 打开 Settings → Startup → Environment Variables
2. 点击 `+` 按钮，添加 `MY_TEST_VAR` = `hello`
3. 关闭设置，新建 Claude 终端
4. 输入 `echo $MY_TEST_VAR`
5. 回到设置，删除 `MY_TEST_VAR` 这一行
6. 再新建一个 Claude 终端
7. 输入 `echo $MY_TEST_VAR`

**预期结果**：
- 第一个新终端输出 `hello`
- 第二个新终端输出空（变量不存在）

### EnvVars_SettingsRename_005 — 设置面板重命名 key

**目标**：验证修改 key 名称后，旧 key 不再注入，新 key 注入

**前置条件**：应用已启动

**操作步骤**：
1. 打开 Settings → Startup → Environment Variables
2. 新增一行 `TEST_OLD_KEY` = `test_value`
3. 关闭设置，新建终端验证 `echo $TEST_OLD_KEY` → `test_value`
4. 回到设置，将 `TEST_OLD_KEY` 的 key 改为 `TEST_NEW_KEY`（点击 key 名编辑）
5. 关闭设置，新建终端
6. 输入 `echo $TEST_OLD_KEY`
7. 输入 `echo $TEST_NEW_KEY`

**预期结果**：
- `TEST_OLD_KEY` 输出空
- `TEST_NEW_KEY` → `test_value`

### EnvVars_NoSettingsJsonWrite_006 — 不写入 ~/.claude/settings.json

**目标**：验证环境变量的修改不再写入 Claude Code 的 settings.json

**前置条件**：应用已启动

**操作步骤**：
1. 先记录 `~/.claude/settings.json` 当前内容（若文件存在）
2. 在 cc-box 设置中修改 `PYTHONUTF8` 为 `0`，新增 `MY_TEST` = `val`
3. 新建终端（触发 PTY 注入）
4. 重新打开 `~/.claude/settings.json` 查看内容
5. 检查 `env` 字段中是否包含 `PYTHONUTF8` 或 `MY_TEST`

**预期结果**：
- `settings.json` 的 `env` 字段中**没有** `PYTHONUTF8` 或 `MY_TEST`（如果 `env` 字段存在的话）
- 如果之前通过旧版 cc-box 写入过这些 key，它们可能仍残留，但本次修改不会更新它们

### EnvVars_ProviderNoConflict_007 — Provider 激活后环境变量仍生效

**目标**：验证激活 Provider 后，cc-box 环境变量注入不受影响

**前置条件**：至少配置了一个 Provider

**操作步骤**：
1. 激活一个 Provider（在 Settings → Providers 中点击「Use」）
2. 新建 Claude 终端
3. 输入 `echo $CLAUDE_CODE_NO_FLICKER`
4. 输入 `echo $PYTHONUTF8`

**预期结果**：
- `CLAUDE_CODE_NO_FLICKER` → `1`
- `PYTHONUTF8` → `1`
- 两者均不受 Provider 激活影响（Provider 写 settings.json，cc-box 注入 PTY 进程，互不干扰）

### EnvVars_PersistAcrossRestart_008 — 重启后环境变量配置保持

**目标**：验证环境变量配置在重启应用后保持不变

**前置条件**：应用已启动

**操作步骤**：
1. 打开 Settings → Startup → Environment Variables
2. 新增 `PERSIST_TEST` = `survived`
3. 关闭应用
4. 重新启动应用
5. 打开 Settings → Startup → Environment Variables，查看列表
6. 新建终端，输入 `echo $PERSIST_TEST`

**预期结果**：
- 设置面板中 `PERSIST_TEST` = `survived` 仍在
- 新终端输出 `survived`
- `~/.cc-box/config.json` 中 `claudeEnvVars` 包含 `PERSIST_TEST`

---

## 应用更新

### Update_OneClick_001 — 一键更新完整流程

**目标**：验证有新版本时，点击"下载并安装"能完成下载→安装→重启

**前置条件**：当前版本低于远程最新版本（可在 tauri.conf.json 中临时降低 version 触发）

**操作步骤**：
1. 启动应用，等待自动检查更新
2. 观察图标栏设置图标是否出现红色角标
3. 点击设置图标，确认自动跳转到"更新"设置页
4. 查看新版本号、release notes 内容
5. 点击「下载并安装」按钮
6. 观察进度条从 0% 推进到 100%
7. 观察状态从"下载中"变为"安装中"
8. 等待应用自动重启

**预期结果**：
- 设置图标右上角出现红色小圆点（脉冲动画）
- 点击设置图标直接跳转到更新 section
- 进度条正常推进，显示百分比和已下载/总大小
- 安装阶段显示脉冲动画文字和"将自动重启"提示
- 应用自动重启后版本号已更新

### Update_ConfirmActivePtys_002 — 有活跃 PTY 时更新确认提示

**目标**：有终端在运行时点击更新，弹出确认对话框

**前置条件**：应用已启动，至少有一个 Claude 终端在运行

**操作步骤**：
1. 打开一个项目，新建 Claude 终端，等待 Claude CLI 启动
2. 设置图标应显示红色更新角标
3. 点击设置图标进入更新页
4. 点击「下载并安装」

**预期结果**：
- 弹出系统确认对话框，内容提示"有正在运行的终端会话，更新将重启应用并关闭所有会话"
- 点击「取消」→ 不执行更新，停留在更新页面
- 再次点击「下载并安装」→ 点击「确定」→ 正常进入下载流程

### Update_ConfirmNoPtys_003 — 无活跃 PTY 时不弹确认

**目标**：没有终端运行时点击更新，直接进入下载流程

**前置条件**：应用已启动，未打开任何项目/终端

**操作步骤**：
1. 在项目选择页（或欢迎页），不打开任何终端
2. 点击设置图标（有红色角标时）进入更新页
3. 点击「下载并安装」

**预期结果**：
- 不弹出任何确认对话框
- 直接进入下载进度界面

### Update_ManualDownload_004 — 手动下载按钮打开 GitHub

**目标**：验证"手动下载"按钮在浏览器中打开正确的 Releases 页面

**前置条件**：应用已启动，检测到有新版本

**操作步骤**：
1. 进入设置 > 更新页
2. 找到「手动下载」按钮
3. 点击该按钮

**预期结果**：
- 系统默认浏览器打开 `https://github.com/orczh-hj/cc-box/releases`
- 页面正常加载，能看到各版本的 Release 列表
- 按钮样式为边框按钮（非主按钮），与"下载并安装"并排

### Update_ErrorRetry_005 — 更新失败后重试

**目标**：验证下载失败时显示错误信息，重试能重新下载

**前置条件**：当前版本低于远程最新版本

**操作步骤**：
1. 进入更新页，点击「下载并安装」
2. 下载过程中断开网络（或模拟网络故障）
3. 等待下载失败
4. 查看错误信息显示
5. 恢复网络，点击「重试」链接

**预期结果**：
- 显示红色错误提示区域，包含具体错误信息
- 出现「重试」链接按钮
- 点击重试后重新开始下载流程（进度归零重新推进）

### Update_UpToDate_006 — 已是最新版本时的反馈

**目标**：验证没有新版本时，检查更新给出正确反馈

**前置条件**：当前版本等于远程最新版本

**操作步骤**：
1. 进入设置 > 更新页
2. 点击「检查更新」按钮

**预期结果**：
- 按钮显示旋转图标 + "检查中..."文字
- 检查完成后显示绿色提示"已是最新版本！"
- 不显示"下载并安装"按钮

### Update_ManualDownloadI18n_007 — 更新页文本随语言切换

**目标**：验证更新页所有文本随语言切换正确变化

**前置条件**：应用已启动，检测到有新版本

**操作步骤**：
1. 设置 > 外观中切换为中文
2. 进入更新页查看所有文本
3. 切换回 English
4. 再次查看更新页

**预期结果**：
- 中文时：软件更新、当前版本、检查更新、版本可用、更新内容、下载并安装、手动下载、已是最新版本
- 英文时：Software Update、Current Version、Check for Updates、is available、What's New、Download & Install、Manual Download、You're up to date!
- 有 PTY 时确认对话框文本也随语言切换

---

## 时间格式显示

### TimeFormat_TimeAgo_001 — 会话列表时间显示正确

**目标**：验证侧边栏会话列表中的相对时间文本正确

**前置条件**：应用已启动，有历史会话

**操作步骤**：
1. 打开一个项目，查看侧边栏会话列表
2. 观察 1 分钟内的会话显示"刚刚"（中文）或"Now"（英文）
3. 观察 5 分钟前的会话显示"5 分钟前"或"5 minutes ago"
4. 观察 2 小时前的会话显示"2 小时前"或"2 hours ago"
5. 观察 3 天前的会话显示"3 天前"或"3 days ago"

**预期结果**：
- 时间文本随当前语言正确显示
- 各时间段使用正确的格式（刚刚/N分钟前/N小时前/N天前）
- 切换语言后时间文本立即更新

---

## 资源管理器右键菜单

### ContextMenu_FolderRightClick_001 — 右键文件夹显示"使用 CC-Box 打开"

**目标**：验证在 Windows 资源管理器中右键文件夹时出现"使用 CC-Box 打开"菜单项

**前置条件**：CC-Box 已通过 NSIS 安装包安装（非 portable/开发模式）

**操作步骤**：
1. 打开 Windows 资源管理器
2. 右键点击任意文件夹

**预期结果**：
- 右键菜单中出现"使用 CC-Box 打开"
- 菜单项左侧显示 CC-Box 图标

### ContextMenu_BackgroundRightClick_002 — 右键空白处显示"在此处打开 CC-Box"

**目标**：验证在文件夹内空白区域右键时出现"在此处打开 CC-Box"菜单项

**前置条件**：CC-Box 已通过 NSIS 安装包安装

**操作步骤**：
1. 打开 Windows 资源管理器
2. 进入一个文件夹（如 `D:\projects`）
3. 在文件列表的空白区域右键

**预期结果**：
- 右键菜单中出现"在此处打开 CC-Box"
- 菜单项左侧显示 CC-Box 图标

### ContextMenu_OpenExistingProject_003 — 右键打开已有项目

**目标**：右键点击一个 CC-Box 中已有缓存的项目文件夹，CC-Box 启动后直接打开该项目

**前置条件**：CC-Box 已安装，`~/.claude/projects/` 中存在该项目的会话记录

**操作步骤**：
1. 确认 CC-Box 未运行
2. 在资源管理器中右键一个之前使用过的项目文件夹
3. 点击"使用 CC-Box 打开"
4. 等待 CC-Box 启动

**预期结果**：
- CC-Box 启动后直接进入终端视图（不显示欢迎页或项目选择页）
- 终端工作目录为右键点击的文件夹
- 侧边栏显示该项目的会话数据

### ContextMenu_OpenNewProject_004 — 右键打开新项目

**目标**：右键点击一个 CC-Box 中没有缓存记录的文件夹，作为新项目打开

**前置条件**：CC-Box 已安装，目标文件夹不在 `~/.claude/projects/` 中

**操作步骤**：
1. 确认 CC-Box 未运行
2. 创建一个新文件夹（如 `D:\test-new-project`）
3. 右键该文件夹，点击"使用 CC-Box 打开"
4. 等待 CC-Box 启动

**预期结果**：
- CC-Box 启动后直接进入终端视图
- 终端工作目录为 `D:\test-new-project`
- Claude CLI 在该目录下启动新会话（不尝试 resume）

### ContextMenu_PathWithSpaces_005 — 路径含空格时正确打开

**目标**：验证路径中包含空格时右键菜单能正确传递目录

**前置条件**：CC-Box 已安装

**操作步骤**：
1. 创建一个名称含空格的文件夹（如 `D:\My Projects\web app`）
2. 右键该文件夹，点击"使用 CC-Box 打开"

**预期结果**：
- CC-Box 启动后工作目录为 `D:\My Projects\web app`
- 路径完整无误，未截断

### ContextMenu_ChinesePath_006 — 中文路径正确打开

**目标**：验证路径中包含中文字符时右键菜单能正确传递目录

**前置条件**：CC-Box 已安装

**操作步骤**：
1. 创建一个中文名称的文件夹（如 `D:\项目\前端开发`）
2. 右键该文件夹，点击"使用 CC-Box 打开"

**预期结果**：
- CC-Box 启动后工作目录为中文路径
- Claude CLI 正常启动，路径无乱码

### ContextMenu_UninstallCleanup_007 — 卸载后右键菜单项消失

**目标**：验证通过 NSIS 卸载 CC-Box 后，右键菜单项被清除

**前置条件**：CC-Box 已安装且右键菜单正常工作

**操作步骤**：
1. 通过系统"设置 > 应用"或卸载程序卸载 CC-Box
2. 在资源管理器中右键文件夹
3. 在文件夹空白区域右键

**预期结果**：
- 右键菜单中不再出现"使用 CC-Box 打开"和"在此处打开 CC-Box"
- 注册表 `HKCU\Software\Classes\Directory\shell\cc-box` 和 `HKCU\Software\Classes\Directory\Background\shell\cc-box` 不存在

### ContextMenu_BackgroundDir_008 — 右键空白处传入当前目录

**目标**：验证右键文件夹空白处时，传入的是当前目录而非空值

**前置条件**：CC-Box 已安装

**操作步骤**：
1. 在资源管理器中进入 `D:\projects\my-app` 目录
2. 在文件列表空白区域右键
3. 点击"在此处打开 CC-Box"

**预期结果**：
- CC-Box 启动后工作目录为 `D:\projects\my-app`
- 不是父目录 `D:\projects`

### ContextMenu_CliDirectInvoke_009 — 命令行直接传入目录路径

**目标**：验证通过命令行直接调用 `cc-box.exe <dir>` 也能正确打开（无需右键菜单）

**前置条件**：CC-Box 已安装，`cc-box.exe` 在 PATH 中或使用完整路径

**操作步骤**：
1. 打开命令提示符或 PowerShell
2. 执行 `"C:\Program Files\CC-Box\cc-box.exe" "D:\projects\test"`

**预期结果**：
- CC-Box 启动后工作目录为 `D:\projects\test`
- 行为与右键菜单打开一致

---

## Claude CLI 更新与安装

### ClaudeCli_VersionDetect_WinNpm_001 — Windows npm 安装的 claude 版本检测

**目标**：验证通过 npm 全局安装的 claude（`.cmd` 文件）能被正确检测到版本号

**前置条件**：Windows 系统，claude 通过 `npm install -g @anthropic-ai/claude-code` 安装

**操作步骤**：
1. 启动 CC-Box，确认启动检查通过（显示 Claude CLI ✓）
2. 进入设置 > 更新页
3. 点击 Claude CLI 区域的「检查更新」按钮

**预期结果**：
- 版本号区域显示 `vX.Y.Z`（而非"未安装"）
- 检查结果正确显示是否有更新可用

### ClaudeCli_OssNoProxy_001 — 系统代理残留时 OSS 请求仍直连

**目标**：验证代理工具退出后系统代理设置残留指向已死端口时，OSS 请求（版本列表/检查更新/下载）不受影响（installer.rs 的 oss_client 强制 no_proxy）

**前置条件**：Windows 系统；开启 Clash Verge 等代理工具后将其退出（注册表 `ProxyServer` 残留 `127.0.0.1:33210`，`ProxyEnable` 可能为 1）；无法自动化（依赖真实系统代理状态）

**操作步骤**：
1. 代理工具退出后，启动 CC-Box（或用已运行的实例）
2. 进入设置 > 更新页，点击 Claude CLI「检查更新」
3. 打开历史版本列表（加载 versions.json）
4. 任选一个历史版本下载

**预期结果**：
- 步骤 2/3 均成功，不出现「加载版本列表失败: ... unexpected EOF during handshake」
- 步骤 4 下载进度正常推进并完成
- 反证：修复前以上请求走系统代理（死端口）直接握手失败

### ClaudeCli_Startup_WinNpm_002 — Windows npm 安装的 claude 能在 CC-Box 终端启动

**目标**：验证 npm 全局安装的 claude（`%APPDATA%\npm\claude.cmd`）通过 git bash 调用时能正确进入 Claude 提示符，而不会被 PATH/PATHEXT 影响

**前置条件**：Windows 系统，claude 通过 `npm install -g @anthropic-ai/claude-code` 安装，`where claude` 返回结果包含 `.cmd` 文件，CC-Box 启动检查通过

**操作步骤**：
1. 启动 CC-Box
2. 在欢迎页点击「选择项目目录」，选择一个项目
3. 等待默认新建的 Claude 会话初始化
4. 观察终端输出，等待 Claude 提示符出现

**预期结果**：
- 终端不报「command not found」或类似错误
- 显示 Claude Code 版本号横幅与提示符（`>`）
- 可以输入消息并收到回复

### ClaudeCli_CustomPath_003 — 失败 overlay 中输入自定义路径后通过检查

**目标**：验证启动检查失败时，在 overlay 中粘贴已有 claude.exe 路径后能保存并立即通过检查

**前置条件**：把已有 claude.exe 移到非 PATH 路径（如 `D:\tools\claude.exe`），或保留它但不让 `where claude` 命中（如临时重命名 PATH 中的 claude → `claude_bak`）

**操作步骤**：
1. 启动 CC-Box，确认启动检查显示 Claude CLI ✗
2. 在 Claude CLI 失败项下的输入框中粘贴 `D:\tools\claude.exe`
3. 点击「保存并重试」按钮

**预期结果**：
- 输入框被清空
- 启动检查立即重新执行
- Claude CLI 变为 ✓，路径显示为 `D:\tools\claude.exe`
- overlay 关闭，进入欢迎页

### ClaudeCli_AutoInstallCancel_004 — Auto Install 取消按钮立即生效

**目标**：验证 Auto Install 过程中点击「取消」能立即停止下载并清理半成品文件

**前置条件**：删除 `~/.local/bin/claude.exe`（或 `claude`），让启动检查失败；网络需要稍慢以便有取消时机

**操作步骤**：
1. 启动 CC-Box，确认启动检查显示 Claude CLI ✗
2. 点击「自动安装」
3. 等下载进度走到 30%~70% 之间时，点击「取消」按钮

**预期结果**：
- 进度条立即停止增长，状态变为「已取消」（灰色）
- 任务条下方显示"已取消"消息
- `~/.local/bin/` 目录下没有半成品 `claude.exe` 或临时文件
- 「自动安装」按钮重新可点

### ClaudeCli_AutoInstallError_005 — Auto Install 失败显示具体错误

**目标**：验证 Auto Install 失败时显示具体错误原因（网络/HTTP/磁盘）而非通用 installFailed 文案

**前置条件**：断网或配置不可达的 OSS（如修改 hosts 把 `cc-box.oss-cn-beijing.aliyuncs.com` 指向 127.0.0.1）

**操作步骤**：
1. 启动 CC-Box，确认启动检查显示 Claude CLI ✗
2. 点击「自动安装」
3. 等待失败

**预期结果**：
- 状态显示「失败」（红色）
- 任务条下方显示具体错误，如「请求失败：...」「HTTP 错误：502」等
- 控制台同步打印错误堆栈

### ClaudeCli_VersionDetect_MacArm_002 — macOS ARM 版本检测

**目标**：验证 Apple Silicon Mac 上 claude 版本能被正确检测

**前置条件**：macOS Apple Silicon，claude 通过任意方式安装

**操作步骤**：
1. 启动 CC-Box
2. 进入设置 > 更新页
3. 点击 Claude CLI 区域的「检查更新」按钮

**预期结果**：
- 版本号区域显示 `vX.Y.Z`
- 不显示"未安装"

### ClaudeCli_InstallPlatform_MacArm_003 — macOS ARM 下载正确架构

**目标**：验证 Apple Silicon Mac 下载安装 `darwin-arm64` 二进制文件

**前置条件**：macOS Apple Silicon，有新版本可用

**操作步骤**：
1. 进入设置 > 更新页
2. 点击 Claude CLI 的「下载并安装」
3. 等待安装完成
4. 在终端执行 `file ~/.local/bin/claude`

**预期结果**：
- 下载进度正常推进
- 安装完成后版本号更新
- `file` 命令输出包含 `arm64` 或 `Mach-O 64-bit executable arm64`
- 无 "Platform xxx not supported" 错误

### ClaudeCli_InstallPlatform_MacIntel_004 — macOS Intel 下载正确架构

**目标**：验证 Intel Mac 下载安装 `darwin-x64` 二进制文件

**前置条件**：macOS Intel

**操作步骤**：
1. 进入设置 > 更新页
2. 点击 Claude CLI 的「下载并安装」
3. 等待安装完成
4. 在终端执行 `file ~/.local/bin/claude`

**预期结果**：
- 下载 `darwin-x64` 二进制文件
- `file` 命令输出包含 `x86_64`

### ClaudeCli_InstallPlatform_LinuxArm64_005 — Linux ARM64 下载正确架构

**目标**：验证 ARM64 Linux 桌面下载安装 `linux-arm64` 二进制文件

**前置条件**：Linux ARM64 桌面环境

**操作步骤**：
1. 启动 CC-Box（从桌面环境启动，非终端）
2. 进入设置 > 更新页
3. 点击「下载并安装」
4. 在终端执行 `file ~/.local/bin/claude`

**预期结果**：
- 下载 `linux-arm64` 二进制文件（不是 `linux-x64`）
- `file` 命令输出包含 `aarch64`

### ClaudeCli_PathPriority_Windows_006 — Windows 安装后 PATH 优先级最高

**目标**：验证安装完成后 `~/.local/bin` 在 PATH 最前面，优先于其他 claude

**前置条件**：Windows 系统，PATH 中已有其他 claude（如 npm 全局安装的）

**操作步骤**：
1. 进入设置 > 更新页，安装或更新 Claude CLI
2. 安装完成后，打开 cmd 执行 `where claude`
3. 观察 `claude.exe` 出现的顺序

**预期结果**：
- `%USERPROFILE%\.local\bin\claude.exe` 排在第一位
- 用户 PATH 环境变量中 `~/.local/bin` 位于最前面

### ClaudeCli_PathPriority_Mac_007 — macOS 安装后 PATH 持久化

**目标**：验证安装完成后 `~/.local/bin` 被持久写入 shell 配置

**前置条件**：macOS，默认 shell 为 zsh

**操作步骤**：
1. 进入设置 > 更新页，安装或更新 Claude CLI
2. 检查 `~/.zshenv` 文件内容
3. 打开新终端窗口，执行 `which claude`
4. 重启 CC-Box，确认启动检查仍通过

**预期结果**：
- `~/.zshenv` 中包含 `export PATH="$HOME/.local/bin:$PATH"`
- 新终端中 `which claude` 返回 `/Users/xxx/.local/bin/claude`
- 重启后启动检查通过

### ClaudeCli_PathPriority_Linux_008 — Linux 安装后 PATH 持久化

**目标**：验证安装完成后 `~/.local/bin` 被持久写入 shell 配置

**前置条件**：Linux，默认 shell 为 bash

**操作步骤**：
1. 进入设置 > 更新页，安装或更新 Claude CLI
2. 检查 `~/.bashrc` 文件内容
3. 打开新终端窗口，执行 `which claude`
4. 重启 CC-Box，确认启动检查仍通过

**预期结果**：
- `~/.bashrc` 中包含 `export PATH="$HOME/.local/bin:$PATH"`
- 新终端中 `which claude` 返回 `/home/xxx/.local/bin/claude`
- 重启后启动检查通过

### ClaudeCli_ReinstallPathPriority_009 — 重复安装后 PATH 不重复

**目标**：验证多次安装/更新后 PATH 中不会出现重复条目

**前置条件**：已安装 Claude CLI

**操作步骤**：
1. 进入设置 > 更新页，点击安装/更新
2. 安装完成后，再次点击安装/更新
3. 检查用户 PATH（Windows: 用户环境变量；macOS: `~/.zshenv`）

**预期结果**：
- PATH 中只有一条 `~/.local/bin` 相关条目
- 该条目在 PATH 最前面

---

## Claude CLI 历史版本切换

### Claude_Version_StartupNoHttp_001 — 启动检查零 HTTP 请求

**目标**：验证启动应用时不再调用 OSS `deps/claude/latest.json`，只读本地 Claude 版本号

**前置条件**：本机已安装 Claude CLI（任意版本）

**操作步骤**：
1. 启动 Fiddler/Wireshark 等抓包工具，过滤 `cc-box.oss-cn-beijing.aliyuncs.com`
2. 断网或保持抓包监听，启动 CC-Box
3. 等待应用完全启动（进入项目选择页或终端页）

**预期结果**：
- 应用启动速度与有网环境一致（不再等待 Claude latest.json 响应）
- 抓包记录中没有对 `deps/claude/latest.json` 的请求
- 抓包记录中仍可能有对 `cc-box/latest.json` 的请求（CC-Box 自身更新检查，保留）

### Claude_Version_StartupLocalVersion_002 — 启动显示本地 Claude 版本

**目标**：验证启动后进入设置 > 更新，Claude CLI 卡片显示本地实际版本号

**前置条件**：本机已安装 Claude CLI（如 `claude --version` 输出 `1.0.33`）

**操作步骤**：
1. 启动 CC-Box
2. 打开 Settings → Update section
3. 查看 Claude CLI 卡片顶部

**预期结果**：
- 卡片顶部显示 `Claude CLI` 与 `v1.0.33`（或当前实际版本）
- 未触发任何加载状态（直接显示版本号）

### Claude_Version_NotInstalled_003 — 未安装时显示「未安装」

**目标**：本机未装 Claude 时，启动后 Claude CLI 卡片显示「未安装」且版本列表正常加载

**前置条件**：本机 PATH 中无 `claude` 命令

**操作步骤**：
1. 启动 CC-Box
2. 打开 Settings → Update section

**预期结果**：
- Claude CLI 卡片顶部显示「未安装」
- 下方历史版本列表正常加载，可下载任意版本
- 不会有「已安装」徽标显示

### Claude_Version_List_004 — 历史版本列表正确展示

**目标**：验证 Claude CLI 卡片能从 OSS 拉取并展示所有历史版本

**前置条件**：能访问 `https://cc-box.oss-cn-beijing.aliyuncs.com/deps/claude/versions.json`

**操作步骤**：
1. 启动 CC-Box，进入 Settings → Update section
2. 等待 Claude CLI 卡片的版本列表加载完成

**预期结果**：
- 列表头部显示「历史版本」标题与版本计数（如「共 N 个版本，最新 v1.0.17」）
- 每行显示版本号、发布日期
- 当前已安装版本行右侧有绿色「已安装」徽标
- 最新版本行右侧有蓝色「最新」徽标
- 列表按版本号降序排列（1.0.17 → 1.0.16 → ...）

### Claude_Version_Refresh_005 — 手动刷新版本列表

**目标**：点击「刷新版本列表」按钮强制重新拉取

**前置条件**：版本列表已加载

**操作步骤**：
1. 点击 Claude CLI 卡片右上角「刷新版本列表」按钮

**预期结果**：
- 按钮显示旋转图标 + 「检查中...」文字
- 列表短暂闪烁后重新加载
- 5 分钟内重复点击会触发实际 HTTP 请求（缓存被强制跳过）

### Claude_Version_Download_006 — 下载历史版本到本地

**目标**：选择某个历史版本下载并自动安装，验证整个流程

**前置条件**：版本列表已加载，有可下载的版本；本机当前未运行 Claude CLI（即所有终端会话已关闭）

**操作步骤**：
1. 选择一个版本（如 v2.1.170），点击右侧「安装」按钮
2. 观察行内进度条
3. 等待下载完成 → 自动进入安装

**预期结果**：
- 按钮变为行内进度条 + 取消按钮，显示百分比
- 进度从 0% 推进到 100%
- 下载完成后短暂显示「正在安装 v2.1.170...」
- 安装完成后按钮变为「在文件夹中显示」
- 卡片顶部「使用中」徽标移到 v2.1.170 这一行
- 用户下载目录中存在 `claude-2.1.170.exe`（Windows）或 `claude-2.1.170`（Unix）
- 文件大小与 OSS 上对应版本一致

### Claude_Version_PlatformMissing_007 — 平台不支持时禁用下载

**目标**：当某版本缺失当前平台产物时，禁用下载按钮并显示提示

**前置条件**：手动改 OSS versions.json，删除某版本的 `win32-x64` 平台条目（仅测试时）

**操作步骤**：
1. 刷新版本列表
2. 查看被改动的版本行

**预期结果**：
- 该版本行的下载按钮变为「当前平台不支持此版本」灰色文字
- 鼠标悬停显示 tooltip
- 其他版本行不受影响

### Claude_Version_VersionsJsonMissing_008 — versions.json 不存在时降级

**目标**：OSS 上 versions.json 被删除时，UI 不崩溃且显示明确提示

**前置条件**：手动删除 OSS `deps/claude/versions.json`（仅测试时）

**操作步骤**：
1. 启动 CC-Box，进入 Update section

**预期结果**：
- 版本列表区域显示「暂无可用版本」灰底提示框
- 卡片不崩溃、其他区域（CC-Box 更新检查）仍正常工作
- 终端功能不受影响

### Claude_Version_DownloadError_009 — 下载失败后可重试

**目标**：网络中断或文件不存在时，显示错误并提供重试

**前置条件**：版本列表已加载

**操作步骤**：
1. 选择一个版本点击下载
2. 下载过程中断网或手动改 OSS 上该版本路径使请求 404

**预期结果**：
- 进度条变为错误状态，显示「下载失败」与「重试」按钮
- 点击「重试」后重新进入下载流程

### Claude_Version_OldCcBoxCompat_010 — 旧版 CC-Box 仍能更新 Claude

**目标**：保证旧版 CC-Box（不读 versions.json）的自动安装 Claude 流程未受影响

**前置条件**：使用改动前的 CC-Box 安装包（v0.11.1 或更早）

**操作步骤**：
1. 安装并启动旧版 CC-Box
2. 进入 Update section，点击 Claude CLI 的「检查更新」/「安装」

**预期结果**：
- 旧版仍能拉取 `deps/claude/latest.json` 并显示最新版本
- 点击「下载并安装」可成功安装最新版 Claude
- 整个流程无错误

### Claude_Version_CancelDownload_012 — 下载过程可取消

**目标**：下载进行中点击「取消」立即终止下载，并清理半成品

**前置条件**：版本列表已加载，下载目录无对应版本的缓存文件

**操作步骤**：
1. 点击某个版本「安装」按钮
2. 进度推进到 30%-60% 之间时，点击进度条旁的「取消」按钮
3. 等待状态切换

**预期结果**：
- 进度条立即消失，按钮恢复为「安装」
- 显示「已取消」灰色文字与「安装」按钮（可重新发起）
- 用户下载目录中无半成品文件（`claude-X.X.X.exe` 不存在）
- 后端日志记录 `Download cancelled`

### Claude_Version_LocalCacheReuse_013 — 本地缓存可复用

**目标**：用户下载目录已有完整文件时，跳过下载直接安装

**前置条件**：之前已成功下载过 v2.1.170，下载目录 `claude-2.1.170.exe` 仍存在且大小匹配

**操作步骤**：
1. 进入 Update section，点击 v2.1.170 行的「安装」按钮
2. 观察状态变化

**预期结果**：
- 不出现下载进度条
- 直接进入「正在安装 v2.1.170...」
- 安装完成后显示「在文件夹中显示」按钮
- 后端日志记录 `Reusing local cache`

### Claude_Version_LocalCacheInvalid_014 — 本地缓存损坏时重新下载

**目标**：本地文件 size 与 OSS 记录不一致时，自动删除并重新下载

**前置条件**：用户下载目录存在 `claude-2.1.170.exe` 但 size 小于实际大小（模拟下载中断）

**操作步骤**：
1. 手动用 `echo > claude-2.1.170.exe` 等方式制造 size 不匹配的文件
2. 点击「安装」按钮

**预期结果**：
- 后端日志记录 `Local file size mismatch, removing`
- 自动删除损坏文件并重新下载
- 流程正常完成

### Claude_Version_ClaudeRunning_015 — Claude 运行时弹窗确认

**目标**：用户在 Claude 终端会话运行时点击安装，提示终止并让用户选择

**前置条件**：在 CC-Box 中打开至少一个 Claude 终端会话（PTY 中运行 claude.exe）

**操作步骤**：
1. 下载完成后，安装阶段会检测到 claude 进程
2. 观察弹窗

**预期结果**：
- 出现确认对话框，标题「检测到 Claude 进程正在运行」
- 提示「继续安装需要终止所有正在运行的 Claude 终端会话」
- 「终止并安装」按钮 + 「取消」按钮
- 点击「取消」后状态回退为「在文件夹中显示」（下载文件保留）
- 点击「终止并安装」后所有 Claude 终端退出 → 文件被覆盖 → 「使用中」徽标更新

### Claude_Version_CancelButtonDuringInstall_016 — 安装阶段无取消按钮

**目标**：确认下载完成后进入安装阶段时不再显示取消按钮（防止部分文件覆盖）

**前置条件**：版本列表已加载，无 Claude 进程运行

**操作步骤**：
1. 点击「安装」
2. 等待下载完成进入安装阶段

**预期结果**：
- 下载阶段：进度条 + 「取消」按钮
- 安装阶段：显示「正在安装...」文字，无取消按钮
- 安装完成后：变为「在文件夹中显示」

### Claude_Version_PublishScript_011 — 发布脚本正确生成 versions.json

**目标**：验证 `npm run download-deps` 执行后，OSS 上 versions.json 被正确更新

**前置条件**：配置好 `scripts/oss-config.json` 与代理

**操作步骤**：
1. 运行 `npm run download-deps`
2. 浏览器访问 `https://cc-box.oss-cn-beijing.aliyuncs.com/deps/claude/versions.json`

**预期结果**：
- 脚本日志输出「versions.json 已更新（共 N 个版本，最新 vX.X.X）」
- 浏览器返回的 JSON 包含 `latest`、`updated_at`、`versions` 三个字段
- `versions` 数组按版本号降序排列
- 当前发布的版本在数组首位，platforms 字段完整
- `latest.json` 仍然存在且内容正确（未被破坏）

---

## 侧边栏 Plugin 子项过滤

### Sidebar_PluginFilter_DisableHidesChildren_001 — 禁用 plugin 后其 skill/agent/mcp 不再展示

**目标**：验证 user-scope plugin 被禁用时，其提供的 skill/agent/mcp 立即从侧边栏对应面板消失

**前置条件**：已安装至少一个 user-scope plugin（如 paper-tool@orczh），且该 plugin 提供 skill / agent / mcp 子项；该 plugin 当前为启用状态

**操作步骤**：
1. 打开 Plugins 面板，记住目标 plugin 提供的 skill / agent / mcp 名称
2. 打开 Skills 面板，确认 Plugin 分组下能看到该 plugin 的 skill
3. 打开 Agents 面板，确认 Plugin 分组下能看到该 plugin 的 agent
4. 打开 MCP 面板，确认 Plugin 分组下能看到该 plugin 的 mcp server
5. 回到 Plugins 面板，点击该 plugin 的 ToggleSwitch 关闭它
6. 不重启、不刷新，依次打开 Skills / Agents / MCP 面板

**预期结果**：
- 步骤 2-4：能看到目标 plugin 的子项
- 步骤 5：plugin ToggleSwitch 立即变为关闭状态（乐观更新），无报错提示
- 步骤 6：三个面板的 Plugin 分组中都不再出现该 plugin 的子项（如果某分组没有其他 plugin 子项，整组隐藏）

### Sidebar_PluginFilter_EnableShowsChildren_001 — 启用 plugin 后其 skill/agent/mcp 重新出现

**目标**：验证 user-scope plugin 从禁用切换为启用后，子项立即回到侧边栏

**前置条件**：目标 plugin 处于禁用状态（步骤同上一用例先关闭它）

**操作步骤**：
1. 在 Plugins 面板点击该 plugin 的 ToggleSwitch 开启它
2. 依次打开 Skills / Agents / MCP 面板

**预期结果**：
- 步骤 1：plugin ToggleSwitch 立即变为开启状态，无报错
- 步骤 2：三个面板的 Plugin 分组重新出现该 plugin 的子项

### Sidebar_PluginFilter_ToggleFailureRollback_001 — plugin toggle 失败时数据与子项展示均回滚

**目标**：验证 toggle API 失败时，plugin.enabled 与子项展示均保持原状

**前置条件**：构造 toggle 失败场景（例如手动停止 claude CLI 后端、或网络异常导致 `claude plugin` 命令失败）

**操作步骤**：
1. 在 Plugins 面板对一个 enabled=true 的 plugin 点击关闭
2. 观察错误反馈

**预期结果**：
- plugin ToggleSwitch 短暂变灰后弹回开启位置（乐观更新被回滚）
- 控制台/Toast 显示错误信息
- Skills / Agents / MCP 面板中该 plugin 的子项仍然展示（未被错误移除）


## Provider 激活不回滚 Claude Code 状态字段

### Provider_Activate_PreservesPluginToggle_001 — 切换/重新激活 Provider 不回滚 plugin 开关

**目标**：验证 activate Provider 走 deep merge + 剥离状态字段，不会用 Provider 快照覆盖用户后续 `claude plugin enable/disable` 的修改

**前置条件**：
- 已安装至少 2 个 Provider（设置 → Providers 列表非空）
- 已安装至少 1 个 Claude Code plugin（侧边栏 Plugins 面板有 user scope 项）
- 关闭其他 cc-box 实例避免干扰

**操作步骤**：
1. 用编辑器打开 `~/.claude/settings.json`，记录当前 `enabledPlugins` 中某个 plugin（如 `user-tool@orczh`）的值
2. 在 cc-box Plugins 面板切换该 plugin 开关（开启↔关闭）
3. 立即查看 `~/.claude/settings.json`，确认 `enabledPlugins` 已变化
4. 进入 Settings → Providers，**保存通用配置**（即使没改动也点保存，触发 `updateCommon` → 重新 activate）
5. 再次查看 `~/.claude/settings.json` 的 `enabledPlugins`
6. 在 Providers 列表切换到另一个 Provider，再切回原 Provider
7. 再次查看 `~/.claude/settings.json` 的 `enabledPlugins`

**预期结果**：
- 步骤 3：`enabledPlugins` 中目标 plugin 的值与 UI 开关一致
- 步骤 5：`enabledPlugins` 中目标 plugin 的值保持步骤 3 的状态（**未被 Provider 快照回滚**）
- 步骤 6/7：`enabledPlugins` 中目标 plugin 的值仍保持步骤 3 的状态

**回归标志**：若步骤 5/6/7 中目标 plugin 的值变回 Provider 创建时的快照值，说明 bug 复现。

### Provider_Activate_MergesEnvFields_001 — 切换 Provider 仍正确覆盖 env/model

**目标**：验证剥离状态字段不影响 Provider 应管的字段（env、model 等）的覆盖语义

**前置条件**：两个 Provider 的 `env.ANTHROPIC_BASE_URL` 不同

**操作步骤**：
1. 激活 Provider A，记录 `~/.claude/settings.json` 的 `env.ANTHROPIC_BASE_URL`
2. 激活 Provider B
3. 查看 `env.ANTHROPIC_BASE_URL`

**预期结果**：
- 步骤 3：`env.ANTHROPIC_BASE_URL` 已切换为 Provider B 的值
- A 独有的 env 子键（如有）保留，不被整体清空

## Undo 快捷键映射

### Undo_Shortcut_BasicUndo_001 — 输入过程中触发 undo 擦除字符

**目标**：验证 macOS `Cmd+Z` / Windows `Ctrl+Z` 在 Claude CLI 输入框中触发 readline 原生 undo（逐字回退）

**前置条件**：已打开项目并启动 Claude CLI 会话，焦点在终端

**操作步骤**：
1. 在 Claude CLI 输入框敲入 `hello world`（不要按 Enter）
2. 按 `Cmd+Z`（macOS）或 `Ctrl+Z`（Windows）
3. 再按一次 `Cmd+Z` / `Ctrl+Z`

**预期结果**：
- 步骤 2：输入框文字按 readline undo 规则回退（通常一次 undo 删除一个"原子输入"，连续敲入的字符可能整段或逐字回退，取决于 readline 行为）
- 步骤 3：进一步回退（多次 undo 累积）
- 全程 Claude CLI 进程**未暂停**（窗口仍能正常交互、Tab 仍可切换、Esc 仍能中断）

### Undo_Shortcut_NoSuspend_001 — Ctrl+Z 不触发 SIGTSTP 暂停进程

**目标**：验证 Win/Linux 上拦截 `Ctrl+Z` 后，不再向 PTY 注入 `\x1a`，Claude 进程不会被挂起

**前置条件**：Windows 或 Linux 系统，已打开项目并启动 Claude CLI 会话

**操作步骤**：
1. 在输入框敲入若干字符
2. 按 `Ctrl+Z`
3. 紧接着按若干普通字符（如 `abc`）
4. 按 `Enter` 提交

**预期结果**：
- 步骤 2 后：Claude 进程仍在前台运行（Hook 状态行不变成 suspended，输入框仍可继续输入）
- 步骤 3：字符正常显示在输入框
- 步骤 4：消息正常发送，Claude 正常响应
- 反证：未做本次修复前，步骤 2 会让进程挂起、步骤 3 字符不显示、步骤 4 无响应

### Undo_Shortcut_ShiftVariant_NotTrigger_001 — Cmd/Ctrl+Shift+Z 不触发 undo

**目标**：验证 Shift 修饰组合不命中 undo（预留给未来 redo）

**前置条件**：已打开项目并启动 Claude CLI 会话

**操作步骤**：
1. 在输入框敲入 `hello`
2. 按 `Cmd+Shift+Z`（macOS）或 `Ctrl+Shift+Z`（Windows）
3. 观察输入框

**预期结果**：
- 输入框内容不被 undo
- `hello` 保持原样（或被 Claude CLI 自己的 `Ctrl+Shift+Z` 行为处理，但不应该是 undo）

### Undo_Shortcut_MultiTabIsolation_001 — 多 Tab 各自独立 undo

**目标**：验证 undo 在多 Tab 间互不干扰

**前置条件**：打开两个 Tab，都启动 Claude CLI

**操作步骤**：
1. Tab A 输入框敲 `aaa`
2. 切换到 Tab B，输入框敲 `bbb`
3. 在 Tab B 按 `Cmd+Z` / `Ctrl+Z`
4. 切回 Tab A，观察 A 的输入框

**预期结果**：
- 步骤 3：Tab B 输入框触发 undo（擦除 `bbb`）
- 步骤 4：Tab A 输入框仍保留 `aaa`（未受 B 的 undo 影响）

## 终端渲染（xterm 6 升级）

> 背景：升级 `@xterm/xterm@6.1.0-beta.303` + `@xterm/addon-webgl@0.20.0-beta.299`，根治 WebGL glyph atlas corruption（[docs/webgl-corruption-fix.md](webgl-corruption-fix.md)），同时适配 xterm 6 滚动条重做与快捷键变化。

### TerminalRender_LongCjkSession_001 — 长中文会话无乱码

**目标**：验证上游 atlas corruption 修复在真实使用场景下生效

**前置条件**：已打开项目并启动 Claude CLI 会话

**操作步骤**：
1. 用中文与 Claude 对话 10 分钟以上（输出包含中文、emoji、代码块混合内容）
2. 滚动查看历史输出（触发 scrollback 重绘）
3. 期间不切换 Tab

**预期结果**：
- 全程无方块（□）、替换字符（�）、字符错位、字形张冠李戴
- 滚动回看历史内容渲染正确

### TerminalRender_MultiTabSwitch_001 — 多 Tab 长时间切换无乱码

**目标**：验证多 Tab 共享 atlas 场景（corruption 原始触发条件）下修复生效

**前置条件**：3 个 Tab 各自启动 Claude CLI 会话

**操作步骤**：
1. 3 个 Tab 分别进行中文对话（各产出 200+ 行含 CJK/emoji 的输出）
2. 来回切换 Tab 持续 10 分钟
3. 观察每次切入的 Tab

**预期结果**：
- 每次切到的 Tab 显示内容正确，无乱码
- 切换瞬间无可见闪烁异常（切 Tab reload 兜底仍会执行，视觉上应无感知）

### TerminalRender_Scrollbar_001 — 滚动条隐藏且滚轮滚动正常

**目标**：验证 xterm 6 自绘滚动条已隐藏（滚动由 Claude CLI 内部实现），且滚轮回看 scrollback 不受影响

**前置条件**：已打开项目并启动 Claude CLI 会话，终端有足够多的输出

**操作步骤**：
1. 观察终端右侧边缘
2. 鼠标悬停在终端右侧、滚动输出（触发滚动条显示条件）
3. 用鼠标滚轮向上滚动查看历史输出，再向下滚回底部

**预期结果**：
- 全程右侧无滚动条（无滑块、无轨道）
- 滚轮滚动 scrollback 正常，历史输出可回看、可滚回
- Claude CLI 内部滚轮行为（如 TUI 内列表滚动）正常

### TerminalRender_ViewportGap_001 — 底部无缝隙黑条

**目标**：验证 viewport 黑色兜底背景已被主题色覆盖（容器高度与行高不整除时不再露黑）

**前置条件**：已打开项目并启动 Claude CLI 会话

**操作步骤**：
1. 暗色主题下调整窗口高度（连续小幅 resize，制造行高不整除的容器高度）
2. 观察终端底部边缘
3. 切换到浅色主题，重复 resize 观察

**预期结果**：
- 终端底部与容器边缘之间无黑色长条
- 露出的缝隙（如存在）与终端背景色一致，肉眼无色差

### TerminalRender_AltArrow_001 — Alt+方向键行为

**目标**：验证 xterm 6 移除 alt→ctrl+方向键默认映射（#5346）后输入行为正常

**前置条件**：已启动 Claude CLI 会话，焦点在终端

**操作步骤**：
1. 在输入框敲入多个英文单词
2. 按 `Alt+←` / `Alt+→`
3. 按 `Ctrl+←` / `Ctrl+→` 对比

**预期结果**：
- `Ctrl+←/→` 按词跳转正常（CLI 自身行为）
- `Alt+←/→` 行为与升级前一致（如 CLI 无绑定则无反应），不出现异常字符注入


