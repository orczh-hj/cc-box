import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { mockIPC } from '@tauri-apps/api/mocks'

// t 直接回 key，便于断言文案节点
const mockT = vi.fn((key: string) => key)
vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

// QR 生成与断言无关（jsdom 无 canvas），mock 掉
vi.mock('qrcode', () => ({
  default: { toDataURL: vi.fn(async () => 'data:image/png;base64,mock') },
}))

import RemoteModal from '@/components/RemoteModal.vue'

/** 直连已启用、中继未配置的典型初始状态 */
const LAN_STATUS = {
  enabled: true,
  running: true,
  hostedElsewhere: false,
  port: 8747,
  token: 'abc',
  url: 'http://192.168.1.2:8747/#token=abc',
  relayEnabled: false,
  relayConnected: false,
  relayPairUrl: '',
  relayPairCode: '',
}

/** set_relay_url 后的状态（中继启用、配对码未就绪） */
const RELAY_PENDING = {
  ...LAN_STATUS,
  relayEnabled: true,
  relayConnected: false,
}

/** 中继已就绪（配对码生成） */
const RELAY_READY = {
  ...LAN_STATUS,
  relayEnabled: true,
  relayConnected: true,
  relayPairUrl: 'https://cc.orczh.top/#pair=584744A5',
  relayPairCode: '584744A5',
}

describe('RemoteModal（统一扫码：默认公网中继 + 局域访问复选框）', () => {
  /** 内容经 Teleport 渲染到 body：attachTo + 读 document.body */
  function mountModal() {
    return mount(RemoteModal, {
      props: { visible: true },
      attachTo: document.body,
    })
  }

  function bodyText() {
    return document.body.textContent ?? ''
  }

  beforeEach(() => {
    mockT.mockClear()
    document.body.innerHTML = ''
  })

  // 未配置中继（无环境变量/config）→ 不调 set_relay_url，默认切到局域视图
  it('RemoteModal_NoRelayFallbackLan_001', async () => {
    let relayCalls = 0
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return LAN_STATUS
      if (cmd === 'set_relay_url') { relayCalls++; return RELAY_PENDING }
      if (cmd === 'set_remote_enabled') return LAN_STATUS
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    expect(relayCalls).toBe(0)
    const text = bodyText()
    expect(text).toContain('192.168.1.2')
    expect(text).toContain('remoteQrHint')
    wrapper.unmount()
  })

  // 已配置中继 → 不再重复 set_relay_url；默认视图显示配对提示与中继 URL
  it('RemoteModal_RelayReadyDefaultView_002', async () => {
    let relayCalls = 0
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return RELAY_READY
      if (cmd === 'set_relay_url') { relayCalls++; return RELAY_READY }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    expect(relayCalls).toBe(0)
    // 默认公网中继视图：配对提示 + 中继配对 URL 可见；局域 URL 不显示
    const text = bodyText()
    expect(text).toContain('remoteRelayPairHint')
    expect(text).toContain('https://cc.orczh.top/#pair=584744A5')
    expect(text).not.toContain('192.168.1.2')
    wrapper.unmount()
  })

  // 勾选"局域访问" → 切换为局域直连 URL 与 Wi-Fi 提示
  it('RemoteModal_LanToggle_003', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return RELAY_READY
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    expect(bodyText()).toContain('cc.orczh.top')

    const checkbox = document.body.querySelector('.lan-toggle input[type="checkbox"]') as HTMLInputElement
    checkbox.click()
    await flushPromises()

    const text = bodyText()
    expect(text).toContain('192.168.1.2')
    expect(text).toContain('remoteQrHint')
    expect(text).not.toContain('cc.orczh.top/#pair')
    wrapper.unmount()
  })

  // 宿主制：远程由另一实例提供（running=false + hostedElsewhere=true）→
  // 显示宿主提示 + 共享 token 的局域 URL（可直接扫码）+ 无停止按钮/无模式切换
  it('RemoteModal_HostedElsewhere_005', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') {
        return { ...LAN_STATUS, running: false, hostedElsewhere: true }
      }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    const text = bodyText()
    expect(text).toContain('remoteHostedElsewhere')
    // 宿主的直连 URL 正常展示（token 共享）
    expect(text).toContain('192.168.1.2')
    expect(text).toContain('remoteQrHint')
    // 停的是宿主的服务：非宿主不显示停止；无中继可切（隐藏 lan-toggle）
    expect(text).not.toContain('remoteStop')
    expect(document.body.querySelector('.lan-toggle')).toBeNull()
    wrapper.unmount()
  })

  // 显式开启：面板打开只查状态不自动启动（安全阻断）；点开启按钮才调
  // set_remote_enabled 并显示二维码
  it('RemoteModal_ExplicitEnable_006', async () => {
    let enableCalls = 0
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') {
        // 首次查询未开启；开启后返回运行态
        return enableCalls === 0
          ? { ...LAN_STATUS, enabled: false, running: false }
          : LAN_STATUS
      }
      if (cmd === 'set_remote_enabled') { enableCalls++; return LAN_STATUS }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    // 打开面板：未自动开启，显示提示与开启按钮
    expect(enableCalls).toBe(0)
    let text = bodyText()
    expect(text).toContain('remoteEnableHint')
    expect(text).not.toContain('192.168.1.2')

    const btn = document.body.querySelector('.enable-btn') as HTMLButtonElement
    btn.click()
    await flushPromises()

    expect(enableCalls).toBe(1)
    text = bodyText()
    expect(text).toContain('192.168.1.2')
    expect(text).toContain('remoteStop')
    wrapper.unmount()
  })

  // 停止远程：点击停止 → set_remote_enabled(false) → 面板关闭（安全阻断闭环，
  // 后端联动断开中继连接）
  it('RemoteModal_StopRemote_007', async () => {
    let stopCalls = 0
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return LAN_STATUS
      if (cmd === 'set_remote_enabled') {
        stopCalls++
        return { ...LAN_STATUS, running: false }
      }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    const stopBtn = [...document.body.querySelectorAll('button')]
      .find(b => b.textContent?.includes('remoteStop')) as HTMLButtonElement
    stopBtn.click()
    await flushPromises()

    expect(stopCalls).toBe(1)
    expect(wrapper.emitted('close')).toBeTruthy()
    wrapper.unmount()
  })

  // 配对码一次性：设备 A 消费后，手动「更新连接码」为设备 B 重申新码——
  // 点击按钮 → refresh_relay_pair_code → 新码 URL/二维码替换旧码
  it('RemoteModal_RefreshPairCode_008', async () => {
    const RELAY_READY_B = {
      ...RELAY_READY,
      relayPairUrl: 'https://cc.orczh.top/#pair=99AABBCC',
      relayPairCode: '99AABBCC',
    }
    let refreshCalls = 0
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return RELAY_READY
      if (cmd === 'refresh_relay_pair_code') {
        refreshCalls++
        return RELAY_READY_B
      }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    expect(bodyText()).toContain('584744A5')
    const btn = [...document.body.querySelectorAll('button')]
      .find(b => b.textContent?.includes('remoteRefreshPair')) as HTMLButtonElement
    expect(btn).toBeTruthy()

    btn.click()
    await flushPromises()

    expect(refreshCalls).toBe(1)
    const text = bodyText()
    expect(text).toContain('99AABBCC')
    expect(text).not.toContain('584744A5')
    wrapper.unmount()
  })

  // 更新失败（中继未连接/超时）→ 显示错误，不更新已展示的码
  it('RemoteModal_RefreshPairCodeError_009', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return RELAY_READY
      if (cmd === 'refresh_relay_pair_code') {
        throw new Error('relay not connected')
      }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    const btn = [...document.body.querySelectorAll('button')]
      .find(b => b.textContent?.includes('remoteRefreshPair')) as HTMLButtonElement
    btn.click()
    await flushPromises()

    const text = bodyText()
    expect(text).toContain('relay not connected')
    // 旧码保持展示（未被清空）
    expect(text).toContain('584744A5')
    wrapper.unmount()
  })

  // 已启用中继但配对码未就绪（连接中/已过期）→ 保底默认局域视图（URL 常可见）；
  // 手动切回中继视图才显示"中继连接中…"
  it('RemoteModal_RelayPendingFallbackLan_004', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return RELAY_PENDING
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    // 默认保底：局域 URL 与二维码提示可见
    let text = bodyText()
    expect(text).toContain('192.168.1.2')
    expect(text).toContain('remoteQrHint')

    // 取消勾选"局域访问" → 中继视图显示"连接中"
    const checkbox = document.body.querySelector('.lan-toggle input[type="checkbox"]') as HTMLInputElement
    checkbox.click()
    await flushPromises()
    text = bodyText()
    expect(text).toContain('remoteRelayConnecting')
    expect(text).not.toContain('remoteQrHint')
    wrapper.unmount()
  })

  // 安全逻辑说明：运行态面板必须展示（令牌认证 / 停止即断 / 刷新密钥作废设备）
  it('RemoteModal_SecurityHintShown_010', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return RELAY_READY
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    expect(bodyText()).toContain('remoteSecurityHint')
    wrapper.unmount()
  })

  // 刷新密钥两段式：首次点击仅进入待确认（不调后端、按钮转确认文案），
  // 再次点击才调 rotate_remote_secret 并以返回状态刷新面板
  it('RemoteModal_RotateSecretTwoStep_011', async () => {
    const ROTATED = {
      ...RELAY_READY,
      relayPairUrl: 'https://cc.orczh.top/#pair=NEWCODE1',
      relayPairCode: 'NEWCODE1',
    }
    let rotateCalls = 0
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') return RELAY_READY
      if (cmd === 'rotate_remote_secret') {
        rotateCalls++
        return ROTATED
      }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    const btn = [...document.body.querySelectorAll('button')]
      .find(b => b.textContent?.includes('remoteRotateSecret')) as HTMLButtonElement
    expect(btn).toBeTruthy()

    // 第一次点击：仅待确认，后端未被调用
    btn.click()
    await flushPromises()
    expect(rotateCalls).toBe(0)
    expect(bodyText()).toContain('remoteRotateConfirm')

    // 第二次点击：执行轮换，面板展示新配对码
    btn.click()
    await flushPromises()
    expect(rotateCalls).toBe(1)
    expect(bodyText()).toContain('NEWCODE1')
    expect(bodyText()).not.toContain('584744A5')
    wrapper.unmount()
  })

  // 宿主制：凭证属宿主实例，非宿主不显示刷新密钥（与停止按钮同规则）
  it('RemoteModal_HostedNoRotate_012', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_remote_status') {
        return { ...LAN_STATUS, running: false, hostedElsewhere: true }
      }
      return null
    })

    const wrapper = mountModal()
    await flushPromises()

    const text = bodyText()
    expect(text).not.toContain('remoteRotateSecret')
    expect(text).not.toContain('remoteStop')
    wrapper.unmount()
  })
})
