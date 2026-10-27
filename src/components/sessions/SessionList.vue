<template>
  <div class="session-list">
    <template v-if="groups.length > 1">
      <div v-for="g in groups" :key="g.key" class="instance-group">
        <button class="group-header" type="button" @click="toggleGroup(g.key)">
          <span class="group-arrow" :class="{ collapsed: collapsedGroups.has(g.key) }">▼</span>
          <span class="group-title">{{ g.name }}</span>
          <span class="group-count">{{ g.items.length }}</span>
        </button>
        <template v-if="!collapsedGroups.has(g.key)">
          <SessionItem
            v-for="item in g.items"
            :key="item.id"
            :id="item.id"
            :name="item.name"
            :is-active="item.id === activeId"
            :is-running="item.isRunning"
            :is-stopped="item.isStopped"
            :working="item.working"
            :pending="item.pending"
            :last-active-at="item.lastActiveAt"
            :can-resume="item.canResume"
            :closable="closable && item.isTab"
            :snippet="item.snippet"
            :show-time="item.showTime"
            @switch="(id) => $emit('switch', id)"
            @rename="(id, name) => $emit('rename', id, name)"
            @restart="(id) => $emit('restart', id)"
            @close="(id) => $emit('close', id)"
          />
        </template>
      </div>
    </template>
    <SessionItem
      v-for="item in flatItems"
      :key="item.id"
      :id="item.id"
      :name="item.name"
      :is-active="item.id === activeId"
      :is-running="item.isRunning"
      :is-stopped="item.isStopped"
      :working="item.working"
      :pending="item.pending"
      :last-active-at="item.lastActiveAt"
      :can-resume="item.canResume"
      :closable="closable && item.isTab"
      :snippet="item.snippet"
      :show-time="item.showTime"
      @switch="(id) => $emit('switch', id)"
      @rename="(id, name) => $emit('rename', id, name)"
      @restart="(id) => $emit('restart', id)"
      @close="(id) => $emit('close', id)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import SessionItem from './SessionItem.vue'
import type { TerminalTab, HistorySession } from '@/stores/session'

const props = defineProps<{
  tabs?: TerminalTab[]
  history?: HistorySession[]
  activeId: string | null
  runningTabIds?: string[]
  closable?: boolean
  snippetMap?: Map<string, string>
}>()

defineEmits<{
  switch: [id: string]
  rename: [id: string, name: string]
  restart: [id: string]
  close: [id: string]
}>()

interface ListItem {
  id: string
  name: string
  isRunning: boolean
  isStopped: boolean
  isTab: boolean
  lastActiveAt: number
  canResume?: boolean
  snippet?: string
  working?: boolean
  pending?: boolean
  showTime?: boolean
  /** 多实例远程：tab 所属实例（历史会话不属于任何实例） */
  groupKey?: string
  groupName?: string
}

const items = computed<ListItem[]>(() => {
  const snippets = props.snippetMap

  const tabItems: ListItem[] = (props.tabs ?? []).map(tab => ({
    id: tab.tabId,
    name: tab.name,
    isRunning: tab.status === 'running',
    isStopped: tab.status === 'stopped',
    isTab: true,
    lastActiveAt: tab.lastActiveAt,
    canResume: tab.status === 'stopped' ? !!tab.sessionId : undefined,
    working: tab.status === 'running' ? tab.working : undefined,
    pending: tab.status === 'running' ? tab.pending : undefined,
    showTime: false,
    groupKey: tab.instanceId,
    groupName: tab.instanceName,
  }))

  const historyItems: ListItem[] = (props.history ?? []).map(s => ({
    id: s.sessionId,
    name: s.name,
    isRunning: false,
    isStopped: false,
    isTab: false,
    lastActiveAt: s.lastActiveAt,
    snippet: snippets?.get(s.sessionId),
    showTime: true,
  }))

  return [...tabItems, ...historyItems]
})

interface ItemGroup {
  key: string
  name: string
  items: ListItem[]
}

/** 实例分组（多实例远程）：tab 条目按 instanceId 分组保序；历史会话不参与
 * 分组，始终归入无组名平铺段。只有一组或全部无实例字段时整体平铺（降级） */
const groups = computed<ItemGroup[]>(() => {
  const grouped: ItemGroup[] = []
  for (const item of items.value) {
    if (!item.groupKey) continue
    const name = item.groupName || '桌面实例'
    let g = grouped.find(x => x.key === item.groupKey)
    if (!g) {
      g = { key: item.groupKey, name, items: [] }
      grouped.push(g)
    }
    g.items.push(item)
  }
  return grouped
})

/** 平铺段：无分组或多于一组时 = 历史会话等无组条目；**单组时 = 全部条目**
 * （单实例 Gateway 场景会话全带实例字段但无需组头，与现状平铺一致——
 * 若此处只返回无组条目，单实例的 tabs 会整组丢失） */
const flatItems = computed<ListItem[]>(() => {
  if (groups.value.length <= 1) return items.value
  return items.value.filter(item => !item.groupKey)
})

const collapsedGroups = ref(new Set<string>())

function toggleGroup(key: string): void {
  const next = new Set(collapsedGroups.value)
  if (next.has(key)) {
    next.delete(key)
  } else {
    next.add(key)
  }
  collapsedGroups.value = next
}
</script>

<style scoped>
.session-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.instance-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.group-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 4px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  text-align: left;
  user-select: none;
}
.group-arrow {
  font-size: 10px;
  transition: transform 0.15s ease;
}
.group-arrow.collapsed {
  transform: rotate(-90deg);
}
.group-title {
  font-weight: 600;
}
.group-count {
  padding: 0 6px;
  border-radius: 8px;
  background: var(--bg-secondary);
  font-size: 11px;
}
</style>
