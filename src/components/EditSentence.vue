<template>
  <div class="edit-term">
    <div class="title-bar">
      <button class="back-button" @click="router.back">
        <i class="bi bi-arrow-left"></i>
      </button>
      <h2 class="page-title">{{ dynamicTitle }}</h2>
    </div>

    <LanguageSelector v-model="selectorState" />

    <div class="form-card" v-if="selectorState.selectedLangs.length > 0">
      <form @submit.prevent="handleSubmit">
        <div v-for="code in selectorState.selectedLangs" :key="code" class="form-language-block">
          <h4>{{ getLangName(code) }}</h4>
          <textarea v-model="form.sentences[code]" placeholder="Sentence" rows="2" spellcheck="false"></textarea>
        </div>

        <div class="form-group" v-if="groups.length > 0">
          <label for="groupSelect" class="group-label">
            <Icon icon="mdi:folder-outline" width="22" height="22" />
            <span>Group</span>
          </label>
          <select id="groupSelect" v-model="selectorState.groupId"
            :key="`group-select-${selectorState.selectedLangs.length}`">
            <option :value="''">None</option>
            <option v-for="group in groups" :key="group.id" :value="group.id">
              {{ group.name }}
            </option>
          </select>
        </div>

        <div class="form-actions">
          <button type="submit" class="btn submit">
            <i class="bi bi-plus-circle"></i>
            <span>Apply Changes</span>
          </button>
          <button type="button" class="btn secondary" @click="resetForm">
            <i class="bi bi-arrow-counterclockwise"></i>
            <span>Reset</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useLanguageStore } from '../stores/languageStore'
import { invoke } from '@tauri-apps/api/core'
import { Icon } from '@iconify/vue'
import LanguageSelector from './LanguageSelector.vue'

const props = defineProps({ entryData: Object })
const router = useRouter()
const languageStore = useLanguageStore()

const form = reactive({ sentences: {} })

const selectorState = ref({
  selectedLangs: [],
  groupId: '',
})

const groups = ref([])

const fetchGroups = async () => {
  try {
    groups.value = await invoke('get_all_groups')
  } catch (err) {
    console.error('Failed to fetch groups:', err)
  }
}

const langPriority = ['pl', 'en', 'nl']

const getLangName = (code) =>
  languageStore.languages.find((l) => l.code === code)?.name || code

const dynamicTitle = computed(() => {
  for (const code of langPriority) {
    const sentence = form.sentences?.[code]
    if (sentence?.trim()) return `Editing sentence: ${sentence.trim()}`
  }
  return 'Editing sentence'
})

const handleSubmit = async () => {
  try {
    const payload = {
      entry_id: props.entryData.entry_id,
      group_id: selectorState.value.groupId === '' ? null : Number(selectorState.value.groupId),
      sentence: selectorState.value.selectedLangs.map(code => ({
        language: code,
        sentence: form.sentences[code] || ''
      }))
    }

    await invoke('edit_sentence', { payload })
    router.push('/explore')
  } catch (err) {
    console.error('Failed to update sentence:', err)
  }
}

const resetForm = () => {
  if (!props.entryData) return

  const entry = props.entryData

  // Clear existing
  Object.keys(form.sentences).forEach(lang => {
    form.sentences[lang] = ''
  })

  // Refill
  entry.translations.forEach(t => {
    form.sentences[t.language] = t.sentence || ''
  })

  selectorState.value.groupId = entry.group_id || ''
}

watch(() => props.entryData, (newVal) => {
  if (newVal?.type === 'sentence' && Array.isArray(newVal.sentences)) {
    form.sentences = {}
    newVal.sentences.forEach(s => {
      form.sentences[s.language] = s.sentence || ''
    })
    selectorState.value.selectedLangs = newVal.sentences.map(s => s.language)
    selectorState.value.groupId = newVal.group_id || ''
  }
}, { immediate: true })

watch(() => selectorState.value.selectedLangs, (langs) => {
  langs.forEach(code => {
    if (!form.sentences[code]) {
      form.sentences[code] = ''
    }
  })
})

onMounted(async () => {
  await fetchGroups()

  console.log('props.entryData:', props.entryData)
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

.markdown-editor {
  font-family: 'Cascadia Code', monospace !important;
}

.form-language-block {
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
  margin-top: 50px;
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
  max-width: 720px;
  margin: auto;
}

.form-group {
  margin-top: 20px;
}

.form-group select {
  width: auto;
  display: block;
}

.form-group label {
  font-size: 0.95rem;
  color: var(--secondary-text-color);
  margin-bottom: 4px;
  display: inline-block;
}

.form-group span {
  margin-left: 5px;
}

.group-label {
  color: var(--secondary-text-color);
  line-height: 1;
}

.group-label :deep(svg) {
  display: inline-block;
  vertical-align: middle;
  transform: translateY(-1px);
}
</style>
