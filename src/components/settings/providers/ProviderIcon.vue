/**
 * Provider 图标：优先用 cc-switch 同步的厂商图标资产（svg/png，按 icon 名精确
 * 匹配文件 stem），缺失时回退字母头像。此前字母映射在 ProviderCard 与
 * ProviderPresetPanel 各有一份，统一收敛到本组件。
 */
<template>
  <img v-if="iconUrl" class="provider-img" :src="iconUrl" alt="" />
  <span v-else class="fallback-char">{{ fallbackChar }}</span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  icon?: string
}>()

const modules = import.meta.glob('../../../assets/icons/providers/*', {
  eager: true,
  query: '?url',
  import: 'default',
}) as Record<string, string>

const iconUrl = computed(() => {
  if (!props.icon) return ''
  for (const [p, url] of Object.entries(modules)) {
    const stem = (p.split('/').pop() ?? '').replace(/\.(svg|png)$/i, '')
    if (stem === props.icon) return url
  }
  return ''
})

/** 字母头像兜底映射（无资产/历史条目） */
const ICON_CHARS: Record<string, string> = {
  anthropic: 'A',
  deepseek: 'D',
  zhipu: 'Z',
  baidu: 'B',
  bailian: 'B',
  kimi: 'K',
  stepfun: 'S',
  minimax: 'M',
  doubao: 'D',
  siliconflow: 'S',
  openrouter: 'O',
  gemini: 'G',
  github: 'G',
  aws: 'A',
  aihubmix: 'A',
  modelscope: 'M',
  generic: 'P',
  custom: 'C',
}

const fallbackChar = computed(() => {
  if (!props.icon) return 'P'
  return ICON_CHARS[props.icon] || props.icon[0].toUpperCase()
})
</script>

<style scoped>
.provider-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  display: block;
}

.fallback-char {
  font-size: 15px;
  font-weight: 700;
  line-height: 1;
}
</style>
