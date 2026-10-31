<template>
  <div class="skills-panel">
    <!-- Header -->
    <PanelHeader :title="t('skills')" @close="$emit('close')">
      <template #actions>
        <button class="action-btn" @click="openEditor(null)" :title="t('skillNew')">
          <img src="@/assets/icons/plus.svg" :alt="t('skillNew')" />
        </button>
        <DocsHelpButton anchor="Skills" />
        <button class="action-btn" @click="handleRefresh" :title="t('refreshSkills')">
          <img src="@/assets/icons/refresh.svg" :alt="t('refreshSkills')" />
        </button>
      </template>
    </PanelHeader>

    <div class="panel-content">
      <!-- Concept Description -->
      <div class="panel-desc">{{ t('skillsDesc') }}</div>

      <!-- Loading -->
      <div v-if="loading" class="loading-state">
        <span class="loading-text">{{ t('loadingSkills') }}</span>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="error-state">
        <span class="error-text">{{ error }}</span>
      </div>

      <!-- Empty -->
      <div v-else-if="allSkills.length === 0" class="empty-state">
        <span class="empty-text">{{ t('noSkillsAvailable') }}</span>
        <span class="empty-hint">{{ t('addSkillsHint') }}</span>
      </div>

      <!-- Skills List (按顺序: Project -> User -> Plugin) -->
      <div v-else class="skills-list">
      <!-- Project Skills -->
      <SkillGroup
        v-if="projectSkills.length > 0"
        :title="t('projectSkills')"
        :expanded="sidebarStore.skillsExpandedGroups.project"
        :count="projectSkills.length"
        :skills="projectSkills"
        @toggle="sidebarStore.toggleSkillGroup('project')"
        @edit="openEditor"
      />

      <!-- User Skills -->
      <SkillGroup
        v-if="userSkills.length > 0"
        :title="t('userSkills')"
        :expanded="sidebarStore.skillsExpandedGroups.user"
        :count="userSkills.length"
        :skills="userSkills"
        @toggle="sidebarStore.toggleSkillGroup('user')"
        @edit="openEditor"
      />

      <!-- Plugin Skills -->
      <SkillGroup
        v-if="pluginSkills.length > 0"
        :title="t('pluginSkills')"
        :expanded="sidebarStore.skillsExpandedGroups.plugin"
        :count="pluginSkills.length"
        :skills="pluginSkills"
        @toggle="sidebarStore.toggleSkillGroup('plugin')"
      />
    </div>
    </div>

    <SkillEditor v-model:visible="editorVisible" :skill="editingSkill" @saved="onSaved" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSidebarStore } from '@/stores/sidebar'
import { useAppStore } from '@/stores/app'

const { t } = useI18n()
import type { SkillInfo } from '@/types'
import SkillGroup from './SkillGroup.vue'
import SkillEditor from '../editor/SkillEditor.vue'
import PanelHeader from '../sidebar/PanelHeader.vue'
import DocsHelpButton from '../sidebar/DocsHelpButton.vue'

const sidebarStore = useSidebarStore()
const appStore = useAppStore()

const error = ref<string | null>(null)

// 使用 sidebar store 的数据（已预加载）
const skills = computed(() => sidebarStore.skills)
const loading = computed(() => sidebarStore.skillsLoading)

// 所有 skills
const allSkills = computed(() => skills.value)

// 按 sourceType 分组
const projectSkills = computed(() =>
  skills.value.filter(s => s.sourceType === 'project')
)

const userSkills = computed(() =>
  skills.value.filter(s => s.sourceType === 'user')
)

const pluginSkills = computed(() =>
  skills.value.filter(s => s.sourceType === 'plugin')
)

// ========== 编辑器 ==========
const editorVisible = ref(false)
const editingSkill = ref<SkillInfo | null>(null)

function openEditor(skill: SkillInfo | null) {
  editingSkill.value = skill
  editorVisible.value = true
}

function onSaved() {
  if (appStore.cwd) sidebarStore.loadSkills(appStore.cwd, { silent: true })
}

function handleRefresh() {
  if (appStore.cwd) {
    error.value = null
    sidebarStore.loadSkills(appStore.cwd)
  }
}

onMounted(() => {
  // 如果 sidebar store 还没有数据，触发加载
  if (appStore.cwd && sidebarStore.skills.length === 0) {
    sidebarStore.loadSkills(appStore.cwd)
  }
})
</script>

<style scoped>
.skills-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
}

/* action-btn slot 样式 */
.skills-panel :deep(.action-btn) {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
}

.skills-panel :deep(.action-btn img) {
  width: 16px;
  height: 16px;
}

.skills-panel :deep(.action-btn:hover) {
  color: var(--text-primary);
}

.panel-desc {
  padding: 8px 12px;
  font-size: 12px;
  line-height: 1.5;
  color: var(--text-secondary);
}

.panel-content {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  gap: 8px;
}

.loading-text,
.error-text,
.empty-text {
  font-size: 13px;
  color: var(--text-secondary);
}

.error-text {
  color: var(--error-color);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.skills-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px 0;
}
</style>
