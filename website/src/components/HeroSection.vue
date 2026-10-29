<script setup lang="ts">
import { computed } from 'vue'
import type { LatestView } from '../lib/latest'
import { detectDesktopPlatform } from '../lib/ua'

const props = defineProps<{ latest: LatestView | null; loading: boolean }>()

const PLATFORM_LABEL: Record<string, string> = {
  windows: 'Windows',
  macos: 'macOS',
  linux: 'Linux',
}

const platform = computed(() => detectDesktopPlatform(navigator.userAgent))

const ctaLabel = computed(() => {
  const label = PLATFORM_LABEL[platform.value]
  return label ? `下载 for ${label}` : '立即下载'
})

const versionBadge = computed(() => {
  if (props.loading) return '…'
  if (!props.latest) return ''
  const date = props.latest.releaseDate ? ` · ${props.latest.releaseDate}` : ''
  return `v${props.latest.version}${date}`
})

const GITHUB_URL = 'https://github.com/orczh-hj/cc-box'
const GITEE_URL = 'https://gitee.com/orczh/cc-box'
</script>

<template>
  <section id="top" class="hero">
    <div class="container hero-inner">
      <img src="/logo.png" alt="CC-Box" class="hero-logo" />
      <h1 class="hero-title">CC-Box</h1>
      <p class="hero-tagline">Claude Code 桌面应用 — 多项目、多会话管理，浏览器远程访问</p>
      <p class="hero-sub">一个窗口。多个项目。快速切换会话。浏览器随时接管。</p>
      <p v-if="versionBadge" class="hero-version">
        <span class="version-badge">{{ versionBadge }}</span>
      </p>
      <div class="hero-actions">
        <a href="#download" class="btn btn-primary hero-cta">{{ ctaLabel }}</a>
        <a :href="GITHUB_URL" target="_blank" rel="noopener" class="btn btn-ghost">GitHub</a>
        <a :href="GITEE_URL" target="_blank" rel="noopener" class="btn btn-ghost">Gitee</a>
      </div>
      <p class="hero-platforms">Windows · macOS · Linux &nbsp;|&nbsp; MIT 开源</p>
    </div>
  </section>
</template>

<style scoped>
.hero {
  padding: 88px 0 72px;
  background:
    radial-gradient(ellipse 600px 300px at 50% -60px, rgba(212, 165, 116, 0.14), transparent),
    var(--bg-primary);
  text-align: center;
}

.hero-inner {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.hero-logo {
  width: 88px;
  height: 88px;
  border-radius: 20px;
  box-shadow: var(--shadow-lg);
  margin-bottom: 20px;
}

.hero-title {
  font-size: 44px;
  font-weight: 800;
  letter-spacing: -0.5px;
  color: var(--text-primary);
}

.hero-tagline {
  margin-top: 12px;
  font-size: 19px;
  font-weight: 600;
  color: var(--accent-primary);
}

.hero-sub {
  margin-top: 6px;
  font-size: 15px;
  color: var(--text-secondary);
}

.hero-version {
  margin-top: 16px;
}

.version-badge {
  display: inline-block;
  font-family: var(--font-mono);
  font-size: 13px;
  color: var(--accent-gold-dark);
  background: rgba(212, 165, 116, 0.15);
  border: 1px solid var(--accent-gold-light);
  border-radius: 999px;
  padding: 3px 14px;
}

.hero-actions {
  margin-top: 28px;
  display: flex;
  gap: 14px;
  flex-wrap: wrap;
  justify-content: center;
}

.hero-cta {
  font-size: 16px;
  padding: 13px 36px;
}

.hero-platforms {
  margin-top: 22px;
  font-size: 13px;
  color: var(--text-tertiary);
}

@media (max-width: 768px) {
  .hero {
    padding: 56px 0 48px;
  }

  .hero-title {
    font-size: 34px;
  }

  .hero-tagline {
    font-size: 16px;
  }
}
</style>
