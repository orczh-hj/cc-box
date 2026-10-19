<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
        <div class="dir-picker">
          <header class="picker-header">
            <h2>{{ t('dirPickerTitle') }}</h2>
            <button class="close-btn" @click="$emit('close')">
              <img src="@/assets/icons/close.svg" :alt="t('close')" />
            </button>
          </header>

          <!-- 当前路径 + 手动输入（桌面浏览器便捷通道） -->
          <div class="path-row">
            <code class="current-path">{{ currentPath || t('dirPickerRoot') }}</code>
          </div>
          <div class="input-row">
            <input
              type="text"
              v-model="manualPath"
              :placeholder="t('dirPickerEnterPath')"
              @keydown.enter="goManual"
            />
            <button class="go-btn" @click="goManual">{{ t('dirPickerGo') }}</button>
          </div>

          <div class="dir-list">
            <p v-if="loading" class="hint">{{ t('loading') }}</p>
            <p v-else-if="error" class="error-text">{{ error }}</p>
            <p v-else-if="entries.length === 0" class="hint">{{ t('dirPickerEmpty') }}</p>
            <template v-else>
              <!-- 排序切换：名称字母序 / 最近修改（找最近建的项目目录） -->
              <div class="sort-row">
                <button
                  v-for="mode in (['name', 'modified'] as const)"
                  :key="mode"
                  class="sort-btn"
                  :class="{ active: sortBy === mode }"
                  @click="sortBy = mode"
                >{{ t(mode === 'name' ? 'dirPickerSortName' : 'dirPickerSortModified') }}</button>
              </div>
              <button
                v-for="e in sortedEntries"
                :key="e.path"
                class="dir-item"
                @click="enter(e)"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
                <span class="dir-name">{{ e.name }}</span>
              </button>
            </template>
          </div>

          <footer class="picker-footer">
            <button
              v-if="currentPath"
              class="secondary-btn"
              @click="goUp"
            >{{ t('dirPickerUp') }}</button>
            <button
              class="primary-btn"
              :disabled="!currentPath || loading"
              @click="chooseCurrent"
            >{{ t('dirPickerSelect') }}</button>
          </footer>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
/**
 * 电脑目录选择器（浏览器远程端的"添加项目"）：rpc list_directories 浏览
 * 电脑目录树（根=盘符/根分区 → 逐级进入），选当前目录作为项目路径；
 * 支持手动输入路径直达。桌面端用系统原生选择器，不渲染本组件。
 */
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { listDirectories, type DirEntryLite } from '@/api/tauri'

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits<{ close: []; select: [path: string] }>()

const { t } = useI18n()

const currentPath = ref('')
const entries = ref<DirEntryLite[]>([])
const loading = ref(false)
const error = ref('')
const manualPath = ref('')
/** 排序方式：名称字母序（默认）/ 最近修改降序 */
const sortBy = ref<'name' | 'modified'>('name')

const sortedEntries = computed(() => {
  const list = [...entries.value]
  if (sortBy.value === 'modified') {
    list.sort((a, b) => (b.modified ?? 0) - (a.modified ?? 0))
  }
  return list
})

watch(() => props.visible, (v) => {
  if (v) {
    // 从根（盘符/根分区）开始
    currentPath.value = ''
    manualPath.value = ''
    sortBy.value = 'name'
    load()
  }
}, { immediate: true })

async function load(path: string) {
  loading.value = true
  error.value = ''
  try {
    entries.value = await listDirectories(path || undefined)
    currentPath.value = path
  } catch (e) {
    error.value = String(e)
    entries.value = []
  } finally {
    loading.value = false
  }
}

function enter(e: DirEntryLite) {
  load(e.path)
}

function goUp() {
  if (!currentPath.value) return
  const norm = currentPath.value.replace(/\\/g, '/')
  // 根判定：盘符根（C:/）或 /
  const parts = norm.split('/').filter(Boolean)
  if (parts.length <= 1) {
    load('')
    return
  }
  parts.pop()
  const parent = parts.length === 1 && /^[a-zA-Z]:$/.test(parts[0])
    ? `${parts[0]}/`
    : parts.join('/')
  load(parent)
}

function goManual() {
  const p = manualPath.value.trim()
  if (p) load(p)
}

function chooseCurrent() {
  if (currentPath.value) emit('select', currentPath.value)
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dir-picker {
  width: 480px;
  max-width: calc(100vw - 32px);
  max-height: min(640px, calc(100dvh - 64px));
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.picker-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px;
  border-bottom: 1px solid var(--border-color);
}

.picker-header h2 {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
}

.close-btn img { width: 16px; height: 16px; }
.close-btn:hover { background: var(--hover-bg); }

.path-row { padding: 10px 18px 0; }

.current-path {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 6px 10px;
  word-break: break-all;
  user-select: all;
}

.input-row {
  display: flex;
  gap: 6px;
  padding: 8px 18px 0;
}

.input-row input {
  flex: 1;
  min-width: 0;
  padding: 6px 10px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 12px;
  color: var(--text-primary);
  outline: none;
}

.input-row input:focus { border-color: var(--focus-ring); }

.go-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  flex-shrink: 0;
}

.dir-list {
  flex: 1;
  min-height: 200px;
  overflow-y: auto;
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.dir-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 10px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-primary);
  font-size: 13px;
  text-align: left;
  cursor: pointer;
}

.dir-item svg { color: var(--accent-gold); flex-shrink: 0; }

/* 排序切换 */
.sort-row {
  display: flex;
  gap: 4px;
  padding: 2px 4px 8px;
}

.sort-btn {
  padding: 4px 10px;
  border: 1px solid var(--border-color);
  border-radius: 999px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 11px;
  cursor: pointer;
}

.sort-btn.active {
  background: var(--selected-bg);
  border-color: var(--accent-gold);
  color: var(--accent-gold);
}

.dir-item:hover { background: var(--hover-bg); }

.dir-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.hint, .error-text {
  font-size: 12px;
  padding: 16px;
  text-align: center;
}

.hint { color: var(--text-secondary); }
.error-text { color: var(--error-color); word-break: break-all; }

.picker-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 18px;
  border-top: 1px solid var(--border-color);
}

.secondary-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
}

.primary-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  background: var(--accent-color);
  color: #fff;
  font-size: 12px;
  cursor: pointer;
}

.primary-btn:disabled { opacity: 0.5; cursor: not-allowed; }

.modal-enter-active, .modal-leave-active { transition: opacity 0.15s ease; }
.modal-enter-from, .modal-leave-to { opacity: 0; }
</style>
