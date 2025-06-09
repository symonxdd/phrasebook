<template>
  <div class="explore-page">
    <div class="filters">
      <!-- Left: Entry type filters -->
      <div class="type-filters">
        <button v-for="type in entryTypes" :key="type"
          :class="['filter-pill', { active: selectedTypes.includes(type) }]" @click="toggleType(type)">
          {{ type }}
        </button>
      </div>

      <div class="right-side">
        <div class="language-filters">
          <label v-for="lang in allLanguages" :key="lang.code" class="language-toggle">
            <input type="checkbox" :checked="visibleLanguages.includes(lang.code)"
              @change="toggleLanguage(lang.code)" />
            <img :src="getFlagSrc(lang.code)" :alt="lang.code" class="flag-icon" />
          </label>
        </div>

        <!-- <div class="entry-stats">
          <div>Total entries: {{ entryStats.total }}</div>
          <div>Terms: {{ entryStats.term }}</div>
          <div>Sentences: {{ entryStats.sentence }}</div>
          <div>Concepts: {{ entryStats.concept }}</div>
        </div> -->
      </div>
    </div>

    <div class="entries">
      <div v-for="entry in filteredEntries" :key="entry.entry_id + '-' + entry.type" class="entry-card">
        <template v-if="entry.type === 'term'">
          <ul>
            <li v-for="(t, index) in sortByPriority(entry.translations.filter(tr => isLangVisible(tr.language)))"
              :key="t.language" class="term-item">
              <button v-if="index === 0" class="icon-button favorite-button" @click="toggleFavorite(entry)">
                <i class="bi" :class="entry.isFavorite ? 'bi-heart-fill' : 'bi-heart'"></i>
              </button>

              <div class="translation-line">
                <span class="term-translation">
                  {{ t.translation }}
                </span>
                <span class="term-definition">: {{ t.definition }}</span>
              </div>
            </li>
          </ul>
        </template>

        <template v-if="entry.type === 'sentence'">
          <ul>
            <li v-for="s in sortByPriority(entry.sentences.filter(st => isLangVisible(st.language)))" :key="s.language">
              <span class="sentence">{{ s.sentence }}</span>
            </li>
          </ul>
        </template>

        <template v-if="entry.type === 'concept'">
          <ul>
            <li v-for="t in entry.titles.filter(tr => isLangVisible(tr.language))" :key="t.language">
              <img v-if="!['en', 'pl', 'nl'].includes(t.language)" :src="getFlagSrc(t.language)" alt="flag"
                class="flag-icon" />
              {{ t.title }}
            </li>
          </ul>
          <div class="markdown" v-html="renderMarkdown(entry.markdown_content)" />
        </template>
      </div>

      <div class="loading" v-if="loading">Loading more...</div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { marked } from 'marked'
import { useLanguageStore } from '../stores/languageStore'

const languageStore = useLanguageStore();

const allLanguages = computed(() =>
  languageStore.languages.filter(lang => lang.code !== 'en')
)

const entryTypes = ['term', 'sentence', 'concept']
const selectedTypes = ref(['term'])
const entries = ref([])
const loading = ref(false)
const offset = ref(0)
const limit = 20
const doneLoading = ref(false)
const visibleLanguages = ref([])
const entryStats = ref({ total: 0, term: 0, sentence: 0, concept: 0 })

watch(
  allLanguages,
  (langs) => {
    if (visibleLanguages.value.length === 0) {
      visibleLanguages.value = langs.map(l => l.code)
    }
  },
  { immediate: true }
)

const toggleFavorite = (entry) => {
  entry.isFavorite = !entry.isFavorite;
  console.log(`Toggled favorite for entry ${entry.entry_id}: ${entry.isFavorite}`);

  // Optionally save to localStorage or backend
}

const toggleLanguage = (lang) => {
  const idx = visibleLanguages.value.indexOf(lang)
  if (idx !== -1) {
    visibleLanguages.value.splice(idx, 1)
  } else {
    visibleLanguages.value.push(lang)
  }
}

const toggleType = (type) => {
  if (selectedTypes.value.includes(type)) {
    selectedTypes.value = selectedTypes.value.filter((t) => t !== type)
  } else {
    selectedTypes.value.push(type)
  }
}

const priorityOrder = computed(() =>
  languageStore.languages.map(lang => lang.code)
)

const filteredEntries = computed(() =>
  entries.value.filter((entry) => selectedTypes.value.includes(entry.type))
)

const sortByPriority = (items) => {
  const order = priorityOrder.value
  return [...items].sort((a, b) => {
    const aIndex = order.indexOf(a.language)
    const bIndex = order.indexOf(b.language)
    return (aIndex === -1 ? 999 : aIndex) - (bIndex === -1 ? 999 : bIndex)
  })
}

// Check if a lang should be shown
const isLangVisible = (lang) => lang === 'en' || visibleLanguages.value.includes(lang)

const renderMarkdown = (content) => marked.parse(content || '')

const loadMoreEntries = async () => {
  if (loading.value || doneLoading.value) return

  loading.value = true
  const result = await invoke('get_explore_entries', { offset: offset.value, limit })
  if (result.entries.length === 0) {
    doneLoading.value = true
  } else {
    entries.value.push(...result.entries)
    offset.value += result.entries.length
  }
  loading.value = false
}

const handleScroll = () => {
  const scrollY = window.scrollY
  const clientHeight = document.documentElement.clientHeight
  const scrollHeight = document.documentElement.scrollHeight

  if (scrollY + clientHeight >= scrollHeight - 200) {
    loadMoreEntries()
  }
}

const fetchEntryStats = async () => {
  try {
    const stats = await invoke('get_entry_stats')
    entryStats.value = stats
  } catch (e) {
    console.error('Failed to fetch entry stats:', e)
  }
}

onMounted(() => {
  loadMoreEntries();
  fetchEntryStats();
  window.addEventListener('scroll', handleScroll, { passive: true })
})

onBeforeUnmount(() => {
  window.removeEventListener('scroll', handleScroll)
})

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
.explore-page {
  background-color: #121212;
  color: #eee;
  padding: 1rem;
  font-family: 'Inter', sans-serif;
}

.filters {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  margin-bottom: 1rem;
}

.right-side {
  display: flex;
  align-items: flex-start;
  gap: 1.5rem;
  flex-wrap: wrap;
}

.entry-stats {
  font-size: 0.7rem;
  color: #7c7c7c;
  line-height: 1.2;
  white-space: nowrap;
}

.type-filters {
  display: flex;
  gap: 0.5rem;
}

.language-filters {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
}

.language-toggle {
  position: relative;
  cursor: pointer;
  margin: 0.3rem 0.4rem;
  border-radius: 6px;
  background-color: transparent;
  border: none;
  display: flex;
  align-items: center;
  transition: background-color 0.2s ease-in-out;
}

.language-toggle:hover {
  background-color: rgba(255, 255, 255, 0.05);
}

.language-toggle input[type="checkbox"] {
  position: absolute;
  opacity: 0;
  pointer-events: none;
  width: 0;
  height: 0;
}

.language-toggle .flag-icon {
  width: 24px;
  height: 16px;
  border-radius: 2px;
  object-fit: cover;
  filter: brightness(50%) grayscale(30%);
  transition: filter 0.3s ease, transform 0.2s ease, box-shadow 0.3s ease;
}

.flag-icon {
  width: 18px;
  height: 12px;
  border-radius: 2px;
  object-fit: cover;
  margin-right: 0.7rem;
}

.language-toggle input[type="checkbox"]:checked + .flag-icon {
  filter: brightness(85%) grayscale(30%);
}

.filter-pill {
  background-color: transparent;
  border: 1px solid #444;
  padding: 0.5rem 1rem;
  border-radius: 999px;
  color: #ccc;
  cursor: pointer;
  user-select: none;
  transition: all 0.2s ease-in-out;
  font-weight: 500;
  backdrop-filter: blur(2px);
}

.filter-pill:hover {
  background-color: rgba(255, 255, 255, 0.05);
  border-color: #666;
}

.filter-pill.active {
  background-color: rgba(255, 255, 255, 0.068);
  border-color: #383838;
  color: #dddddd;
}

.entry-card {
  background-color: #1a1a1a;
  border: 1px solid #2a2a2a;
  padding: 0.5rem 1rem;
  margin-bottom: 0.4rem;
  border-radius: 0.5rem;
}

.markdown {
  margin-top: 1rem;
}

.loading {
  text-align: center;
  padding: 1rem;
  color: #888;
  user-select: none;
}

ul {
  list-style-type: none;
  padding-left: 0;
  margin-left: 0;
}

li {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  font-size: 0.85rem;
  line-height: 1.2;
  position: relative;
}

li:not(:last-child) {
  margin-bottom: 0.4rem;
}

.term-translation {
  display: inline-flex;
  align-items: center;
  font-weight: 500;
  color: #cacaca;
  gap: 0.6rem;
}

.sentence,
.term-definition {
  color: #aaa;
  font-size: 0.85rem;
}

.term-definition {
  color: #aaa;
  font-size: 0.85rem;
}

.icon-button.favorite-button {
  position: absolute;
  top: 0;
  right: 0;
  background: none;
  border: none;
  color: #919191;
  font-size: 1rem;
  cursor: pointer;
  padding: 0.1rem;
  transition: color 0.2s ease;
}

.icon-button.favorite-button:hover {
  color: #ff4d4f;
}

.icon-button.favorite-button .bi-heart-fill {
  color: #ff4d4f;
}

.term-item {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.term-item > .translation-line {
  display: flex;
  align-items: center;
}

.inline-flag {
  width: 18px;
  height: 12px;
  object-fit: cover;
  border-radius: 2px;
  margin-right: 0.2rem;
  filter: brightness(80%) grayscale(30%);
}
</style>
