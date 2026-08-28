<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="CC-Box" width="80" height="80">
</p>

<h1 align="center">CC-Box</h1>

<p align="center">
  <strong>A desktop app for <a href="https://docs.anthropic.com/en/docs/claude-code">Claude Code</a> — multi-project, multi-session management</strong><br>
  One window. Multiple projects. Instant session switching.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue" alt="Platform">
  <img src="https://img.shields.io/badge/Tauri-2.x-orange" alt="Tauri">
  <img src="https://img.shields.io/badge/Vue-3-green" alt="Vue">
  <img src="https://img.shields.io/badge/License-MIT-yellow" alt="License">
</p>

---

English | [简体中文](README_CN.md)

---

## Why CC-Box?

Claude Code's CLI is excellent for single-session work. But when you're managing **multiple projects** and need to **view, enter, and switch between sessions quickly** — the terminal alone becomes cumbersome.

CC-Box is essentially a **desktop application for Claude Code**. It wraps the CLI with a native terminal experience and adds the things the CLI can't do well: multi-project management, session overview, and quick switching.

**Think of it as a desktop app purpose-built for Claude Code power users.**

---

## Screenshots

<p align="center">
  <img src="screenshots/projectselect.png" alt="Project Selection" width="400">
  <img src="screenshots/project.png" alt="Session Management" width="400">
</p>

---

## Highlights

### Multi-Project Management

Browse all your projects in one place. See which projects have active sessions, launch a new session with one click, and switch between projects instantly. No more `cd` between directories or managing multiple terminal windows.

### Multi-Session in One Window

Open as many Claude Code sessions as you need — each runs independently in its own terminal tab. View all sessions in the sidebar, switch between them instantly, output is preserved when you switch back.

### Quick Launch with Presets

Set per-project startup options like `--resume`, `--model`, or custom flags. Launch sessions with your preferred configuration without typing the same arguments every time.

### Provider Management

Switch between API providers with one click. 50+ built-in presets for popular vendors, import existing configs from [cc-switch](https://github.com/farion1231/cc-switch), and keep shared settings in a common config applied to every provider. Activation merges safely into Claude Code's `settings.json` — unrelated fields are never touched.

### Sidebar Panels

A side drawer with contextual panels — no overlay, no focus stealing:

- **Sessions** — Browse, search, and switch between all sessions. Status indicators show running/thinking/waiting states.
- **MCP Servers** — Inspect connected MCP servers, browse available tools and their input schemas
- **Skills & Agents** — Quick access to your Claude Code skills and agent configurations
- **Plugins** — View installed plugins and their components

### Native Terminal, Zero Compromise

The app runs the real Claude CLI binary through a pseudo-terminal. Everything works exactly as in your terminal — slash commands, keyboard shortcuts, streaming output, colors, and interactive prompts.

---

## Prerequisites

- **[Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code)** installed and authenticated
- **Windows only**: [Git for Windows](https://git-scm.com/download/win) (provides Git Bash)

---

## Quick Start

### 1. Download & Install

Head to the [**Releases**](https://github.com/orczh-hj/cc-box/releases) page and grab the installer for your platform:

| Platform | File |
|----------|------|
| **Windows** | `.exe` (NSIS installer) or `.msi` |
| **macOS** | `.dmg` (universal binary) |
| **Linux** | `.deb` or `.AppImage` |

Users in China can download from [Gitee Releases](https://gitee.com/orczh/cc-box/releases).

### 2. Launch & Go

1. Open the app
2. Select or add a project directory
3. A Claude Code session starts — just type as you would in the terminal
4. Open more sessions from the sidebar, each runs independently

---

## Building from Source

<details>
<summary>Click to expand</summary>

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) stable toolchain
- [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) installed and authenticated
- **Windows only**: [Git for Windows](https://git-scm.com/download/win)

### Setup

```bash
git clone https://github.com/orczh-hj/cc-box.git
cd cc-box
npm install
```

### Development

```bash
npm run tauri:dev     # Start dev mode with hot reload
```

### Build

```bash
npm run tauri:build   # Build for current platform

# Or platform-specific:
npm run build:win     # Windows (x86_64-pc-windows-gnu)
npm run build:mac     # macOS (universal)
npm run build:linux   # Linux (x86_64)
```

Output goes to `src-tauri/target/release/bundle/`.

</details>

---

## FAQ

<details>
<summary><strong>Does this modify my Claude Code config?</strong></summary>

The app only **reads** native Claude Code files, with one exception: activating a provider merges its environment config into `settings.json` (unrelated fields are never touched, and GUI state fields are stripped). All GUI-specific settings are stored separately in `~/.cc-box/`. You can go back to the CLI at any time.
</details>

<details>
<summary><strong>Can I use all CLI features?</strong></summary>

Yes. Slash commands, keyboard shortcuts, model switching, permission prompts — everything passes through to the real CLI transparently.
</details>

<details>
<summary><strong>What's the performance like?</strong></summary>

Built with Tauri 2 (Rust backend), the app is ~10 MB installed and uses minimal RAM. The terminal renders via xterm.js, matching native terminal performance.
</details>

<details>
<summary><strong>Will it break when Claude Code updates?</strong></summary>

The app runs the CLI binary directly — it doesn't depend on any internal API. As long as the CLI is on your PATH, it works with any version.
</details>

---

## Tech Stack

Tauri 2 (Rust) + Vue 3 + TypeScript + xterm.js + portable-pty

---

## License

[MIT](LICENSE)