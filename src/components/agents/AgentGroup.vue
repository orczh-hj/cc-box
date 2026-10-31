<template>
  <div class="agent-group">
    <!-- Group Header -->
    <div class="group-header" @click="$emit('toggle')">
      <img
        class="expand-icon"
        :class="{ expanded }"
        src="@/assets/icons/chevron.svg"
        alt="Toggle"
      />
      <span class="group-title">{{ title }}</span>
      <span class="group-count">{{ count }}</span>
    </div>

    <!-- Agents List -->
    <div v-if="expanded" class="agents-container">
      <AgentItemCard
        v-for="agent in agents"
        :key="agent.name + agent.sourceType"
        :agent="agent"
      @edit="$emit('edit', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AgentInfo } from '@/types'
import AgentItemCard from './AgentItem.vue'

defineProps<{
  title: string
  expanded: boolean
  count: number
  agents: AgentInfo[]
}>()

defineEmits<{
  toggle: []
  edit: [agent: AgentInfo]
}>()
</script>

<style scoped>
.agent-group {
  display: flex;
  flex-direction: column;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  user-select: none;
}

.group-header:hover {
  background: var(--hover-bg);
}

.expand-icon {
  width: 14px;
  height: 14px;
  color: var(--text-secondary);
  flex-shrink: 0;
  transition: transform 0.2s ease;
}

.expand-icon.expanded {
  transform: rotate(90deg);
}

.group-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.group-count {
  font-size: 11px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 2px 6px;
  border-radius: 10px;
}

.agents-container {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 4px 12px 8px 12px;
}
</style>