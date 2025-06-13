<template>
  <div class="add-entry-wrapper">

    <div class="title-bar">
      <button class="back-button" @click="() => router.back()">
        <i class="bi bi-arrow-left"></i>
        <span>Back</span>
      </button>
      <h2 class="page-title">Add new entry</h2>
    </div>

    <div class="top-controls">
      <div class="type-controls">
        <small class="hint">Choose entry kind</small>
        <div class="type-filters">
          <button v-for="t in types" :key="t" :class="['filter-pill', { active: entryType === t }]"
            @click="entryType = t">
            {{ t.charAt(0).toUpperCase() + t.slice(1) }}
          </button>
        </div>
      </div>

      <div class="language-controls">
        <small class="hint">Select language versions</small>
        <div class="language-selector">
          <button v-for="lang in languageStore.languages" :key="lang.code"
            :class="['flag-pill', { active: selectedLangs.includes(lang.code) }]" @click="toggleLang(lang.code)">
            <img :src="getFlagSrc(lang.code)" :alt="lang.code" />
          </button>
        </div>
      </div>

      <div class="group-toggle">
        <small class="hint">Attach to a group</small>
        <label class="checkbox-container">
          <input type="checkbox" v-model="showGroup" />
          <span class="checkmark"></span>
          Add to a group
        </label>
      </div>
    </div>

    <div class="form-card" v-if="selectedLangs.length > 0">
      <form @submit.prevent="handleSubmit">
        <div class="form-group" v-if="showGroup">
          <select v-model="form.groupId">
            <option value="">None</option>
            <option v-for="group in entryGroups" :key="group.id" :value="group.id">
              {{ group.name }}
            </option>
          </select>
        </div>

        <template v-if="entryType === 'term'">
          <div v-for="code in selectedLangs" :key="code" class="form-language-block">
            <h4>{{ getLangName(code) }}</h4>
            <input type="text" v-model="form.term[code].translation" placeholder="Translation" spellcheck="false" />
            <textarea v-model="form.term[code].definition" spellcheck="false" placeholder="Definition"
              rows="2"></textarea>
          </div>
        </template>

        <template v-if="entryType === 'concept'">
          <textarea v-model="form.concept.markdown" spellcheck="false" placeholder="Markdown content" rows="5"
            class="markdown-editor"></textarea>
          <div v-for="code in selectedLangs" :key="code" class="form-language-block">
            <h4>{{ getLangName(code) }}</h4>
            <input type="text" v-model="form.concept.titles[code]" placeholder="Title" spellcheck="false" />
          </div>
        </template>

        <template v-if="entryType === 'sentence'">
          <div v-for="code in selectedLangs" :key="code" class="form-language-block">
            <h4>{{ getLangName(code) }}</h4>
            <textarea v-model="form.sentence[code]" spellcheck="false" placeholder="Sentence" rows="2"></textarea>
          </div>
        </template>

        <div class="form-actions">
          <button class="btn submit" type="submit">
            <i class="bi bi-plus-circle"></i>
            <span>Add Entry</span>
          </button>
          <button class="btn secondary" type="button" @click="resetForm">
            <i class="bi bi-arrow-counterclockwise"></i>
            <span>Reset</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, watchEffect, onMounted } from 'vue'
import { useLanguageStore } from '../stores/languageStore'
import { useRouter } from 'vue-router'

const router = useRouter()
const languageStore = useLanguageStore()

const types = ['term', 'sentence', 'concept']
const entryType = ref('term')
const selectedLangs = ref([])
const showGroup = ref(false)

const entryGroups = ref([
  { id: 1, name: 'Basics' },
  { id: 2, name: 'Advanced' },
])

const form = reactive({
  groupId: '',
  term: {},
  concept: {
    markdown: '',
    titles: {},
  },
  sentence: {},
})

function getLangName(code) {
  const lang = languageStore.languages.find((l) => l.code === code)
  return lang ? lang.name : code
}

function toggleLang(code) {
  const i = selectedLangs.value.indexOf(code)
  if (i >= 0) {
    selectedLangs.value.splice(i, 1)
  } else {
    selectedLangs.value.push(code)
  }
}

function resetForm() {
  form.groupId = ''
  form.term = {}
  form.concept.markdown = ''
  form.concept.titles = {}
  form.sentence = {}
  showGroup.value = false
}

function handleSubmit() {
  console.log('Submitted:', {
    type: entryType.value,
    data: JSON.parse(JSON.stringify(form)),
  })
  resetForm()
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

watchEffect(() => {
  selectedLangs.value.forEach((code) => {
    if (entryType.value === 'term') {
      if (!form.term[code]) {
        form.term[code] = { translation: '', definition: '' }
      }
    } else if (entryType.value === 'concept') {
      if (!form.concept.titles[code]) {
        form.concept.titles[code] = ''
      }
    } else if (entryType.value === 'sentence') {
      if (!form.sentence[code]) {
        form.sentence[code] = ''
      }
    }
  })
})

onMounted(() => {
  const preferred = ['en', 'pl']
  preferred.forEach(code => {
    if (languageStore.languages.some(l => l.code === code)) {
      selectedLangs.value.push(code)
    }
  })
})
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
  display: grid;
  grid-auto-flow: column;
  grid-template-columns: 1fr 1fr 1fr;
  /* Adjust depending on how wide you want each section */
  gap: 10px;
  margin-bottom: 20px;
  align-items: start;
}

.type-controls,
.language-controls,
.group-toggle {
  align-self: start;
  text-align: center;

  padding: 11px;
  border-radius: 17px;
  border: 1px solid var(--border-color)
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

.filter-pill {
  font-size: 0.95rem;
  font-family: 'Inter', sans-serif;
  height: 38px;
  padding: 0 1rem;
  background-color: transparent;
  border: 1px solid var(--border-color);
  border-radius: 999px;
  color: var(--secondary-text-color);
  cursor: pointer;
  user-select: none;
  font-weight: 500;
  display: flex;
  align-items: center;
  line-height: 1;
  white-space: nowrap;
  transition: all 0.2s ease-in-out;
}

.filter-pill:hover {
  background-color: var(--btn-hover-bg);
}

.filter-pill.active {
  background-color: var(--btn-hover-bg);
  border-color: var(--border-color);
  color: var(--text-color);
}

.checkbox-container {
  display: inline-flex;
  align-items: center;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  gap: 6px;
  position: relative;
  user-select: none;
  color: var(--secondary-text-color);
}

.checkbox-container input[type="checkbox"] {
  appearance: none;
  width: 18px;
  height: 18px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-color);
  cursor: pointer;
  position: relative;
}

.checkbox-container input[type="checkbox"]:checked::after {
  content: '';
  position: absolute;
  top: 3px;
  left: 6px;
  width: 4px;
  height: 9px;
  border: solid var(--text-color);
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
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

.form-group select {
  width: auto;
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
