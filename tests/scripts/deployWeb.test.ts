import { describe, it, expect } from 'vitest'
import { scpTargets, indexBundleOf, servedBundleFromHtml, isSafeName } from '../../scripts/lib/deploy-web.mjs'

describe('deploy-web 路径与校验辅助', () => {
  // 远端命令拼接前的文件名白名单：杜绝理论注入面
  it('DeployWeb_IsSafeName_000', () => {
    expect(isSafeName('index-BpNvwc7t.js')).toBe(true)
    expect(isSafeName('assets')).toBe(true)
    expect(isSafeName('icon.png')).toBe(true)
    expect(isSafeName('a;b')).toBe(false)
    expect(isSafeName('$(rm -rf /)')).toBe(false)
    expect(isSafeName('a b')).toBe(false)
  })

  // scp 参数用相对路径 + 引号，避免 Windows 盘符冒号被 scp 当成远程主机
  it('DeployWeb_ScpTargets_001', () => {
    expect(scpTargets(['assets', 'icon.png', 'index.html']))
      .toBe('"dist/assets" "dist/icon.png" "dist/index.html"')
  })

  // 只识别 Vite 入口 bundle，排除 CSS 与 vendor 分块
  it('DeployWeb_IndexBundle_002', () => {
    const files = ['vendor-abc.js', 'index-DM_5t05U.css', 'index-BpNvwc7t.js', 'xterm-vendor-x.js']
    expect(indexBundleOf(files)).toBe('index-BpNvwc7t.js')
    expect(indexBundleOf(['vendor-abc.js'])).toBeNull()
  })

  // 从首页 HTML 提取线上实际引用的入口 bundle
  it('DeployWeb_ServedBundle_003', () => {
    expect(servedBundleFromHtml('<script type="module" src="/assets/index-BpNvwc7t.js"></script>'))
      .toBe('assets/index-BpNvwc7t.js')
    expect(servedBundleFromHtml('<html></html>')).toBe('')
  })
})
