/** deploy-web 的纯函数（便于测试，不在导入时执行部署） */

/** 远端命令拼接前的文件名白名单：杜绝理论注入面（产物名均由构建工具生成） */
export function isSafeName(name) {
  return /^[\w.-]+$/.test(name)
}

/** dist 顶层条目 → scp 参数（相对路径 + 引号；避免 Windows 盘符冒号被当成远程主机） */
export function scpTargets(entries) {
  return entries.map(e => `"dist/${e}"`).join(' ')
}

/** dist/assets 中的入口 bundle 文件名（Vite 每版只产出一个 index-*.js） */
export function indexBundleOf(files) {
  return files.find(f => /^index-.*\.js$/.test(f)) ?? null
}

/** 从首页 HTML 提取声明的入口 bundle 路径（如 assets/index-xxx.js） */
export function servedBundleFromHtml(html) {
  const m = html.match(/assets\/index-[^"']+\.js/)
  return m ? m[0] : ''
}
