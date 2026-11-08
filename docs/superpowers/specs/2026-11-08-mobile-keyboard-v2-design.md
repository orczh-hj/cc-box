# 手机端键盘三模式（v2 重构）设计

日期：2026-11-08 · 状态：已确认（用户 5 条反馈）

## 变更原因

v1 实现了 QuickKeyBar 增补 + 独立虚拟键盘弹层，但用户真机试用后提出 5 条修订：

1. **弹出键盘的按钮只有一个**，打开后可切换键盘类型
2. **快捷键列表要列全 Claude Code 的快捷键**（来源：`website/content/claude-code-docs/reference/interactive-mode.md` + `configuration/interface/keybindings.md`）
3. **三种类型**：原生键盘、快捷键、电脑键盘
4. **电脑键盘布局与真实键盘一致**
5. **按钮写功能名而非组合键**（如 `Ctrl+J` → 「换行」）

## 设计

### 入口

QuickKeyBar 只保留**一个键盘按钮**（图标）。点击弹出键盘面板，面板顶部三个 tab：

```
[原生键盘] [快捷键] [电脑键盘]
```

- **原生键盘**：唤出系统软键盘（现有行为），再点一次收起
- **快捷键**：分组快捷键列表（滚动），每项显示「功能名 + 键位」
- **电脑键盘**：与真实 PC 键盘一致的完整 QWERTY 布局

### 快捷键清单（主会话可用 = Global + Chat + History + Autocomplete）

只收录主会话内有效的快捷键；排除仅在特定对话框（Select/ThemePicker/DiffDialog/Plugin/Pane/ModelPicker 等）内生效的键。

**常规控制**
| 功能名 | 键位 | 序列 |
|---|---|---|
| 中断 / 清除输入 | Ctrl+C | `\x03` |
| 退出会话 | Ctrl+D | `\x04` |
| 重绘屏幕 | Ctrl+L | `\x0c` |
| 记录查看器 | Ctrl+O | `\x0f` |
| 反向搜索历史 | Ctrl+R | `\x12` |
| 后台运行任务 | Ctrl+B | `\x02` |
| 任务清单 | Ctrl+T | `\x14` |
| 隐藏/恢复提示 | Ctrl+S | `\x13` |
| 暂停进程（Unix） | Ctrl+Z | `\x1a` |
| 提交消息 | Enter | `\r` |
| 换行不提交 | Ctrl+J | `\x0a` |
| 中断 / 关闭对话框 | Esc | `\x1b` |
| 循环权限模式 | Shift+Tab | `\x1b[Z` |
| 切换模型 | Alt+P | `\x1bp` |
| 切换扩展思考 | Alt+T | `\x1bt` |
| 切换快速模式 | Alt+O | `\x1bo` |
| 外部编辑器 | Ctrl+G | `\x07` |
| 粘贴图像 | Alt+V | `\x1bv` |

**文本编辑**
| 功能名 | 键位 | 序列 |
|---|---|---|
| 移到行首 | Ctrl+A | `\x01` |
| 移到行尾 | Ctrl+E | `\x05` |
| 删除到行尾 | Ctrl+K | `\x0b` |
| 删除到行首 | Ctrl+U | `\x15` |
| 删除前一个词 | Ctrl+W | `\x17` |
| 粘贴已删文本 | Ctrl+Y | `\x19` |
| 撤销编辑 | Ctrl+_ | `\x1f` |
| 前移一个词 | Alt+B | `\x1bb` |
| 后移一个词 | Alt+F | `\x1bf` |
| 删除到词尾 | Alt+D | `\x1bd` |
| 删除字符 | Delete | `\x1b[3~` |
| 退格 | Backspace | `\x7f` |

**历史与补全**
| 功能名 | 键位 | 序列 |
|---|---|---|
| 上一条历史 | ↑ | `\x1b[A` |
| 下一条历史 | ↓ | `\x1b[B` |
| 接受补全 | Tab | `\t` |

**和弦（连发即可，间隔必然 <3s）**
| 功能名 | 键位 | 序列 |
|---|---|---|
| 停止所有后台代理 | Ctrl+X Ctrl+K | `\x18\x0b` |
| 立即发送排队消息 | Ctrl+X Ctrl+S | `\x18\x13` |
| 排队提交 | Ctrl+X Enter | `\x18\r` |
| 外部编辑器（和弦式） | Ctrl+X Ctrl+E | `\x18\x05` |
| 折叠/展开输入上方区域 | Ctrl+X Ctrl+A | `\x18\x01` |

### 电脑键盘布局（真实 PC 键盘）

```
Esc F1 F2 ... F12                              (功能排)
` 1 2 3 4 5 6 7 8 9 0 - = ⌫                    (数字排)
Tab Q W E R T Y U I O P [ ] \                  (第一排)
CapsLock A S D F G H J K L ; ' Enter           (第二排)
Shift Z X C V B N M , . / Shift                (第三排)
Ctrl Win Alt   Space   Alt Win  ←↑↓→           (底排)
```

- 粘滞修饰键（Ctrl/Alt/Shift/Win）沿用 v1 状态机
- 符号键（数字排 Shift 符号）沿用 `vkeyToBytes` 映射
- 宽度：主键区 flex 自适应，功能排与数字排 13 格等分

### 工具条（QuickKeyBar）

只保留：**键盘按钮（唯一入口，图标）** + **选择复制**。其余快捷键进入面板（避免与面板重复，工具条更清爽）。

## 非目标（不变）

- 不实现系统级按键注入（Alt+Tab / Win 键等 OS 级组合，PTY 场景无意义）
- 不做快捷键自定义（Claude CLI 的 `~/.claude/keybindings.json` 已覆盖）
- 桌面端不渲染

## 实现分工

- `src/utils/virtual-keys.ts`：保留（物理键→ANSI），**新增符号键全集**
- `src/config/claude-shortcuts.ts`：**新增**，快捷键数据（功能名 i18n key + 键位 + 序列 + 分组）
- `src/components/KeyboardPanel.vue`：**新增**，三 tab 面板
- `src/components/VirtualKeyboard.vue`：**改写**为「电脑键盘」布局（真实 QWERTY）
- `src/components/QuickKeyBar.vue`：**简化**为单键盘按钮 + 选择复制
- `src/components/TerminalView.vue`：接线三模式状态
