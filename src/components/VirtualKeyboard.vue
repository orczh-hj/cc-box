<template>
  <!-- 电脑键盘（手机端，贴近真实 PC 键盘）：数字排（Esc 在最左）+ 三排字母 +
       底排（修饰键/中英/空格/方向键）。粘滞 Ctrl/Alt/Shift 可叠加、组合后自动
       释放；CapsLock 本地切换（与 Shift 异或决定字母大小写）。按键经
       vkeyToBytes 转 ANSI 序列 emit send。容器 mousedown.prevent 保持终端焦点。
       已精简：F1-F12（Claude Code 无绑定）、Win/⌘（终端无序列）、右 Alt（重复）。 -->
  <div
    class="virtual-keyboard"
    :class="{ 'vk-fw': fullwidth }"
    @mousedown.prevent
    @selectstart.prevent
    @contextmenu.prevent
    @pointerup="onPointerEnd"
    @pointercancel="onPointerEnd"
    @pointerleave="onPointerEnd"
  >
    <!-- 数字排（Esc 在最左） -->
    <div class="vk-row">
      <button
        v-for="k in rowNumber"
        :key="k.key"
        class="vk-btn"
        :style="{ flexGrow: k.units }"
        :class="keyClass(k)"
        :aria-label="k.key"
        @pointerdown="onPointerDown(k)"
        @click="onKeyClick(k)"
      >{{ keyLabel(k) }}</button>
    </div>

    <!-- 第一排 -->
    <div class="vk-row">
      <button
        v-for="k in row1"
        :key="k.key"
        class="vk-btn"
        :style="{ flexGrow: k.units }"
        :class="keyClass(k)"
        :aria-label="k.key"
        @pointerdown="onPointerDown(k)"
        @click="onKeyClick(k)"
      >{{ keyLabel(k) }}</button>
    </div>

    <!-- 第二排 -->
    <div class="vk-row">
      <button
        v-for="k in row2"
        :key="k.key"
        class="vk-btn"
        :style="{ flexGrow: k.units }"
        :class="keyClass(k)"
        :aria-label="k.key"
        @pointerdown="onPointerDown(k)"
        @click="onKeyClick(k)"
      >{{ keyLabel(k) }}</button>
    </div>

    <!-- 第三排 -->
    <div class="vk-row">
      <button
        v-for="(k, i) in row3"
        :key="`${k.key}-${i}`"
        class="vk-btn"
        :style="{ flexGrow: k.units }"
        :class="keyClass(k)"
        :aria-label="k.key"
        @pointerdown="onPointerDown(k)"
        @click="onKeyClick(k)"
      >{{ keyLabel(k) }}</button>
    </div>

    <!-- 底排：修饰键 / 空格 / 方向键 -->
    <div class="vk-row">
      <button
        v-for="(k, i) in bottomRow"
        :key="`${k.key}-${i}`"
        class="vk-btn"
        :style="{ flexGrow: k.units }"
        :class="keyClass(k)"
        :aria-label="k.key"
        :disabled="k.disabled"
        :title="k.disabled ? t('vkSystemKeyHint') : k.key === 'ime' ? imeHint : ''"
        @pointerdown="onPointerDown(k)"
        @click="onKeyClick(k)"
      >{{ keyLabel(k) }}</button>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * 电脑键盘（仅手机渲染，由 KeyboardPanel 控制）：真实 PC 键盘布局。
 * 修饰键点击切粘滞（可叠加、再点同键取消）；CapsLock 为本地开关（不发送字节，
 * 与 Shift 异或决定字母大小写）；普通键组装修饰 → vkeyToBytes → emit send，
 * 无论映射成败粘滞全部清空。
 */
import { computed, reactive, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { vkeyToBytes, SHIFT_SYMBOL, type VirtualKeyEvent } from '@/utils/virtual-keys'
import { isPrintableAscii, toFullWidth } from '@/utils/fullwidth'

const props = defineProps<{
  /** 目标电脑 OS（hello.ok platform）：决定 Alt/Option、Win/Cmd 键名 */
  platform?: string | null
}>()

const emit = defineEmits<{ send: [bytes: string] }>()

const { t } = useI18n()

/** macOS：Option(⌥) 为 Meta；Command(⌘) 终端不识别（禁用） */
const isMacTarget = computed(() => (props.platform ?? '').startsWith('mac'))

/** 按压中的按键 key（补偿 :active 被 mousedown.prevent 吞掉的高亮反馈） */
const pressedKey = ref('')

/** 粘滞修饰键状态（下一键组合后自动清空） */
const sticky = reactive({ ctrl: false, alt: false, shift: false })

/** CapsLock 本地状态（不发送字节） */
const caps = ref(false)

/** 全角模式（中/英全半角切换）：只转换可打印 ASCII 输出，控制键与方向/
 *  功能键的转义序列不受影响。纯本地状态，不发送字节 */
const fullwidth = ref(false)

/** 可长按连发的键（当前仅退格：手机上逐字点删太慢） */
const REPEATABLE_KEYS = new Set(['backspace'])
/** 长按触发延迟与连发间隔（毫秒） */
const REPEAT_DELAY = 300
const REPEAT_INTERVAL = 80

let repeatDelayTimer: number | null = null
let repeatTimer: number | null = null
/** 长按已连发时，抑制松开后浏览器补发的 click（避免多删一个） */
let suppressNextClick = false

function stopRepeat() {
  if (repeatDelayTimer !== null) {
    window.clearTimeout(repeatDelayTimer)
    repeatDelayTimer = null
  }
  if (repeatTimer !== null) {
    window.clearInterval(repeatTimer)
    repeatTimer = null
  }
}

function startRepeat(k: VKey) {
  if (!REPEATABLE_KEYS.has(k.key)) return
  stopRepeat()
  repeatDelayTimer = window.setTimeout(() => {
    suppressNextClick = true
    press(k) // 长按首次连发（短按首发由 click 完成）
    repeatTimer = window.setInterval(() => press(k), REPEAT_INTERVAL)
  }, REPEAT_DELAY)
}

function onPointerDown(k: VKey) {
  pressedKey.value = k.key
  startRepeat(k)
}

/** 指针离开/抬起：清按压态并停止连发 */
function onPointerEnd() {
  pressedKey.value = ''
  stopRepeat()
}

/** click 入口：长按已连发时跳过补发的 click（否则松手会多删一次） */
function onKeyClick(k: VKey) {
  if (suppressNextClick) {
    suppressNextClick = false
    return
  }
  handleKey(k)
}

function toggleSticky(mod: 'ctrl' | 'alt' | 'shift') {
  sticky[mod] = !sticky[mod]
}

interface VKey extends Partial<VirtualKeyEvent> {
  key: string
  label: string
  /** 布局宽度单位（1 = 标准键宽） */
  units: number
  /** 粘滞修饰键标识 */
  sticky?: 'ctrl' | 'alt' | 'shift'
  /** 占位键（无发送功能）：当前布局已无此类键，机制保留供后续扩展 */
  disabled?: boolean
}

/** 数字排：Esc ` 1-0 - = ⌫（Esc 并入数字排最左，对应真实键盘左上角位置；
 *  Claude Code 不绑定任何 F 键，F1-F12 整排已移除） */
const rowNumber: VKey[] = [
  { key: 'esc', label: 'Esc', units: 1.6 },
  ...('`1234567890-='.split('') as string[]).map(c => ({ key: c, label: c, units: 1 })),
  { key: 'backspace', label: '⌫', units: 2 },
]

/** 第一排：Tab QWERTYUIOP [ ] \ */
const row1: VKey[] = [
  { key: 'tab', label: 'Tab', units: 1.5 },
  ...'qwertyuiop'.split('').map(c => ({ key: c, label: c.toUpperCase(), units: 1 })),
  { key: '[', label: '[', units: 1 },
  { key: ']', label: ']', units: 1 },
  { key: '\\', label: '\\', units: 1.5 },
]

/** 第二排：Caps ASDFGHJKL ; ' Enter */
const row2: VKey[] = [
  { key: 'caps', label: 'Caps', units: 1.75 },
  ...'asdfghjkl'.split('').map(c => ({ key: c, label: c.toUpperCase(), units: 1 })),
  { key: ';', label: ';', units: 1 },
  { key: "'", label: "'", units: 1 },
  { key: 'enter', label: 'Enter', units: 2.25 },
]

/** 第三排：Shift ZXCVBNM , . / Shift */
const row3: VKey[] = [
  { key: 'shift', label: 'Shift', units: 2.25, sticky: 'shift' },
  ...'zxcvbnm'.split('').map(c => ({ key: c, label: c.toUpperCase(), units: 1 })),
  { key: ',', label: ',', units: 1 },
  { key: '.', label: '.', units: 1 },
  { key: '/', label: '/', units: 1 },
  { key: 'shift', label: 'Shift', units: 2.25, sticky: 'shift' },
]

/** 底排（手机精简）：Ctrl / Alt(⌥) / 中英 / 空格 / 方向键。
 *  已移除 Win(⌘) 与右侧重复 Alt——前者终端无标准序列（纯占位），后者与
 *  左 Alt 重复且手机单手操作够不到。
 *  「中/英」键 = 全角/半角输出切换（标签与提示按目标平台习惯）：
 *  Windows 输入法用 Shift+Space 切全半角；macOS 用 Caps Lock 切中英。 */
const bottomRow = computed<VKey[]>(() => {
  const altLabel = isMacTarget.value ? '⌥' : 'Alt'
  return [
    { key: 'ctrl', label: 'Ctrl', units: 1.7, sticky: 'ctrl' },
    { key: 'alt', label: altLabel, units: 1.5, sticky: 'alt' },
    { key: 'ime', label: fullwidth.value ? '中' : 'En', units: 1.5 },
    { key: 'space', label: '', units: 6 },
    { key: 'left', label: '←', units: 1.25 },
    { key: 'up', label: '↑', units: 1.25 },
    { key: 'down', label: '↓', units: 1.25 },
    { key: 'right', label: '→', units: 1.25 },
  ]
})

/** 「中/英」键提示：按目标平台展示对应的系统习惯（仅提示，切换由本键执行） */
const imeHint = computed(() =>
  isMacTarget.value ? t('vkImeHintMac') : t('vkImeHintWin'),
)

/** 键帽动态显示：字符键随修饰状态切换形态（响应式——依赖 sticky/caps/fullwidth）。
 *  - 字母：Shift/Caps 异或决定大小写（无修饰显示小写）
 *  - 符号/数字：Shift 显示上档符号（, → <、1 → !；美式 QWERTY，mac/win 一致）
 *  - 全角模式：当前形态再转全角字符（，＜！）；静态功能键（Esc/Tab/⌫…）不变 */
function keyLabel(k: VKey): string {
  const isLetter = /^[a-z]$/.test(k.key)
  if (!isLetter && SHIFT_SYMBOL[k.key] === undefined) return k.label
  const shift = sticky.shift !== (caps.value && isLetter)
  let label: string
  if (isLetter) label = shift ? k.key.toUpperCase() : k.key
  else label = shift ? SHIFT_SYMBOL[k.key] : k.key
  return fullwidth.value ? toFullWidth(label) : label
}

/** 按键视觉状态：按压、粘滞、Caps、全角、禁用（所有排共用） */
function keyClass(k: VKey) {
  return {
    pressing: pressedKey.value === k.key,
    'vk-sticky': !!k.sticky && sticky[k.sticky],
    'vk-caps': k.key === 'caps' && caps.value,
    'vk-ime': k.key === 'ime' && fullwidth.value,
    'vk-disabled': !!k.disabled,
  }
}

/** 统一按键处理：修饰键切粘滞、CapsLock/中英键切本地状态、其余走映射发送。
 *  所有排都必须走这里（曾因第三排 Shift / 第二排 Caps 直接调 press 导致
 *  粘滞与 Caps 不生效）。 */
function handleKey(k: VKey) {
  if (k.disabled) return
  if (k.sticky) return toggleSticky(k.sticky)
  if (k.key === 'caps') {
    caps.value = !caps.value
    return
  }
  if (k.key === 'ime') {
    fullwidth.value = !fullwidth.value
    return
  }
  press(k)
}

/** 普通键：组装修饰 → 映射 → 发射；无论成败粘滞清空（null 静默忽略） */
function press(k: VKey) {
  // CapsLock 只作用于字母键；与 Shift 异或决定大小写（数字/符号不受 Caps 影响）
  const isLetter = /^[a-z]$/.test(k.key)
  const shift = sticky.shift !== (caps.value && isLetter)
  const bytes = vkeyToBytes({
    key: k.key,
    ctrl: sticky.ctrl || undefined,
    alt: sticky.alt || undefined,
    shift: shift || undefined,
  })
  sticky.ctrl = false
  sticky.alt = false
  sticky.shift = false
  if (bytes === null) return
  // 全角模式：仅转换可打印 ASCII（字母/数字/符号/空格），
  // 控制字符与转义序列（\x03、\x1b[A 等）保持原样
  emit('send', fullwidth.value && isPrintableAscii(bytes) ? toFullWidth(bytes) : bytes)
}
</script>

<style scoped>
.virtual-keyboard {
  /* 文档流内（TerminalView 的 terminal-container flex 列）：终端区随面板
     出现自动收缩 refit（ResizeObserver），输入行始终可见 */
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 3px;
  padding: 4px;
  background: var(--bg-secondary);
  /* 禁止选中/长按菜单：Safari 需 -webkit- 前缀，callout 阻止 iOS 长按弹菜单；
     touch-action: manipulation 去掉双击缩放与双击选中 */
  user-select: none;
  -webkit-user-select: none;
  -webkit-touch-callout: none;
  touch-action: manipulation;
  -webkit-tap-highlight-color: transparent;
}

/* 子按钮同样禁用（部分 WebKit 版本不继承到 button 文本节点） */
.vk-btn {
  user-select: none;
  -webkit-user-select: none;
  -webkit-touch-callout: none;
}

.vk-row {
  display: flex;
  gap: 3px;
}




.vk-btn {
  /* 同排按键按 units 比例分配宽度；min-width:0 允许压缩不撑破 */
  flex: 1 1 0;
  min-width: 0;
  padding: 9px 1px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 11px;
  font-family: var(--font-mono, monospace);
  line-height: 1.1;
  white-space: nowrap;
  overflow: hidden;
  cursor: pointer;
}

.vk-btn:active,
.vk-btn.pressing {
  background: var(--selected-bg);
  color: var(--accent-gold);
}

/* 全角模式：字符键换 CJK 字体栈并略放大——全角字形以全宽方块呈现，
   与半角等宽字形差异一目了然（功能键文字不受影响） */
.virtual-keyboard.vk-fw .vk-btn {
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans CJK SC', 'Noto Sans SC', sans-serif;
  font-size: 12px;
}

/* 粘滞修饰键、CapsLock 与全角模式开启态 */
.vk-btn.vk-sticky,
.vk-btn.vk-caps,
.vk-btn.vk-ime {
  background: var(--selected-bg);
  color: var(--accent-gold);
  border-color: var(--accent-color);
}

/* 系统键（Win/Cmd）：占位展示，终端无标准序列 */
.vk-btn.vk-disabled {
  opacity: 0.35;
  cursor: default;
}

/* 修饰键与功能键略小字号，避免窄屏溢出 */
.vk-btn[aria-label='ctrl'],
.vk-btn[aria-label='alt'],
.vk-btn[aria-label='win'],
.vk-btn[aria-label='shift'],
.vk-btn[aria-label='caps'],
.vk-btn[aria-label='tab'],
.vk-btn[aria-label='enter'],
.vk-btn[aria-label='backspace'] {
  font-size: 10px;
}
</style>
