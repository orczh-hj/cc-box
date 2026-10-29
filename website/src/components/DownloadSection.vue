<script setup lang="ts">
import { computed } from 'vue'
import { formatBytes } from '../lib/format'
import { PLATFORM_ORDER, type LatestView } from '../lib/latest'
import { detectDesktopPlatform, type DesktopPlatform } from '../lib/ua'

const props = defineProps<{ latest: LatestView | null; loading: boolean; failed: boolean }>()
const emit = defineEmits<{ retry: [] }>()

const PLATFORM_META: Record<
  'windows' | 'macos' | 'linux',
  { name: string; arch: string; icon: string }
> = {
  windows: {
    name: 'Windows',
    arch: 'x64 · NSIS 安装包',
    icon: '<rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/>',
  },
  macos: {
    name: 'macOS',
    arch: 'Apple Silicon · DMG',
    icon: '<path d="M21 2v6h-6"/><path d="M3 12a9 9 0 0 1 15-6.7L21 8"/>',
  },
  linux: {
    name: 'Linux',
    arch: 'x86_64 · AppImage',
    icon: '<rect x="4" y="4" width="16" height="16" rx="2"/><line x1="9" y1="9" x2="9" y2="15"/><line x1="15" y1="9" x2="15" y2="15"/>',
  },
}

const currentPlatform = computed<DesktopPlatform>(() => detectDesktopPlatform(navigator.userAgent))

// 当前桌面平台排第一（其余保持固定顺序）；移动设备（unknown）不重排
const orderedPlatforms = computed(() => {
  const cur = currentPlatform.value
  if (cur !== 'unknown' && (PLATFORM_ORDER as string[]).includes(cur)) {
    return [cur, ...PLATFORM_ORDER.filter((p) => p !== cur)] as ('windows' | 'macos' | 'linux')[]
  }
  return PLATFORM_ORDER
})

const versionText = computed(() => {
  if (props.loading) return '…'
  if (!props.latest) return ''
  return `v${props.latest.version}${props.latest.releaseDate ? ` · ${props.latest.releaseDate}` : ''}`
})
</script>

<template>
  <section id="download" class="section section-alt">
    <div class="container">
      <h2 class="section-title">下载安装</h2>
      <p class="section-subtitle">
        <template v-if="versionText">最新版本 {{ versionText }}</template>
        <template v-else>免费开源 · 三平台支持</template>
      </p>

      <div v-if="failed" class="notice notice-error">
        <p>获取版本信息失败，请检查网络后重试，或前往发布页手动下载。</p>
        <div class="notice-actions">
          <button class="btn btn-primary notice-btn" @click="emit('retry')">重试</button>
          <a
            class="btn btn-ghost notice-btn"
            href="https://github.com/orczh-hj/cc-box/releases/latest"
            target="_blank"
            rel="noopener"
            >GitHub Releases</a
          >
        </div>
      </div>

      <div v-if="currentPlatform === 'unknown'" class="notice notice-info">
        检测到移动设备，请在桌面设备下载安装 CC-Box。
      </div>

      <div class="download-grid">
        <div
          v-for="p in orderedPlatforms"
          :key="p"
          class="dl-card"
          :class="{ 'dl-card-current': p === currentPlatform }"
        >
          <span v-if="p === currentPlatform" class="dl-badge">当前平台</span>
          <span class="dl-icon">
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              v-html="PLATFORM_META[p].icon"
            ></svg>
          </span>
          <h3 class="dl-name">{{ PLATFORM_META[p].name }}</h3>
          <p class="dl-arch">{{ PLATFORM_META[p].arch }}</p>
          <p class="dl-file">
            <template v-if="latest">{{ latest.assets[p].fileName }}</template>
            <template v-else>…</template>
          </p>
          <p class="dl-size">
            <template v-if="latest">{{ formatBytes(latest.assets[p].size) }}</template>
            <template v-else>…</template>
          </p>
          <a
            class="btn btn-primary dl-btn"
            :href="latest ? latest.assets[p].url : undefined"
            :aria-disabled="!latest"
            :class="{ 'dl-btn-disabled': !latest }"
            >下载</a
          >
        </div>
      </div>

      <div class="dl-notes">
        <p>
          历史版本与发布说明：
          <a href="https://github.com/orczh-hj/cc-box/releases" target="_blank" rel="noopener"
            >GitHub Releases</a
          >
          ·
          <a href="https://gitee.com/orczh/cc-box/releases" target="_blank" rel="noopener"
            >Gitee Releases</a
          >
        </p>
      </div>
    </div>
  </section>
</template>

<style scoped>
.notice {
  max-width: 640px;
  margin: 0 auto 28px;
  padding: 14px 20px;
  border-radius: var(--radius-lg);
  font-size: 14px;
  text-align: center;
}

.notice-error {
  background: rgba(196, 92, 74, 0.08);
  border: 1px solid rgba(196, 92, 74, 0.3);
  color: var(--status-error);
}

.notice-info {
  background: rgba(30, 58, 95, 0.06);
  border: 1px solid var(--border-color);
  color: var(--accent-primary);
}

.notice-actions {
  margin-top: 12px;
  display: flex;
  gap: 12px;
  justify-content: center;
}

.notice-btn {
  padding: 8px 22px;
  font-size: 14px;
}

.download-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  margin-bottom: 32px;
}

.dl-card {
  position: relative;
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-xl);
  padding: 28px 24px;
  text-align: center;
  box-shadow: var(--shadow-md);
  display: flex;
  flex-direction: column;
  align-items: center;
}

.dl-card-current {
  border: 2px solid var(--accent-primary);
}

.dl-badge {
  position: absolute;
  top: -12px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-inverse);
  background: var(--accent-primary);
  border-radius: 999px;
  padding: 2px 14px;
  white-space: nowrap;
}

.dl-icon {
  display: inline-flex;
  width: 44px;
  height: 44px;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  background: rgba(30, 58, 95, 0.08);
  color: var(--accent-primary);
  margin-bottom: 14px;
}

.dl-icon svg {
  width: 22px;
  height: 22px;
}

.dl-name {
  font-size: 17px;
  font-weight: 700;
}

.dl-arch {
  margin-top: 4px;
  font-size: 13px;
  color: var(--text-secondary);
}

.dl-file {
  margin-top: 12px;
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--text-tertiary);
  word-break: break-all;
  max-width: 100%;
}

.dl-size {
  margin-top: 2px;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 18px;
}

.dl-btn {
  width: 100%;
  justify-content: center;
}

.dl-btn-disabled {
  pointer-events: none;
  opacity: 0.6;
}

.dl-notes {
  text-align: center;
  font-size: 13px;
  color: var(--text-tertiary);
  display: flex;
  flex-direction: column;
  gap: 6px;
}

@media (max-width: 768px) {
  .download-grid {
    grid-template-columns: 1fr;
    max-width: 400px;
    margin-left: auto;
    margin-right: auto;
  }
}
</style>
