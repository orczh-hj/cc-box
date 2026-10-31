/** 自动更新开关解析：config 缺字段（旧版本写入的 config）= 默认开启 */
export function resolveAutoUpdate(v: boolean | undefined): boolean {
  return v ?? true
}
