// 目录 scrollspy：观察各 section，进入视口顶部区域时高亮对应 TOC 项
export function setupScrollSpy(ids: string[]): () => void {
  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (!entry.isIntersecting) continue
        const id = entry.target.id
        document.querySelectorAll('.toc-link').forEach((el) => {
          el.classList.toggle('toc-link-active', (el as HTMLElement).dataset.toc === id)
        })
      }
    },
    { rootMargin: '-15% 0px -70% 0px' },
  )
  ids.forEach((id) => {
    const el = document.getElementById(id)
    if (el) observer.observe(el)
  })
  return () => observer.disconnect()
}
