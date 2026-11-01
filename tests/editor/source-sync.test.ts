import { describe, expect, it } from 'vitest'
import {
  buildSkillSource,
  buildAgentSource,
  parseFrontmatter,
  pickExtras,
  skillKnownKeys,
  agentKnownKeys,
} from '@/components/editor/model'

describe('skill/agent 源码生成与未知字段保留', () => {
  // 表单改动应反映到源码；未知 frontmatter 键不能被丢掉（如 allowed-tools / color）
  it('SourceSync_SkillKeepsExtras_1', () => {
    const source = '---\nname: demo\ndescription: "旧描述"\nallowed-tools: Bash\n---\n\n正文'
    const fm = parseFrontmatter(source)!
    const extras = pickExtras(fm.fields, skillKnownKeys())

    const out = buildSkillSource(
      { name: 'demo', description: '新描述', content: '新正文' },
      extras
    )

    expect(out).toContain('description: "新描述"')
    expect(out).toContain('allowed-tools: "Bash"')
    expect(out).toContain('新正文')
    expect(out).not.toContain('旧描述')

    // 生成结果可再次解析，且未知键仍在
    const back = parseFrontmatter(out)!
    expect(back.fields['allowed-tools']).toBe('Bash')
    expect(back.fields.description).toBe('新描述')
    expect(back.body).toBe('新正文')
  })

  // Agent：可选字段为空时不出现在 frontmatter，未知键同样保留
  it('SourceSync_AgentOptionalAndExtras_2', () => {
    const out = buildAgentSource(
      { name: 'reviewer', description: '审查', tools: '', model: 'sonnet', content: 'body' },
      { color: 'blue' }
    )
    const fm = parseFrontmatter(out)!
    expect(fm.fields.name).toBe('reviewer')
    expect(fm.fields.model).toBe('sonnet')
    expect(fm.fields.tools).toBeUndefined()
    expect(fm.fields.color).toBe('blue')
    expect(agentKnownKeys()).toContain('tools')
  })
})
