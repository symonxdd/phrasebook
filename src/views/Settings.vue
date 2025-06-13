<template>
  <div class="settings-wrapper">
    <div class="settings-container">

      <!-- Row: Language settings + Theme -->
      <div class="settings-top-row">
        <div class="setting-group setting-group-combined">
          <div class="sub-setting">
            <h5 class="label-heading">
              <i class="bi bi-flag" style="margin-right: 6px;"></i>
              Shown on Explore page
            </h5>
            <small class="label-subtext">Click flags to toggle which languages should be visible when browsing
              entries</small>
            <div class="flag-row toggle-flags">
              <div v-for="lang in languageStore.languages" :key="lang.code" class="flag-toggle"
                :class="{ inactive: !languageStore.visibleLanguages.includes(lang.code) }"
                @click="languageStore.toggleLanguageVisibility(lang.code)">
                <img :src="getFlagSrc(lang.code)" :alt="lang.code" class="flag-icon" />
              </div>
            </div>
          </div>

          <div class="sub-setting draggable-flags">
            <h5 class="label-heading">
              <i class="bi bi-list-nested" style="margin-right: 6px;"></i>
              Display order
            </h5>
            <small class="label-subtext">Drag and drop the flags to set your preferred language display order</small>
            <draggable :animation="200" v-model="languageStore.languages" item-key="code"
              :component-data="{ tag: 'div', class: 'flag-row' }">
              <template #item="{ element }">
                <div :key="element.code">
                  <img :src="getFlagSrc(element.code)" :alt="element.code" class="flag-icon" />
                </div>
              </template>
            </draggable>
          </div>
        </div>

        <div class="setting-group theme-setting-group">
          <h5 class="label-heading">
            <i class="bi bi-palette" style="margin-right: 6px;"></i>
            Theme
          </h5>
          <small class="label-subtext">Choose app appearance</small>
          <div class="btn-group">
            <button class="btn" @click="setTheme('light')"
              :class="{ active: themeStore.theme === 'light' }">Light</button>
            <button class="btn" @click="setTheme('dark')" :class="{ active: themeStore.theme === 'dark' }">Dark</button>
            <button class="btn" @click="setTheme('system')"
              :class="{ active: themeStore.theme === 'system' }">System</button>
          </div>
        </div>
      </div>

      <!-- Row: Add & Import + Open Location -->
      <div class="responsive-settings-row">
        <div class="setting-group setting-group-combined">
          <div class="sub-setting">
            <h5 class="label-heading">
              <i class="bi bi-cloud-arrow-up" style="margin-right: 6px;"></i>
              Add & Import
            </h5>
            <small class="label-subtext">Add new entries manually or import at once</small>

            <div class="btn-group">
              <button class="btn" @click="$router.push('/add')">
                <i class="bi bi-plus-circle"></i>
                <span>Add</span>
              </button>
              <button class="btn" @click="$router.push('/import')">
                <i class="bi bi-file-earmark-arrow-up"></i>
                <span>Import</span>
              </button>
            </div>
          </div>

          <div class="sub-setting">
            <h5 class="label-heading">
              <i class="bi bi-folder2-open" style="margin-right: 6px;"></i>
              Open install location
            </h5>
            <small class="label-subtext">Where the application files are located</small>

            <button class="btn open-btn" @click="openAppLocalAppData">
              <i class="bi bi-folder"></i>
              <span>Open</span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="app-footer">
      <div class="footer-section footer-left meta-text version">
        v{{ version }} <span class="env">({{ environment }})</span>
      </div>
      <div class="footer-section footer-center meta-text tech-links">
        Powered by
        <a href="#" class="tauri" @click.prevent="openLink('https://v2.tauri.app/')">Tauri</a>,
        <a href="#" class="rust" @click.prevent="openLink('https://www.rust-lang.org/')">Rust</a> &
        <a href="#" class="vue" @click.prevent="openLink('https://vuejs.org/')">Vue</a>
      </div>
      <div class="footer-section footer-right meta-text">
        Made with ❤️ by Symon from Belgium
      </div>
    </div>
  </div>
</template>


<script setup>
import draggable from 'vuedraggable'
import { invoke } from "@tauri-apps/api/core"
import { revealItemInDir, openUrl } from '@tauri-apps/plugin-opener'
import { useLanguageStore } from '../stores/languageStore'
import { useThemeStore } from '../stores/themeStore'
import { getVersion } from '@tauri-apps/api/app'
import { ref, onMounted } from 'vue'

const version = ref('...')
const environment = ref(import.meta.env.MODE === 'development' ? 'dev' : 'release')

onMounted(async () => {
  version.value = await getVersion()
})

const openLink = async (url) => {
  try {
    await openUrl(url)
  } catch (error) {
    console.error('Failed to open URL:', url, error)
  }
}

const languageStore = useLanguageStore()
const themeStore = useThemeStore()

const setTheme = (mode) => {
  themeStore.setTheme(mode)
}

const getFlagSrc = (code) => {
  const flagMap = {
    en: 'en.png',
    pl: 'pl.png',
    nl: 'nl.png',
    kr: 'kr.png',
    fr: 'fr.png',
    de: 'de.png',
    es: 'es.png',
  }
  return new URL(`../assets/flags/${flagMap[code]}`, import.meta.url).href
}

const openAppLocalAppData = async () => {
  const path = await invoke('get_app_localappdata')
  await revealItemInDir(`${path}/`)
}
</script>

<style scoped>
.settings-wrapper {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  padding: 10px;
}

.settings-container {
  flex: 1 0 auto;
  padding: 20px;
  color: var(--text-color);
}

/* Row containing languages + theme */
.settings-top-row {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  margin-bottom: 20px;
}

.setting-group,
.setting-group-combined {
  background: var(--background-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  padding: 20px;
  width: fit-content;
}

.setting-group-combined .sub-setting + .sub-setting {
  margin-top: 15px;
  padding-top: 15px;
  border-top: 1px dashed var(--border-color);
}

.label-heading {
  font-size: 0.95rem;
  font-weight: 500;
  color: var(--text-color);
}

.label-subtext {
  display: block;
  font-size: 0.8rem;
  color: var(--secondary-text-color);
  margin-bottom: 10px;
}

.btn {
  background-color: transparent;
  color: var(--text-color);
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  font-weight: 400;
  font-family: Inter, sans-serif;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  transition: background-color 0.2s ease-in-out, color 0.2s ease-in-out;
}

.btn:hover {
  background-color: var(--btn-hover-bg);
}

.btn i,
.btn span {
  font-size: 0.92rem;
  color: var(--text-color);
  opacity: 0.85;
}

.btn.active {
  border-color: #8e44ad;
  background-color: rgba(142, 68, 173, 0.2);
  color: #8e44ad;
}

.btn-group {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.flag-row {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

.flag-icon {
  width: 40px;
  height: 40px;
  object-fit: cover;
  border-radius: 4px;
  transition: transform 0.2s ease, filter 0.2s ease;
}

.toggle-flags .flag-toggle {
  cursor: pointer;
}

.flag-toggle:hover {
  transform: scale(1.05);
}

.flag-toggle.inactive img {
  filter: brightness(40%) grayscale(70%);
}

.draggable-flags .flag-icon {
  cursor: grab;
}

.draggable-flags .flag-icon:active {
  transform: scale(0.95);
  cursor: grabbing;
}

.theme-setting-group {
  align-self: flex-start;
}

.responsive-settings-row {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  margin-top: 10px;
}

.app-footer {
  display: flex;
  width: 100%;
  /* padding-top: 10px; */
  font-size: 0.75rem;
  color: var(--secondary-text-color);
  justify-content: space-between;
  align-items: center;
  text-align: center;
  flex-wrap: wrap;
  position: sticky;
  bottom: 0;
  background: var(--background-secondary);
  /* border-top: 1px solid var(--border-color); */
  z-index: 10;
}

.footer-section {
  /* flex: 1; */
  line-height: 1.6;
  padding: 5px;
}

/* Position helpers */
.footer-left {
  /* text-align: left; */
  order: 2;
}

.footer-center {
  /* text-align: center; */
  order: 1;
}

.footer-right {
  /* text-align: right; */
  order: 3;
}

.version .env {
  opacity: 0.6;
}

.tech-links a {
  font-weight: 500;
  text-decoration: none;
  transition: all 0.2s ease;
}

.tech-links a.tauri {
  color: #18abbb;
}

.tech-links a.rust {
  color: #e53d1e;
}

.tech-links a.vue {
  color: #47ba87;
}

.tech-links a:hover {
  text-decoration: underline;
}
</style>
