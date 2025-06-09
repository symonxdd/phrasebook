<template>
  <div class="settings container">
    <div class="setting-group">
      <h5>Open install folder</h5>
      <button class="btn open-btn" @click="openAppLocalAppData">
        <i class="bi bi-folder"></i>
        <span>Open</span>
      </button>
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
      <h5>Language display order</h5>
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
}

.flag-icon {
  margin-right: 1.2rem;
  width: 40px;
  height: 40px;
  transition: transform 0.2s ease;
  cursor: grab;
}

.flag-icon:active {
  transform: scale(0.95);
  cursor: grabbing;
}
</style>
