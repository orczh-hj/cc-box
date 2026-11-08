# 手机端虚拟键盘（快捷键工具条增强 + 完整键盘弹层）Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 手机远程端补齐键盘能力——QuickKeyBar 增补 Tab/Delete，新增完整虚拟键盘弹层（粘滞修饰键），按键直发 ANSI 转义序列到 PTY。

**Architecture:** 纯前端三层：`virtual-keys.ts` 纯映射模块（keySpec+修饰键 → ANSI bytes，唯一核心数据、可测）→ `VirtualKeyboard.vue` 弹层组件（消费映射、粘滞修饰键状态机）→ `TerminalView.vue` 接线（复用 `handleQuickKey → sendText`）。协议/Rust 零改动。

**Tech Stack:** Vue 3 + TypeScript + Vitest + @vue/test-utils，沿用 QuickKeyBar 的 pointer/mousedown.prevent 焦点保持模式。

**Spec:** `docs/superpowers/specs/2026-11-07-mobile-virtual-keyboard-design.md`

## Global Constraints

- 全部改动仅前端（src/、tests/），不碰 Rust/协议/i18n 大括号转义规则（新增文案不含 `{`）
- **工具条不放 Ctrl+C / Ctrl+D 直发键**（历史用户决策「^C 已按用户决策移除」：误触打断/退出 CLI；这些组合经虚拟键盘粘滞 Ctrl 达成）
- 虚拟键盘仅 `isMobile` 渲染；按键经 `terminalRef.sendText`（内部已有 isMobile 不聚焦守卫）
- 弹层与工具条容器一律 `@mousedown.prevent`（不偷终端 textarea 焦点、不召唤软键盘）
- 组件测试期望值必须是**字面 ANSI 序列**（来自 VT100/xterm 规范），禁止调用 `vkeyToBytes` 计算期望值（自闭环）
- i18n 新键加 `src/i18n/locales/zh.ts` 与 `en.ts` 两处；测试命名 `Feature_SubFeature_SeqNum` + 中文注释
- Rust 结构体规则等后端约束不涉及（零后端改动）

## Review Focus

实现后最可能咬人的五类输入/状态（每条已加入 owning task 的测试）：

1. **粘滞 Ctrl 后点修饰键自身/空格**（`Ctrl` 亮着再点 `Ctrl` 或 `space`）：应取消粘滞 / 发送 `\x00` 还是原文？——`Ctrl+Space` 无 readline 标准，返回 null 忽略且粘滞释放（Task 3 测试）
2. **多修饰键叠加**（Ctrl+Shift+Tab = `\x1b[1;5Z`… 参数化序列 code 计算）：修饰参数错位是最高概率 bug（Task 1 全组合测试）
3. **Shift+数字排符号**（Shift+1 应发 `!` 而非 `1`）：符号表缺失会发错字符（Task 1 符号表测试）
4. **Alt 前缀对多字节序列键**（Alt+↑ 应发 `\x1b\x1b[A` 而非 `\x1b[A`）：ESC 前缀要加在完整序列外（Task 1 测试）
5. **弹层打开时软键盘状态**（弹层叠在软键盘上会双重挤压视口）：打开弹层必须先 blur 软键盘、关闭弹层不自动回弹（Task 4 手动条目 + TerminalView 行为）

---

### Task 1: `virtual-keys.ts` 键码映射纯模块

**Files:**
- Create: `src/utils/virtual-keys.ts`
- Test: `tests/utils/virtual-keys.test.ts`

**Interfaces:**
- Produces: `interface VirtualKeyEvent { key: string; ctrl?: boolean; alt?: boolean; shift?: boolean }`（key 为规范键名：`'a'`-`'z'`、`'0'`-`'9'`、`'space' 'enter' 'tab' 'esc' 'backspace' 'delete' 'up' 'down' 'left' 'right' 'home' 'end' 'pageup' 'pagedown' 'f1'`-`'f12'`、符号 `'-','=','[',']',';','\'','`',',','.','/','\\'`）
- Produces: `function vkeyToBytes(e: VirtualKeyEvent): string | null`（null = 该组合无标准映射，调用方忽略且释放粘滞）

- [ ] **Step 1: 写失败测试**（期望值全部为规范字面量）

```ts
// tests/utils/virtual-keys.test.ts —— 断言值来自 xterm/VT100 转义序列规范
import { describe, it, expect } from 'vitest'
import { vkeyToBytes } from '@/utils/virtual-keys'

describe('vkeyToBytes（虚拟键 → ANSI 转义序列）', () => {
  // 无修饰基础键
  it('vkeyToBytes_PlainKeys_001', () => {
    expect(vkeyToBytes({ key: 'a' })).toBe('a')
    expect(vkeyToBytes({ key: 'space' })).toBe(' ')
    expect(vkeyToBytes({ key: 'enter' })).toBe('\r')
    expect(vkeyToBytes({ key: 'tab' })).toBe('\t')
    expect(vkeyToBytes({ key: 'esc' })).toBe('\x1b')
    expect(vkeyToBytes({ key: 'backspace' })).toBe('\x7f')
    expect(vkeyToBytes({ key: 'delete' })).toBe('\x1b[3~')
    expect(vkeyToBytes({ key: 'up' })).toBe('\x1b[A')
    expect(vkeyToBytes({ key: 'down' })).toBe('\x1b[B')
    expect(vkeyToBytes({ key: 'right' })).toBe('\x1b[C')
    expect(vkeyToBytes({ key: 'left' })).toBe('\x1b[D')
    expect(vkeyToBytes({ key: 'home' })).toBe('\x1b[H')
    expect(vkeyToBytes({ key: 'end' })).toBe('\x1b[F')
    expect(vkeyToBytes({ key: 'pageup' })).toBe('\x1b[5~')
    expect(vkeyToBytes({ key: 'pagedown' })).toBe('\x1b[6~')
  })

  // Shift：字母大写、数字排符号表、Shift+Tab 反向 Tab
  it('vkeyToBytes_ShiftKeys_002', () => {
    expect(vkeyToBytes({ key: 'a', shift: true })).toBe('A')
    expect(vkeyToBytes({ key: '1', shift: true })).toBe('!')
    expect(vkeyToBytes({ key: '2', shift: true })).toBe('@')
    expect(vkeyToBytes({ key: 'tab', shift: true })).toBe('\x1b[Z')
  })

  // Ctrl+字母 → 控制字符 0x01-0x1A（readline/CLI 核心组合）
  it('vkeyToBytes_CtrlLetters_003', () => {
    expect(vkeyToBytes({ key: 'c', ctrl: true })).toBe('\x03')
    expect(vkeyToBytes({ key: 'd', ctrl: true })).toBe('\x04')
    expect(vkeyToBytes({ key: 'u', ctrl: true })).toBe('\x15')
    expect(vkeyToBytes({ key: 'a', ctrl: true })).toBe('\x01')
    expect(vkeyToBytes({ key: 'e', ctrl: true })).toBe('\x05')
    expect(vkeyToBytes({ key: 'l', ctrl: true })).toBe('\x0c')
    expect(vkeyToBytes({ key: 'z', ctrl: true })).toBe('\x1a')
  })

  // 方向/功能键 + 修饰 → 参数化序列（code = 1+shift+alt*2+ctrl*4）
  it('vkeyToBytes_ModifiedSequences_004', () => {
    expect(vkeyToBytes({ key: 'up', ctrl: true })).toBe('\x1b[1;5A')      // 5=ctrl
    expect(vkeyToBytes({ key: 'left', shift: true, ctrl: true })).toBe('\x1b[1;6D') // 6=ctrl+shift
    expect(vkeyToBytes({ key: 'tab', ctrl: true })).toBe('\x1b[1;5I') === undefined ?? true // 见下：Tab 特例
  })

  // F1-F12（F1-F4 = SS3，F5-F12 = CSI ~ 序列）
  it('vkeyToBytes_FunctionKeys_005', () => {
    expect(vkeyToBytes({ key: 'f1' })).toBe('\x1bOP')
    expect(vkeyToBytes({ key: 'f4' })).toBe('\x1bOS')
    expect(vkeyToBytes({ key: 'f5' })).toBe('\x1b[15~')
    expect(vkeyToBytes({ key: 'f12' })).toBe('\x1b[24~')
  })

  // Alt 前缀：ESC + 完整基础序列（多字节序列键前缀在外层）
  it('vkeyToBytes_AltPrefix_006', () => {
    expect(vkeyToBytes({ key: 'b', alt: true })).toBe('\x1bb')
    expect(vkeyToBytes({ key: 'up', alt: true })).toBe('\x1b\x1b[A')
    expect(vkeyToBytes({ key: 'backspace', alt: true })).toBe('\x1b\x7f')
  })

  // 无标准映射的组合 → null（调用方忽略并释放粘滞）
  it('vkeyToBytes_UnmappedCombos_Null_007', () => {
    expect(vkeyToBytes({ key: 'space', ctrl: true })).toBe(null)
    expect(vkeyToBytes({ key: 'f1', ctrl: true })).toBe(null)
    expect(vkeyToBytes({ key: '1', ctrl: true })).toBe(null)
  })
})
```

注意 Step 1 中 `_004` 的 Tab 断言是占位错误示例——实际写测试时 Ctrl+Tab 按 xterm 标准应为 `\x1b[9;5I`（CSI 9;5I）或直接 null；**实现取 null**（readline 无 Ctrl+Tab 消费者），把该断言改为 `expect(vkeyToBytes({ key: 'tab', ctrl: true })).toBe(null)`，同时在 `_007` 补充。

- [ ] **Step 2: 跑测试确认失败**

Run: `npx vitest run tests/utils/virtual-keys.test.ts`
Expected: FAIL（模块不存在）

- [ ] **Step 3: 实现 `src/utils/virtual-keys.ts`**

结构：`const SHIFT_SYMBOL: Record<string,string>`（数字排+符号排 shift 表）、`const SEQ: Record<string,string>`（基础多字节序列）、`const FN: Record<string,string>`（F1-F12）、控制字符区间 `ctrl && /^[a-z]$/ → String.fromCharCode(key.charCodeAt(0)-96)`、参数化修饰 `modCode = 1+(shift?1:0)+(alt?2:0)+(ctrl?4:0)` 把 `SEQ` 中 `\x1b[A` 形态改写为 `\x1b[1;<code>A`（仅方向/Home/End；`~` 形态改为 `\x1b[<n>;<code>~`）；Alt 在最终结果外层加 `\x1b`；无映射返回 null。文件头注释注明序列依据（xterm ctlseqs）与消费方（VirtualKeyboard / PTY）。

- [ ] **Step 4: 跑测试确认通过**

Run: `npx vitest run tests/utils/virtual-keys.test.ts`
Expected: PASS 全绿

- [ ] **Step 5: Commit**

```bash
git add src/utils/virtual-keys.ts tests/utils/virtual-keys.test.ts
git commit -m "feat(mobile): 虚拟键→ANSI 转义序列纯映射模块 vkeyToBytes"
```

---

### Task 2: QuickKeyBar 增补 Tab/Delete 键与虚拟键盘唤出按钮

**Files:**
- Modify: `src/components/QuickKeyBar.vue`
- Modify: `tests/components/quickKeyBar.test.ts`
- Modify: `src/i18n/locales/zh.ts`、`src/i18n/locales/en.ts`（新键 `quickKeyVkb`、`quickKeyVkbHint`）

**Interfaces:**
- Consumes: 无（纯 UI）
- Produces: prop `vkeyboardOpen?: boolean`；emit `toggleVkeyboard: []`；`QuickKey.action` 新值 `'vkeyboard'`（图标按钮，位于「键盘」开关之后第二位）

- [ ] **Step 1: 更新组件测试（失败）**

`tests/components/quickKeyBar.test.ts` 修改点：
- `EXPECT` 表增加 `'Tab': '\t'`（插在 ⇧Tab 前）、`'Del': '\x1b[3~'`（End 后）
- `_001` 键序断言改为 `['', '', 'quickKeySelectCopy', ...Object.keys(EXPECT)]`（前两位图标按钮：键盘开关、虚拟键盘）
- 新增 `QuickKeyBar_虚拟键盘按钮_007`：第二位按钮含 svg、点击 emit `toggleVkeyboard` 不 emit send；`vkeyboardOpen: true` 时含 `qk-open` class

- [ ] **Step 2: 跑测试确认失败**

Run: `npx vitest run tests/components/quickKeyBar.test.ts`
Expected: FAIL（新键/新按钮缺失）

- [ ] **Step 3: 实现 QuickKeyBar 修改**

`keys` 数组：keyboard 项后插入 `{ key: 'vkb', action: 'vkeyboard', icon: 'keyboard' }`（新 SVG 可复用 keyboard 图标或加角标 +，label 用 `t('quickKeyVkb')` 作 aria-label）；`⇧Tab` 前插 `{ key: 'tab', label: 'Tab', bytes: '\t' }`；`end` 后插 `{ key: 'del', label: 'Del', bytes: '\x1b[3~' }`。template click 分支与 `qk-open` class 条件加 `k.action === 'vkeyboard' && vkeyboardOpen`。props/emits 补 `vkeyboardOpen` / `toggleVkeyboard`。zh/en 加文案（无大括号）。

- [ ] **Step 4: 跑测试确认通过**

Run: `npx vitest run tests/components/quickKeyBar.test.ts`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/components/QuickKeyBar.vue tests/components/quickKeyBar.test.ts src/i18n/locales/zh.ts src/i18n/locales/en.ts
git commit -m "feat(mobile): QuickKeyBar 增补 Tab/Del 键与虚拟键盘唤出按钮"
```

---

### Task 3: `VirtualKeyboard.vue` 完整键盘弹层组件

**Files:**
- Create: `src/components/VirtualKeyboard.vue`
- Test: `tests/components/virtualKeyboard.test.ts`

**Interfaces:**
- Consumes: `vkeyToBytes(e: VirtualKeyEvent): string | null`（Task 1）
- Produces: 组件 `VirtualKeyboard.vue`，props 无；emits `send: [bytes: string]`、`close: []`

**组件结构**（实现者按此布局，样式细节自定）：
- 顶排：Esc / Tab / F1-F12（横向滚动）
- 主区左侧功能列：↑←↓→（2×2）/ Home End PgUp PgDn / Del Backspace
- 主区 QWERTY：数字排 + 三排字母 + 底排（粘滞 Ctrl/Alt/Shift、空格、Enter、Backspace 若未在侧列）
- 头部右侧「收起」按钮 → emit close
- 粘滞修饰键状态机：点击 Ctrl/Alt/Shift 切换粘滞（可叠加）；点普通键 → 组 VirtualKeyEvent 调 `vkeyToBytes`，非 null 则 emit send，随后清空全部粘滞；返回 null 同样清空（静默忽略）；修饰键自身不 emit
- 容器 `@mousedown.prevent`（同 QuickKeyBar 焦点保持）

- [ ] **Step 1: 写失败测试**（期望值字面 ANSI，禁止用 vkeyToBytes 算）

```ts
// tests/components/virtualKeyboard.test.ts
import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import VirtualKeyboard from '@/components/VirtualKeyboard.vue'

// 按 aria-label 定位按键（字母键 aria-label 即字母，修饰键为 Ctrl/Alt/Shift）
function key(wrapper: ReturnType<typeof mount>, label: string) {
  const btn = wrapper.findAll('button').find(b => b.attributes('aria-label') === label)
  expect(btn, `key ${label}`).toBeDefined()
  return btn!
}

describe('VirtualKeyboard（手机完整虚拟键盘弹层）', () => {
  // 布局：QWERTY 字母齐全 + 功能键齐全（spot check）
  it('VirtualKeyboard_RendersLayout_001', () => { /* 断言 q/w/e/r/t/y 与 f1/f12/esc/tab/up/down/del 存在 */ })

  // 普通键直发：点 c 发 'c'，点 enter 发 '\r'，点 up 发 '\x1b[A'
  it('VirtualKeyboard_PlainKeySends_002', async () => { /* 三组字面量断言 emitted('send') */ })

  // 粘滞 Ctrl：点 Ctrl（高亮）→ 点 c → emit '\x03'，粘滞自动清（再点 a 发 'a' 非 \x01）
  it('VirtualKeyboard_StickyCtrl_003', async () => { /* 断言 \x03 后接 'a'；Ctrl 按钮 sticky class 出现又消失 */ })

  // 修饰叠加：Ctrl+Shift 粘滞 → Tab → '\x1b[Z'（shift+tab；ctrl 参与 null 路径则此处改断言 null 忽略——按 Task1 决策 tab+ctrl=null，则用 Shift 粘滞 + Tab）
  it('VirtualKeyboard_StickyShiftTab_004', async () => { /* Shift 粘滞 → Tab → '\x1b[Z'，随后 c 发 'C' */ })

  // 无映射组合：Ctrl 粘滞 → space → 不 emit send 且粘滞已清（再点 a 发 'a'）
  it('VirtualKeyboard_UnmappedReleasesSticky_005', async () => {})

  // 再点同一修饰键取消粘滞；「收起」emit close 不 emit send
  it('VirtualKeyboard_ToggleStickyAndClose_006', async () => {})

  // mousedown 被阻止（不偷终端焦点）
  it('VirtualKeyboard_MousedownPrevented_007', async () => { /* 同 quickKeyBar _004 模式 */ })
})
```

- [ ] **Step 2: 跑测试确认失败**

Run: `npx vitest run tests/components/virtualKeyboard.test.ts`
Expected: FAIL（组件不存在）

- [ ] **Step 3: 实现 `VirtualKeyboard.vue`**

布局按上方结构；键位数据用静态数组（row 分组，每键 `{ key, label, aria }`）；修饰键 `{ key: 'ctrl', sticky: true }`；点击处理：sticky 键切集合，普通键组装 VirtualKeyEvent → `vkeyToBytes` → 非 null `emit('send', bytes)` → 无论如何 `sticky.clear()`。样式：fixed 底部弹层、`padding-bottom: var(--safe-area-bottom, 0px)`、按钮 grid/flex、按压高亮复用 QuickKeyBar 视觉（`.pressing` pointer 模式或 `:active`——弹层内无 `mousedown.prevent` 与 :active 冲突问题？有：容器 prevent 吞 :active，沿用 pressedLabel pointer 模拟）。slide-up 动画 CSS transition 可选。

- [ ] **Step 4: 跑测试确认通过**

Run: `npx vitest run tests/components/virtualKeyboard.test.ts`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/components/VirtualKeyboard.vue tests/components/virtualKeyboard.test.ts
git commit -m "feat(mobile): 完整虚拟键盘弹层组件（粘滞修饰键）"
```

---

### Task 4: TerminalView 接线 + 打开时收起软键盘 + 手动测试条目登记

**Files:**
- Modify: `src/components/TerminalView.vue`（QuickKeyBar 处接线 + VirtualKeyboard 渲染）
- Modify: `docs/manual-test-cases.md`（手机虚拟键盘手动条目）

**Interfaces:**
- Consumes: `VirtualKeyboard.vue`（emits send/close）、QuickKeyBar 新 prop/emit（Task 2）
- Produces: 无下游依赖（终端功能闭环）

- [ ] **Step 1: TerminalView 接线**

`const vkOpen = ref(false)`；QuickKeyBar 传 `:vkeyboard-open="vkOpen"` + `@toggle-vkeyboard="toggleVkKeyboard"`；`toggleVkKeyboard()`：`vkOpen.value = !vkOpen.value`，**打开时若 `terminalRef.softKeyboardOpen` 先 `toggleSoftKeyboard()` 收起软键盘**（防双重键盘挤压视口），关闭弹层不自动回弹软键盘；TerminalView 模板 `.terminal-container` 内加 `<VirtualKeyboard v-if="isMobile && vkOpen" @send="handleQuickKey" @close="vkOpen = false" />`。

- [ ] **Step 2: 全量前端测试回归**

Run: `npm test`
Expected: 全绿（含既有 quickKeyBar/terminal 相关测试）

- [ ] **Step 3: 手动测试条目登记**

`docs/manual-test-cases.md` 增「手机虚拟键盘」节：真机打开弹层→粘滞 Ctrl+C 打断 Claude→Shift+Tab 切模式→F 键/翻页→弹层与软键盘互斥→收起后软键盘不自动弹。条目式（前置+步骤+预期），与既有条目格式一致。

- [ ] **Step 4: Commit**

```bash
git add src/components/TerminalView.vue docs/manual-test-cases.md
git commit -m "feat(mobile): TerminalView 接入虚拟键盘弹层，与软键盘互斥"
```
