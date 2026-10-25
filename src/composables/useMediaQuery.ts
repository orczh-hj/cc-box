import { ref, onUnmounted, getCurrentInstance, type Ref } from 'vue'

/**
 * 响应式媒体查询（matchMedia 监听变化）。
 * 仅在 CSS 无法表达的"结构变化"处使用（如两套导航条件渲染）；
 * 纯样式分支一律走 global.css / 组件内 @media 断点。
 *
 * 断点约定：手机 = 视口 ≤768px 且主输入无 hover（触屏），
 * 与各 @media (max-width: 768px) and (hover: none) 保持一致。
 */
export function useMediaQuery(query: string): Ref<boolean> {
  const matches = ref(false)

  // jsdom 无 matchMedia（Vitest 环境恒 false）
  if (typeof window !== 'undefined' && typeof window.matchMedia === 'function') {
    const mql = window.matchMedia(query)
    matches.value = mql.matches
    const onChange = (e: MediaQueryListEvent) => {
      matches.value = e.matches
    }
    mql.addEventListener('change', onChange)
    // 非组件 setup 上下文（测试直调）无生命周期钩子，跳过清理
    if (getCurrentInstance()) {
      onUnmounted(() => mql.removeEventListener('change', onChange))
    }
  }

  return matches
}

/** 手机判定：窄视口 + 触屏。只按宽度会把桌面窄窗（分屏右半屏）误判为手机，
 *  hover: none 把鼠标设备排除在外（与 CSS 断点一致）。 */
export const MOBILE_MEDIA_QUERY = '(max-width: 768px) and (hover: none)'

export function useIsMobile(): Ref<boolean> {
  return useMediaQuery(MOBILE_MEDIA_QUERY)
}
