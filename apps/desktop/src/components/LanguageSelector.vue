<template>
  <div class="top-controls">
    <div class="language-controls">
      <small class="hint">Select language versions</small>
      <div class="language-selector">
        <button v-for="lang in languages" :key="lang.code"
          :class="['flag-pill', { active: modelValue.selectedLangs.includes(lang.code) }]"
          @click="toggleLang(lang.code)">
          <img :src="getFlagSrc(lang.code)" :alt="lang.code" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useLanguageStore } from '../stores/languageStore'

const props = defineProps({
  modelValue: {
    type: Object,
    required: true,
  }
})

const emit = defineEmits(['update:modelValue'])

const languageStore = useLanguageStore()
const languages = computed(() => languageStore.languages)

const updateModel = (key, value) => {
  emit('update:modelValue', { ...props.modelValue, [key]: value })
}

const toggleLang = (code) => {
  const selected = [...props.modelValue.selectedLangs]
  const index = selected.indexOf(code)

  if (index >= 0) {
    selected.splice(index, 1)
  } else {
    selected.push(code)
  }

  updateModel('selectedLangs', selected)
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
</script>

<style scoped>
.add-entry-wrapper {
  padding: 20px;
  color: var(--text-color);
  background-color: var(--bg-color);
  min-height: 100vh;
  font-family: 'Inter', sans-serif;
}

.page-title {
  max-width: 720px;
  margin-left: 11px;
  padding-bottom: 11px;
  font-size: 1.25rem;
  font-weight: 600;
  font-family: 'Inter', sans-serif;
}

.top-controls {
  max-width: 720px;
  margin: auto;
  display: flex;
  justify-content: flex-end;
  /* Push content to right */

  /* grid-auto-flow: column; */
  /* grid-template-columns: 1fr 1fr 1fr; */
  /* Adjust depending on how wide you want each section */
  gap: 10px;
  margin-bottom: 20px;
  align-items: start;
}

.type-controls,
.language-controls {
  align-self: start;
  text-align: center;

  padding: 11px;
  border-radius: 17px;
  /* border: 1px solid var(--border-color); */

  /* justify-self: end; */
}

.language-selector {
  display: flex;
  gap: 6px;
  align-items: center;
  flex-wrap: wrap;
  margin-top: 6px;
  justify-content: center;
}

.language-controls .hint {
  margin-bottom: 4px;
  font-size: 0.85rem;
  color: var(--secondary-text-color);
}

.type-controls {
  display: flex;
  flex-direction: column;
}

.form-card {
  max-width: 720px;
  margin: auto;
  padding: 20px 25px 20px 25px;
  background-color: var(--card-bg);
  border: 1px solid var(--border-color);
  border-radius: 12px;
}

.title-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
  /* reduce gap since hint comes next */
}

.form-title {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
}

.hint {
  text-align: center;
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--secondary-text-color);
  margin-bottom: 6px;
}

.entry-type-selector {
  display: flex;
  gap: 10px;
}

.flag-pill {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 4px;
  background: transparent;
  cursor: pointer;
  transition: transform 0.15s ease-in-out;
  width: 30px;
  height: 30px;
  display: grid;
  place-items: center;
}

.flag-pill.active {
  border-color: var(--nav-indicator-color);
}

.flag-pill img {
  width: 20px;
  height: 20px;
  object-fit: contain;
}

.type-filters {
  display: flex;
  gap: 0.4rem;
}

input,
textarea,
select {
  width: 100%;
  padding: 10px;
  border: 1px solid var(--input-border);
  border-radius: 6px;
  background-color: var(--bg-color);
  color: var(--text-color);
  margin-top: 5px;
  margin-bottom: 10px;
  font-family: 'Inter', sans-serif;
}

input[type="text"],
select {
  max-width: 280px;
}

select {
  width: auto;
  min-width: fit-content;
  padding-right: 1.5rem;
}

.markdown-editor {
  font-family: 'Cascadia Code', monospace !important;
}

.form-language-block {
  border-top: 1px dashed var(--border-color);
  padding-top: 10px;
  margin-top: 10px;
}

.form-language-block h4 {
  margin: 0 0 5px;
  font-size: 0.95rem;
  color: var(--secondary-text-color);
}

.form-language-block:first-of-type {
  border-top: none;
  padding-top: 0;
  margin-top: 0;
}

.form-actions {
  margin-top: 20px;
  display: flex;
  gap: 12px;
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

.btn i {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  line-height: 1;
  vertical-align: middle;
}

.custom-checkbox-label {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
  cursor: pointer;
  color: var(--text-color);
}

.custom-checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-color);
  appearance: none;
  cursor: pointer;
  position: relative;
}

.custom-checkbox-label input[type="checkbox"]:checked::after {
  content: '';
  position: absolute;
  width: 8px;
  height: 8px;
  top: 3px;
  left: 3px;
  background-color: var(--nav-indicator-color);
  border-radius: 2px;
}

.back-button {
  display: flex;
  align-items: center;
  gap: 6px;
  border: none;
  background-color: transparent;
  color: var(--text-color);
  font-size: 0.95rem;
  cursor: pointer;
  margin-bottom: 10px;
  transition: all 0.2s ease;
  opacity: 0.85;
}

.back-button i {
  font-size: 1.1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
}

.back-button:hover {
  transform: translateX(-2px);
  opacity: 1;
  color: var(--nav-indicator-color);
}

.back-button:active {
  transform: scale(0.96) translateX(-1px);
}

.title-bar {
  color: var(--text-color);
  display: flex;
  align-items: center;
  gap: 12px;
  /* space between back button and title */
  max-width: 720px;
  margin: auto;
  margin-bottom: 15px;
}
</style>