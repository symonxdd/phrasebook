<template>
  <div class="settings container">
    <div class="setting-group setting-group-combined">
      <div class="sub-setting">
        <h5 class="label-heading">
          <i class="bi bi-flag" style="margin-right: 6px;"></i>
          Languages shown on Explore page
        </h5>
        <small class="label-subtext">Click flags to toggle which languages should be visible when browsing
          entries.</small>
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
          Language display order
        </h5>
        <small class="label-subtext">Drag and drop the flags to set your preferred language display order.</small>
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

    <div class="setting-group">
      <h5 class="label-heading">
        <i class="bi bi-palette" style="margin-right: 6px;"></i>
        Theme
      </h5>
      <small class="label-subtext">Choose app appearance</small>
      <div class="btn-group">
        <button class="btn" @click="setTheme('light')" :class="{ active: themeStore.theme === 'light' }">Light</button>
        <button class="btn" @click="setTheme('dark')" :class="{ active: themeStore.theme === 'dark' }">Dark</button>
        <button class="btn" @click="setTheme('system')"
          :class="{ active: themeStore.theme === 'system' }">System</button>
      </div>
    </div>

    <div class="setting-group setting-group-combined">
      <div class="sub-setting">
        <h5 class="label-heading">
          <i class="bi bi-cloud-arrow-up" style="margin-right: 6px;"></i>
          Add & Import
        </h5>
        <div class="btn-group">
          <button class="btn" @click="$router.push('/import')">
            <i class="bi bi-file-earmark-arrow-up"></i>
            <span>Import</span>
          </button>
          <button class="btn" @click="$router.push('/add')">
            <i class="bi bi-plus-circle"></i>
            <span>Add manually</span>
          </button>
        </div>
      </div>

      <div class="sub-setting">
        <h5 class="label-heading">
          <i class="bi bi-folder2-open" style="margin-right: 6px;"></i>
          Open install location
        </h5>
        <button class="btn open-btn" @click="openAppLocalAppData">
          <i class="bi bi-folder"></i>
          <span>Open install location</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import draggable from 'vuedraggable'
import { invoke } from "@tauri-apps/api/core"
import { revealItemInDir } from '@tauri-apps/plugin-opener'
import { useLanguageStore } from '../stores/languageStore'
import { useThemeStore } from '../stores/themeStore'

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
.settings {
  padding: 20px;
  color: var(--text-color);
}

.setting-group {
  width: fit-content;
  margin-bottom: 18px;
  padding: 16px;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  border: 1px solid var(--border-color);
  transition: background-color 0.2s ease;
}

h5.label-heading {
  font-size: 0.95rem;
  color: var(--text-color);
  opacity: 0.7;
  margin-bottom: 5px;
  font-weight: 500;
}

.label-subtext {
  display: block;
  font-size: 0.8rem;
  color: var(--text-color);
  opacity: 0.5;
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
  font-family: Inter;
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

.btn-group {
  display: flex;
  gap: 10px;
}

.btn.active {
  border-color: #8e44ad;
  background-color: rgba(142, 68, 173, 0.2);
  color: #8e44ad;
}

.flag-row {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

.flag-icon {
  display: block;
  width: 40px;
  height: 40px;
  border-radius: 4px;
  object-fit: cover;
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

.setting-group-combined {
  width: fit-content;
  max-width: 100%;
  padding: 20px;
  border-radius: 14px;
  border: 1px solid var(--border-color);
  box-shadow: 0 1px 5px rgba(0, 0, 0, 0.06);
}

.setting-group-combined .sub-setting + .sub-setting {
  margin-top: 10px;
  padding-top: 9px;
  border-top: 1px dashed var(--border-color);
}

.setting-group-combined i {
  filter: drop-shadow(0 0 0.6px currentColor);
}
</style>
