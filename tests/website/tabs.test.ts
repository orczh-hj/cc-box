import { describe, expect, it } from 'vitest'
import { enhanceBlocks } from '../../website/src/lib/tabs'

// Mintlify 标记折叠：tab 组多选一、accordion 原生折叠、callout 提示盒
describe('enhanceBlocks', () => {
  function chip(title: string, level: number, kind: 'tab' | 'accordion' = 'tab') {
    const d = document.createElement('div')
    d.className = 'md-tab-title'
    d.dataset.kind = kind
    d.dataset.level = String(level)
    d.textContent = title
    return d
  }

  function calloutMarker(type: string, title = '') {
    const d = document.createElement('div')
    d.className = 'md-callout-marker'
    d.dataset.type = type
    if (title) d.dataset.title = title
    return d
  }

  function end() {
    const d = document.createElement('div')
    d.className = 'md-block-end'
    return d
  }

  function para(text: string) {
    const p = document.createElement('p')
    p.textContent = text
    return p
  }

  it('同级连续 chip 折叠为一组 tab，默认只显示第一个面板', () => {
    const root = document.createElement('div')
    root.append(
      para('前言'),
      chip('原生安装', 1),
      para('curl 安装'),
      chip('Homebrew', 1),
      para('brew 安装'),
      end(),
      para('结尾'),
    )
    enhanceBlocks(root)

    const widget = root.querySelector('.md-tabs')
    expect(widget).toBeTruthy()
    expect(root.firstChild?.textContent).toBe('前言')

    const btns = [...widget!.querySelectorAll('.md-tabs-btn')]
    expect(btns.map((b) => b.textContent)).toEqual(['原生安装', 'Homebrew'])
    const panels = [...widget!.querySelectorAll('.md-tabs-panel')]
    expect(panels).toHaveLength(2)
    expect(panels[0].classList.contains('active')).toBe(true)
    expect(panels[0].textContent).toContain('curl 安装')
    expect(panels[1].classList.contains('active')).toBe(false)
    // 结束标记后的内容不被吞进面板
    expect(widget!.nextSibling?.textContent).toBe('结尾')
  })

  it('点击 tab 按钮切换面板显示', () => {
    const root = document.createElement('div')
    root.append(chip('A', 1), para('内容 A'), chip('B', 1), para('内容 B'), end())
    enhanceBlocks(root)

    const btns = [...root.querySelectorAll('.md-tabs-btn')] as HTMLElement[]
    const panels = [...root.querySelectorAll('.md-tabs-panel')]
    btns[1].click()
    expect(panels[1].classList.contains('active')).toBe(true)
    expect(panels[0].classList.contains('active')).toBe(false)
  })

  it('嵌套 chip（更深 level）留在上一层面板内并递归折叠', () => {
    const root = document.createElement('div')
    root.append(
      chip('Terminal', 1),
      para('CLI 说明'),
      chip('原生安装', 2),
      para('curl 安装'),
      chip('Homebrew', 2),
      para('brew 安装'),
      chip('VS Code', 1),
      para('插件说明'),
      end(),
    )
    enhanceBlocks(root)

    // 同层级 chip 同组：Terminal / VS Code 为一组平台 tab
    const top = [...root.children].filter((c) =>
      c.classList.contains('md-tabs'),
    ) as HTMLElement[]
    expect(top).toHaveLength(1)
    const topNav = [...top[0].children].find((c) => c.classList.contains('md-tabs-nav'))!
    const topBtns = [...topNav.children] as HTMLElement[]
    expect(topBtns.map((b) => b.textContent)).toEqual(['Terminal', 'VS Code'])

    // Terminal 面板内嵌套一组（原生安装 / Homebrew），VS Code 面板没有
    const topPanels = [...top[0].children].filter((c) =>
      c.classList.contains('md-tabs-panel'),
    ) as HTMLElement[]
    const nestedInFirst = topPanels[0].querySelectorAll('.md-tabs')
    expect(nestedInFirst).toHaveLength(1)
    const nestedBtns = [...nestedInFirst[0].querySelectorAll('.md-tabs-btn')]
    expect(nestedBtns.map((b) => b.textContent)).toEqual(['原生安装', 'Homebrew'])
    expect(nestedInFirst[0].textContent).toContain('curl 安装')
    expect(topPanels[1].querySelector('.md-tabs')).toBeNull()
    expect(topPanels[1].textContent).toContain('插件说明')
  })

  it('accordion chip 折叠为 details，标题在 summary、内容在体内', () => {
    const root = document.createElement('div')
    root.append(
      para('前言'),
      chip('折叠标题', 1, 'accordion'),
      para('折叠内容'),
      end(),
      para('结尾'),
    )
    enhanceBlocks(root)

    const details = root.querySelector('details.md-accordion') as HTMLElement
    expect(details).toBeTruthy()
    expect(details.querySelector('summary')?.textContent).toBe('折叠标题')
    expect(details.querySelector('.md-accordion-body')?.textContent).toBe('折叠内容')
    expect(details.nextSibling?.textContent).toBe('结尾')
  })

  it('callout 标记折叠为提示盒：默认标签按类型，内容不越界', () => {
    const root = document.createElement('div')
    root.append(
      para('前言'),
      calloutMarker('warning'),
      para('警告内容'),
      end(),
      para('结尾'),
    )
    enhanceBlocks(root)

    const box = root.querySelector('.md-callout') as HTMLElement
    expect(box).toBeTruthy()
    expect(box.classList.contains('md-callout-warning')).toBe(true)
    expect(box.querySelector('.md-callout-label')?.textContent).toBe('警告')
    expect(box.querySelector('.md-callout-body')?.textContent).toBe('警告内容')
    expect(box.nextSibling?.textContent).toBe('结尾')
  })

  it('callout 嵌在 tab 面板内：随面板递归折叠，不参与 tab 分组', () => {
    const root = document.createElement('div')
    root.append(
      chip('Terminal', 1),
      para('说明'),
      calloutMarker('note'),
      para('注内容'),
      end(),
      chip('VS Code', 1),
      para('插件'),
      end(),
    )
    enhanceBlocks(root)

    const btns = [...root.querySelectorAll('.md-tabs-btn')]
    expect(btns.map((b) => b.textContent)).toEqual(['Terminal', 'VS Code'])
    const box = root.querySelector('.md-callout')
    expect(box?.textContent).toContain('注内容')
    // callout 位于 Terminal（第一个）面板内
    const panels = [...root.querySelectorAll('.md-tabs-panel')]
    expect(panels[0].contains(box)).toBe(true)
  })
})
