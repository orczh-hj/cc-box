/**
 * Provider 预设配置模板 —— 生成物，勿手改
 * 上游：farion1231/cc-switch（src/config/claudeProviderPresets.ts）
 * 生成：2026-10-21T12:27:35.337Z | 133 个条目（全量同步，排除 hidden；
 * 剥离推广跟踪码与 partner 赞助字段；CC-Box 自有 Custom 恒在尾部）
 * 重新生成：手动下载上游 zip 后 npm run sync:presets -- --zip <zip路径>
 * 转换规则：scripts/lib/transform-presets.ts
 */
import type { ProviderPreset } from '@/types/provider'

export const providerPresets: ProviderPreset[] = [
  {
    name: 'Claude Official',
    websiteUrl: 'https://www.anthropic.com/claude-code',
    settingsConfig: {
      env: {},
    },
    isOfficial: true,
    category: 'official',
    icon: 'anthropic',
    iconColor: '#D4915D',
  },
  {
    name: '火山 Agent Plan',
    websiteUrl: 'https://www.volcengine.com/activity/agentplan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://ark.cn-beijing.volces.com/api/plan',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'ark-code-latest',
      },
    },
    apiKeyUrl: 'https://www.volcengine.com/activity/agentplan',
    category: 'cn_official',
    icon: 'huoshan',
    iconColor: '#3370FF',
  },
  {
    name: '火山 Coding Plan',
    websiteUrl: 'https://www.volcengine.com/activity/codingplan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://ark.cn-beijing.volces.com/api/coding',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'ark-code-latest',
      },
    },
    apiKeyUrl: 'https://www.volcengine.com/activity/codingplan',
    category: 'cn_official',
    icon: 'huoshan',
    iconColor: '#3370FF',
  },
  {
    name: '千问AI平台',
    websiteUrl: 'https://platform.qianwenai.com/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://dashscope.aliyuncs.com/apps/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'qwen3.8-max[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'qwen3.8-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'qwen3.7-plus[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'qwen3.8-max[1M]',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '983616',
      },
    },
    apiKeyUrl: 'https://platform.qianwenai.com/home/api-keys',
    category: 'cn_official',
    icon: 'qianwenai',
    iconColor: '#624AFF',
  },
  {
    name: '千问AI平台 Coding Plan',
    websiteUrl: 'https://bailian.console.aliyun.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://coding.dashscope.aliyuncs.com/apps/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    category: 'cn_official',
    icon: 'qianwenai',
    iconColor: '#624AFF',
  },
  {
    name: '千问AI平台 Token Plan',
    websiteUrl: 'https://platform.qianwenai.com/pricing/token-plan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://token-plan.cn-beijing.maas.aliyuncs.com/apps/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'qwen3.8-max[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'qwen3.8-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'qwen3.7-plus[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'qwen3.8-max[1M]',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '983616',
      },
    },
    apiKeyUrl: 'https://platform.qianwenai.com/home/api-keys',
    category: 'cn_official',
    icon: 'qianwenai',
    iconColor: '#624AFF',
  },
  {
    name: 'Baidu Qianfan Coding Plan',
    websiteUrl: 'https://cloud.baidu.com/product/qianfan_modelbuilder',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://qianfan.baidubce.com/anthropic/coding',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'qianfan-code-latest',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'qianfan-code-latest',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'qianfan-code-latest',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'qianfan-code-latest',
      },
    },
    apiKeyUrl: 'https://console.bce.baidu.com/qianfan/ais/console/applicationConsole/application',
    category: 'cn_official',
    icon: 'baidu',
    iconColor: '#2932E1',
    endpointCandidates: [
      'https://qianfan.baidubce.com/anthropic/coding',
    ],
  },
  {
    name: 'Baidu Qianfan Token Plan',
    websiteUrl: 'https://cloud.baidu.com/product/codingplan.html',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://qianfan.baidubce.com/anthropic/tokenplan/personal',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek-v4-pro[1M]',
      },
    },
    apiKeyUrl: 'https://console.bce.baidu.com/qianfan/resource/token-plan',
    category: 'cn_official',
    icon: 'baidu',
    iconColor: '#2932E1',
    endpointCandidates: [
      'https://qianfan.baidubce.com/anthropic/tokenplan/personal',
    ],
  },
  {
    name: 'BaiLing',
    websiteUrl: 'https://developer.ant-ling.com/zh-CN/docs/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.ant-ling.com/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'Ling-2.6-1T',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'Ling-2.6-1T',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'Ling-2.6-1T',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'Ling-2.6-1T',
      },
    },
    apiKeyUrl: 'https://chat.ant-ling.com/open',
    category: 'cn_official',
    icon: 'bailing',
  },
  {
    name: 'BytePlus',
    websiteUrl: 'https://www.byteplus.com/en/product/modelark',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://ark.ap-southeast.bytepluses.com/api/coding',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'ark-code-latest',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'ark-code-latest',
      },
    },
    apiKeyUrl: 'https://www.byteplus.com/en/product/modelark',
    category: 'cn_official',
    icon: 'byteplus',
    iconColor: '#3370FF',
  },
  {
    name: 'DeepSeek',
    websiteUrl: 'https://platform.deepseek.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.deepseek.com/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek-v4-pro[1M]',
      },
    },
    category: 'cn_official',
    modelsUrl: 'https://api.deepseek.com/models',
    icon: 'deepseek',
    iconColor: '#1E88E5',
  },
  {
    name: 'Kimi',
    websiteUrl: 'https://platform.kimi.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.moonshot.cn/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'kimi-k2.7-code',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'kimi-k2.7-code',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'kimi-k2.7-code',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kimi-k2.7-code',
      },
    },
    category: 'cn_official',
    icon: 'kimi',
    iconColor: '#6366F1',
  },
  {
    name: 'Kimi For Coding',
    websiteUrl: 'https://www.kimi.com/code/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.kimi.com/coding/',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'kimi-for-coding',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'kimi-for-coding',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'kimi-for-coding',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kimi-for-coding',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '262144',
        CLAUDE_CODE_AUTO_COMPACT_WINDOW: '262144',
      },
    },
    category: 'cn_official',
    icon: 'kimi',
    iconColor: '#6366F1',
  },
  {
    name: 'Kimi For Coding Global',
    websiteUrl: 'https://www.kimi.ai/code',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.kimi.ai/coding/',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'kimi-for-coding',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'kimi-for-coding',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'kimi-for-coding',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kimi-for-coding',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '262144',
        CLAUDE_CODE_AUTO_COMPACT_WINDOW: '262144',
      },
    },
    category: 'cn_official',
    icon: 'kimi',
    iconColor: '#6366F1',
  },
  {
    name: 'Kimi Global',
    websiteUrl: 'https://platform.kimi.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.moonshot.ai/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'kimi-k2.7-code',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'kimi-k2.7-code',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'kimi-k2.7-code',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kimi-k2.7-code',
      },
    },
    category: 'cn_official',
    icon: 'kimi',
    iconColor: '#6366F1',
  },
  {
    name: 'Longcat',
    websiteUrl: 'https://longcat.chat/platform',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.longcat.chat/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'LongCat-2.0[1M]',
        ANTHROPIC_SMALL_FAST_MODEL: 'LongCat-2.0',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'LongCat-2.0[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'LongCat-2.0[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'LongCat-2.0[1M]',
        CLAUDE_CODE_MAX_OUTPUT_TOKENS: '131072',
      },
    },
    apiKeyUrl: 'https://longcat.chat/platform/api_keys',
    category: 'cn_official',
    icon: 'longcat',
    iconColor: '#29E154',
  },
  {
    name: 'Meta Model API',
    websiteUrl: 'https://dev.meta.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.meta.ai',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'muse-wonder-1.0[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'muse-wonder-1.0[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'muse-spark-1.3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'muse-spark-1.3[1M]',
        CLAUDE_CODE_SUBAGENT_MODEL: 'muse-spark-1.3[1M]',
        ENABLE_TOOL_SEARCH: 'true',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '1048576',
      },
    },
    apiKeyUrl: 'https://dev.meta.ai',
    category: 'cn_official',
    icon: 'meta',
    endpointCandidates: [
      'https://api.meta.ai',
    ],
  },
  {
    name: 'MiniMax',
    websiteUrl: 'https://platform.minimax.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.minimax.cn/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        CLAUDE_CODE_AUTO_COMPACT_WINDOW: '1000000',
        ANTHROPIC_MODEL: 'MiniMax-M3.1-Flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'MiniMax-M3.1-Flash[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'MiniMax-M3.1-Flash[1M]',
      },
    },
    apiKeyUrl: 'https://platform.minimax.cn/subscribe/token-plan',
    category: 'cn_official',
    icon: 'minimax',
    iconColor: '#FF6B6B',
  },
  {
    name: 'MiniMax en',
    websiteUrl: 'https://platform.minimax.io',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.minimax.io/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        CLAUDE_CODE_AUTO_COMPACT_WINDOW: '1000000',
        ANTHROPIC_MODEL: 'MiniMax-M3.1-Flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'MiniMax-M3.1-Flash[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'MiniMax-M3.1-Flash[1M]',
      },
    },
    apiKeyUrl: 'https://platform.minimax.io/subscribe/coding-plan',
    category: 'cn_official',
    icon: 'minimax',
    iconColor: '#FF6B6B',
  },
  {
    name: 'QwenCloud',
    websiteUrl: 'https://home.qwencloud.com/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://dashscope-intl.aliyuncs.com/apps/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'qwen3.8-max[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'qwen3.8-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'qwen3.7-plus[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'qwen3.8-max[1M]',
      },
    },
    apiKeyUrl: 'https://home.qwencloud.com/api-keys',
    category: 'cn_official',
    icon: 'qwencloud',
    iconColor: '#6336E7',
  },
  {
    name: 'QwenCloud For Coding',
    websiteUrl: 'https://www.qwencloud.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://coding-intl.dashscope.aliyuncs.com/apps/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'qwen3.7-plus[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'qwen3.7-plus[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'qwen3.7-plus[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'qwen3.7-plus[1M]',
      },
    },
    apiKeyUrl: 'https://home.qwencloud.com/api-keys',
    category: 'cn_official',
    icon: 'qwencloud',
    iconColor: '#6336E7',
  },
  {
    name: 'QwenCloud Token Plan',
    websiteUrl: 'https://www.qwencloud.com/pricing/token-plan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://token-plan.ap-southeast-1.maas.aliyuncs.com/apps/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'qwen3.8-max[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'qwen3.8-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'qwen3.7-plus[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'qwen3.8-max[1M]',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '983616',
      },
    },
    apiKeyUrl: 'https://home.qwencloud.com/api-keys',
    category: 'cn_official',
    icon: 'qwencloud',
    iconColor: '#6336E7',
  },
  {
    name: 'StepFun',
    websiteUrl: 'https://platform.stepfun.com/step-plan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.stepfun.com/step_plan',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'step-3.5-flash-2603',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'step-3.5-flash-2603',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'step-3.5-flash-2603',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'step-3.5-flash-2603',
      },
    },
    apiKeyUrl: 'https://platform.stepfun.com/interface-key',
    category: 'cn_official',
    icon: 'stepfun',
    iconColor: '#16D6D2',
    endpointCandidates: [
      'https://api.stepfun.com/step_plan',
    ],
  },
  {
    name: 'StepFun en',
    websiteUrl: 'https://platform.stepfun.ai/step-plan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.stepfun.ai/step_plan',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'step-3.5-flash-2603',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'step-3.5-flash-2603',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'step-3.5-flash-2603',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'step-3.5-flash-2603',
      },
    },
    apiKeyUrl: 'https://platform.stepfun.ai/interface-key',
    category: 'cn_official',
    icon: 'stepfun',
    iconColor: '#16D6D2',
    endpointCandidates: [
      'https://api.stepfun.ai/step_plan',
    ],
  },
  {
    name: 'StreamLake',
    websiteUrl: 'https://www.streamlake.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://wanqing.streamlakeapi.com/api/gateway/v1/endpoints/kat-coder-pro-v2.5/claude-code-proxy',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'kat-coder-pro-v2.5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'kat-coder-pro-v2.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'kat-coder-pro-v2.5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kat-coder-pro-v2.5',
      },
    },
    apiKeyUrl: 'https://console.streamlake.com/console/wanqing/api-key',
    category: 'cn_official',
    icon: 'streamlake',
  },
  {
    name: 'StreamLake en',
    websiteUrl: 'https://console.streamlake.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://vanchin.streamlake.ai/api/gateway/v1/endpoints/kat-coder-pro-v2.5/claude-code-proxy',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'kat-coder-pro-v2.5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'kat-coder-pro-v2.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'kat-coder-pro-v2.5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kat-coder-pro-v2.5',
      },
    },
    apiKeyUrl: 'https://console.streamlake.ai/console/api-key',
    category: 'cn_official',
    icon: 'streamlake',
  },
  {
    name: 'Tencent Token Plan',
    websiteUrl: 'https://cloud.tencent.com/product/tokenhub',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.lkeap.cloud.tencent.com/plan/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'tc-code-latest',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'tc-code-latest',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'tc-code-latest',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'tc-code-latest',
      },
    },
    apiKeyUrl: 'https://console.cloud.tencent.com/tokenhub/tokenplan',
    category: 'cn_official',
    modelsUrl: 'https://api.lkeap.cloud.tencent.com/plan/v3/models',
    icon: 'tencent',
    iconColor: '#0052D9',
    endpointCandidates: [
      'https://api.lkeap.cloud.tencent.com/plan/anthropic',
    ],
  },
  {
    name: 'Tencent Token Plan (Intl)',
    websiteUrl: 'https://www.tencentcloud.com/products/tokenhub',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://tokenhub-intl.tencentcloudmaas.com/plan/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'auto',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'auto',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'auto',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'auto',
      },
    },
    apiKeyUrl: 'https://console.tencentcloud.com/tokenhub/tokenplan',
    category: 'cn_official',
    icon: 'tencent',
    iconColor: '#0052D9',
    endpointCandidates: [
      'https://tokenhub-intl.tencentcloudmaas.com/plan/anthropic',
    ],
  },
  {
    name: 'Tencent Token Plan Enterprise Lite',
    websiteUrl: 'https://cloud.tencent.com/product/tokenhub',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://tokenhub.tencentmaas.com/plan/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'auto',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'auto',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'auto',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'auto',
      },
    },
    apiKeyUrl: 'https://console.cloud.tencent.com/tokenhub/tokenplan-e',
    category: 'cn_official',
    icon: 'tencent',
    iconColor: '#0052D9',
    endpointCandidates: [
      'https://tokenhub.tencentmaas.com/plan/anthropic',
      'https://tokenhub-intl.tencentmaas.com/plan/anthropic',
    ],
  },
  {
    name: 'Tencent Token Plan Enterprise Lite (Intl)',
    websiteUrl: 'https://www.tencentcloud.com/products/tokenhub',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://tokenhub-intl.tencentcloudmaas.com/plan/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'auto',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'auto',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'auto',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'auto',
      },
    },
    apiKeyUrl: 'https://console.tencentcloud.com/tokenhub/tokenplan-e',
    category: 'cn_official',
    icon: 'tencent',
    iconColor: '#0052D9',
    endpointCandidates: [
      'https://tokenhub-intl.tencentcloudmaas.com/plan/anthropic',
      'https://tokenhub.tencentcloudmaas.com/plan/anthropic',
    ],
  },
  {
    name: 'Tencent Token Plan Enterprise Pro',
    websiteUrl: 'https://cloud.tencent.com/product/tokenhub',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://tokenhub.tencentmaas.com/plan/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'auto',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'auto',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'auto',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'auto',
      },
    },
    apiKeyUrl: 'https://console.cloud.tencent.com/tokenhub/tokenplan-e',
    category: 'cn_official',
    icon: 'tencent',
    iconColor: '#0052D9',
    endpointCandidates: [
      'https://tokenhub.tencentmaas.com/plan/anthropic',
      'https://tokenhub-intl.tencentmaas.com/plan/anthropic',
    ],
  },
  {
    name: 'Tencent Token Plan Enterprise Pro (Intl)',
    websiteUrl: 'https://www.tencentcloud.com/products/tokenhub',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://tokenhub-intl.tencentcloudmaas.com/plan/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'auto',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'auto',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'auto',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'auto',
      },
    },
    apiKeyUrl: 'https://console.tencentcloud.com/tokenhub/tokenplan-e',
    category: 'cn_official',
    icon: 'tencent',
    iconColor: '#0052D9',
    endpointCandidates: [
      'https://tokenhub-intl.tencentcloudmaas.com/plan/anthropic',
      'https://tokenhub.tencentcloudmaas.com/plan/anthropic',
    ],
  },
  {
    name: 'Volcengine Doubao',
    websiteUrl: 'https://console.volcengine.com/ark/region:ark+cn-beijing/apiKey?apikey=%7B%7D',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://ark.cn-beijing.volces.com/api/compatible',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'doubao-seed-2-1-pro-260628',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'doubao-seed-2-1-pro-260628',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'doubao-seed-2-1-pro-260628',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'doubao-seed-2-1-pro-260628',
      },
    },
    apiKeyUrl: 'https://console.volcengine.com/ark/region:ark+cn-beijing/apiKey?apikey=%7B%7D',
    category: 'cn_official',
    icon: 'doubao',
    iconColor: '#3370FF',
  },
  {
    name: 'Xiaomi MiMo',
    websiteUrl: 'https://platform.xiaomimimo.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.xiaomimimo.com/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'mimo-v2.6-pro[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'mimo-v2.6-pro[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'mimo-v2.6-pro[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'mimo-v2.6-pro[1M]',
      },
    },
    apiKeyUrl: 'https://platform.xiaomimimo.com/#/console/api-keys',
    category: 'cn_official',
    icon: 'xiaomimimo',
    iconColor: '#000000',
  },
  {
    name: 'Xiaomi MiMo Token Plan (China)',
    websiteUrl: 'https://platform.xiaomimimo.com/#/token-plan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://token-plan-cn.xiaomimimo.com/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'mimo-v2.6-pro[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'mimo-v2.6-pro[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'mimo-v2.6-pro[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'mimo-v2.6-pro[1M]',
      },
    },
    apiKeyUrl: 'https://platform.xiaomimimo.com/#/console/plan-manage',
    category: 'cn_official',
    icon: 'xiaomimimo',
    iconColor: '#000000',
  },
  {
    name: 'Zhipu GLM',
    websiteUrl: 'https://open.bigmodel.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://open.bigmodel.cn/api/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'glm-5.3[1M]',
      },
    },
    apiKeyUrl: 'https://www.bigmodel.cn/claude-code',
    category: 'cn_official',
    icon: 'zhipu',
    iconColor: '#0F62FE',
  },
  {
    name: 'Zhipu GLM en',
    websiteUrl: 'https://z.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.z.ai/api/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'glm-5.3[1M]',
      },
    },
    apiKeyUrl: 'https://z.ai/subscribe',
    category: 'cn_official',
    icon: 'zhipu',
    iconColor: '#0F62FE',
  },
  {
    name: 'AWS Bedrock (AKSK)',
    websiteUrl: 'https://aws.amazon.com/bedrock/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://bedrock-runtime.${AWS_REGION}.amazonaws.com',
        AWS_ACCESS_KEY_ID: '${AWS_ACCESS_KEY_ID}',
        AWS_SECRET_ACCESS_KEY: '${AWS_SECRET_ACCESS_KEY}',
        AWS_REGION: '${AWS_REGION}',
        ANTHROPIC_MODEL: 'global.anthropic.claude-opus-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'global.anthropic.claude-haiku-4-5-20251001-v1:0',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'global.anthropic.claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'global.anthropic.claude-opus-5',
        CLAUDE_CODE_USE_BEDROCK: '1',
      },
    },
    category: 'cloud_provider',
    icon: 'aws',
    iconColor: '#FF9900',
    templateValues: {
      AWS_REGION: {
        label: 'AWS Region',
        placeholder: 'us-west-2',
        editorValue: 'us-west-2',
      },
      AWS_ACCESS_KEY_ID: {
        label: 'Access Key ID',
        placeholder: 'AKIA...',
        editorValue: '',
      },
      AWS_SECRET_ACCESS_KEY: {
        label: 'Secret Access Key',
        placeholder: 'your-secret-key',
        editorValue: '',
      },
    },
  },
  {
    name: 'AWS Bedrock (API Key)',
    websiteUrl: 'https://aws.amazon.com/bedrock/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://bedrock-runtime.${AWS_REGION}.amazonaws.com',
        AWS_BEARER_TOKEN_BEDROCK: '',
        AWS_REGION: '${AWS_REGION}',
        ANTHROPIC_MODEL: 'global.anthropic.claude-opus-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'global.anthropic.claude-haiku-4-5-20251001-v1:0',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'global.anthropic.claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'global.anthropic.claude-opus-5',
        CLAUDE_CODE_USE_BEDROCK: '1',
      },
    },
    category: 'cloud_provider',
    icon: 'aws',
    iconColor: '#FF9900',
    templateValues: {
      AWS_REGION: {
        label: 'AWS Region',
        placeholder: 'us-west-2',
        editorValue: 'us-west-2',
      },
    },
  },
  {
    name: 'Cloudflare AI Gateway',
    websiteUrl: 'https://developers.cloudflare.com/ai-gateway/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://gateway.ai.cloudflare.com/v1/${ACCOUNT_ID}/${GATEWAY_ID}/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'claude-sonnet-5-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-5-5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-5-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-opus-5-5',
        CLAUDE_CODE_AUTO_MODE_SERVER: '0',
      },
    },
    apiKeyUrl: 'https://dash.cloudflare.com/profile/api-tokens',
    category: 'cloud_provider',
    icon: 'cloudflare',
    templateValues: {
      ACCOUNT_ID: {
        label: 'Account ID',
        placeholder: '32-character account ID',
        editorValue: '',
      },
      GATEWAY_ID: {
        label: 'Gateway ID',
        placeholder: 'default',
        editorValue: 'default',
      },
    },
  },
  {
    name: 'DigitalOcean',
    websiteUrl: 'https://www.digitalocean.com/products/ai-platform',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://inference.do-ai.run',
        ANTHROPIC_AUTH_TOKEN: '',
        CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS: '1',
      },
    },
    apiKeyUrl: 'https://cloud.digitalocean.com/model-studio/manage-keys',
    category: 'cloud_provider',
    icon: 'digitalocean',
    endpointCandidates: [
      'https://inference.do-ai.run',
    ],
  },
  {
    name: 'Fireworks AI',
    websiteUrl: 'https://fireworks.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.fireworks.ai/inference',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'accounts/fireworks/models/glm-5p3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'accounts/fireworks/models/deepseek-v4p1-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'accounts/fireworks/models/glm-5p3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'accounts/fireworks/models/kimi-k3[1M]',
      },
    },
    apiKeyUrl: 'https://app.fireworks.ai/settings/users/api-keys',
    category: 'cloud_provider',
    icon: 'fireworks',
    endpointCandidates: [
      'https://api.fireworks.ai/inference',
    ],
  },
  {
    name: 'Microsoft Foundry',
    websiteUrl: 'https://azure.microsoft.com/en-us/products/ai-foundry',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://${AZURE_RESOURCE_NAME}.services.ai.azure.com/anthropic',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'claude-opus-5-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-5-5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-5-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-opus-5-5',
        CLAUDE_CODE_AUTO_MODE_SERVER: '0',
      },
    },
    apiKeyUrl: 'https://ai.azure.com/',
    category: 'cloud_provider',
    apiKeyField: 'ANTHROPIC_API_KEY',
    icon: 'azure',
    iconColor: '#0078D4',
    templateValues: {
      AZURE_RESOURCE_NAME: {
        label: 'Azure Resource Name',
        placeholder: 'my-foundry-resource',
        editorValue: '',
      },
    },
  },
  {
    name: 'Ollama Cloud',
    websiteUrl: 'https://ollama.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://ollama.com',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'glm-5.3-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kimi-k3[1M]',
      },
    },
    apiKeyUrl: 'https://ollama.com/settings/keys',
    category: 'cloud_provider',
    icon: 'ollama',
    endpointCandidates: [
      'https://ollama.com',
    ],
  },
  {
    name: 'SambaNova',
    websiteUrl: 'https://sambanova.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.sambanova.ai',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'MiniMax-M3[1M]',
      },
    },
    apiKeyUrl: 'https://cloud.sambanova.ai/apis',
    category: 'cloud_provider',
    icon: 'sambanova',
    endpointCandidates: [
      'https://api.sambanova.ai',
    ],
  },
  {
    name: '88API',
    websiteUrl: 'https://88api.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.88api.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://88api.ai/sign-up',
    category: 'aggregator',
    icon: '88api',
    endpointCandidates: [
      'https://api.88api.ai',
      'https://88api.ai',
    ],
  },
  {
    name: '9527CODE',
    websiteUrl: 'https://9527.codes',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://9527.codes',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://9527.codes/register',
    category: 'aggregator',
    icon: '9527code',
    endpointCandidates: [
      'https://9527.codes',
      'https://api.9527.codes',
      'https://cdn.9527.codes',
    ],
  },
  {
    name: '模力方舟',
    websiteUrl: 'https://moark.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://moark.com/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek-v4-flash-0731[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek-v4-flash-0731[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek-v4-flash-0731[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek-v4-flash-0731[1M]',
      },
    },
    apiKeyUrl: 'https://moark.com/dashboard/tokens',
    category: 'aggregator',
    icon: 'moark',
    endpointCandidates: [
      'https://moark.com/anthropic',
    ],
  },
  {
    name: 'A6API',
    websiteUrl: 'https://www.a6api.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.a6api.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://a6api.com/register',
    category: 'aggregator',
    icon: 'a6api',
  },
  {
    name: 'AICodeWith',
    websiteUrl: 'https://aicodewith.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.aicodewith.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://aicodewith.ai/login?tab=register',
    category: 'aggregator',
    icon: 'aicodewith',
    iconColor: '#3A3B40',
    endpointCandidates: [
      'https://api.aicodewith.ai',
    ],
  },
  {
    name: 'AIComing',
    websiteUrl: 'https://aicoming.top',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.aicoming.top',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://aicoming.top/console#keys',
    category: 'aggregator',
    icon: 'aicoming',
  },
  {
    name: 'AiHubMix',
    websiteUrl: 'https://aihubmix.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://aihubmix.com',
        ANTHROPIC_API_KEY: '',
      },
    },
    apiKeyUrl: 'https://aihubmix.com',
    category: 'aggregator',
    apiKeyField: 'ANTHROPIC_API_KEY',
    icon: 'aihubmix',
    iconColor: '#006FFB',
    endpointCandidates: [
      'https://aihubmix.com',
      'https://api.aihubmix.com',
    ],
  },
  {
    name: 'AllTokens',
    websiteUrl: 'https://alltokens.cc',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://alltokens.cc',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://alltokens.cc/register',
    category: 'aggregator',
    icon: 'alltokens',
  },
  {
    name: 'AllTokens SI',
    websiteUrl: 'https://alltokens.si',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://alltokens.si',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'claude-sonnet-5.5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-sonnet-5.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-5.5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-opus-5.5',
      },
    },
    apiKeyUrl: 'https://alltokens.si/enterprise-register',
    category: 'aggregator',
    icon: 'alltokens-si',
  },
  {
    name: 'Amux',
    websiteUrl: 'https://amux.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.amux.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://amux.ai',
    category: 'aggregator',
    icon: 'amux',
  },
  {
    name: 'AtlasCloud',
    websiteUrl: 'https://www.atlascloud.ai/console/coding-plan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.atlascloud.ai',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'zai-org/glm-5.1',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'zai-org/glm-5.1',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'zai-org/glm-5.1',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'zai-org/glm-5.1',
        CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS: '1',
      },
    },
    apiKeyUrl: 'https://www.atlascloud.ai/console/coding-plan',
    category: 'aggregator',
    icon: 'atlascloud',
    endpointCandidates: [
      'https://api.atlascloud.ai',
    ],
  },
  {
    name: 'Baseten',
    websiteUrl: 'https://www.baseten.co',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://inference.baseten.co',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'zai-org/GLM-5.3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'zai-org/GLM-5.3-Flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'zai-org/GLM-5.3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'moonshotai/Kimi-K3[1M]',
      },
    },
    apiKeyUrl: 'https://app.baseten.co/settings/api_keys',
    category: 'aggregator',
    icon: 'baseten',
    endpointCandidates: [
      'https://inference.baseten.co',
    ],
  },
  {
    name: 'BotCF',
    websiteUrl: 'https://botcf.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://botcf.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://botcf.com/console/token',
    category: 'aggregator',
    icon: 'botcf',
  },
  {
    name: 'CCSub',
    websiteUrl: 'https://www.ccsub.net',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://www.ccsub.net',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.ccsub.net/register?ref=Y6Z8DXEA',
    category: 'aggregator',
    icon: 'ccsub',
  },
  {
    name: 'CherryIN',
    websiteUrl: 'https://open.cherryin.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://open.cherryin.net',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'anthropic/claude-haiku-4.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'anthropic/claude-opus-5',
      },
    },
    apiKeyUrl: 'https://open.cherryin.ai/console/token',
    category: 'aggregator',
    icon: 'cherryin',
    endpointCandidates: [
      'https://open.cherryin.net',
    ],
  },
  {
    name: 'ClaudeAPI',
    websiteUrl: 'https://www.apito.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://gw.apito.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://console.apito.ai/agent/register/pQBql2buaqiX3dDS',
    category: 'aggregator',
    icon: 'claudeapi',
  },
  {
    name: 'Code0',
    websiteUrl: 'https://code0.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://code0.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://code0.ai/agent/register/B2XHxGjGmRvqgznY',
    category: 'aggregator',
    icon: 'code0',
  },
  {
    name: 'Compshare',
    websiteUrl: 'https://www.compshare.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.modelverse.cn',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.compshare.cn/coding-plan?ytag=GPU_YY_YX_git_cc-switch',
    category: 'aggregator',
    icon: 'ucloud',
    iconColor: '#000000',
    endpointCandidates: [
      'https://api.modelverse.cn',
    ],
  },
  {
    name: 'Compshare Coding Plan',
    websiteUrl: 'https://www.compshare.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://cp.compshare.cn',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.compshare.cn/coding-plan?ytag=GPU_YY_YX_git_cc-switch',
    category: 'aggregator',
    icon: 'ucloud',
    iconColor: '#000000',
    endpointCandidates: [
      'https://cp.compshare.cn',
    ],
  },
  {
    name: 'DeepInfra',
    websiteUrl: 'https://deepinfra.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.deepinfra.com/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek-ai/DeepSeek-V4-Flash-0731[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'inclusionAI/Ling-3.0-flash',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek-ai/DeepSeek-V4-Flash-0731[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek-ai/DeepSeek-V4-Pro-0813[1M]',
      },
    },
    apiKeyUrl: 'https://deepinfra.com/dash/api_keys',
    category: 'aggregator',
    icon: 'deepinfra',
    endpointCandidates: [
      'https://api.deepinfra.com/anthropic',
    ],
  },
  {
    name: 'DIT.ai',
    websiteUrl: 'https://dit.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.dit.ai',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-4-5-20251001',
      },
    },
    apiKeyUrl: 'https://dit.ai/register',
    category: 'aggregator',
    icon: 'dit',
  },
  {
    name: 'DMXAPI',
    websiteUrl: 'https://www.dmxapi.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://www.dmxapi.cn',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.dmxapi.cn',
    category: 'aggregator',
    icon: 'dmxapi',
    endpointCandidates: [
      'https://www.dmxapi.cn',
      'https://api.dmxapi.cn',
    ],
  },
  {
    name: 'EveryAPI',
    websiteUrl: 'https://everyapi.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.everyapi.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://app.everyapi.ai/signin',
    category: 'aggregator',
    icon: 'everyapi',
    endpointCandidates: [
      'https://api.everyapi.ai',
      'https://api-cn.everyapi.ai',
    ],
  },
  {
    name: 'FennoAI',
    websiteUrl: 'https://api.fenno.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.fenno.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://api.fenno.ai/register?redirect=/purchase?tab=subscription%26group=16',
    category: 'aggregator',
    icon: 'fenno',
  },
  {
    name: 'FluxA Token Plan',
    websiteUrl: 'https://agentmarket.fluxapay.xyz/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.baiduqianfan.ai/anthropic/tokenplan/team',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek-v4-pro[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek-v4-pro[1M]',
      },
    },
    apiKeyUrl: 'https://agentmarket.fluxapay.xyz/marketplace/tokenplans',
    category: 'aggregator',
    icon: 'fluxa',
    endpointCandidates: [
      'https://api.baiduqianfan.ai/anthropic/tokenplan/team',
    ],
  },
  {
    name: 'GrooRoute Pro',
    websiteUrl: 'https://grooroute.pro',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://grooroute.pro',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-sonnet-5-5',
      },
    },
    apiKeyUrl: 'https://grooroute.pro/sign-up',
    category: 'aggregator',
    icon: 'grooroute',
    endpointCandidates: [
      'https://grooroute.pro',
      'https://grooroute.xyz',
    ],
  },
  {
    name: 'Groq',
    websiteUrl: 'https://groq.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.groq.com/openai/v1',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'openai/gpt-oss-120b',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'openai/gpt-oss-20b',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'openai/gpt-oss-120b',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'openai/gpt-oss-120b',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '131072',
      },
    },
    apiKeyUrl: 'https://console.groq.com/keys',
    category: 'aggregator',
    apiFormat: 'openai_chat',
    icon: 'groq',
    endpointCandidates: [
      'https://api.groq.com/openai/v1',
    ],
  },
  {
    name: 'Hugging Face',
    websiteUrl: 'https://huggingface.co/inference/models',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://router.huggingface.co',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'zai-org/GLM-5.3:baseten[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'zai-org/GLM-5.3-Flash:baseten[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'zai-org/GLM-5.3:baseten[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'zai-org/GLM-5.3:baseten[1M]',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '1048576',
      },
    },
    apiKeyUrl: 'https://huggingface.co/settings/tokens/new?ownUserPermissions=inference.serverless.write&tokenType=fineGrained',
    category: 'aggregator',
    icon: 'huggingface',
    endpointCandidates: [
      'https://router.huggingface.co',
    ],
  },
  {
    name: 'IKunCode',
    websiteUrl: 'https://api.ikuncode.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.ikuncode.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://api.ikuncode.ai/console/token',
    category: 'aggregator',
    icon: 'ikuncode',
    endpointCandidates: [
      'https://api.ikuncode.ai',
      'https://api.ikuncode.cc',
    ],
  },
  {
    name: 'JieKou AI',
    websiteUrl: 'https://jiekou.ai/#model-library',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.jiekou.ai/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'claude-fable-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-fable-5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-fable-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-fable-5',
      },
    },
    apiKeyUrl: 'https://jiekou.ai/settings/key-management',
    category: 'aggregator',
    modelsUrl: 'https://api.jiekou.ai/openai/v1/models',
    icon: 'jiekou',
    iconColor: '#000000',
    endpointCandidates: [
      'https://api.jiekou.ai/anthropic',
    ],
  },
  {
    name: 'Modelflare',
    websiteUrl: 'https://modelflare.dev',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://modelflare.dev',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-sonnet-5-5',
      },
    },
    apiKeyUrl: 'https://modelflare.dev/sign-up',
    category: 'aggregator',
    icon: 'modelflare',
    endpointCandidates: [
      'https://modelflare.dev',
      'https://origin.modelflare.dev',
    ],
  },
  {
    name: 'ModelScope',
    websiteUrl: 'https://modelscope.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api-inference.modelscope.cn',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'ZhipuAI/GLM-5.2[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'ZhipuAI/GLM-5.2[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'ZhipuAI/GLM-5.2[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'ZhipuAI/GLM-5.2[1M]',
      },
    },
    category: 'aggregator',
    icon: 'modelscope',
    iconColor: '#624AFF',
  },
  {
    name: 'Nebius Token Factory',
    websiteUrl: 'https://nebius.com/token-factory',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.tokenfactory.nebius.com',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'zai-org/GLM-5.3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'zai-org/GLM-5.3-Flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'zai-org/GLM-5.3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'zai-org/GLM-5.3[1M]',
      },
    },
    apiKeyUrl: 'https://tokenfactory.nebius.com/project/api-keys',
    category: 'aggregator',
    apiFormat: 'openai_chat',
    icon: 'nebius',
    endpointCandidates: [
      'https://api.tokenfactory.nebius.com',
    ],
  },
  {
    name: 'NekoCode',
    websiteUrl: 'https://nekoapi.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://nekoapi.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://nekoapi.ai/token',
    category: 'aggregator',
    icon: 'nekocode',
  },
  {
    name: 'Novita AI',
    websiteUrl: 'https://novita.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.novita.ai/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'zai-org/glm-5.1',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'zai-org/glm-5.1',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'zai-org/glm-5.1',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'zai-org/glm-5.1',
      },
    },
    apiKeyUrl: 'https://novita.ai',
    category: 'aggregator',
    modelsUrl: 'https://api.novita.ai/openai/v1/models',
    icon: 'novita',
    iconColor: '#000000',
    endpointCandidates: [
      'https://api.novita.ai/anthropic',
    ],
  },
  {
    name: 'Nvidia',
    websiteUrl: 'https://build.nvidia.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://integrate.api.nvidia.com',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'moonshotai/kimi-k3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'moonshotai/kimi-k3[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'moonshotai/kimi-k3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'moonshotai/kimi-k3[1M]',
      },
    },
    apiKeyUrl: 'https://build.nvidia.com/settings/api-keys',
    category: 'aggregator',
    apiFormat: 'openai_chat',
    icon: 'nvidia',
    iconColor: '#000000',
  },
  {
    name: 'OpenCode Zen',
    websiteUrl: 'https://opencode.ai/zen',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://opencode.ai/zen',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'claude-sonnet-5-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-4-5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-5-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-opus-5-5',
      },
    },
    apiKeyUrl: 'https://opencode.ai/auth',
    category: 'aggregator',
    apiKeyField: 'ANTHROPIC_API_KEY',
    icon: 'opencode',
    iconColor: '#211E1E',
    endpointCandidates: [
      'https://opencode.ai/zen',
    ],
  },
  {
    name: 'OpenModel',
    websiteUrl: 'https://www.openmodel.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.openmodel.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.openmodel.ai?ref=cc-switch',
    category: 'aggregator',
    icon: 'openmodel',
  },
  {
    name: 'OpenRouter',
    websiteUrl: 'https://openrouter.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://openrouter.ai/api',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'anthropic/claude-haiku-4.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'anthropic/claude-opus-5',
      },
    },
    apiKeyUrl: 'https://openrouter.ai/keys',
    category: 'aggregator',
    icon: 'openrouter',
    iconColor: '#6566F1',
  },
  {
    name: 'PIPELLM',
    websiteUrl: 'https://code.pipellm.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://cc-api.pipellm.ai',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'claude-opus-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-4-5-20251001',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-opus-5',
      },
    },
    apiKeyUrl: 'https://code.pipellm.ai/login?ref=uvw650za',
    category: 'aggregator',
    icon: 'pipellm',
  },
  {
    name: 'PPIO',
    websiteUrl: 'https://ppio.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.ppio.com/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek/deepseek-v4-flash-0731[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek/deepseek-v4-flash-0731[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek/deepseek-v4-flash-0731[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek/deepseek-v4-flash-0731[1M]',
      },
    },
    apiKeyUrl: 'https://ppio.com/activity/ccswitch',
    category: 'aggregator',
    modelsUrl: 'https://api.ppio.com/openai/v1/models',
    icon: 'ppio',
    iconColor: '#2874FF',
    endpointCandidates: [
      'https://api.ppio.com/anthropic',
    ],
  },
  {
    name: 'Qiniu',
    websiteUrl: 'https://s.qiniu.com/nMvAvy',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.qnaigc.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://s.qiniu.com/nMvAvy',
    category: 'aggregator',
    icon: 'qiniu',
    endpointCandidates: [
      'https://api.qnaigc.com',
      'https://api.modelink.ai',
    ],
  },
  {
    name: 'RunAPI',
    websiteUrl: 'https://runapi.host',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://runapi.host',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://runapi.host/register',
    category: 'aggregator',
    icon: 'runapi',
    endpointCandidates: [
      'https://runapi.host',
      'https://runapi.co',
    ],
  },
  {
    name: 'RunToken',
    websiteUrl: 'https://www.runtoken.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://www.runtoken.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.runtoken.ai/console/token',
    category: 'aggregator',
    icon: 'runtoken',
    endpointCandidates: [
      'https://www.runtoken.ai',
      'https://fast.runtoken.ai',
    ],
  },
  {
    name: 'Shengsuanyun',
    websiteUrl: 'https://www.shengsuanyun.com/',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://router.shengsuanyun.com/api',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'anthropic/claude-haiku-4.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'anthropic/claude-opus-5',
      },
    },
    apiKeyUrl: 'https://www.shengsuanyun.com/',
    category: 'aggregator',
    icon: 'shengsuanyun',
  },
  {
    name: 'SiliconFlow',
    websiteUrl: 'https://siliconflow.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.siliconflow.cn',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek-ai/DeepSeek-V4-Flash[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek-ai/DeepSeek-V4-Flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek-ai/DeepSeek-V4-Flash[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek-ai/DeepSeek-V4-Flash[1M]',
      },
    },
    apiKeyUrl: 'https://cloud.siliconflow.cn',
    category: 'aggregator',
    icon: 'siliconflow',
    iconColor: '#6E29F6',
  },
  {
    name: 'SiliconFlow en',
    websiteUrl: 'https://siliconflow.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.siliconflow.com',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'MiniMaxAI/MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'MiniMaxAI/MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'MiniMaxAI/MiniMax-M3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'MiniMaxAI/MiniMax-M3[1M]',
      },
    },
    apiKeyUrl: 'https://cloud.siliconflow.cn',
    category: 'aggregator',
    icon: 'siliconflow',
    iconColor: '#000000',
  },
  {
    name: 'SoleAPI',
    websiteUrl: 'https://soleapi.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://soleapi.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://soleapi.com/r/ccswitch',
    category: 'aggregator',
    icon: 'soleapi',
    endpointCandidates: [
      'https://soleapi.com',
    ],
  },
  {
    name: 'SoruxGPT',
    websiteUrl: 'https://ai.soruxgpt.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://ai.soruxgpt.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://ai.soruxgpt.com/console/token',
    category: 'aggregator',
    icon: 'soruxgpt',
    endpointCandidates: [
      'https://ai.soruxgpt.com',
      'https://api-cf.soruxgpt.com',
      'https://api-host.soruxgpt.com',
      'https://api-node6.soruxgpt.com',
      'https://api-node3.soruxgpt.com',
    ],
  },
  {
    name: 'SoruxGPT Coding Plan',
    websiteUrl: 'https://app.soruxgpt.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://app.soruxgpt.com/api/cc',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    category: 'aggregator',
    icon: 'soruxgpt',
    endpointCandidates: [
      'https://app.soruxgpt.com/api/cc',
      'https://center.soruxgpt.com/api/cc',
      'https://u.soruxgpt.com/api/cc',
    ],
  },
  {
    name: 'Soshow',
    websiteUrl: 'https://aimarket.so-show.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://maas.so-show.com',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-sonnet-5',
        CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS: '1',
      },
    },
    apiKeyUrl: 'https://aimarket.so-show.com/workbench/access-key',
    category: 'aggregator',
    icon: 'soshow',
  },
  {
    name: 'SubRouter',
    websiteUrl: 'https://subrouter.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://subrouter.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://subrouter.ai/register',
    category: 'aggregator',
    icon: 'subrouter',
  },
  {
    name: 'TeamoRouter',
    websiteUrl: 'https://teamorouter.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.teamorouter.cn',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://teamorouter.cn/',
    category: 'aggregator',
    icon: 'teamorouter',
    endpointCandidates: [
      'https://api.teamorouter.cn',
      'https://api.teamorouter.com',
    ],
  },
  {
    name: 'TheRouter',
    websiteUrl: 'https://therouter.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.therouter.ai',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'anthropic/claude-haiku-4.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'anthropic/claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'anthropic/claude-opus-5',
      },
    },
    apiKeyUrl: 'https://dashboard.therouter.ai',
    category: 'aggregator',
    icon: 'therouter',
    endpointCandidates: [
      'https://api.therouter.ai',
    ],
  },
  {
    name: 'TokenFactory',
    websiteUrl: 'https://tokenfactory.cn',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.tokenfactory.cn/anthropic',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek-v4.1-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'glm-5.3[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'kimi-k3[1M]',
      },
    },
    apiKeyUrl: 'https://console.tokenfactory.cn/login',
    category: 'aggregator',
    icon: 'tokenfactory',
    endpointCandidates: [
      'https://api.tokenfactory.cn/anthropic',
    ],
  },
  {
    name: 'TokenFlux',
    websiteUrl: 'https://tokenflux.dev',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://tokenflux.dev',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://tokenflux.dev/keys',
    category: 'aggregator',
    icon: 'tokenflux',
  },
  {
    name: 'Tu-zi',
    websiteUrl: 'https://api.tu-zi.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.tu-zi.com',
        ANTHROPIC_API_KEY: '',
      },
    },
    apiKeyUrl: 'https://api.tu-zi.com/token',
    category: 'aggregator',
    apiKeyField: 'ANTHROPIC_API_KEY',
    icon: 'tuzi',
    endpointCandidates: [
      'https://api.tu-zi.com',
      'https://api.ourzhishi.top',
      'https://api.sydney-ai.com',
      'https://apicdn.tu-zi.com',
    ],
  },
  {
    name: 'Vercel AI Gateway',
    websiteUrl: 'https://vercel.com/ai-gateway',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://ai-gateway.vercel.sh/claude-code',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'anthropic/claude-sonnet-5.5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'anthropic/claude-haiku-5.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'anthropic/claude-sonnet-5.5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'anthropic/claude-opus-5.5',
      },
    },
    apiKeyUrl: 'https://vercel.com/d?to=%2F%5Bteam%5D%2F%7E%2Fai-gateway%2Fapi-keys',
    category: 'aggregator',
    modelsUrl: 'https://ai-gateway.vercel.sh/coding-agent/v1/models',
    icon: 'vercel',
    endpointCandidates: [
      'https://ai-gateway.vercel.sh/claude-code',
    ],
  },
  {
    name: 'XycAi',
    websiteUrl: 'https://xycai.us',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://apicdn.xycai.us',
        ANTHROPIC_API_KEY: '',
      },
    },
    apiKeyUrl: 'https://xycai.us/register',
    category: 'aggregator',
    apiKeyField: 'ANTHROPIC_API_KEY',
    icon: 'xycai',
    endpointCandidates: [
      'https://apicdn.xycai.us',
      'https://apicdn.xyc.ai',
    ],
  },
  {
    name: 'YYLX',
    websiteUrl: 'https://yylx.io',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://app.yylx.io',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://app.yylx.io/keys',
    category: 'aggregator',
    icon: 'yylx',
    endpointCandidates: [
      'https://app.yylx.io',
      'https://global.yylx.io',
      'https://cn.yylx.io',
    ],
  },
  {
    name: 'AICodeMirror',
    websiteUrl: 'https://www.aicodemirror.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.aicodemirror.ai/api/claudecode',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.aicodemirror.ai/register?invitecode=9915W3',
    category: 'third_party',
    icon: 'aicodemirror',
    iconColor: '#000000',
    endpointCandidates: [
      'https://api.aicodemirror.ai/api/claudecode',
    ],
  },
  {
    name: 'AICoding',
    websiteUrl: 'https://aicoding.inc',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.aicoding.inc',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://aicoding.inc',
    category: 'third_party',
    icon: 'aicoding',
    iconColor: '#000000',
    endpointCandidates: [
      'https://api.aicoding.inc',
    ],
  },
  {
    name: 'AIGoCode',
    websiteUrl: 'https://aigocode.net',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.aigocode.dev',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://aigocode.net',
    category: 'third_party',
    icon: 'aigocode',
    iconColor: '#5B7FFF',
    endpointCandidates: [
      'https://api.aigocode.dev',
      'https://api.aigocode.app',
    ],
  },
  {
    name: 'APIKEY.FUN',
    websiteUrl: 'https://apikey.fan',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.apikey.fan',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://apikey.fan/register',
    category: 'third_party',
    icon: 'apikeyfun',
    endpointCandidates: [
      'https://api.apikey.fan',
      'https://api.apikey.fun',
      'https://slb.apikey.fun',
    ],
  },
  {
    name: 'APINebula',
    websiteUrl: 'https://apinebula.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://apinebula.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://apinebula.ai/VjM74M',
    category: 'third_party',
    icon: 'apinebula',
    endpointCandidates: [
      'https://apinebula.ai',
    ],
  },
  {
    name: 'ClaudeCN',
    websiteUrl: 'https://claudecn.top',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://claudecn.top',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://claudecn.ai/register',
    category: 'third_party',
    icon: 'claudecn',
  },
  {
    name: 'Codex',
    websiteUrl: 'https://openai.com/chatgpt/pricing',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://chatgpt.com/backend-api/codex',
        ANTHROPIC_MODEL: 'gpt-5.6-sol',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'gpt-5.6-luna',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'gpt-5.6-sol',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'gpt-5.6-sol',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '272000',
        CLAUDE_CODE_AUTO_COMPACT_WINDOW: '272000',
      },
    },
    category: 'third_party',
    apiFormat: 'openai_responses',
    providerType: 'codex_oauth',
    requiresOAuth: true,
    icon: 'openai',
    iconColor: '#000000',
  },
  {
    name: 'Command Code',
    websiteUrl: 'https://commandcode.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.commandcode.ai/provider',
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_MODEL: 'deepseek/deepseek-v4.1-flash[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek/deepseek-v4.1-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek/deepseek-v4.1-flash[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek/deepseek-v4.1-flash[1M]',
      },
    },
    apiKeyUrl: 'https://commandcode.ai/settings/keys',
    category: 'third_party',
    apiFormat: 'openai_chat',
    modelsUrl: 'https://api.commandcode.ai/provider/v1/models',
    icon: 'commandcode',
    endpointCandidates: [
      'https://api.commandcode.ai/provider',
    ],
  },
  {
    name: 'CrazyRouter',
    websiteUrl: 'https://www.crazyrouter.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://cn.crazyrouter.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.crazyrouter.com/register?ref=cc-switch',
    category: 'third_party',
    icon: 'crazyrouter',
    iconColor: '#000000',
    endpointCandidates: [
      'https://cn.crazyrouter.com',
    ],
  },
  {
    name: 'Cubence',
    websiteUrl: 'https://cubence.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.cubence.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://cubence.com/signup',
    category: 'third_party',
    icon: 'cubence',
    iconColor: '#000000',
    endpointCandidates: [
      'https://api.cubence.com',
      'https://api-cf.cubence.com',
      'https://api-dmit.cubence.com',
      'https://api-bwg.cubence.com',
    ],
  },
  {
    name: 'E-FlowCode',
    websiteUrl: 'https://e-flowcode.cc',
    settingsConfig: {
      env: {
        ANTHROPIC_AUTH_TOKEN: '',
        ANTHROPIC_BASE_URL: 'https://e-flowcode.cc',
        ENABLE_TOOL_SEARCH: 'true',
      },
    },
    apiKeyUrl: 'https://e-flowcode.cc',
    category: 'third_party',
    icon: 'eflowcode',
    iconColor: '#000000',
    endpointCandidates: [
      'https://e-flowcode.cc',
    ],
  },
  {
    name: 'Gemini Native',
    websiteUrl: 'https://ai.google.dev/gemini-api',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://generativelanguage.googleapis.com',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'gemini-3.6-flash',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'gemini-3.6-flash',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'gemini-3.6-flash',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'gemini-3.6-flash',
      },
    },
    apiKeyUrl: 'https://aistudio.google.com/app/apikey',
    category: 'third_party',
    apiKeyField: 'ANTHROPIC_API_KEY',
    apiFormat: 'gemini_native',
    icon: 'gemini',
    iconColor: '#4285F4',
    endpointCandidates: [
      'https://generativelanguage.googleapis.com',
    ],
  },
  {
    name: 'GitHub Copilot',
    websiteUrl: 'https://github.com/features/copilot',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.githubcopilot.com',
        ANTHROPIC_MODEL: 'claude-sonnet-5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'claude-haiku-4.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'claude-sonnet-5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'claude-sonnet-5',
      },
    },
    category: 'third_party',
    apiFormat: 'openai_chat',
    providerType: 'github_copilot',
    requiresOAuth: true,
    icon: 'github',
    iconColor: '#000000',
  },
  {
    name: 'Micu',
    websiteUrl: 'https://www.micuapi.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://www.micuapi.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.micuapi.ai/register',
    category: 'third_party',
    icon: 'micu',
    iconColor: '#000000',
    endpointCandidates: [
      'https://www.micuapi.ai',
    ],
  },
  {
    name: 'OpenCode Go',
    websiteUrl: 'https://opencode.ai/go',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://opencode.ai/zen/go',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'deepseek-v4-flash[1M]',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'deepseek-v4-flash[1M]',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'deepseek-v4-flash[1M]',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'deepseek-v4-flash[1M]',
      },
    },
    apiKeyUrl: 'https://opencode.ai/go',
    category: 'third_party',
    apiKeyField: 'ANTHROPIC_API_KEY',
    icon: 'opencode',
    iconColor: '#211E1E',
    endpointCandidates: [
      'https://opencode.ai/zen/go',
    ],
  },
  {
    name: 'PackyCode',
    websiteUrl: 'https://www.packyapi.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://www.packyapi.ai',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.packyapi.ai/register',
    category: 'third_party',
    icon: 'packycode',
    endpointCandidates: [
      'https://www.packyapi.ai',
      'https://cf.api.fan',
      'https://slb-v1.api.fan',
      'https://www.packyapi.com',
    ],
  },
  {
    name: 'PatewayAI',
    websiteUrl: 'https://pateway.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.pateway.ai',
        ANTHROPIC_API_KEY: '',
      },
    },
    apiKeyUrl: 'https://pateway.ai/?ch=etzpm8',
    category: 'third_party',
    apiKeyField: 'ANTHROPIC_API_KEY',
    icon: 'pateway',
  },
  {
    name: 'RelaxyCode',
    websiteUrl: 'https://www.relaxycode.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://www.relaxycode.com',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.relaxycode.com/register',
    category: 'third_party',
    icon: 'relaxcode',
  },
  {
    name: 'RightCode',
    websiteUrl: 'https://www.rightapi.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://www.rightapi.ai/claude',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://www.rightapi.ai/register',
    category: 'third_party',
    icon: 'rc',
    iconColor: '#E96B2C',
  },
  {
    name: 'SSSAiCode',
    websiteUrl: 'https://sssaicodeapi.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://node-hk.sssaicodeapi.com/api',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://sssaicodeapi.com/register?ref=DCP0SM',
    category: 'third_party',
    icon: 'sssaicode',
    iconColor: '#000000',
    endpointCandidates: [
      'https://node-hk.sssaicodeapi.com/api',
      'https://node-hk.sssaiapi.com/api',
      'https://node-cf.sssaicodeapi.com/api',
    ],
  },
  {
    name: 'SudoCode.chat',
    websiteUrl: 'https://sudocode.chat',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.sudocode.chat',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://sudocode.chat/sign-up',
    category: 'third_party',
    icon: 'sudocode',
    endpointCandidates: [
      'https://api.sudocode.chat',
      'https://api.sudorelay.com',
    ],
  },
  {
    name: 'SudoCode.us',
    websiteUrl: 'https://sudocode.us',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://sudocode.us',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    apiKeyUrl: 'https://sudocode.us',
    category: 'third_party',
    icon: 'sudocode-us',
    endpointCandidates: [
      'https://sudocode.us',
      'https://sudocode.run',
    ],
  },
  {
    name: 'xAI (Grok)',
    websiteUrl: 'https://x.ai/grok',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'https://api.x.ai/v1',
        ANTHROPIC_MODEL: 'grok-4.5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'grok-4.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'grok-4.5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'grok-4.5',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '500000',
      },
    },
    category: 'third_party',
    apiFormat: 'openai_responses',
    providerType: 'xai_oauth',
    requiresOAuth: true,
    icon: 'xai',
    iconColor: '#000000',
  },
  {
    name: 'llama.cpp',
    websiteUrl: 'https://github.com/ggml-org/llama.cpp',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'http://127.0.0.1:9931',
        ANTHROPIC_AUTH_TOKEN: 'no-key',
        ANTHROPIC_MODEL: 'ggml-org/Qwen3-Coder-30B-A3B-Instruct-Q8_0-GGUF',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'ggml-org/Qwen3-Coder-30B-A3B-Instruct-Q8_0-GGUF',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'ggml-org/Qwen3-Coder-30B-A3B-Instruct-Q8_0-GGUF',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'ggml-org/Qwen3-Coder-30B-A3B-Instruct-Q8_0-GGUF',
      },
    },
    category: 'local',
    icon: 'llamacpp',
    endpointCandidates: [
      'http://127.0.0.1:9931',
      'http://127.0.0.1:8080',
    ],
  },
  {
    name: 'LM Studio',
    websiteUrl: 'https://lmstudio.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'http://localhost:1234',
        ANTHROPIC_AUTH_TOKEN: 'lmstudio',
        ANTHROPIC_MODEL: 'openai/gpt-oss-20b',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'openai/gpt-oss-20b',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'openai/gpt-oss-20b',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'openai/gpt-oss-20b',
        CLAUDE_CODE_MAX_CONTEXT_TOKENS: '131072',
      },
    },
    category: 'local',
    icon: 'lmstudio',
    endpointCandidates: [
      'http://localhost:1234',
      'http://127.0.0.1:1234',
    ],
  },
  {
    name: 'Ollama',
    websiteUrl: 'https://ollama.com',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'http://localhost:11434',
        ANTHROPIC_AUTH_TOKEN: 'ollama',
        ANTHROPIC_API_KEY: '',
        ANTHROPIC_MODEL: 'qwen3.5',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'qwen3.5',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'qwen3.5',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'qwen3.5',
      },
    },
    category: 'local',
    icon: 'ollama',
    endpointCandidates: [
      'http://localhost:11434',
      'http://127.0.0.1:11434',
    ],
  },
  {
    name: 'vLLM',
    websiteUrl: 'https://vllm.ai',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: 'http://localhost:8000',
        ANTHROPIC_AUTH_TOKEN: 'dummy',
        ANTHROPIC_MODEL: 'Qwen/Qwen3.6-27B',
        ANTHROPIC_DEFAULT_HAIKU_MODEL: 'Qwen/Qwen3.6-27B',
        ANTHROPIC_DEFAULT_SONNET_MODEL: 'Qwen/Qwen3.6-27B',
        ANTHROPIC_DEFAULT_OPUS_MODEL: 'Qwen/Qwen3.6-27B',
      },
    },
    category: 'local',
    icon: 'vllm',
    endpointCandidates: [
      'http://localhost:8000',
      'http://127.0.0.1:8000',
    ],
  },
  {
    name: 'Custom Provider',
    nameKey: 'customProvider',
    websiteUrl: '',
    settingsConfig: {
      env: {
        ANTHROPIC_BASE_URL: '',
        ANTHROPIC_AUTH_TOKEN: '',
      },
    },
    category: 'custom',
    icon: 'custom',
    iconColor: '#6366F1',
  },
]
