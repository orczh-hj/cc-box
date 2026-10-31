// 将渲染后文档中的 Mintlify 标记节点折叠为交互组件：
// - .md-tab-title（data-kind=tab）   同层级连续 chip → 多选一 tab
// - .md-tab-title（data-kind=accordion）单个标题 → 原生 <details> 折叠段
// - .md-callout-marker（data-type）  → 提示盒（NOTE/TIP/WARNING/CAUTION/IMPORTANT）
// - .md-block-end                    → 边界节点（消费一个，界定内容范围）
// 嵌套标记位于外层组件面板内部，递归处理。

interface TabChip {
  kind: 'tab' | 'accordion'
  level: number
  title: string
}

const CALLOUT_LABELS: Record<string, string> = {
  note: '注',
  tip: '提示',
  warning: '警告',
  caution: '谨慎',
  important: '重要',
}

function asTabChip(node: Node): TabChip | null {
  if (!(node instanceof HTMLElement) || !node.classList.contains('md-tab-title')) return null
  const level = Number(node.dataset.level ?? '1')
  return {
    kind: (node.dataset.kind === 'accordion' ? 'accordion' : 'tab'),
    level: Number.isFinite(level) ? level : 1,
    title: node.textContent ?? '',
  }
}

function asCallout(node: Node): { type: string; title: string } | null {
  if (!(node instanceof HTMLElement) || !node.classList.contains('md-callout-marker')) return null
  return { type: node.dataset.type ?? 'note', title: node.dataset.title ?? '' }
}

function isBlockEnd(node: Node): boolean {
  return node instanceof HTMLElement && node.classList.contains('md-block-end')
}

/** 任何标记节点都可作为内容边界（更内层的组件在面板内递归构建） */
function isAnyMarker(node: Node): boolean {
  return asTabChip(node) !== null || asCallout(node) !== null
}

/** 收集从 start+1 开始的内容节点，直到结束节点（消费）、任一标记节点（不消费）或末尾 */
function collectPanel(kids: Node[], start: number): { panel: Node[]; end: number } {
  const panel: Node[] = []
  let j = start + 1
  while (j < kids.length) {
    if (isBlockEnd(kids[j])) {
      j++
      break
    }
    if (isAnyMarker(kids[j])) break
    panel.push(kids[j])
    j++
  }
  return { panel, end: j }
}

function makeTabsWidget(titles: string[], panels: Node[][]): HTMLElement {
  const wrap = document.createElement('div')
  wrap.className = 'md-tabs'

  const nav = document.createElement('div')
  nav.className = 'md-tabs-nav'
  const btns = titles.map((t, i) => {
    const b = document.createElement('button')
    b.type = 'button'
    b.className = 'md-tabs-btn'
    b.textContent = t
    b.addEventListener('click', () => {
      btns.forEach((x, k) => x.classList.toggle('active', k === i))
      panelEls.forEach((p, k) => p.classList.toggle('active', k === i))
    })
    nav.appendChild(b)
    return b
  })

  const panelEls = panels.map((nodes) => {
    const p = document.createElement('div')
    p.className = 'md-tabs-panel'
    p.append(...nodes)
    return p
  })

  btns[0]?.classList.add('active')
  panelEls[0]?.classList.add('active')
  wrap.append(nav, ...panelEls)
  return wrap
}

function makeAccordion(title: string, panel: Node[]): HTMLElement {
  const details = document.createElement('details')
  details.className = 'md-accordion'
  const summary = document.createElement('summary')
  summary.textContent = title
  const body = document.createElement('div')
  body.className = 'md-accordion-body'
  body.append(...panel)
  details.append(summary, body)
  return details
}

function makeCallout(type: string, title: string, panel: Node[]): HTMLElement {
  const wrap = document.createElement('div')
  wrap.className = `md-callout md-callout-${type}`
  const label = document.createElement('div')
  label.className = 'md-callout-label'
  label.textContent = title || CALLOUT_LABELS[type] || '注'
  const body = document.createElement('div')
  body.className = 'md-callout-body'
  body.append(...panel)
  wrap.append(label, body)
  return wrap
}

export function enhanceBlocks(root: HTMLElement): void {
  const process = (container: Element) => {
    const kids = [...container.childNodes]
    const out: Node[] = []
    let i = 0
    while (i < kids.length) {
      // 顶层游离的边界哨兵：组件已闭合，直接丢弃
      if (isBlockEnd(kids[i])) {
        i++
        continue
      }
      const chip = asTabChip(kids[i])

      // accordion：单个标题折叠段
      if (chip?.kind === 'accordion') {
        const { panel, end } = collectPanel(kids, i)
        out.push(makeAccordion(chip.title, panel))
        i = end
        continue
      }

      // callout：提示盒
      const callout = asCallout(kids[i])
      if (callout) {
        const { panel, end } = collectPanel(kids, i)
        out.push(makeCallout(callout.type, callout.title, panel))
        i = end
        continue
      }

      // tab：同层级连续 chip 为一组
      if (chip) {
        const level = chip.level
        const titles: string[] = [chip.title]
        const panels: Node[][] = []
        let panel: Node[] = []
        let j = i + 1
        while (j < kids.length) {
          if (isBlockEnd(kids[j])) {
            j++
            break
          }
          const next = asTabChip(kids[j])
          if (next && next.kind === 'tab' && next.level <= level) {
            panels.push(panel)
            panel = []
            if (next.level < level) break // 更浅层级属于外层组，不消费
            titles.push(next.title)
          } else if (isAnyMarker(kids[j])) {
            panel.push(kids[j]) // accordion/callout 留在面板内递归处理
          } else {
            panel.push(kids[j])
          }
          j++
        }
        panels.push(panel)
        out.push(makeTabsWidget(titles, panels))
        i = j
        continue
      }

      out.push(kids[i])
      i++
    }
    container.replaceChildren(...out)

    // 面板与折叠体内可能还有嵌套标记，递归折叠
    container
      .querySelectorAll('.md-tabs-panel, .md-accordion-body, .md-callout-body')
      .forEach(process)
  }
  process(root)
}
