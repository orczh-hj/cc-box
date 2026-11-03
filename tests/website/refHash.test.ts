import { describe, expect, it } from 'vitest'
import { parseRefHash } from '../../website/src/lib/refHash'

// reference.html 路由 hash 解析：#/<key>?a=<编码锚点>；非 / 开头视为页内锚点（维持现有行为）
// 真值来源：计划锁定的 URL 形态与 encodeURIComponent 语义

describe('parseRefHash', () => {
  it('ParseRefHash_1 纯 key 路由', () => {
    expect(parseRefHash('#/getting-started/overview')).toEqual({ key: 'getting-started/overview' })
  })

  it('ParseRefHash_2 key + 编码锚点', () => {
    expect(parseRefHash('#/a/b?a=' + encodeURIComponent('安装'))).toEqual({ key: 'a/b', anchor: '安装' })
  })

  it('ParseRefHash_3 无 ?a 的裸查询参数不产生锚点，key 剥到 ? 前', () => {
    expect(parseRefHash('#/a/b?x')).toEqual({ key: 'a/b' })
  })

  it('ParseRefHash_4 尾部斜杠剥除', () => {
    expect(parseRefHash('#/a/')).toEqual({ key: 'a' })
  })

  it('ParseRefHash_5 非 / 开头 = 页内锚点，key 为空', () => {
    expect(parseRefHash('#安装')).toEqual({ key: '' })
    expect(parseRefHash('')).toEqual({ key: '' })
  })
})
