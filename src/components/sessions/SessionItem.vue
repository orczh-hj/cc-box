<template>
  <div
    class="session-item"
    :class="{ active: isActive, stopped: isStopped }"
    @click="handleClick"
  >
    <!-- 运行状态指示器 -->
    <span class="status-dot" :class="dotClass"></span>

    <!-- 会话名称 -->
    <div class="session-name-wrapper">
      <span v-if="!isRenaming" class="session-name">{{ name }}</span>
      <input
        v-else
        ref="renameInputRef"
        v-model="newName"
        class="rename-input"
        :style="{ width: inputWidth }"
        @keyup.enter="confirmRename"
        @keyup.escape="cancelRename"
        @blur="confirmRename"
        @click.stop
      />
      <span v-if="snippet" class="session-snippet">{{ snippet }}</span>
    </div>

    <!-- 操作按钮 -->
    <!-- 重启按钮（已停止的 Tab，无 sessionId 时禁用） -->
    <button
      v-if="isStopped"
      class="action-btn restart-btn"
      :disabled="!canResume"
      @click.stop="canResume && $emit('restart', id)"
      :title="canResume ? t('restart') : t('waitingForSessionId')"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="23 4 23 10 17 10"/>
        <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
      </svg>
    </button>
    <!-- 重命名按钮 -->
    <button
      v-if="isActive"
      class="action-btn rename-btn"
      @click.stop="startRename"
      :title="t('rename')"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/>
        <path d="m15 5 4 4"/>
      </svg>
    </button>
    <!-- 关闭按钮 -->
    <button
      v-if="closable"
      class="action-btn close-action-btn"
      @click.stop="$emit('close', id)"
      :title="t('close')"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>

    <!-- 时间信息 -->
    <span v-if="showTime" class="time-info">{{ timeAgo }}</span>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  id: string
  name: string
  isActive: boolean
  isRunning?: boolean
  isStopped?: boolean
  canResume?: boolean
  working?: boolean
  pending?: boolean
  lastActiveAt: number
  closable?: boolean
  snippet?: string
  showTime?: boolean
}>()

const dotClass = computed(() => {
  if (props.isStopped && !props.isActive) return 'closed'
  if (props.isStopped) return 'stopped'
  if (props.working) return 'working'
  if (props.pending && !props.isActive) return 'pending'
  if (props.isRunning) return 'running'
  return ''
})

const emit = defineEmits<{
  switch: [id: string]
  rename: [id: string, name: string]
  restart: [id: string]
  close: [id: string]
}>()

const isRenaming = ref(false)
const newName = ref('')
const renameInputRef = ref<HTMLInputElement>()
const inputWidth = ref('auto')

// 计算时间差
const timeAgo = computed(() => {
  const now = Date.now()
  const diff = now - props.lastActiveAt

  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return t('justNow')
  if (minutes < 60) return `${minutes}m`
  if (hours < 24) return `${hours}h`
  return `${days}d`
})

function handleClick() {
  if (!isRenaming.value) {
    emit('switch', props.id)
  }
}

function startRename() {
  isRenaming.value = true
  newName.value = props.name

  const nameLen = props.name.length
  inputWidth.value = `${Math.max(50, Math.min(nameLen * 8 + 16, 180))}px`

  nextTick(() => {
    renameInputRef.value?.focus()
    renameInputRef.value?.select()
  })
}

function confirmRename() {
  if (!isRenaming.value) return
  isRenaming.value = false
  if (newName.value.trim() && newName.value !== props.name) {
    emit('rename', props.id, newName.value.trim())
  }
}

function cancelRename() {
  isRenaming.value = false
  newName.value = props.name
}
</script>

<style scoped>
.session-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 6px;
  transition: background 0.15s ease;
  position: relative;
}

.session-item:hover {
  background: var(--hover-bg);
}

.session-item.active {
  background: var(--selected-bg);
}

.session-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 4px;
  bottom: 4px;
  width: 3px;
  background: var(--accent-gold);
  border-radius: 0 2px 2px 0;
}

.session-item.active .session-name {
  color: var(--accent-color);
  font-weight: 600;
}

.session-item.stopped .session-name {
  color: var(--text-secondary);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
  transition: background 0.3s ease;
}

/* 默认（无状态数据） */
.status-dot:not(.working):not(.pending):not(.running):not(.stopped):not(.closed) {
  background: var(--text-tertiary);
}

/* 连接（idle — 绿色静止） */
.status-dot.running {
  background: var(--status-success);
}

/* 工作中（绿色脉冲） */
.status-dot.working {
  background: var(--status-success);
  animation: status-pulse 2.5s ease-in-out infinite;
}

/* 待处理（waiting_permission/waiting_input） */
.status-dot.pending {
  background: var(--accent-gold);
  animation: status-pulse 2s ease-in-out infinite;
}

/* 已停止（当前活跃 tab） */
.status-dot.stopped {
  background: var(--text-tertiary);
  animation: none;
}

/* 已关闭（非当前活跃 tab） */
.status-dot.closed {
  width: 6px;
  height: 6px;
  border: 1.5px solid var(--border-color);
  background: transparent;
  animation: none;
}

@keyframes status-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

.session-name-wrapper {
  flex: 1;
  min-width: 0;
}

.session-name {
  font-size: 13px;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

.session-snippet {
  font-size: 11px;
  color: var(--text-tertiary);
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-top: 1px;
}

.rename-input {
  font-size: 13px;
  font-weight: 500;
  padding: 2px 6px;
  border: 1px solid var(--accent-color);
  border-radius: 3px;
  background: var(--bg-primary);
  color: var(--text-primary);
  outline: none;
}

.action-btn {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 3px;
  flex-shrink: 0;
}

/* hover 才显示操作按钮——仅指针设备（触屏 hover:none 无悬停，常显） */
@media (hover: hover) {
  .session-item:not(:hover) .action-btn {
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.15s ease, visibility 0s 0.15s;
  }

  .session-item:hover .action-btn {
    opacity: 1;
    visibility: visible;
    transition: opacity 0.15s ease, visibility 0s;
  }
}

.action-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.action-btn:disabled {
  cursor: not-allowed;
  opacity: 0.4;
}

.time-info {
  font-size: 11px;
  color: var(--text-tertiary);
  flex-shrink: 0;
}
</style>
