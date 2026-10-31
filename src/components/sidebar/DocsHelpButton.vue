<template>
  <button class="action-btn" :title="t('viewDocs')" @click="openExternal(url)">
    <img src="@/assets/icons/help.svg" :alt="t('viewDocs')" />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { openExternal } from '@/api/platform'

const props = defineProps<{
  /** 官网 docs.html 中对应章节的锚点（claude-code.md 的 h2 id） */
  anchor: string
}>()

const { t } = useI18n()

const url = computed(() => `https://ccbox.orczh.top/docs.html#${props.anchor}`)
</script>
