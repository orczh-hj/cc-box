# WebGL Renderer Atlas Corruption 修复方案

> **状态**:已升级到包含上游修复的版本(v0.13.7 起),切 Tab reload 保留为兜底
> **复现项目**:`../xterm-webgl-repro/`(sibling 仓库,可一键复现)
> **上游跟踪**:[xtermjs/xterm.js#6038](https://github.com/xtermjs/xterm.js/issues/6038)(已关闭) · [PR #6042](https://github.com/xtermjs/xterm.js/pull/6042)(已合并) · [PR #6043](https://github.com/xtermjs/xterm.js/pull/6043)(已合并) · [microsoft/vscode#322756](https://github.com/microsoft/vscode/issues/322756)

---

## 1. 问题现象

用户在长时间使用 cc-box 时,终端字符会出现以下乱码表现:

- **方块/豆腐块**:中文位置显示 `□`
- **替换字符**:某些字符变成 `�`
- **错位**:A 位置显示了 B 字符
- **空白**:某些字符位置完全空白
- **字形错误**:中文位置显示 emoji 或符号

**关键特征**:
- buffer 数据**正确**(`term.buffer.active.getLine(i).getCell(j).getChars()` 返回正确字符)
- 屏幕渲染**错误**
- 多 Tab 场景下,切换 Tab 会加剧污染

---

## 2. 根因(直白解释)

### WebGL renderer 的工作方式

xterm.js 的 WebGL renderer 不能直接「画字」,WebGL 只会贴图。所以它维护一张**字符小图册(atlas)**:
- 每个用过的字符都画到 atlas 上
- 渲染时:buffer 说「这里是字符 X」→ 查 atlas 找到 X 的位置 → GPU 贴图到 canvas

### atlas 满了会怎样

atlas 容量有限:默认 4 个 page × 1024 glyph ≈ 4000 字符。

**满了之后**触发 **page merge**:把利用率最高的 4 个 page 合并成 1 个 2 倍大的 page。

### Bug 在 merge 这一步

合并后,**原本指向「page 2 第 5 个位置」的字符引用失效了**(page 2 已被合并到新位置)。

xtermjs/xterm.js#6038 描述的 bug:
- merge 后,**有些 renderer 实例没收到通知**
- 它们还在用过时的 page index 取字符
- → 取到错位纹理 / 别的字符 / 空白

这就是「buffer 数据正确但屏幕显示错乱」的原因——buffer 存的是字符 ID,渲染器从过时坐标取到了错的纹理。

### 上游修复进度

- **2026-07-04** issue #6038 由 VS Code 团队(anthonykim1)提交,关联 microsoft/vscode#322756
- **2026-07-13** [PR #6042](https://github.com/xtermjs/xterm.js/pull/6042) 合并:修复共享 atlas 页合并后 renderer 状态过期(乱码根因)
- **2026-07-15** [PR #6043](https://github.com/xtermjs/xterm.js/pull/6043) 合并:修复 atlas 页数超纹理容量导致渲染循环崩溃
- 修复落在 6.1.0-beta 线(xterm stable 6.0.0 发布于 2025-12,早于修复,无 backport)
- 其他终端项目已跟进验证(bump addon-webgl 到 0.20.0-beta.297+ 的 atlas page eviction)
- **2026-08-28** cc-box 升级到 `@xterm/xterm@6.1.0-beta.303` + `@xterm/addon-webgl@0.20.0-beta.299`

> 早期记录「0.20.0-beta.290 仍乱码(2026-07 验证)」发生在修复合并之前,结论已失效。

---

## 3. 已验证的方案对比

在 `xterm-webgl-repro` 项目中实测 5+ 方案,结果如下:

| 方案 | 效果 | 副作用 | 结论 |
|---|---|---|---|
| `term.refresh(0, rows-1)` 定时刷新 | ❌ 无效 | 无 | **v0.13.4 当前方案,实测不能消除已发生的 corruption** |
| `clearTextureAtlas()` | ❌ 无效 | 无 | 0.19.0 / 0.20.0-beta.290 均无效 |
| `display:none → block`(切 Tab) | ❌ 无效 | 加剧污染 | 切 Tab 反而让其他 Tab 乱码更多 |
| dispose + reload WebGL(手动) | ✅ 当前 Tab 恢复 | ❌ 污染其他 Tab | **v0.13.3 已回滚** |
| 定时 reload 所有 Tab(30s) | ✅ 无乱码 | 闪烁频繁 | 用户接受度低 |
| **reload 非活跃 Tab** | ✅ 活跃 Tab 不受影响 | 无可见副作用 | **关键验证** |
| 监听 `onRemoveTextureAtlasCanvas` 自动 reload | ✅ 无乱码 | aggressive 模式 2s/次 reload | 事件驱动,真实场景下罕见 |
| 升级到 `@xterm/addon-webgl@0.20.0-beta.290` | ❌ 仍乱码 | beta API 不稳 | 上游未修 |

### 关键发现

1. **corruption 一旦发生,只能通过 dispose + reload WebGL 消除**(其他轻量手段均无效)
2. **reload 活跃 Tab 会污染其他 Tab**(共享 atlas 副作用)
3. **reload 非活跃 Tab 不影响活跃 Tab**(已严格验证)
4. **真实场景字符流(Claude CLI 输出)几乎不会触发 corruption**——atlas 不会满,merge 罕见

---

## 4. 选定方案:切 Tab 时主动 reload

### 核心策略

> **每次用户切到一个 Tab,主动 reload 该 Tab 的 WebGL addon**

### 为什么这是最优方案

| 维度 | 评估 |
|---|---|
| 正确性 | ✅ 用户看到的 Tab 永远是干净的(reload 后的) |
| 用户感知 | ✅ 切 Tab 时本来就有切换动画,reload 的几十毫秒重建隐藏在切换过程中,几乎无感 |
| 副作用 | ✅ reload 当前 Tab 时,其他 Tab 处于非活跃状态,即使被短暂污染也用户看不见 |
| 复杂度 | ✅ 极简,只需在已有的 `watch(activeTabId)` 里加一行调用 |
| 依赖 | ✅ 不依赖 atlas merge 事件,不依赖定时器,不依赖版本升级 |
| 上游兼容 | ✅ 上游修复后这个方案也无害(切 Tab 时多一次 reload 而已) |

### 为什么不选其他方案

- **定时 reload 所有 Tab**:每 30s 全屏闪烁,用户烦
- **atlas-merge 监听自动 reload**:在 aggressive 极端场景下 2s 一次 reload,真实场景又几乎不触发(冗余)
- **切走时 reload 旧 Tab**:能解决问题但语义不如「切到时 reload 新 Tab」直接

---

## 5. 实施代码

### 修改位置:`src/components/XTermTerminal.vue`

### Step 1 · 加 WebGL addon 引用管理

`loadRendererAddons` 改造,记录每个 term 对应的 WebglAddon 实例:

```ts
// 跟踪每个 Terminal 的 WebGL addon 实例(用于 reload)
const webglAddons = new WeakMap<Terminal, WebglAddon>()

function loadRendererAddons(term: Terminal) {
  try {
    const unicode11 = new Unicode11Addon()
    term.loadAddon(unicode11)
    term.unicode.activeVersion = '11'
  } catch (err) {
    console.warn('[XTerm] Unicode 11 addon unavailable, fallback to default:', err)
  }

  attachWebgl(term)

  // ⚠️ 移除原来的 setInterval(refresh, 10s)
  // 实测 term.refresh 不能消除已发生的 atlas corruption,详见 docs/webgl-corruption-fix.md
}

function attachWebgl(term: Terminal) {
  try {
    const webglAddon = new WebglAddon()
    webglAddon.onContextLoss(() => {
      try { webglAddon.dispose() } catch { /* 已 dispose */ }
      webglAddons.delete(term)
      // context loss 后延迟重建
      setTimeout(() => attachWebgl(term), 0)
    })
    term.loadAddon(webglAddon)
    webglAddons.set(term, webglAddon)
  } catch (err) {
    console.warn('[XTerm] WebGL init failed, fallback to DOM renderer:', err)
  }
}

/**
 * Reload Terminal 的 WebGL addon。
 * 用途:清除 glyph atlas corruption(详见 docs/webgl-corruption-fix.md)。
 *
 * 副作用:reload 时会触发 atlas page merge,可能短暂污染共享 atlas 的其他 Tab。
 * 因此只在「Tab 即将成为活跃 Tab」时调用 —— 此时其他 Tab 处于不可见状态,
 * 即使被污染用户也看不见。
 */
function reloadWebgl(term: Terminal) {
  const old = webglAddons.get(term)
  if (old) {
    try { old.dispose() } catch { /* ignore */ }
    webglAddons.delete(term)
  }
  attachWebgl(term)
}
```

### Step 2 · 切 Tab 时主动 reload

修改 `watch(() => sessionStore.activeTabId, ...)` 中「切换到已有 Tab」分支:

```ts
watch(() => sessionStore.activeTabId, async (newTabId, oldTabId) => {
  if (!newTabId) return

  if (newTabId === oldTabId) {
    fitCurrentTerminal()
    return
  }

  if (isPtyStarting.value) return

  currentDisplayTabId.value = newTabId

  await nextTick()

  const existingInstance = terminalInstances.get(newTabId)

  if (existingInstance) {
    // ⚡ 切到的 Tab 主动 reload WebGL,清除可能的 atlas corruption
    // 详见 docs/webgl-corruption-fix.md
    reloadWebgl(existingInstance.term)

    const buf = existingInstance.term.buffer.active
    existingInstance.term.refresh(0, Math.max(buf.length - 1, 0))
    requestAnimationFrame(() => existingInstance.fitAddon.fit())
  } else {
    // ... 原有创建新 Terminal 的逻辑
  }
})
```

### Step 3 · 清理过时的 setInterval refresh 代码

- 删除 `loadRendererAddons` 末尾的 `setInterval(() => term.refresh(...), 10_000)`
- 删除 `refreshTimers: WeakMap<Terminal, ReturnType<typeof setInterval>>`
- 删除 `disposeTerminal` 中的 `clearInterval(timer)` 部分

### Step 4 · 更新注释

`XTermTerminal.vue` 中 `loadRendererAddons` 上方的注释(2026-08-28 随上游修复升级更新):

```ts
// 在 term.open(el) 之后加载 Unicode 11 + WebGL addon
//
// glyph atlas corruption(xtermjs/xterm.js#6038)已由上游修复:
// PR #6042(共享 atlas 页合并后 renderer 状态过期)+ PR #6043(atlas 页数超容量),
// 包含在 @xterm/xterm@6.1.0-beta.303 / @xterm/addon-webgl@0.20.0-beta.299。
//
// 每次切到 Tab 时主动 reload 该 Tab 的 WebGL addon(见 reloadWebgl)——保留作兜底:
// beta 线仍可能出现回归,reload 是唯一能消除已发生 corruption 的方式;
// 放在「Tab 切入」时机执行,切换动画遮盖了 reload 的几十毫秒重建,用户无感。
//
// 详见 docs/webgl-corruption-fix.md
```

---

## 6. 测试验证

### 自动化测试

不写自动测试——corruption 是渲染层 bug,buffer 数据正确,无法从 buffer 判断。需要视觉验证。

### 手动测试

记录到 `docs/manual-test-cases.md`,核心场景:

| 场景 | 操作 | 预期 |
|---|---|---|
| 长 CJK 会话 | 单 Tab 跑 Claude 中文对话 10 分钟 | 不出现乱码 |
| 多 Tab 切换 | 3 个 Tab 各跑 Claude,来回切换 10 分钟 | 切到的 Tab 始终干净 |
| 极端字符流 | 多 Tab 跑含 emoji+特殊符号+CJK 混合的输出 | 切到的 Tab 始终干净 |
| 切 Tab 性能 | 快速连续切换 Tab,观察 reload 延迟 | 切换体感无变化(reload < 50ms) |

### 复现项目验证

在 `../xterm-webgl-repro/` 中验证方案:
- 启用 `webgl-smart-reload-on-switch` 策略
- 创建 3-5 个 Tab + mixed 字符流 + 200 行/秒
- 跑 10 分钟,来回切 Tab
- 预期:每次切到的 Tab 都干净,无 reload 体感

---

## 7. 上游跟踪

上游已修复(见第 2 节),cc-box 已升级。后续关注:

| 信号 | 行动 |
|---|---|
| `@xterm/xterm@6.1.0` stable 发布 | 从 beta 切换到 stable(`package.json` 中 beta 为精确版本号) |
| `@xterm/addon-webgl@0.20.0` stable 发布 + 稳定 6 个月以上 | 考虑移除切 Tab reload 兜底,纯依赖上游 |
| beta 线出现新回归 | 回退到已验证版本号即可(beta 均为精确锁定) |

### 升级时适配的 breaking changes(xterm 5 → 6)

- **viewport/滚动条重做**:不再使用 `.xterm-viewport` 原生 webkit 滚动条,改为 VS Code 风格 overlay 自绘滚动条。cc-box 中滚动由 Claude CLI 内部实现,已通过 `scrollbar: { showScrollbar: false }` 隐藏(滚轮滚动 scrollback 不受影响);viewport 的 `#000` 兜底背景以 `--term-bg` CSS 变量覆盖为主题终端背景色,消除底部缝隙黑条
- **alt→ctrl+方向键 hack 移除**(xtermjs/xterm.js#5346):alt+方向键不再默认映射为 ctrl+方向键(按词跳转),需嵌入方自行处理。macOS 上项目用 `macOptionIsMeta: true`,行为以手动测试为准
- ESM 化、overviewRuler 结构变化:项目未受影响(构建与类型检查通过)

---

## 8. 备选方案(如果选定方案未来失效)

### 8.1 atlas-merge 事件驱动 reload

监听 `WebglAddon.onRemoveTextureAtlasCanvas`,触发后 debounce 300ms reload。

优点:精准(只在 corruption 即将发生时 reload)
缺点:aggressive 模式下 2s/次 reload,可能更频繁

### 8.2 定时 reload 非活跃 Tab

每 N 秒只 reload 非活跃 Tab,活跃 Tab 永不主动 reload。

优点:活跃 Tab 用户零感知
缺点:活跃 Tab 长期不被 reload 仍会出 corruption

### 8.3 DOM renderer 降级

CJK 密度检测,超过阈值自动切 DOM renderer。

优点:彻底无 corruption
缺点:DOM renderer 性能差(CPU 渲染),Claude CLI 输出量大时卡顿

---

## 9. 相关链接

- 上游 issue:https://github.com/xtermjs/xterm.js/issues/6038
- 上游 PR:https://github.com/xtermjs/xterm.js/pull/6033
- VS Code 同源问题:https://github.com/microsoft/vscode/issues/322756
- 复现项目:`../xterm-webgl-repro/`(sibling 仓库)
- cc-box 实施位置:`src/components/XTermTerminal.vue`
