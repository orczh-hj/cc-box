<script setup lang="ts">
import { onMounted, ref } from 'vue'
import DownloadSection from './components/DownloadSection.vue'
import FeaturesSection from './components/FeaturesSection.vue'
import HeroSection from './components/HeroSection.vue'
import ScreenshotSection from './components/ScreenshotSection.vue'
import SiteFooter from './components/SiteFooter.vue'
import SiteHeader from './components/SiteHeader.vue'
import { fetchLatestRelease, type LatestView } from './lib/latest'

const latest = ref<LatestView | null>(null)
const loading = ref(true)
const loadFailed = ref(false)

async function load() {
  loading.value = true
  loadFailed.value = false
  const result = await fetchLatestRelease()
  if (result) {
    latest.value = result
  } else {
    loadFailed.value = true
  }
  loading.value = false
}

onMounted(load)

function retry() {
  void load()
}
</script>

<template>
  <SiteHeader page="home" />
  <main>
    <HeroSection :latest="latest" :loading="loading" />
    <ScreenshotSection />
    <FeaturesSection />
    <DownloadSection :latest="latest" :loading="loading" :failed="loadFailed" @retry="retry" />
  </main>
  <SiteFooter />
</template>
