<template>
  <Teleport to="body">
    <Transition name="market-fade">
      <div v-if="visible" class="market-overlay" @click.self="close">
        <div class="market-card" :class="{ sheet: isMobile }" role="dialog" aria-modal="true" :aria-label="t('marketTitle')">
          <!-- 头部 -->
          <header class="market-head">
            <div class="head-text">
              <template v-if="!isNarrow">
                <h2 class="head-title">{{ t('marketTitle') }}</h2>
                <p class="head-sub">{{ headStat }}</p>
              </template>
              <!-- 窄屏：市场入口（打开左侧抽屉） -->
              <button v-else class="market-picker" @click="railOpen = true">
                <svg class="picker-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
                  <path d="M4 7h16M4 12h16M4 17h10" />
                </svg>
                <span class="picker-name">{{ selected?.name || t('marketDrawerTitle') }}</span>
                <span v-if="selected" class="picker-count">{{ selected.plugins.length }}</span>
                <svg class="picker-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
                  <path d="M9 18l6-6-6-6" />
                </svg>
              </button>
            </div>
            <div class="head-actions">
              <button
                class="icon-btn"
                :class="{ spinning: busyKey === 'market:refresh' }"
                :disabled="!selected || anyBusy"
                :title="t('marketRefreshSelected')"
                @click="refreshSelected"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
                  <path d="M21 12a9 9 0 1 1-2.64-6.36" />
                  <path d="M21 3v6h-6" />
                </svg>
              </button>
              <button class="icon-btn" :title="t('close')" @click="close">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
                  <path d="M18 6 6 18M6 6l12 12" />
                </svg>
              </button>
            </div>
          </header>

          <div class="market-body">
            <!-- 窄屏：抽屉遮罩 -->
            <Transition name="rail-fade">
              <div v-if="isNarrow && railOpen" class="rail-backdrop" @click="railOpen = false" />
            </Transition>

            <!-- 市场栏（宽屏常驻左列 / 窄屏左侧抽屉） -->
            <aside class="rail" :class="{ drawer: isNarrow, open: isNarrow && railOpen }">
              <div v-if="isNarrow" class="drawer-head">
                <span class="drawer-title">{{ t('marketDrawerTitle') }}</span>
                <button class="icon-btn" :title="t('close')" @click="railOpen = false">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
                    <path d="M18 6 6 18M6 6l12 12" />
                  </svg>
                </button>
              </div>

              <!-- 我的市场（用户添加：可浏览；已注册/未注册徽标） -->
              <div class="rail-block">
                <div class="rail-label">{{ t('marketMySection') }}</div>
                <button
                  v-for="m in myMarkets"
                  :key="m.name"
                  class="rail-item"
                  :class="{ active: selected?.name === m.name, broken: !!m.loadError }"
                  :title="m.loadError || m.description || m.source"
                  @click="selectMarket(m)"
                >
                  <span v-if="m.icon" class="p-icon small">
                    <img v-if="isIconUrl(m.icon)" :src="m.icon" alt="" @error="hideIcon" />
                    <span v-else class="p-emoji">{{ m.icon }}</span>
                  </span>
                  <span class="rail-name">{{ m.name }}</span>
                  <span class="rail-badge" :class="{ reg: m.registered }">
                    {{ t(m.registered ? 'marketBadgeRegistered' : 'marketBadgeUnregistered') }}
                  </span>
                  <span v-if="busyKey === 'market:refresh'" class="rail-count">…</span>
                  <span v-else class="rail-count">{{ m.plugins.length }}</span>
                </button>
                <p v-if="!myMarkets.length" class="rail-empty">{{ t('marketNoneRegistered') }}</p>
                <div v-if="selected && selected.origin === 'user'" class="market-ops">
                  <button
                    v-if="!selected.registered"
                    class="remove-market-btn register-btn"
                    :disabled="anyBusy"
                    :title="t('marketRegisterConfirm', { name: selected.name })"
                    @click="registerMarket(selected)"
                  >
                    <span class="rm-text">{{ t('marketRegister') }}</span>
                    <span class="rm-icon">⊕</span>
                  </button>
                  <!-- ccbox-market 是官方精选来源，不可移除 -->
                  <button
                    v-if="selected.name.toLowerCase() !== CCBOX_MARKET"
                    class="remove-market-btn"
                    :disabled="anyBusy"
                    :title="t('marketRemoveListConfirm', { name: selected.name })"
                    @click="removeMarket(selected)"
                  >
                    <span class="rm-text">{{ t('marketRemoveMarket') }}</span>
                    <span class="rm-icon">×</span>
                  </button>
                </div>
              </div>

              <!-- Claude 注册（CLI/其他途径注册，未在 CC-Box 列表；已注册市场选中时也提供卸载入口） -->
              <div v-if="claudeMarkets.length || (selected && selected.registered)" class="rail-block">
                <div class="rail-label">{{ t('marketRegistered') }}</div>
                <button
                  v-for="m in claudeMarkets"
                  :key="m.name"
                  class="rail-item"
                  :class="{ active: selected?.name === m.name, broken: !!m.loadError }"
                  :title="m.loadError || m.installLocation"
                  @click="selectMarket(m)"
                >
                  <span class="rail-mark" />
                  <span class="rail-name">{{ m.name }}</span>
                  <span class="rail-count">{{ m.plugins.length }}</span>
                </button>
                <button
                  v-if="selected && selected.registered && selected.name.toLowerCase() !== CCBOX_MARKET"
                  class="remove-market-btn"
                  :disabled="anyBusy"
                  :title="t('marketRemoveConfirm', { name: selected.name })"
                  @click="unregisterMarket(selected)"
                >
                  <span class="rm-text">{{ t('marketUnregister') }}</span>
                  <span class="rm-icon">⊘</span>
                </button>
              </div>

              <!-- 精选推荐（未添加，一键加入列表） -->
              <div v-if="presetMarkets.length" class="rail-block">
                <div class="rail-label">{{ t('marketPresetSection') }}</div>
                <button
                  v-for="p in presetMarkets"
                  :key="p.name"
                  class="rail-item preset"
                  :disabled="anyBusy"
                  :title="p.description || p.source"
                  @click="addPreset(p)"
                >
                  <span v-if="p.icon" class="p-icon small">
                    <img v-if="isIconUrl(p.icon)" :src="p.icon" alt="" @error="hideIcon" />
                    <span v-else class="p-emoji">{{ p.icon }}</span>
                  </span>
                  <span v-if="p.pinned" class="pin-star" :title="t('marketFeatured')">★</span>
                  <span class="rail-name">{{ p.name }}</span>
                  <span v-if="busyKey === 'market:add:' + p.name" class="rail-count">{{ t('marketUpdating') }}</span>
                  <span v-else class="preset-plus">＋</span>
                </button>
              </div>

              <div class="rail-block rail-add">
                <button v-if="!adding" class="add-btn" @click="startAdding">＋ {{ t('marketAdd') }}</button>
                <form v-else class="add-form" @submit.prevent="confirmAdd">
                  <input
                    v-model="newSource"
                    class="field"
                    :placeholder="t('marketAddPlaceholder')"
                    spellcheck="false"
                    autofocus
                  />
                  <p class="add-hint">{{ t('marketAddHint') }}</p>
                  <p v-if="busyKey === 'market:add'" class="add-hint add-progress">
                    {{ t('marketAddProgress', { s: busySeconds }) }}
                  </p>
                  <div class="add-actions">
                    <button type="submit" class="btn primary" :disabled="!newSource.trim() || anyBusy">
                      {{ busyKey === 'market:add' ? t('marketUpdating') : t('marketAddConfirm') }}
                    </button>
                    <button type="button" class="btn ghost" @click="adding = false">{{ t('cancel') }}</button>
                  </div>
                  <p v-if="actionError && errorScope === 'add'" class="inline-error">{{ actionError }}</p>
                </form>
              </div>

              <p v-if="actionError && errorScope === 'market'" class="inline-error rail-error">{{ actionError }}</p>
            </aside>

            <!-- 插件列表 -->
            <section class="list-pane" :class="{ 'is-hidden': isNarrow && detail }">
              <div class="search-box">
                <svg class="search-glyph" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
                  <circle cx="11" cy="11" r="7" />
                  <path d="m20 20-3.6-3.6" />
                </svg>
                <input
                  v-model="query"
                  class="search-input"
                  :placeholder="t('marketSearchPlaceholder')"
                  spellcheck="false"
                />
                <button v-if="query" class="search-clear" :title="t('cancel')" @click="query = ''">×</button>
              </div>

              <!-- 市场信息：本地缓存时间来自 Claude 自己的 known_marketplaces.json -->
              <div v-if="selected" class="list-meta">
                <span>{{ t('marketPluginCount', { n: selected.plugins.length }) }}</span>
                <template v-if="selected.lastUpdated">
                  <span class="meta-sep">·</span>
                  <span :title="t('marketCacheHint')">{{ t('marketUpdatedAt', { date: formatDate(selected.lastUpdated) }) }}</span>
                </template>
              </div>

              <p v-if="loadError" class="inline-error">{{ loadError }}</p>

              <div v-if="loading" class="list-empty">{{ t('marketLoading') }}</div>
              <div v-else-if="!selected" class="list-empty">{{ t('marketNoMarketplace') }}</div>
              <div v-else-if="selected.loadError" class="inline-error">{{ selected.loadError }}</div>
              <div v-else-if="!filtered.length" class="list-empty">{{ query ? t('marketNoMatch') : t('marketEmpty') }}</div>

              <div v-else class="plugin-list">
                <button
                  v-for="p in filtered"
                  :key="p.name"
                  class="plugin-row"
                  :class="{ active: detail?.name === p.name }"
                  @click="toggleDetail(p)"
                >
                  <span class="row-title">
                    <span v-if="p.icon" class="p-icon" :title="p.category || p.name">
                      <img v-if="isIconUrl(p.icon)" :src="p.icon" alt="" @error="hideIcon" />
                      <span v-else class="p-emoji">{{ p.icon }}</span>
                    </span>
                    <span v-if="p.pinned" class="pin-star" :title="t('marketFeatured')">★</span>
                    <span class="row-name">{{ p.name }}</span>
                    <span v-if="p.installed" class="chip chip-ok">{{ t('marketInstalled') }}</span>
                  </span>
                  <span v-if="p.description" class="row-desc">{{ p.description }}</span>
                  <span class="row-foot">
                    <span v-if="p.version" class="row-version">v{{ p.version }}</span>
                    <span v-if="p.category" class="row-cat">{{ p.category }}</span>
                    <span v-if="p.installs" class="row-installs" :title="t('marketInstallsTitle')">
                      {{ t('marketInstallsShort', { n: formatInstalls(p.installs) }) }}
                    </span>
                    <span v-if="busyKey === pluginActionKey(p, 'install') || busyKey === pluginActionKey(p, 'update')" class="row-busy">
                      {{ t('marketUpdating') }}
                    </span>
                  </span>
                </button>
              </div>
            </section>

            <!-- 详情 -->
            <section class="detail-pane" :class="{ 'has-detail': !!detail }">
              <template v-if="detail">
                <div class="detail-head">
                  <button v-if="isNarrow" class="back-btn" :title="t('marketBack')" @click="detail = null">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
                      <path d="M15 18l-6-6 6-6" />
                    </svg>
                  </button>
                  <span v-if="detail.icon" class="p-icon large" :title="detail.category || detail.name">
                    <img v-if="isIconUrl(detail.icon)" :src="detail.icon" alt="" @error="hideIcon" />
                    <span v-else class="p-emoji">{{ detail.icon }}</span>
                  </span>
                  <div class="detail-title-wrap">
                    <h3 class="detail-title">{{ detail.name }}</h3>
                    <div class="detail-chips">
                      <span v-if="detail.version" class="chip">v{{ detail.version }}</span>
                      <span v-if="detail.category" class="chip">{{ detail.category }}</span>
                      <span v-if="detail.installs" class="chip muted">
                        {{ t('marketInstallsShort', { n: formatInstalls(detail.installs) }) }}
                      </span>
                      <span v-for="tag in detail.tags" :key="tag" class="chip muted">#{{ tag }}</span>
                      <span v-if="detail.installed" class="chip" :class="detail.enabled ? 'chip-ok' : 'chip-off'">
                        {{ detail.enabled ? t('marketEnabled') : t('marketDisabledState') }}
                      </span>
                    </div>
                  </div>
                </div>

                <div class="detail-scroll">
                  <div class="detail-section">
                    <div class="section-label">{{ t('marketAbout') }}</div>
                    <p class="detail-desc">{{ detail.description || t('marketNoDescription') }}</p>
                  </div>

                  <div class="detail-section">
                    <div class="section-label">
                      {{ t('marketComponents') }}
                      <span v-if="totalComponents" class="section-count">{{ totalComponents }}</span>
                    </div>
                    <div v-if="compGroups.length" class="comp-grid">
                      <div v-for="g in compGroups" :key="g.kind" class="comp-card">
                        <div class="comp-head">
                          <span class="comp-dot" :data-kind="g.kind" />
                          <span class="comp-name">{{ g.label }}</span>
                          <span class="comp-num">{{ g.items.length }}</span>
                        </div>
                        <div class="comp-tags">
                          <span v-for="item in g.items" :key="item" class="tag">{{ item }}</span>
                        </div>
                      </div>
                    </div>
                    <p v-else class="comp-none">{{ t('marketNoComponents') }}</p>
                  </div>
                </div>

                <div class="detail-foot">
                  <div class="scope-field">
                    <label class="foot-label">{{ t('marketScope') }}</label>
                    <select v-model="scope" class="field field-select" :disabled="anyBusy">
                      <option value="user">{{ t('scopeUser') }}</option>
                      <option v-if="hasProject" value="project">{{ t('scopeProject') }}</option>
                      <option v-if="hasProject" value="local">{{ t('scopeLocal') }}</option>
                    </select>
                  </div>
                  <div class="foot-actions">
                    <template v-if="!detail.installed">
                      <button
                        class="btn primary"
                        :disabled="anyBusy"
                        @click="install(detail)"
                      >
                        {{ busyOn(detail, 'install') ? t('marketUpdating') : t('marketInstall') }}
                      </button>
                    </template>
                    <template v-else>
                      <button class="btn" :disabled="anyBusy" @click="updateEntry(detail)">
                        {{ busyOn(detail, 'update') ? t('marketUpdating') : t('marketUpdate') }}
                      </button>
                      <button class="btn danger" :disabled="anyBusy" @click="uninstall(detail)">
                        {{ busyOn(detail, 'uninstall') ? t('marketUpdating') : t('marketUninstall') }}
                      </button>
                    </template>
                  </div>
                  <p v-if="actionError && errorScope === 'plugin'" class="inline-error foot-error">{{ actionError }}</p>
                </div>
              </template>

              <div v-else class="detail-empty">
                <div class="empty-mark">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round">
                    <path d="M2 7l1.5-4h17L22 7" />
                    <path d="M2 7a2.4 2.4 0 0 0 4.8 0 2.4 2.4 0 0 0 4.8 0 2.4 2.4 0 0 0 4.8 0 2.4 2.4 0 0 0 4.8 0" />
                    <path d="M4 10v9a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-9" />
                    <path d="M9 20v-5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v5" />
                  </svg>
                </div>
                <p class="empty-hint">{{ t('marketDetailEmpty') }}</p>
              </div>
            </section>
          </div>

          <!-- 进行中：统一进度条（添加/刷新/安装都可能等几十秒，避免"卡住"错觉） -->
          <div v-if="busyKey" class="busy-bar">
            <span class="busy-spinner" />
            <span class="busy-text">{{ busyLabel }}</span>
            <span class="busy-sec">{{ t('marketBusyElapsed', { s: busySeconds }) }}</span>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/stores/app'
import {
  addPluginMarketplace,
  getPluginMarketplaces,
  installPlugin,
  refreshPluginMarketplace,
  removePluginMarketplace,
  registerPluginMarketplace,
  uninstallPlugin,
  updatePlugin,
  type MarketplaceComponents,
  type MarketplaceInfo,
  type MarketplacePluginEntry,
} from '@/api/tauri'
import { useSidebarStore } from '@/stores/sidebar'
import { uiConfirm } from '@/composables/useConfirm'
import { useIsMobile, useMediaQuery } from '@/composables/useMediaQuery'
import { CCBOX_MARKET, filterPlugins, isIconUrl, orderMarkets } from '../editor/model'

const { t } = useI18n()
const appStore = useAppStore()
const sidebarStore = useSidebarStore()
/** 手机（触屏窄屏）：底部抽屉形态 */
const isMobile = useIsMobile()
/** 窄视口（含桌面窄窗）：市场栏 + 列表 / 详情钻取，避免三栏挤成一条 */
const isNarrow = useMediaQuery('(max-width: 860px)')

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  'update:visible': [v: boolean]
}>()

const marketplaces = ref<MarketplaceInfo[]>([])
const selected = ref<MarketplaceInfo | null>(null)
const detail = ref<MarketplacePluginEntry | null>(null)
/** 窄屏市场抽屉开关 */
const railOpen = ref(false)
const query = ref('')
const scope = ref<'user' | 'project' | 'local'>('user')
const loadError = ref('')
const actionError = ref('')
const errorScope = ref<'add' | 'plugin' | 'market' | ''>('')
const loading = ref(false)
/** 单动作忙标记：只有发起动作的行/按钮显示进行中，其余仅禁用 */
const busyKey = ref('')
const adding = ref(false)
const newSource = ref('')

const hasProject = computed(() => !!appStore.cwd)
/** 三段市场分组：我的市场（用户添加）/ Claude 注册 / 精选推荐（未添加） */
const myMarkets = computed(() => orderMarkets(marketplaces.value.filter(m => m.origin === 'user')))
const claudeMarkets = computed(() => marketplaces.value.filter(m => m.origin === 'registered'))
const presetMarkets = computed(() => marketplaces.value.filter(m => m.origin === 'preset'))
/** 默认选中：我的市场第一个（ccbox-market 置顶），否则 Claude 注册第一个 */
const defaultMarket = computed(
  () => myMarkets.value[0] ?? claudeMarkets.value[0] ?? null
)
const filtered = computed(() => (selected.value ? filterPlugins(selected.value.plugins, query.value) : []))
const anyBusy = computed(() => !!busyKey.value)
const headStat = computed(() =>
  t('marketStat', {
    markets: marketplaces.value.filter(m => m.origin !== 'preset').length,
    plugins: marketplaces.value.reduce((n, m) => n + m.plugins.length, 0),
  })
)

/** 组件分组（按展示优先级：skills（含 commands，老版形态不区分）→ mcp → agents → hooks → lsp） */
const COMP_DEFS: Array<{ kind: string; labelKey: string; field: keyof MarketplaceComponents }> = [
  { kind: 'skills', labelKey: 'marketCompSkills', field: 'skills' },
  { kind: 'mcpServers', labelKey: 'marketCompMcp', field: 'mcpServers' },
  { kind: 'agents', labelKey: 'marketCompAgents', field: 'agents' },
  { kind: 'hooks', labelKey: 'marketCompHooks', field: 'hooks' },
  { kind: 'lspServers', labelKey: 'marketCompLsp', field: 'lspServers' },
]

const compGroups = computed(() => {
  const c = detail.value?.components
  if (!c) return []
  const groups = COMP_DEFS.map(d => ({ kind: d.kind, label: t(d.labelKey), items: [...c[d.field]] }))
  // commands 是老版 skill 形态，并入 skills 展示不区分
  groups[0].items.push(...c.commands)
  return groups.filter(g => g.items.length > 0)
})

const totalComponents = computed(() => compGroups.value.reduce((n, g) => n + g.items.length, 0))

// ---------- 进行中进度（CLI 同步阻塞，用已耗时给出确定反馈） ----------
const busySince = ref(0)
const nowMs = ref(0)
let busyTimer: ReturnType<typeof setInterval> | undefined

watch(busyKey, key => {
  if (key) {
    busySince.value = Date.now()
    nowMs.value = Date.now()
    busyTimer = setInterval(() => (nowMs.value = Date.now()), 1000)
  } else if (busyTimer) {
    clearInterval(busyTimer)
    busyTimer = undefined
  }
})

onUnmounted(() => {
  if (busyTimer) clearInterval(busyTimer)
})

const busySeconds = computed(() =>
  busySince.value ? Math.max(0, Math.floor((nowMs.value - busySince.value) / 1000)) : 0
)

const busyLabel = computed(() => {
  const key = busyKey.value
  if (key.startsWith('market:add')) return t('marketAddProgressLabel')
  if (key === 'market:refresh') return t('marketRefreshProgressLabel')
  if (key === 'market:remove') return t('marketRemoveProgressLabel')
  return t('marketInstallProgressLabel')
})

watch(
  () => props.visible,
  v => {
    if (v) void fetchMarkets()
    else railOpen.value = false
  }
)

function close() {
  railOpen.value = false
  emit('update:visible', false)
}

/** 远程图标加载失败：隐藏整个图标位，避免空框 */
function hideIcon(e: Event) {
  ;(e.target as HTMLElement).closest('.p-icon')?.classList.add('broken')
}

/** 安装量紧凑展示：1234 → 1.2k，12345 → 12k，1200000 → 1.2M */
function formatInstalls(n?: number): string {
  if (!n || n < 0) return ''
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`
  if (n >= 10_000) return `${Math.round(n / 1000)}k`
  if (n >= 1000) return `${(n / 1000).toFixed(1)}k`
  return String(n)
}

/** ISO 时间 → YYYY-MM-DD（只用于展示本地缓存新旧） */
function formatDate(iso: string): string {
  const d = new Date(iso)
  return Number.isNaN(d.getTime()) ? iso.slice(0, 10) : d.toISOString().slice(0, 10)
}

const NETWORK_ERROR_HINTS = [
  /connection (was )?reset/i,
  /timed out/i,
  /timeout/i,
  /failed to connect/i,
  /could not resolve/i,
  /network is unreachable/i,
  /unable to access/i,
]

/** 网络类失败补一句可执行的提示（刷新/添加市场都要访问 GitHub） */
function friendlyError(e: unknown): string {
  const msg = String(e)
  if (NETWORK_ERROR_HINTS.some(re => re.test(msg))) {
    return `${msg}\n${t('marketNetworkHint')}`
  }
  return msg
}

function pluginActionKey(p: MarketplacePluginEntry, action: string) {
  return `${selected.value?.name ?? ''}/${p.name}/${action}`
}

function busyOn(p: MarketplacePluginEntry, action: string) {
  return busyKey.value === pluginActionKey(p, action)
}

async function fetchMarkets() {
  loadError.value = ''
  if (!marketplaces.value.length) loading.value = true
  try {
    // 后端已合并三来源（我的/Claude 注册/精选推荐；推荐获取失败只影响 preset 分组）
    const markets = await getPluginMarketplaces()
    marketplaces.value = markets
    if (selected.value) {
      selected.value = marketplaces.value.find(m => m.name === selected.value!.name) ?? null
      if (detail.value && selected.value) {
        detail.value = selected.value.plugins.find(p => p.name === detail.value!.name) ?? null
      }
    }
    // 首次打开默认选中我的市场第一个（ccbox-market 置顶），避免右侧空白
    if (!selected.value && defaultMarket.value) {
      selected.value = defaultMarket.value
    }
  } catch (e) {
    loadError.value = String(e)
  } finally {
    loading.value = false
  }
}

function selectMarket(m: MarketplaceInfo) {
  selected.value = m
  detail.value = null
  query.value = ''
  actionError.value = ''
  railOpen.value = false
}

function toggleDetail(p: MarketplacePluginEntry) {
  actionError.value = ''
  detail.value = detail.value?.name === p.name ? null : p
}

function startAdding() {
  adding.value = true
  actionError.value = ''
  newSource.value = ''
}

async function addPreset(preset: { name: string; source?: string }) {
  if (!preset.source) return
  let added = ''
  const ok = await runAction('market', `market:add:${preset.name}`, async () => {
    added = await addPluginMarketplace(preset.source!)
  })
  if (ok && added) selectByName(added)
}

async function confirmAdd() {
  const source = newSource.value.trim()
  if (!source) return
  let added = ''
  const ok = await runAction('add', 'market:add', async () => {
    added = await addPluginMarketplace(source)
  })
  if (ok) {
    adding.value = false
    newSource.value = ''
    if (added) selectByName(added)
  }
}

/** 添加市场后按清单里的市场名定位（CLI 以 marketplace.json 的 name 为注册键） */
function selectByName(candidate: string) {
  const hit = marketplaces.value.find(m => m.name.toLowerCase() === candidate.toLowerCase())
  if (hit) selectMarket(hit)
}

/** 只刷新当前市场（全量刷新会把每个市场都 clone/pull一遍，通常几十秒以上） */
async function refreshSelected() {
  const target = selected.value?.name
  if (!target) return
  await runAction('market', 'market:refresh', () => refreshPluginMarketplace(target))
}

/** 移除该市场 = 仅从 CC-Box 列表移除（含浏览缓存；不动 Claude 注册） */
async function removeMarket(m: MarketplaceInfo) {
  if (!(await uiConfirm(t('marketRemoveListConfirm', { name: m.name }), { danger: true }))) return
  const ok = await runAction('market', 'market:remove', () =>
    removePluginMarketplace(m.name, false)
  )
  if (ok && m.origin !== 'user') selected.value = null
}

/** 注册该市场 = 显式注册到 Claude（未注册时显示；已注册幂例成功） */
async function registerMarket(m: MarketplaceInfo) {
  if (m.registered) return
  if (!(await uiConfirm(t('marketRegisterConfirm', { name: m.name })))) return
  await runAction('market', 'market:register', () => registerPluginMarketplace(m.name))
}

/** 卸载市场 = 从 Claude 注销（列表条目保留，仍可浏览） */
async function unregisterMarket(m: MarketplaceInfo) {
  if (!(await uiConfirm(t('marketRemoveConfirm', { name: m.name }), { danger: true }))) return
  await runAction('market', 'market:unregister', () => removePluginMarketplace(m.name, true))
}

function scopeArg(): string | undefined {
  return scope.value !== 'user' ? appStore.cwd : undefined
}

async function install(p: MarketplacePluginEntry) {
  if (!selected.value) return
  const scopeLabel = t(
    scope.value === 'user' ? 'scopeUser' : scope.value === 'project' ? 'scopeProject' : 'scopeLocal'
  )
  const msg = t('marketInstallConfirm', { name: p.name, market: selected.value.name, scope: scopeLabel })
  if (!(await uiConfirm(msg, { okText: t('marketInstall') }))) return
  const ref = `${p.name}@${selected.value.name}`
  // 未注册市场：安装时后端按 market_source 自动注册到 Claude
  const marketSource = selected.value.registered ? undefined : selected.value.source
  const ok = await runAction('plugin', pluginActionKey(p, 'install'), () =>
    installPlugin(ref, scope.value, scopeArg(), marketSource)
  )
  if (ok && appStore.cwd) sidebarStore.loadPlugins(appStore.cwd, { silent: true })
}

async function uninstall(p: MarketplacePluginEntry) {
  if (!selected.value) return
  if (!(await uiConfirm(t('marketUninstallConfirm', { name: p.name }), { danger: true, okText: t('marketUninstall') }))) return
  const ref = `${p.name}@${selected.value.name}`
  const ok = await runAction('plugin', pluginActionKey(p, 'uninstall'), () =>
    uninstallPlugin(ref, scope.value, scopeArg())
  )
  if (ok && appStore.cwd) sidebarStore.loadPlugins(appStore.cwd, { silent: true })
}

async function updateEntry(p: MarketplacePluginEntry) {
  if (!selected.value) return
  const ref = `${p.name}@${selected.value.name}`
  await runAction('plugin', pluginActionKey(p, 'update'), () =>
    updatePlugin(ref, scope.value, scopeArg())
  )
}

/** 统一动作包装：单动作忙标记 + 错误归属 + 完成后重拉市场列表 */
async function runAction(
  scopeOfError: 'add' | 'plugin' | 'market',
  key: string,
  fn: () => Promise<unknown>
): Promise<boolean> {
  if (busyKey.value) return false
  busyKey.value = key
  actionError.value = ''
  errorScope.value = scopeOfError
  try {
    await fn()
    await fetchMarkets()
    return true
  } catch (e) {
    actionError.value = friendlyError(e)
    return false
  } finally {
    busyKey.value = ''
  }
}
</script>

<style scoped>
/* ---------- 外壳 ---------- */
.market-overlay {
  position: fixed;
  inset: 0;
  background: rgba(26, 24, 22, 0.42);
  backdrop-filter: blur(2px);
  z-index: 1400;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 28px;
}

.market-card {
  position: relative;
  width: min(1000px, 92vw);
  height: min(80vh, 700px);
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  box-sizing: border-box;
  box-shadow: 0 24px 64px rgba(26, 24, 22, 0.22), 0 2px 8px rgba(26, 24, 22, 0.08);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ---------- 头部 ---------- */
.market-head {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 18px 22px 16px;
  border-bottom: 1px solid var(--border-light);
  background: linear-gradient(180deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
  flex-shrink: 0;
}

.head-text {
  flex: 1;
  min-width: 0;
}

.head-title {
  margin: 0;
  font-family: var(--font-sans);
  font-size: 17px;
  font-weight: 600;
  letter-spacing: 0.2px;
  color: var(--text-primary);
}

.head-sub {
  margin: 3px 0 0;
  font-size: 11.5px;
  color: var(--text-tertiary);
  letter-spacing: 0.2px;
}

.head-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  padding: 0;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

.icon-btn:hover:not(:disabled) {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.icon-btn:disabled {
  opacity: 0.45;
  cursor: default;
}

.icon-btn.spinning svg {
  animation: spin 0.9s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* ---------- 主体三栏 ---------- */
.market-body {
  flex: 1;
  min-height: 0;
  display: flex;
}

/* 左：市场栏 */
.rail {
  width: 220px;
  box-sizing: border-box;
  flex-shrink: 0;
  border-right: 1px solid var(--border-light);
  padding: 16px 12px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
  background: var(--bg-secondary);
}

.rail-block {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.rail-label {
  font-size: 10.5px;
  font-weight: 600;
  letter-spacing: 0.9px;
  text-transform: uppercase;
  color: var(--text-tertiary);
  padding: 0 8px 6px;
}

.rail-item {
  position: relative;
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 9px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-primary);
  font-size: 12.5px;
  text-align: left;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
}

.rail-item:hover:not(:disabled) {
  background: var(--hover-bg);
}

.rail-item.active {
  background: var(--bg-tertiary);
  color: var(--accent-primary);
  font-weight: 600;
}

.rail-item.active::before {
  content: '';
  position: absolute;
  left: -12px;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 18px;
  border-radius: 0 3px 3px 0;
  background: var(--accent-gold);
}

.rail-item:disabled {
  opacity: 0.55;
  cursor: default;
}

.rail-mark {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--border-dark);
  flex-shrink: 0;
}

.rail-item.active .rail-mark {
  background: var(--accent-gold);
}

.rail-item.broken .rail-mark {
  background: var(--status-error);
}

.rail-name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 我的市场：注册状态徽标（未注册=灰、已注册=金点绿） */
.rail-badge {
  font-size: 9.5px;
  color: var(--text-tertiary);
  border: 1px solid var(--border-color);
  padding: 0 5px;
  border-radius: 8px;
  flex-shrink: 0;
  line-height: 1.5;
}

.rail-badge.reg {
  color: var(--accent-green, #3e9c5c);
  border-color: var(--accent-green, #3e9c5c);
}

.rail-count {
  font-size: 10.5px;
  font-family: var(--font-mono);
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 1px 6px;
  border-radius: 9px;
}

.rail-item.active .rail-count {
  background: var(--bg-primary);
}

.preset-plus {
  font-size: 12px;
  color: var(--accent-gold-dark);
}

.market-ops {
  display: flex;
  gap: 4px;
  margin: 4px 0 0 8px;
}

.remove-market-btn {
  align-self: flex-start;
  padding: 3px 6px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-tertiary);
  font-size: 10.5px;
  cursor: pointer;
  transition: color 0.15s ease, background 0.15s ease;
}

.remove-market-btn:hover:not(:disabled) {
  color: var(--status-error);
  background: var(--hover-bg);
}

/* 注册该市场：绿色系（与移除/卸载的红色系相对） */
.remove-market-btn.register-btn,
.remove-market-btn.register-btn:hover:not(:disabled) {
  color: var(--accent-green, #3e9c5c);
}

.remove-market-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.rm-icon {
  display: none;
}

.rail-empty {
  margin: 2px 8px 0;
  font-size: 11.5px;
  color: var(--text-tertiary);
}

.rail-add {
  margin-top: auto;
}

.add-btn {
  width: 100%;
  padding: 8px 10px;
  border: 1px dashed var(--border-color);
  border-radius: 8px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  text-align: left;
  cursor: pointer;
  transition: color 0.15s ease, border-color 0.15s ease;
}

.add-btn:hover {
  color: var(--text-primary);
  border-color: var(--accent-gold-dark);
}

.add-form {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.add-hint {
  margin: -2px 0 0;
  font-size: 10.5px;
  line-height: 1.5;
  color: var(--text-tertiary);
}

.add-actions {
  display: flex;
  gap: 6px;
}

.rail-error {
  margin-top: 8px;
}

/* 中：插件列表 */
.list-pane {
  width: 300px;
  flex-shrink: 0;
  box-sizing: border-box;
  min-height: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 16px 14px;
  border-right: 1px solid var(--border-light);
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.search-glyph {
  position: absolute;
  left: 10px;
  width: 14px;
  height: 14px;
  color: var(--text-tertiary);
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding: 8px 28px 8px 30px;
  border: 1px solid var(--border-color);
  border-radius: 9px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 12.5px;
  font-family: var(--font-sans);
  outline: none;
  transition: border-color 0.15s ease, background 0.15s ease;
}

.search-input::placeholder {
  color: var(--text-tertiary);
}

.search-input:focus {
  border-color: var(--accent-gold-dark);
  background: var(--bg-primary);
}

.list-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
  padding: 0 2px;
  font-size: 10.5px;
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.meta-sep {
  color: var(--border-dark);
}

.search-clear {
  position: absolute;
  right: 6px;
  width: 18px;
  height: 18px;
  padding: 0;
  border: none;
  border-radius: 50%;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  font-size: 12px;
  line-height: 1;
  cursor: pointer;
}

.plugin-list {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin: 0 -6px;
  padding: 0 6px 4px;
}

.plugin-row {
  display: flex;
  flex-direction: column;
  gap: 3px;
  width: 100%;
  padding: 9px 10px;
  border: 1px solid transparent;
  border-radius: 9px;
  background: transparent;
  text-align: left;
  cursor: pointer;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.plugin-row:hover {
  background: var(--hover-bg);
}

.plugin-row.active {
  background: var(--bg-secondary);
  border-color: var(--border-color);
}

.plugin-row.active .row-name {
  color: var(--accent-primary);
}

.row-title {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

/* 精选图标：emoji/短文本或 http(s) 图片；失败整体隐藏 */
.p-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  width: 26px;
  height: 26px;
  border-radius: 8px;
  background: var(--bg-tertiary);
  overflow: hidden;
  font-size: 14px;
  line-height: 1;
}

.p-icon.large {
  width: 34px;
  height: 34px;
  border-radius: 10px;
  font-size: 18px;
}

.p-icon.small {
  width: 18px;
  height: 18px;
  border-radius: 5px;
  font-size: 11px;
}

.p-icon img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.p-icon.broken {
  display: none;
}

.pin-star {
  flex-shrink: 0;
  font-size: 11px;
  line-height: 1;
  color: var(--accent-gold-dark);
}

.row-name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-family: var(--font-mono);
  font-size: 12.5px;
  font-weight: 500;
  color: var(--text-primary);
}

.row-desc {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  font-size: 11.5px;
  line-height: 1.45;
  color: var(--text-secondary);
}

.row-foot {
  display: flex;
  align-items: center;
  gap: 8px;
  min-height: 14px;
}

.row-version,
.row-cat {
  font-size: 10.5px;
  color: var(--text-tertiary);
  font-family: var(--font-mono);
}

.row-cat::before {
  content: '·';
  margin-right: 8px;
  color: var(--border-dark);
}

.row-installs {
  margin-left: auto;
  font-size: 10.5px;
  font-family: var(--font-mono);
  color: var(--text-tertiary);
  white-space: nowrap;
}

.row-busy {
  margin-left: auto;
  font-size: 10.5px;
  color: var(--accent-gold-dark);
}

.row-installs + .row-busy {
  margin-left: 6px;
}

/* 右：详情 */
.detail-pane {
  flex: 1;
  min-width: 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.detail-head {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 18px 22px 14px;
  border-bottom: 1px solid var(--border-light);
  flex-shrink: 0;
}

.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  padding: 0;
  margin-left: -6px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
}

.back-btn svg {
  width: 18px;
  height: 18px;
}

.back-btn:hover {
  background: var(--hover-bg);
}

.detail-title-wrap {
  flex: 1;
  min-width: 0;
}

.detail-title {
  margin: 0;
  font-family: var(--font-mono);
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  overflow-wrap: anywhere;
}

.detail-chips {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 6px;
  margin-top: 8px;
}

.chip {
  display: inline-flex;
  align-items: center;
  align-self: flex-start;
  width: fit-content;
  max-width: 100%;
  padding: 1px 8px;
  border-radius: 999px;
  font-size: 10.5px;
  line-height: 1.6;
  white-space: nowrap;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  flex: 0 0 auto;
}

.chip.muted {
  background: transparent;
  color: var(--text-tertiary);
  padding-left: 0;
  padding-right: 0;
}

.chip-ok {
  background: rgba(61, 140, 110, 0.12);
  color: var(--status-success);
}

.chip-off {
  background: rgba(138, 134, 128, 0.14);
  color: var(--text-tertiary);
}

.detail-scroll {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 18px 22px 20px;
  display: flex;
  flex-direction: column;
  gap: 22px;
}

.section-label {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 9px;
  font-size: 10.5px;
  font-weight: 600;
  letter-spacing: 0.9px;
  text-transform: uppercase;
  color: var(--text-tertiary);
}

.section-count {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 500;
  letter-spacing: 0;
  padding: 0 5px;
  border-radius: 8px;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
}

.detail-desc {
  margin: 0;
  font-size: 12.5px;
  line-height: 1.7;
  color: var(--text-secondary);
  white-space: pre-line;
}

.comp-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.comp-card {
  padding: 10px 12px;
  border: 1px solid var(--border-light);
  border-radius: 10px;
  background: var(--bg-secondary);
}

.comp-head {
  display: flex;
  align-items: center;
  gap: 7px;
  margin-bottom: 7px;
}

.comp-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

.comp-dot[data-kind='skills'] {
  background: var(--accent-gold);
}

.comp-dot[data-kind='mcpServers'] {
  background: var(--accent-primary);
}

.comp-dot[data-kind='agents'] {
  background: var(--status-success);
}

.comp-dot[data-kind='hooks'] {
  background: var(--status-warning);
}

.comp-dot[data-kind='commands'] {
  background: var(--text-tertiary);
}

.comp-name {
  font-size: 11.5px;
  font-weight: 600;
  color: var(--text-primary);
}

.comp-num {
  margin-left: auto;
  font-family: var(--font-mono);
  font-size: 10.5px;
  color: var(--text-tertiary);
}

.comp-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
}

.tag {
  display: inline-block;
  max-width: 100%;
  padding: 2px 7px;
  border-radius: 5px;
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  font-family: var(--font-mono);
  font-size: 10.5px;
  color: var(--text-secondary);
  overflow-wrap: anywhere;
}

.comp-none {
  margin: 0;
  padding: 12px;
  border: 1px dashed var(--border-color);
  border-radius: 10px;
  font-size: 11.5px;
  line-height: 1.6;
  color: var(--text-tertiary);
}

/* 详情底部操作 */
.detail-foot {
  display: flex;
  align-items: flex-end;
  gap: 12px;
  flex-wrap: wrap;
  padding: 14px 22px;
  border-top: 1px solid var(--border-light);
  background: var(--bg-secondary);
  flex-shrink: 0;
}

.scope-field {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.foot-label {
  font-size: 10.5px;
  letter-spacing: 0.6px;
  text-transform: uppercase;
  color: var(--text-tertiary);
}

.foot-actions {
  display: flex;
  gap: 8px;
  margin-left: auto;
}

.foot-error {
  flex-basis: 100%;
  margin: 0;
}

/* 空态 */
.detail-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 32px;
  color: var(--text-tertiary);
}

.empty-mark {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 52px;
  height: 52px;
  border-radius: 14px;
  border: 1px dashed var(--border-color);
  color: var(--border-dark);
}

.empty-mark svg {
  width: 26px;
  height: 26px;
}

.empty-hint {
  margin: 0;
  font-size: 12px;
  text-align: center;
  line-height: 1.6;
}

.list-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px 12px;
  text-align: center;
  font-size: 12px;
  color: var(--text-tertiary);
}

/* 表单元素 */
.field {
  width: 100%;
  box-sizing: border-box;
  padding: 7px 10px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12.5px;
  font-family: var(--font-sans);
  outline: none;
  transition: border-color 0.15s ease;
}

.field:focus {
  border-color: var(--accent-gold-dark);
}

.field:disabled {
  opacity: 0.55;
  cursor: not-allowed;
}

.field-select {
  width: 132px;
  padding: 6px 8px;
}

.btn {
  padding: 7px 15px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12.5px;
  cursor: pointer;
  transition: background 0.15s ease, border-color 0.15s ease, opacity 0.15s ease;
}

.btn:hover:not(:disabled) {
  background: var(--hover-bg);
}

.btn.primary {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: #fff;
}

.btn.primary:hover:not(:disabled) {
  opacity: 0.9;
}

.btn.ghost {
  border-color: transparent;
  background: transparent;
  color: var(--text-secondary);
}

.btn.ghost:hover:not(:disabled) {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.btn.danger {
  color: var(--status-error);
  border-color: var(--status-error);
  background: transparent;
}

.btn.danger:hover:not(:disabled) {
  background: rgba(196, 92, 74, 0.08);
}

.btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.inline-error {
  margin: 0;
  font-size: 11.5px;
  line-height: 1.5;
  color: var(--status-error);
  overflow-wrap: anywhere;
  white-space: pre-wrap;
}

/* 进出场 */
.market-fade-enter-active,
.market-fade-leave-active {
  transition: opacity 0.18s ease;
}

.market-fade-enter-active .market-card,
.market-fade-leave-active .market-card {
  transition: transform 0.18s ease;
}

.market-fade-enter-from,
.market-fade-leave-to {
  opacity: 0;
}

.market-fade-enter-from .market-card,
.market-fade-leave-to .market-card {
  transform: translateY(6px) scale(0.995);
}

/* ---------- 窄视口（含桌面窄窗）：市场 = 左侧抽屉 + 列表/详情钻取 ---------- */
@media (max-width: 860px) {
  .market-body {
    flex-direction: column;
  }

  /* 市场栏变成覆盖式抽屉：竖向列表，数量多也能读清楚 */
  .rail.drawer {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    width: min(300px, 84%);
    z-index: 6;
    border-right: 1px solid var(--border-color);
    box-shadow: 12px 0 32px rgba(26, 24, 22, 0.18);
    transform: translateX(-101%);
    transition: transform 0.2s ease;
  }

  .rail.drawer.open {
    transform: none;
  }

  .rail-backdrop {
    position: absolute;
    inset: 0;
    z-index: 5;
    background: rgba(26, 24, 22, 0.32);
  }

  .drawer-head {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 2px 4px 8px;
  }

  .drawer-title {
    flex: 1;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.9px;
    text-transform: uppercase;
    color: var(--text-tertiary);
  }

  .rail-block {
    gap: 2px;
  }

  .rail-item {
    padding: 9px 10px;
  }

  .remove-market-btn {
    margin-left: 10px;
    font-size: 11px;
  }

  .list-pane {
    width: 100%;
    border-right: none;
    padding: 12px;
    flex: 1;
  }

  .list-pane.is-hidden {
    display: none;
  }

  .detail-pane:not(.has-detail) {
    display: none;
  }

  .detail-pane.has-detail {
    flex: 1;
    min-height: 0;
  }

  .detail-scroll {
    padding: 14px 16px 18px;
  }

  .detail-head {
    padding: 14px 16px 12px;
  }

  .detail-foot {
    padding: 12px 16px;
  }

  .field-select {
    width: 120px;
  }
}

/* 窄屏头部：市场选择器 */
.market-picker {
  display: flex;
  align-items: center;
  gap: 8px;
  max-width: 100%;
  padding: 7px 10px;
  border: 1px solid var(--border-color);
  border-radius: 9px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
}

.market-picker:hover {
  background: var(--hover-bg);
}

.picker-icon,
.picker-chevron {
  width: 15px;
  height: 15px;
  flex-shrink: 0;
  color: var(--text-tertiary);
}

.picker-name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  text-align: left;
  font-family: var(--font-mono);
}

.picker-count {
  flex-shrink: 0;
  font-size: 10.5px;
  font-family: var(--font-mono);
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 1px 6px;
  border-radius: 9px;
}

/* 抽屉遮罩淡入 */
.rail-fade-enter-active,
.rail-fade-leave-active {
  transition: opacity 0.18s ease;
}

.rail-fade-enter-from,
.rail-fade-leave-to {
  opacity: 0;
}

/* ---------- 进行中进度条 ---------- */
.busy-bar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 22px;
  border-top: 1px solid var(--border-light);
  background: var(--bg-secondary);
  font-size: 12px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.busy-spinner {
  width: 13px;
  height: 13px;
  flex-shrink: 0;
  border: 2px solid var(--border-color);
  border-top-color: var(--accent-gold-dark);
  border-radius: 50%;
  animation: spin 0.9s linear infinite;
}

.busy-text {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.busy-sec {
  font-family: var(--font-mono);
  color: var(--text-tertiary);
}

.add-progress {
  color: var(--accent-gold-dark);
}

/* ---------- 手机：底部抽屉（触屏窄屏） ---------- */
@media (max-width: 768px) and (hover: none) {
  .market-overlay {
    padding: 0;
    align-items: flex-end;
  }

  .market-card,
  .market-card.sheet {
    width: 100%;
    height: 92vh;
    max-height: 92vh;
    border-radius: 16px 16px 0 0;
    border-left: none;
    border-right: none;
    border-bottom: none;
    padding-bottom: var(--safe-area-bottom);
  }

  .market-head {
    padding: 14px 16px 12px;
  }

  /* 触屏目标放大 */
  .plugin-row {
    min-height: 56px;
    padding: 11px 12px;
  }

  .row-name {
    font-size: 13.5px;
  }

  .back-btn {
    width: 34px;
    height: 34px;
    margin-left: -8px;
  }

  .btn {
    min-height: 38px;
    padding: 9px 16px;
  }

  .icon-btn {
    width: 34px;
    height: 34px;
  }

  .field {
    padding: 9px 11px;
  }

  .field-select {
    width: 104px;
  }
}
</style>
