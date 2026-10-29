<template>
  <section class="section section-alt">
    <div class="container">
      <h2 class="section-title">界面一览</h2>
      <div class="shots">
        <figure class="shot">
          <div class="browser-frame">
            <div class="browser-bar">
              <span class="dot dot-red"></span>
              <span class="dot dot-yellow"></span>
              <span class="dot dot-green"></span>
            </div>
            <img src="/screenshots/docs/project-page.png" alt="项目与近期会话总览" loading="lazy" />
          </div>
          <figcaption>项目与近期会话</figcaption>
        </figure>
        <figure class="shot">
          <div class="browser-frame">
            <div class="browser-bar">
              <span class="dot dot-red"></span>
              <span class="dot dot-yellow"></span>
              <span class="dot dot-green"></span>
            </div>
            <img src="/screenshots/docs/terminal-sessions.png" alt="会话终端与侧边栏" loading="lazy" />
          </div>
          <figcaption>多会话终端</figcaption>
        </figure>
      </div>
    </div>
  </section>
</template>

<style scoped>
.shots {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 40px;
}

.shot {
  max-width: 760px;
  width: 100%;
}

.browser-frame {
  border: 1px solid var(--border-color);
  border-radius: var(--radius-xl);
  overflow: hidden;
  box-shadow: var(--shadow-xl);
  background: #fff;
}

.browser-bar {
  display: flex;
  gap: 6px;
  padding: 10px 14px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-light);
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.dot-red {
  background: #ed6a5e;
}

.dot-yellow {
  background: #f5bf4f;
}

.dot-green {
  background: #62c554;
}

figcaption {
  margin-top: 12px;
  text-align: center;
  font-size: 14px;
  color: var(--text-secondary);
}
</style>
