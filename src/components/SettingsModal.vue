<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
        <div class="modal-content">
          <header class="modal-header">
            <h2>{{ t('settings') }}</h2>
            <button class="close-btn" @click="$emit('close')">
              <img src="@/assets/icons/close.svg" :alt="t('close')" />
            </button>
          </header>

          <div class="modal-body">
            <!-- Font Size -->
            <section class="settings-section">
              <h3>{{ t('fontSize') }}</h3>
              <div class="font-size-control">
                <button class="size-btn" @click="decreaseFontSize">-</button>
                <span class="size-value">{{ fontSize }}</span>
                <button class="size-btn" @click="increaseFontSize">+</button>
              </div>
            </section>

            <!-- Theme -->
            <section class="settings-section">
              <h3>{{ t('theme') }}</h3>
              <div class="theme-options">
                <label class="theme-option">
                  <input type="radio" v-model="theme" value="light" />
                  <span>{{ t('light') }}</span>
                </label>
                <label class="theme-option">
                  <input type="radio" v-model="theme" value="dark" />
                  <span>{{ t('dark') }} (Coming soon)</span>
                </label>
              </div>
            </section>

            <!-- Startup Options -->
            <section class="settings-section">
              <h3>{{ t('startupDefaults') }}</h3>
              <label class="option-item">
                <input type="checkbox" v-model="defaultContinue" />
                <span class="option-label">Continue last session</span>
                <span class="option-flag">-c</span>
              </label>
              <label class="option-item">
                <input type="checkbox" v-model="defaultSkipPermissions" />
                <span class="option-label">{{ t('skipPermissions') }}</span>
                <span class="option-flag warning">--dangerously-skip</span>
              </label>
              <div class="option-item text-option">
                <span class="option-label">{{ t('customArgs') }}</span>
                <input type="text" v-model="defaultCustomArgs" :placeholder="t('customArgsPlaceholder')" />
              </div>
            </section>

            <!-- About -->
            <section class="settings-section about-section">
              <div class="about-info">
                <span class="version">CC-Box v0.1.0</span>
                <a class="link" @click="openDocs">{{ t('claudeDocs') }}</a>
              </div>
            </section>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import { getAppConfig, saveDefaultClaudeOptions } from '@/api/tauri'

defineProps<{
  visible: boolean
}>()

const { t } = useI18n()
const emit = defineEmits<{
  close: []
}>()

const appStore = useAppStore()
const fontSize = ref(appStore.fontSize)
const theme = ref(appStore.theme)
const defaultContinue = ref(appStore.claudeOptions.continue)
const defaultSkipPermissions = ref(appStore.claudeOptions.skipPermissions)
const defaultCustomArgs = ref(appStore.claudeOptions.customArgs)

onMounted(async () => {
  const config = await getAppConfig()
  fontSize.value = config.fontSize || 12
  theme.value = config.theme || 'light'
})

watch(fontSize, (val) => {
  appStore.setFontSize(val)
})

watch(theme, (val) => {
  appStore.setTheme(val)
})

watch([defaultContinue, defaultSkipPermissions, defaultCustomArgs], () => {
  saveDefaultClaudeOptions({
    continue: defaultContinue.value,
    skipPermissions: defaultSkipPermissions.value,
    customArgs: defaultCustomArgs.value
  })
})

function increaseFontSize() {
  fontSize.value = Math.min(24, fontSize.value + 1)
}

function decreaseFontSize() {
  fontSize.value = Math.max(10, fontSize.value - 1)
}

function openDocs() {
  window.open('https://docs.anthropic.com/en/docs/claude-code', '_blank')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 2000;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-content {
  width: 400px;
  max-height: 80vh;
  background: var(--bg-primary);
  border-radius: 12px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.12);
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
}

.close-btn img {
  width: 16px;
  height: 16px;
}

.close-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.modal-body {
  padding: 20px;
  overflow-y: auto;
}

.settings-section {
  margin-bottom: 20px;
}

.settings-section:last-child {
  margin-bottom: 0;
}

.settings-section h3 {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 10px;
}

.font-size-control {
  display: flex;
  align-items: center;
  gap: 12px;
}

.size-btn {
  width: 32px;
  height: 32px;
  border: 1px solid var(--border-color);
  background: var(--bg-secondary);
  color: var(--text-primary);
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
}

.size-btn:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.size-value {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  width: 40px;
  text-align: center;
}

.theme-options {
  display: flex;
  gap: 16px;
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.theme-option input[type="radio"] {
  accent-color: var(--accent-color);
}

.option-item {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 10px;
  cursor: pointer;
}

.option-item input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: var(--accent-color);
}

.option-label {
  font-size: 13px;
  color: var(--text-primary);
}

.option-flag {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: monospace;
}

.option-flag.warning {
  color: var(--status-error);
}

.text-option {
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
}

.text-option input[type="text"] {
  width: 100%;
  padding: 6px 10px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-primary);
}

.text-option input[type="text"]:focus {
  outline: none;
  border-color: var(--accent-color);
}

.about-section {
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.about-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.version {
  font-size: 12px;
  color: var(--text-secondary);
}

.link {
  font-size: 12px;
  color: var(--accent-color);
  cursor: pointer;
}

.link:hover {
  text-decoration: underline;
}

/* Transitions */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>