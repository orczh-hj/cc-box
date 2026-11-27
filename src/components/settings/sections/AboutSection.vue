<template>
  <div class="section-content">
    <h2 class="section-heading">{{ t('aboutTitle') }}</h2>

    <div class="about-card">
      <div class="about-logo">
        <img src="@/assets/icons/app-icon.png" alt="CC-Box" class="logo-icon" />
      </div>
      <div class="about-details">
        <span class="app-name">CC-Box</span>
        <span class="app-version">{{ t('version', { version: currentVersion }) }}</span>
        <span class="app-desc">{{ t('aboutDesc') }}</span>
      </div>
    </div>

    <div class="links-group">
      <a class="about-link" @click="openExternal('https://ccbox.orczh.top/')">
        <div class="link-content">
          <span class="link-title">{{ t('officialSite') }}</span>
          <span class="link-url">ccbox.orczh.top</span>
        </div>
        <span class="link-arrow">→</span>
      </a>
      <a class="about-link" @click="openExternal('https://github.com/orczh-hj/cc-box')">
        <div class="link-content">
          <span class="link-title">{{ t('githubRepo') }}</span>
          <span class="link-url">github.com/orczh-hj/cc-box</span>
        </div>
        <span class="link-arrow">→</span>
      </a>
      <a class="about-link" @click="openExternal('https://code.claude.com/docs')">
        <div class="link-content">
          <span class="link-title">{{ t('claudeDocs') }}</span>
          <span class="link-url">code.claude.com/docs</span>
        </div>
        <span class="link-arrow">→</span>
      </a>
    </div>

    <div class="reward-card">
      <div class="reward-info">
        <span class="reward-title">{{ t('rewardTitle') }}</span>
        <span class="reward-desc">{{ t('rewardDesc') }}</span>
      </div>
      <button class="reward-code" :title="t('rewardHint')" @click="showRewardPreview = true">
        <img src="@/assets/images/wechat-reward.jpg" :alt="t('rewardTitle')" />
        <span class="reward-hint">{{ t('rewardHint') }}</span>
      </button>
    </div>

    <!-- 放大预览：便于手机对着屏幕扫 -->
    <Teleport to="body">
      <div v-if="showRewardPreview" class="reward-preview" @click="showRewardPreview = false">
        <img src="@/assets/images/wechat-reward.jpg" :alt="t('rewardTitle')" />
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { openExternal } from '@/api/platform'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const currentVersion = __APP_VERSION__

const showRewardPreview = ref(false)

function closeOnEscape(e: KeyboardEvent) {
  if (e.key === 'Escape') showRewardPreview.value = false
}

onMounted(() => window.addEventListener('keydown', closeOnEscape))
onBeforeUnmount(() => window.removeEventListener('keydown', closeOnEscape))
</script>

<style scoped>
.section-content {
  padding: 8px 0;
}

.section-heading {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 24px;
}

.about-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  margin-bottom: 24px;
}

.about-logo {
  width: 56px;
  height: 56px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  overflow: hidden;
}

.logo-icon {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.about-details {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.app-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.app-version {
  font-size: 13px;
  color: var(--text-secondary);
  font-variant-numeric: tabular-nums;
}

.app-desc {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-top: 4px;
}

.links-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.about-link {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s ease;
  text-decoration: none;
  color: var(--text-primary);
}

.about-link:hover {
  background: var(--bg-secondary);
}

.link-content {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.link-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.link-url {
  font-size: 12px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

.link-arrow {
  color: var(--text-tertiary);
  font-size: 14px;
}

.reward-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  margin-top: 8px;
  padding: 12px 14px;
  border-radius: 6px;
}

.reward-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.reward-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-tertiary);
}

.reward-desc {
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-primary);
}

.reward-code {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
  padding: 0;
  border: none;
  background: transparent;
  cursor: zoom-in;
}

.reward-code img {
  width: 112px;
  height: auto;
  border-radius: 8px;
  box-shadow: 0 0 0 1px var(--border-color);
  transition: box-shadow 0.15s ease;
}

.reward-code:hover img {
  box-shadow: 0 0 0 1px var(--text-tertiary);
}

.reward-hint {
  font-size: 11px;
  color: var(--text-tertiary);
}

.reward-preview {
  position: fixed;
  inset: 0;
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  background: rgba(0, 0, 0, 0.75);
  cursor: zoom-out;
}

.reward-preview img {
  max-width: min(480px, 100%);
  max-height: 100%;
  border-radius: 8px;
}
</style>
