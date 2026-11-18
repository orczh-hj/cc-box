<template>
  <div class="section-content">
    <h2 class="section-heading">{{ t('keyboardShortcuts') }}</h2>

    <div class="shortcuts-group" v-for="group in filteredGroups" :key="group.title">
      <h3 class="group-title">{{ group.title }}</h3>
      <p v-if="group.hint" class="group-hint">{{ group.hint }}</p>
      <div class="shortcuts-table">
        <div class="shortcut-row" v-for="item in group.items" :key="item.key">
          <kbd>{{ item.key }}</kbd>
          <span class="shortcut-desc">{{ item.desc }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { ctrl, alt, cmd, isMac } from '@/utils/platform'

const { t } = useI18n()

const filteredGroups = computed(() => [
  {
    title: t('applicationShortcuts'),
    hint: '',
    items: [
      { key: `${ctrl}+C`, desc: t('shortcut_copySelection') },
      { key: `${ctrl}+Shift+C`, desc: t('shortcut_forceCopy') },
      { key: `${ctrl}+V`, desc: t('shortcut_pasteClipboard') },
      { key: `${cmd}+Shift+N`, desc: t('shortcut_openNewWindow') },
      { key: `${cmd}+Shift+← / →`, desc: t('shortcut_snapWindow') },
      { key: `${cmd}+Shift+R`, desc: t('shortcut_restartApp') },
      { key: `${cmd}+Shift+H`, desc: t('shortcut_toggleHome') },
      { key: `${cmd}+Shift+/`, desc: t('shortcut_showShortcuts') },
      { key: `${cmd}+Shift+S`, desc: t('shortcut_toggleSessions') },
      { key: `${cmd}+Shift+T`, desc: t('shortcut_toggleAlwaysOnTop') },
      { key: `${cmd},`, desc: t('shortcut_toggleSettings') },
      { key: `${cmd}+Plus / −`, desc: t('shortcut_fontSize') },
      { key: `${cmd}+0`, desc: t('shortcut_resetFontSize') },
    ]
  },
  {
    title: t('sessionManagement'),
    hint: t('sessionManagementHint'),
    items: [
      { key: `${alt}+N`, desc: t('shortcut_newSession') },
      { key: `${alt}+R`, desc: t('shortcut_restartSession') },
      { key: `${alt}+W`, desc: t('shortcut_closeTab') },
      { key: `${ctrl}+Tab`, desc: t('shortcut_switchNext') },
      { key: `${ctrl}+Shift+Tab`, desc: t('shortcut_switchPrev') },
      { key: `${alt}+↑ / ↓`, desc: t('shortcut_switchAlt') },
    ]
  },
  {
    title: t('claudeCodeShortcuts'),
    hint: t('claudeCodeShortcutsHint'),
    items: [
      { key: `${ctrl}+C`, desc: t('shortcut_cancelInput') },
      { key: 'Esc', desc: t('shortcut_escapeInterrupt') },
      { key: 'Shift+Tab', desc: t('shortcut_cyclePermissionMode') },
      { key: 'Esc Esc', desc: t('shortcut_rewind') },
      { key: `${alt}+P`, desc: t('shortcut_switchModel') },
      { key: `${ctrl}+O`, desc: t('shortcut_toggleTranscript') },
      { key: `${ctrl}+T`, desc: t('shortcut_toggleTaskList') },
      { key: `${ctrl}+R`, desc: t('shortcut_reverseSearch') },
      { key: `${ctrl}+B`, desc: t('shortcut_backgroundTask') },
      { key: `${ctrl}+L`, desc: t('shortcut_clearPrompt') },
      { key: `${ctrl}+S`, desc: t('shortcut_stashPrompt') },
      { key: `${ctrl}+V / ${alt}+V`, desc: t('shortcut_pasteImage') },
      { key: `${ctrl}+Enter`, desc: t('shortcut_sendQueued') },
      { key: `${alt}+T`, desc: t('shortcut_toggleThinking') },
      { key: `${alt}+O`, desc: t('shortcut_toggleFast') },
      { key: `${ctrl}+G`, desc: t('shortcut_externalEditor') },
      { key: `${ctrl}+_`, desc: t('shortcut_undoInput') },
      { key: `${ctrl}+X ${ctrl}+K`, desc: t('shortcut_killAgents') },
      { key: `${ctrl}+D`, desc: t('shortcut_exitSession') },
    ]
  },
  {
    title: t('textEditing'),
    hint: '',
    items: [
      { key: `${ctrl}+A`, desc: t('shortcut_cursorStart') },
      { key: `${ctrl}+E`, desc: t('shortcut_cursorEnd') },
      { key: `${ctrl}+W`, desc: t('shortcut_deletePrevWord') },
      { key: `${ctrl}+U`, desc: t('shortcut_deleteToStart') },
      { key: `${ctrl}+K`, desc: t('shortcut_deleteToEnd') },
      { key: `${ctrl}+Y`, desc: t('shortcut_pasteDeleted') },
      { key: isMac ? 'Option+Delete' : 'Ctrl+Backspace', desc: t('shortcut_deletePrevWordAlt') },
      { key: `${alt}+B`, desc: t('shortcut_moveBackWord') },
      { key: `${alt}+F`, desc: t('shortcut_moveForwardWord') },
      { key: `${alt}+D`, desc: t('shortcut_deleteNextWord') },
      { key: `${alt}+Y`, desc: t('shortcut_cyclePasteHistory') },
    ]
  },
  {
    title: t('multilineInput'),
    hint: '',
    items: [
      { key: '\\ + Enter', desc: t('shortcut_insertNewline') },
      { key: `${ctrl}+J`, desc: t('shortcut_insertNewlineAny') },
      ...(isMac
        ? [
            { key: 'Shift+Enter', desc: t('shortcut_insertNewlineIterm') },
            { key: 'Option+Enter', desc: t('shortcut_insertNewlineOption') },
          ]
        : [{ key: 'Shift+Enter', desc: t('shortcut_insertNewlineSupported') }]
      ),
      { key: 'Paste', desc: t('shortcut_pasteMode') },
    ]
  },
  {
    title: t('quickInput'),
    hint: '',
    items: [
      { key: '/ at start', desc: t('shortcut_commandOrSkill') },
      { key: '! at start', desc: t('shortcut_bashMode') },
      { key: '@', desc: t('shortcut_fileMention') },
      { key: ':', desc: t('shortcut_emojiShortcode') },
      { key: '? on empty', desc: t('shortcut_helpPanel') },
    ]
  },
  {
    title: t('slashCommands'),
    hint: t('slashCommandsHint'),
    items: [
      { key: '/clear', desc: t('shortcut_slashClear') },
      { key: '/compact', desc: t('shortcut_slashCompact') },
      { key: '/model', desc: t('shortcut_slashModel') },
      { key: '/usage', desc: t('shortcut_slashUsage') },
      { key: '/context', desc: t('shortcut_slashContext') },
      { key: '/resume', desc: t('shortcut_slashResume') },
      { key: '/config', desc: t('shortcut_slashConfig') },
      { key: '/permissions', desc: t('shortcut_slashPermissions') },
      { key: '/init', desc: t('shortcut_slashInit') },
      { key: '/diff', desc: t('shortcut_slashDiff') },
      { key: '/plan', desc: t('shortcut_slashPlan') },
      { key: '/review', desc: t('shortcut_slashReview') },
      { key: '/copy', desc: t('shortcut_slashCopy') },
      { key: '/export', desc: t('shortcut_slashExport') },
      { key: '/tasks', desc: t('shortcut_slashTasks') },
      { key: '/skills', desc: t('shortcut_slashSkills') },
      { key: '/effort', desc: t('shortcut_slashEffort') },
      { key: '/theme', desc: t('shortcut_slashTheme') },
      { key: '/memory', desc: t('shortcut_slashMemory') },
      { key: '/rename', desc: t('shortcut_slashRename') },
      { key: '/btw <q>', desc: t('shortcut_slashBug') },
      { key: '/branch', desc: t('shortcut_slashBranch') },
      { key: '/fork', desc: t('shortcut_slashFork') },
      { key: '/keybindings', desc: t('shortcut_slashKeybindings') },
      { key: '/tui', desc: t('shortcut_slashTui') },
      { key: '/help', desc: t('shortcut_slashHelp') },
      { key: '/doctor', desc: t('shortcut_slashDoctor') },
      { key: '/exit', desc: t('shortcut_slashExit') },
    ]
  },
])
</script>

<style scoped>
.section-content {
  padding: 8px 0;
}

.section-heading {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 24px;
}

.shortcuts-group {
  margin-bottom: 24px;
}

.group-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.group-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-bottom: 12px;
}

.shortcuts-table {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.shortcut-row {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 6px 8px;
  border-radius: 4px;
}

.shortcut-row:hover {
  background: var(--bg-secondary);
}

kbd {
  display: inline-block;
  padding: 4px 10px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-family: var(--font-mono);
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
  min-width: 120px;
  text-align: center;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.shortcut-desc {
  font-size: 13px;
  color: var(--text-primary);
}
</style>
