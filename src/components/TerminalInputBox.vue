<template>
  <Teleport to="body">
    <Transition name="ib-fade" appear>
      <div v-if="visible" class="ib-mask" @click.self="emit('close')">
        <Transition name="ib-slide" appear>
          <div class="ib-sheet" @contextmenu.stop>
            <!-- 顶栏右上角关闭：位于 textarea 外部，避免与文本选择手柄混淆 -->
            <div class="ib-topbar">
              <button class="ib-close" :aria-label="t('close')" @click="emit('close')">
                <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
                  <path d="M6 6l12 12M18 6L6 18" />
                </svg>
              </button>
            </div>
            <!-- 正文 textarea：粘贴/复制/光标操作全由系统原生菜单接管，组件零干预 -->
            <textarea
              class="ib-textarea"
              :value="draft"
              :placeholder="t('inputBoxPlaceholder')"
              @input="emit('update:draft', ($event.target as HTMLTextAreaElement).value)"
            ></textarea>
            <div class="ib-footer">
              <!-- 清空在左、上屏在右；关闭独立在右上角，不与操作按钮混排 -->
              <button class="ib-btn ghost" @click="clearDraft">{{ t('inputBoxClear') }}</button>
              <button
                class="ib-btn primary"
                :disabled="!draft"
                @click="commit"
              >{{ t('inputBoxCommit') }}</button>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
/**
 * 输入中转框（手机补充输入）：QuickKeyBar「输入」按钮唤出的底部弹层。
 *
 * 背景：系统粘贴菜单只认可见可编辑元素，终端画布上无处落脚（透明代理
 * 方案被国产 ROM 系统级识别抢走，已废弃）。本组件提供普通可见 textarea
 * ——打字/粘贴/复制全走系统原生长按菜单，全平台稳定。
 *
 * 上屏 ≠ 提交：经 term.paste 写入终端输入行（括号粘贴归一，多行不逐行
 * 执行），内容进终端后仍可用软键盘编辑，回车才提交。不做反向同步
 * （PTY 字节流读不回 CLI 输入行，协议边界）。
 */
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  visible: boolean
  draft: string
}>()

const emit = defineEmits<{
  close: []
  'update:draft': [text: string]
  commit: [text: string]
}>()

const { t } = useI18n()

function commit() {
  const text = props.draft
  if (!text) return
  emit('commit', text)
}

function clearDraft() {
  emit('update:draft', '')
}
</script>

<style scoped>
.ib-mask {
  position: fixed;
  inset: 0;
  z-index: 1800;
  background: rgba(26, 24, 22, 0.42);
  display: flex;
  align-items: flex-end;
}

.ib-sheet {
  position: relative;
  width: 100%;
  padding: 12px;
  padding-bottom: calc(12px + var(--safe-area-bottom, 0px));
  background: var(--bg-primary);
  border-radius: 12px 12px 0 0;
  box-shadow: 0 -4px 16px rgba(26, 24, 22, 0.18);
}

.ib-textarea {
  width: 100%;
  min-height: 38vh;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: 14px;
  line-height: 1.5;
  resize: none;
  /* 系统原生长按选择/粘贴手柄 */
  -webkit-user-select: text;
  user-select: text;
  -webkit-touch-callout: default;
}

.ib-topbar {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 4px;
}

.ib-close {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  border-radius: 50%;
}

.ib-close:active {
  background: var(--hover-bg);
}

.ib-footer {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  margin-top: 10px;
}

.ib-btn {
  padding: 8px 18px;
  border-radius: var(--radius-lg);
  font-size: 14px;
}

.ib-btn.ghost {
  border: 1px solid var(--border-color);
  background: var(--bg-secondary);
  color: var(--text-secondary);
}

.ib-btn.ghost:active {
  background: var(--hover-bg);
}

.ib-btn.primary {
  border: none;
  background: var(--accent-color);
  color: var(--bg-primary);
}

.ib-btn.primary:disabled {
  opacity: 0.45;
}

/* 遮罩淡入淡出 */
.ib-fade-enter-active,
.ib-fade-leave-active {
  transition: opacity 0.18s ease;
}
.ib-fade-enter-from,
.ib-fade-leave-to {
  opacity: 0;
}

/* 弹层底部滑入滑出 */
.ib-slide-enter-active,
.ib-slide-leave-active {
  transition: transform 0.22s ease;
}
.ib-slide-enter-from,
.ib-slide-leave-to {
  transform: translateY(100%);
}
</style>
