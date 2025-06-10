<template>
  <div class="settings container">
    <div class="setting-group">
      <h5 class="label-heading">Languages shown on Explore page</h5>
      <small class="label-subtext">Click flags to toggle which languages should be visible when
        browsing entries.</small>
      <div class="flag-row toggle-flags">
        <div v-for="lang in languageStore.languages" :key="lang.code" class="flag-toggle"
          :class="{ inactive: !languageStore.visibleLanguages.includes(lang.code) }"
          @click="languageStore.toggleLanguageVisibility(lang.code)">
          <img :src="getFlagSrc(lang.code)" :alt="lang.code" class="flag-icon" />
        </div>
      </div>
    </div>

    <div class="setting-group draggable-flags">
      <h5 class="label-heading">Language display order</h5>
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

    <div class="setting-group">
      <h5>Add & import</h5>
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

    <div class="setting-group">
      <!-- <h5>Open install location</h5> -->
      <button class="btn open-btn" @click="openAppLocalAppData">
        <i class="bi bi-folder"></i>
        <span>Open install location</span>
      </button>
    </div>
  </div>
</template>

<script setup>
import draggable from 'vuedraggable'
import { invoke } from "@tauri-apps/api/core";
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import { useLanguageStore } from '../stores/languageStore'

const languageStore = useLanguageStore()

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
  const path = await invoke('get_app_localappdata');
  console.log(path);
  await revealItemInDir(`${path}/`);
};
</script>

<style scoped>
.settings {
  padding: 20px;
  color: #333;
}

.setting-group {
  margin-bottom: 24px;
}

h5 {
  font-size: 0.95rem;
  color: #adadad;
  margin-bottom: 10px;
  font-weight: 500;
}

.btn {
  background-color: transparent;
  color: #adadad;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  font-weight: 400;
  font-family: Inter;
  border-radius: 8px;
  transition: all 0.2s ease-in-out;
  color: #333;
  border: 1px solid #363636;
}

.btn span {
  font-size: 0.92rem;
  color: #adadad;
  margin-left: 2px;
}

.btn i {
  font-size: 1.1rem;
  color: #adadad;
}

.btn:hover {
  background-color: #2b2b2b;
}

.btn-group {
  display: flex;
  gap: 10px;
}

.flag-row {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 8px;
}

.flag-icon {
  margin-right: 1.2rem;
  width: 40px;
  height: 40px;
  cursor: default;
  transition: transform 0.2s ease;
}

/* Only show grab cursor in draggable-flags section */
.draggable-flags .flag-icon {
  cursor: grab;
}

.draggable-flags .flag-icon:active {
  transform: scale(0.95);
  cursor: grabbing;
}

.flag-row.toggle-flags {
  /* margin-top: 10px; */
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.flag-toggle {
  cursor: pointer;
  transition: filter 0.2s ease, transform 0.15s ease;
}

.flag-toggle:hover {
  transform: scale(1.05);
}

.flag-toggle.inactive img {
  filter: brightness(40%) grayscale(70%);
}

.flag-toggle img {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  object-fit: cover;
  transition: filter 0.2s ease;
}

.label-heading {
  margin-bottom: 0;
}

.label-subtext {
  display: block;
  font-size: 0.8rem;
  color: #888;
  margin-bottom: 10px;
}
</style>
