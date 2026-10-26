import { describe, it, expect } from 'vitest'
import { buildTapMouseInit } from '@/utils/tap-mouse'

// tap → 合成 MouseEvent 参数换算（手机端点击切光标补齐路径的纯函数部分）
describe('buildTapMouseInit', () => {
  it('TapMouseInit_ConvertsPageToClientCoords_1', () => {
    const init = buildTapMouseInit(120, 80, 1, 0, 0)
    expect(init).not.toBeNull()
    expect(init!.clientX).toBe(120)
    expect(init!.clientY).toBe(80)
  })

  it('TapMouseInit_ConvertsPageToClientCoords_2', () => {
    // 页面有滚动偏移时扣除 scrollX/scrollY
    const init = buildTapMouseInit(320, 240, 1, 20, 40)
    expect(init!.clientX).toBe(300)
    expect(init!.clientY).toBe(200)
  })

  it('TapMouseInit_RejectsInvalidCoords_1', () => {
    expect(buildTapMouseInit(undefined, 80, 1, 0, 0)).toBeNull()
    expect(buildTapMouseInit(120, undefined, 1, 0, 0)).toBeNull()
  })

  it('TapMouseInit_RejectsInvalidCoords_2', () => {
    // 惯性等派生的手势事件坐标为 NaN/Infinity 时跳过合成
    expect(buildTapMouseInit(NaN, 80, 1, 0, 0)).toBeNull()
    expect(buildTapMouseInit(120, Infinity, 1, 0, 0)).toBeNull()
  })

  it('TapMouseInit_MapsTapCountToDetail_1', () => {
    // 单击 detail=1、双击 detail=2，与浏览器原生事件语义一致
    expect(buildTapMouseInit(0, 0, 1, 0, 0)!.detail).toBe(1)
    expect(buildTapMouseInit(0, 0, 2, 0, 0)!.detail).toBe(2)
    // tapCount 缺失按单击处理
    expect(buildTapMouseInit(0, 0, undefined, 0, 0)!.detail).toBe(1)
  })

  it('TapMouseInit_AlwaysLeftButtonAndBubbling_1', () => {
    const init = buildTapMouseInit(10, 10, 1, 0, 0)!
    expect(init.button).toBe(0)
    expect(init.bubbles).toBe(true)
    expect(init.cancelable).toBe(true)
  })
})
