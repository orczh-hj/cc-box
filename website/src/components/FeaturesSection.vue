<script setup lang="ts">
interface Feature {
  icon: string
  title: string
  desc: string
}

// Feather Icons（MIT）路径
const features: Feature[] = [
  {
    icon: '<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>',
    title: '多项目管理',
    desc: '一个窗口浏览所有项目，查看活跃会话，一键启动新会话、即时切换，告别频繁 cd 与多终端窗口。',
  },
  {
    icon: '<polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/>',
    title: '多会话并行',
    desc: '任意数量的 Claude Code 会话在独立终端标签页运行，侧边栏快速切换，切换后输出内容保持不变。',
  },
  {
    icon: '<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>',
    title: '快速启动与预设',
    desc: '为每个项目设置 --resume、--model 等启动选项，一键以预设配置启动会话，无需重复输入参数。',
  },
  {
    icon: '<path d="M12 22v-5"/><path d="M9 8V2"/><path d="M15 8V2"/><path d="M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8z"/>',
    title: 'Provider 管理',
    desc: '一键切换 API Provider。内置 50+ 厂商预设，支持导入 cc-switch 配置，以合并方式写入 settings.json。',
  },
  {
    icon: '<rect x="3" y="3" width="18" height="18" rx="2"/><line x1="9" y1="3" x2="9" y2="21"/>',
    title: '侧边栏面板',
    desc: '会话、MCP 服务器、Skills & Agents、插件一目了然，状态指示灯实时显示运行/思考/等待，可一键开关。',
  },
  {
    icon: '<rect x="5" y="2" width="14" height="20" rx="2"/><line x1="12" y1="18" x2="12.01" y2="18"/>',
    title: '浏览器远程访问',
    desc: '电脑或手机浏览器扫码配对一次即可接管桌面会话。局域网直连或自建中继穿透公网，双向文件传输，默认安全。',
  },
]
</script>

<template>
  <section id="features" class="section">
    <div class="container">
      <h2 class="section-title">核心功能</h2>
      <p class="section-sub">GUI 做增强不做替代，让 CLI 做不好的事变得容易</p>
      <div class="grid">
        <div v-for="f in features" :key="f.title" class="card">
          <span class="card-icon">
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              v-html="f.icon"
            ></svg>
          </span>
          <h3 class="card-title">{{ f.title }}</h3>
          <p class="card-desc">{{ f.desc }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
}

.card {
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-xl);
  padding: 26px 24px;
  box-shadow: var(--shadow-md);
  transition: box-shadow 0.15s ease, transform 0.15s ease;
}

.card:hover {
  box-shadow: var(--shadow-lg);
  transform: translateY(-2px);
}

.card-icon {
  display: inline-flex;
  width: 40px;
  height: 40px;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  background: rgba(30, 58, 95, 0.08);
  color: var(--accent-primary);
  margin-bottom: 14px;
}

.card-icon svg {
  width: 20px;
  height: 20px;
}

.card-title {
  font-size: 16px;
  font-weight: 700;
  margin-bottom: 8px;
}

.card-desc {
  font-size: 13.5px;
  color: var(--text-secondary);
  line-height: 1.65;
}

@media (max-width: 900px) {
  .grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .grid {
    grid-template-columns: 1fr;
  }
}
</style>
