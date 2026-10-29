<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="CC-Box" width="80" height="80">
</p>

<h1 align="center">CC-Box</h1>

<p align="center">
  <strong>Claude Code 桌面应用 — 多项目、多会话管理，手机远程连接</strong><br>
  一个窗口。多个项目。快速切换会话。手机随时接管。
</p>

<p align="center">
  <img src="https://img.shields.io/badge/平台-Windows%20%7C%20macOS%20%7C%20Linux-blue" alt="Platform">
  <img src="https://img.shields.io/badge/Tauri-2.x-orange" alt="Tauri">
  <img src="https://img.shields.io/badge/Vue-3-green" alt="Vue">
  <img src="https://img.shields.io/badge/许可证-MIT-yellow" alt="License">
</p>

---

[English](README.md) | 简体中文

---

## 为什么选择 CC-Box？

Claude Code 的 CLI 在单会话工作中表现优秀。但当你需要管理**多个项目**，并希望**便捷地查看、进入、切换会话**时，纯终端就显得力不从心。

CC-Box 本质上是 **Claude Code 的桌面应用**。它保留了原生终端体验，并添加了 CLI 不擅长做的事：多项目管理、会话总览、快速切换——再加上手机远程完全接管，桌面会话随身携带。

**把它看作专为 Claude Code 重度用户打造的桌面应用。**

---

## 界面截图

<p align="center">
  <img src="screenshots/project-page.png" alt="项目与近期会话" width="400">
  <img src="screenshots/terminal-sessions.png" alt="多会话终端" width="400">
</p>

---

## 核心功能

### 多项目管理

在一个窗口中浏览所有项目。查看哪些项目有活跃会话，一键启动新会话，即时切换项目。不再需要频繁 `cd` 切换目录或管理多个终端窗口。

### 多会话并行

打开任意数量的 Claude Code 会话，每个会话在独立的终端标签页中运行。在侧边栏查看所有会话，快速切换，切换后输出内容保持不变。

### 快速启动与预设

为每个项目设置启动选项，如 `--resume`、`--model` 或自定义参数。无需每次输入相同参数，一键以预设配置启动会话。

### Provider 管理

一键切换 API Provider。内置 50+ 厂商预设模板，支持从 [cc-switch](https://github.com/farion1231/cc-switch) 导入现有配置；通用配置跨 Provider 共享，激活时以合并方式写入 Claude Code 的 `settings.json`，不覆盖无关字段。

### 侧边栏面板

非遮罩式侧边栏，不会抢占焦点：

- **会话** — 浏览、搜索、切换所有会话。状态指示灯显示运行/思考/等待状态。
- **MCP 服务器** — 查看已连接的 MCP 服务器、可用工具及其参数结构，可一键开关
- **Skills & Agents** — 浏览并开关 Claude Code skills 和 agent 配置
- **插件** — 查看已安装的插件及其组件，可一键开关

### 手机远程连接

扫码配对一次，之后用任意手机浏览器打开桌面会话——在家走局域网直连，在外经自建中继穿透公网（桌面零入站端口）。

- **完全的会话控制** — 查看终端、输入、审批、新建/关闭/重命名会话，与坐在电脑前操作一致
- **会话级控制权** — 多设备可同时操作不同会话；被接管会有明确提示，绝不静默踢人
- **双向文件传输** — 手机照片/日志直接上传进项目（自动在终端插入路径），项目文件经一次性票据直链下载，微信、系统浏览器都能用
- **设备列表连通性探测** — 点之前就知道在线/离线/令牌失效；不可达的设备会说明原因
- **默认安全** — 远程功能默认关闭；每台设备独立配对，可一键吊销（「刷新密钥」立即解绑所有设备）

### 托盘、多窗口与自动更新

CC-Box 常驻后台 Broker 并带托盘图标。每个窗口都是托盘一级菜单项，单击即达；手机端会话按窗口分组，零窗口时还能远程拉起新窗口。支持开机自启动（仅 Broker），应用内自动更新，新版本在托盘亮红点提醒。

### 会话状态实时同步

CLI hook 事件实时汇入状态机——运行中/思考中/等待输入——显示在标签页与侧边栏，并同步到手机。有新动态的会话带未读标记，打开即在全设备清除。首页可隐藏不想看到的项目（重新添加会恢复其最近会话）。

### 原生终端体验

通过伪终端直接运行 Claude CLI 二进制文件。所有功能与终端中完全一致 — slash 命令、快捷键、流式输出、颜色、交互式提示。

---

## 先决条件

- **[Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code)** 已安装并完成认证
- **Windows 用户**: [Git for Windows](https://git-scm.com/download/win)（提供 Git Bash）

---

## 快速开始

### 1. 下载安装

前往 [**Releases**](https://github.com/orczh-hj/cc-box/releases) 页面下载对应平台的安装包：

| 平台 | 文件 |
|------|------|
| **Windows** | `.exe` (NSIS 安装包) 或 `.msi` |
| **macOS** | `.dmg` (通用二进制) |
| **Linux** | `.deb` 或 `.AppImage` |

国内用户可从 [Gitee Releases](https://gitee.com/orczh/cc-box/releases) 下载。

### 2. 启动使用

1. 打开应用
2. 选择或添加项目目录
3. Claude Code 会话启动 — 像在终端中一样输入
4. 从侧边栏打开更多会话，每个独立运行

---

## 从源码构建

<details>
<summary>点击展开</summary>

### 前置要求

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) stable 工具链
- [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) 已安装并认证
- **Windows 用户**: [Git for Windows](https://git-scm.com/download/win)

### 安装

```bash
git clone https://github.com/orczh-hj/cc-box.git
cd cc-box
npm install
```

### 开发

```bash
npm run tauri:dev     # 启动开发模式（热重载）
```

### 构建

```bash
npm run tauri:build   # 构建当前平台

# 或指定平台：
npm run build:win     # Windows (x86_64-pc-windows-gnu)
npm run build:mac     # macOS (当前架构；线上更新目前仅发布 Apple Silicon/arm64)
npm run build:linux   # Linux (x86_64)
```

构建产物在 `src-tauri/target/release/bundle/`。

</details>

---

## 常见问题

<details>
<summary><strong>会修改我的 Claude Code 配置吗？</strong></summary>

应用对 Claude Code 原生文件**只读**，仅有一个例外：激活 Provider 时会以合并方式将其环境配置写入 `settings.json`（不触碰无关字段，且剥离 GUI 状态字段）。所有 GUI 设置独立保存在 `~/.cc-box/` 中。你可以随时回到纯 CLI 使用。
</details>

<details>
<summary><strong>能用所有 CLI 功能吗？</strong></summary>

可以。Slash 命令、快捷键、模型切换、权限提示 — 所有功能透明传递给真实 CLI。
</details>

<details>
<summary><strong>性能如何？</strong></summary>

基于 Tauri 2 (Rust 后端)，安装后约 10 MB，内存占用极低。终端通过 xterm.js 渲染，性能与原生终端相当。
</details>

<details>
<summary><strong>Claude Code 更新后会失效吗？</strong></summary>

应用直接运行 CLI 二进制文件，不依赖任何内部 API。只要 CLI 在 PATH 中，任何版本都能正常工作。
</details>

<details>
<summary><strong>远程连接安全吗？</strong></summary>

远程功能**默认关闭**。每台设备通过一次性二维码配对、持有独立令牌；「刷新密钥」可立即吊销所有已配对设备。局域网模式流量不出内网；中继模式走你自己部署的中继，只路由 TLS 流量、不解析应用内容。文件下载使用 10 分钟过期的一次性票据。所有远程操作有审计日志，存于 `~/.cc-box/logs`。
</details>

---

## 技术栈

Tauri 2 (Rust) + Vue 3 + TypeScript + xterm.js + portable-pty

---

## 许可证

[MIT](LICENSE)