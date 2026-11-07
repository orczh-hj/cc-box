# 手机端三模式键盘（快捷键工具条 + 虚拟键盘弹层）设计

日期：2026-11-07 · 状态：已确认（方案 B）

## 意图

手机远程操作电脑时，软键盘按不出控制键。向日葵/ToDesk 等给三模式键盘（手机键盘 / 快捷键 / 模拟电脑键盘），CC-Box 当前只有第一种 + 简易快捷键条。本设计补齐：**快捷键面板增强** + **完整虚拟键盘弹层（粘滞修饰键）**，让手机端能完成 Claude CLI 的全部键盘操作（Esc 打断、Ctrl+C、方向键翻历史、Shift+Tab 切模式、F 键等）。

调研结论：无开源 web 终端虚拟键盘组件可直接复用（noVNC extra keys / Guacamole OnScreenKeyboard / RustDesk 键盘菜单仅作交互参考）；CC-Box 下游是 PTY 只吃终端转义序列，不需要系统级按键注入，**纯前端即可，协议/Rust 零改动**。

## 方案选择（B，渐进式）

- **A（向日葵式菜单）**：键盘图标弹三选一菜单，工具条变简洁 —— 放弃：破坏现有肌肉记忆，高频快捷键多一次点击
- **B（渐进式，已选）**：QuickKeyBar 保持常驻（本身即快捷键面板，增补键位）；工具条加「⌨+」图标单独唤出完整虚拟键盘弹层。完整键盘是低频兜底，放弹层

## 目标

1. QuickKeyBar 增补：`Ctrl+C`、`Ctrl+D`、`Tab`、`Delete`（横滚布局已支持）
2. 新组件 `VirtualKeyboard.vue`：底部完整键盘面板（**文档流内**、排在 QuickKeyBar 后——终端区随面板出现自动收缩 refit，不 fixed 遮挡输入行；终审修正）
   - 数字排 + 三排 QWERTY + 底排（粘滞 Ctrl / Alt / Shift、空格、回车、退格）
   - 顶排功能键：Esc / Tab / F1-F12（紧凑）
   - 侧列：↑↓←→ + Home/End/PgUp/PgDn
3. **粘滞修饰键**：点 Ctrl/Alt/Shift 高亮，下一键组合后自动释放（再点同键取消；换点另一修饰键可叠加 Ctrl+Shift 等）
4. 弹层内按键直发 ANSI 序列（经 `terminalRef.sendText`，沿用其 isMobile 不聚焦守卫）

## 非目标

- 不做系统级按键注入（Alt+Tab / Win 键等 OS 级组合——PTY 场景无意义且做不了）
- 不做软键盘（模式 1）改动——现状已好
- 不做键盘布局自定义/设置项
- 桌面端不渲染（沿用 `isMobile` 分支）

## 核心实现

### 键码映射模块（唯一核心数据）

`src/utils/virtual-keys.ts` 纯函数：`keySpec + activeModifiers → ANSI bytes string`。

- Ctrl+字母 → 控制字符（0x01-0x1A）；Ctrl+方向 → `\x1b[1;5A` 等
- Alt+键 → `\x1b` + 键序列（ESC 前缀）
- Shift+字母 → 大写；Shift+Tab → `\x1b[Z`
- F1-F12 → `\x1bOP/OQ/OR/OS`、`\x1b[15~` 等 xterm 序列
- 方向/Home/End/PgUp/PgDn/Del/Backspace → 标准 xterm 序列
- 无映射组合返回 null（按钮置灰或忽略）

### UI

- `VirtualKeyboard.vue`：fixed 底部弹层，`--safe-area-bottom` 适配；打开时收起软键盘（避免双重键盘挤压视口），关闭时恢复；`mousedown.prevent` 同 QuickKeyBar（不偷焦点）
- QuickKeyBar 加 `action: 'vkeyboard'` 按钮与 `vkeyboardOpen` 常亮态（同现有 keyboard 按钮模式）
- TerminalView 接线：`<VirtualKeyboard v-if="isMobile && vkOpen" @send="handleQuickKey">`，复用 `handleQuickKey → sendText`

### 测试（`tests/utils/virtual-keys.test.ts`）

映射表纯函数全覆盖：控制字符区间、修饰键组合（Ctrl/Ctrl+Shift/Alt 前缀）、F 键序列、方向键带修饰、Shift+Tab、无映射组合返回 null。期望值来自 xterm/VT100 转义序列规范（独立真值来源）。

## 留给实现的决定

- 弹层具体键位排布与尺寸（按 375px 宽度基准一排 10 键自适应）
- 弹层开合动画（简单 slide-up，CSS transition 即可）
