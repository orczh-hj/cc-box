<template>
  <!-- 电脑键盘（手机端，与真实 PC 键盘同布局）：功能排 + 数字排 + 三排字母 +
       底排（修饰键/空格/方向键）。粘滞 Ctrl/Alt/Shift/Win 可叠加、组合后自动
       释放；CapsLock 本地切换（与 Shift 异或决定字母大小写）。按键经
       vkeyToBytes 转 ANSI 序列 emit send。容器 mousedown.prevent 保持终端焦点。 -->
  <div
    class="virtual-keyboard"
    @mousedown.prevent
    @selectstart.prevent
    @contextmenu.prevent
    @pointerdown="onPointerDown"
    @pointerup="pressedKey = ''"
    @pointercancel="pressedKey = ''"
    @pointerleave="pressedKey = ''"
  >
    <!-- 功能排：Esc + F1-F12（横滚） -->
    <div class="vk-row vk-fn-row">
      <button
        v-for="k in fnRow"
        :key="k.key"
        class="vk-btn"
        :class="keyClass(k)"
        :aria-label="k.key"
        @click="handleKey(k)"
      >{{ k.label }}</button>
    </div>

    <!-- 数字排 -->
    <div class="vk-row">
      <button
        v-for="k in rowNumber"
        :key="k.key"
        class="vk-btn"
        :style="{ flexGrow: k.units }"
        :class="keyClass(k)"
        :aria-label="k.key"
        @click="handleKey(k)"
      >{{ k.label }}</button>
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
        @click="handleKey(k)"
      >{{ k.label }}</button>
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
        @click="handleKey(k)"
      >{{ k.label }}</button>
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
        @click="handleKey(k)"
      >{{ k.label }}</button>
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
        @click="handleKey(k)"
      >{{ k.label }}</button>
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
import { vkeyToBytes, type VirtualKeyEvent } from '@/utils/virtual-keys'
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

function onPointerDown(e: PointerEvent) {
  const btn = (e.target as HTMLElement).closest('.vk-btn')
  pressedKey.value = btn?.getAttribute('aria-label') ?? ''
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
  /** 系统键（Win/Cmd）终端无标准序列：只占位不发送 */
  disabled?: boolean
}

/** 功能排：Esc + F1-F12（横滚，units 不参与比例） */
const fnRow: VKey[] = [
  { key: 'esc', label: 'Esc', units: 1 },
  ...Array.from({ length: 12 }, (_, i) => ({
    key: `f${i + 1}`, label: `F${i + 1}`, units: 1,
  })),
]

/** 数字排：` 1-0 - = ⌫ */
const rowNumber: VKey[] = [
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

/** 底排（平台差异）：macOS 显示 Option(⌥)/Command(⌘)，其余 Alt/Win。
 *  Command/Win 在终端无标准序列（Windows 终端不接收 Win 键、macOS Cmd 组需
 *  Kitty 协议），故只占位不发送。
 *  「中/英」键 = 全角/半角输出切换（标签与提示按目标平台习惯）：
 *  Windows 输入法用 Shift+Space 切全半角；macOS 用 Caps Lock 切中英。 */
const bottomRow = computed<VKey[]>(() => {
  const altLabel = isMacTarget.value ? '⌥' : 'Alt'
  const sysLabel = isMacTarget.value ? '⌘' : 'Win'
  return [
    // Ctrl 加宽、空格收窄（其余保持标准 1.25），单体宽度更均衡
    { key: 'ctrl', label: 'Ctrl', units: 1.7, sticky: 'ctrl' },
    { key: 'win', label: sysLabel, units: 1.25, disabled: true },
    { key: 'alt', label: altLabel, units: 1.25, sticky: 'alt' },
    { key: 'ime', label: fullwidth.value ? '中' : 'En', units: 1.25 },
    { key: 'space', label: '', units: 3.2 },
    { key: 'alt', label: altLabel, units: 1.25, sticky: 'alt' },
    { key: 'win', label: sysLabel, units: 1.25, disabled: true },
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

.vk-fn-row {
  overflow-x: auto;
  overscroll-behavior-x: contain;
  scrollbar-width: none;
}

.vk-fn-row::-webkit-scrollbar {
  display: none;
}

.vk-fn-row .vk-btn {
  flex: 0 0 auto;
  min-width: 34px;
  font-size: 10px;
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
