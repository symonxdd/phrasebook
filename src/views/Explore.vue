<template>
  <div class="explore-page">

    <div class="filters">
      <div class="left-side">
        <div class="search-and-types">

          <div class="search-bar-wrapper">
            <input v-model="searchQuery" @input="handleSearch" type="text" ref="searchInput" class="search-bar"
              placeholder="Search..." spellcheck="false" />
            <button v-if="searchQuery" @mousedown.stop.prevent @click="clearSearch" ref="clearButton"
              class="clear-button" type="button" tabindex="-1" title="Clear">
              <i class="bi bi-x"></i>
            </button>
          </div>

          <!-- Left: Entry type filters -->
          <div class="type-filters">
            <button v-for="type in entryTypes" :key="type"
              :class="['filter-pill', { active: selectedTypes.includes(type) }]" @click="toggleType(type)">
              {{ type }}
            </button>
          </div>

        </div>
      </div>

      <div class="right-side">
        <div class="language-filters">
          <div v-for="lang in languageStore.languages.filter(l => languageStore.visibleLanguages.includes(l.code))"
            :key="lang.code" class="language-toggle"
            :class="{ inactive: !languageStore.exploreVisibleLanguages.includes(lang.code) }"
            @click="languageStore.toggleExploreLanguageVisibility(lang.code)" role="button" tabindex="0"
            @keydown.enter.prevent="languageStore.toggleExploreLanguageVisibility(lang.code)">

            <img :src="getFlagSrc(lang.code)" :alt="lang.code" class="flag-icon" />
          </div>
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
import { ref, computed, onMounted, onBeforeUnmount, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { marked } from 'marked'
import { useLanguageStore } from '../stores/languageStore'

const languageStore = useLanguageStore();

// Use localVisible for checking visibility in the UI
const isLangVisible = (code) => languageStore.exploreVisibleLanguages.includes(code);

const entryTypes = ['term', 'sentence', 'concept']
const selectedTypes = ref(['term'])
const entries = ref([])
const loading = ref(false)
const offset = ref(0)
const limit = 20
const doneLoading = ref(false)
const entryStats = ref({ total: 0, term: 0, sentence: 0, concept: 0 })
const localVisibleLanguages = ref([])

const searchQuery = ref('');
const isSearching = ref(false);

const searchInput = ref(null);
const clearButton = ref(null);

const onClearClick = async (event) => {
  event.preventDefault(); // prevent unwanted form submission
  event.stopPropagation(); // avoid triggering outer handlers

  await clearSearch(); // this already waits for DOM update and refocuses
};

const clearSearch = async () => {
  searchQuery.value = '';
  await nextTick();
  handleSearch();
  searchInput.value?.focus();
};

const handleSearch = () => {
  offset.value = 0;
  doneLoading.value = false;
  entries.value = [];
  isSearching.value = !!searchQuery.value.trim();
  loadMoreEntries(); // triggers either regular or search fetch
};

watch(() => languageStore.languages, (langs) => {
  localVisibleLanguages.value = langs.map(l => l.code)
}, { immediate: true })

const toggleFavorite = (entry) => {
  entry.isFavorite = !entry.isFavorite;
  console.log(`Toggled favorite for entry ${entry.entry_id}: ${entry.isFavorite}`);
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
  entries.value.filter((entry) =>
    selectedTypes.value.includes(entry.type) &&
    isEntryVisibleInCurrentLanguages(entry)
  )
)

function isEntryVisibleInCurrentLanguages(entry) {
  const { translations = [], sentences = [], titles = [] } = entry;
  const visibleLangs = languageStore.exploreVisibleLanguages;

  switch (entry.type) {
    case 'term':
      return translations.some(tr => visibleLangs.includes(tr.language));
    case 'sentence':
      return sentences.some(s => visibleLangs.includes(s.language));
    case 'concept':
      return titles.some(t => visibleLangs.includes(t.language));
    default:
      return false;
  }
}

const sortByPriority = (items) => {
  const order = priorityOrder.value
  return [...items].sort((a, b) => {
    const aIndex = order.indexOf(a.language)
    const bIndex = order.indexOf(b.language)
    return (aIndex === -1 ? 999 : aIndex) - (bIndex === -1 ? 999 : bIndex)
  })
}

const renderMarkdown = (content) => marked.parse(content || '')

const loadMoreEntries = async () => {
  if (loading.value || doneLoading.value) return;

  loading.value = true;
  let result;

  const params = {
    offset: offset.value,
    limit,
    search: searchQuery.value.trim(),
    types: selectedTypes.value,
    languages: languageStore.exploreVisibleLanguages,
  };

  try {
    result = isSearching.value
      ? await invoke('search_explore_entries', params)
      : await invoke('get_explore_entries', { offset: offset.value, limit });
  } catch (e) {
    console.error('Failed to load entries:', e);
    loading.value = false;
    return;
  }

  if (result.entries.length === 0) {
    doneLoading.value = true;
  } else {
    entries.value.push(...result.entries);
    offset.value += result.entries.length;
  }
  loading.value = false;
};

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

onMounted(async () => {
  loadMoreEntries();
  fetchEntryStats();
  window.addEventListener('scroll', handleScroll, { passive: true })

  await nextTick();
  searchInput.value?.focus();
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
  background-color: var(--bg-color);
  color: var(--text-color);
  padding: 1rem;
  font-family: 'Inter', sans-serif;
}

.filters {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  margin-bottom: 1rem;

  gap: 1rem;
}

.right-side {
  display: flex;
  align-items: flex-start;
  gap: 1.5rem;
  flex-wrap: wrap;
}

.entry-stats {
  font-size: 0.7rem;
  color: var(--text-color);
  opacity: 0.6;
  line-height: 1.2;
  white-space: nowrap;
}

.type-filters {
  display: flex;
  gap: 0.4rem;
  flex-wrap: wrap;
}

.language-filters {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.4rem;
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
  background-color: var(--btn-hover-bg);
}

.language-toggle .flag-icon {
  width: 24px;
  height: auto;
  filter: var(--flag-filter);
  transition: filter 0.3s ease, transform 0.2s ease, box-shadow 0.3s ease;
}

.flag-icon {
  width: 18px;
  height: 12px;
  border-radius: 2px;
  object-fit: cover;
}

.language-toggle.inactive .flag-icon {
  filter: brightness(70%);
}

.filter-pill {
  font-size: 0.95rem;
  font-family: Inter;
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
  backdrop-filter: blur(2px);
  transition: all 0.2s ease-in-out;
}

.filter-pill:hover {
  background-color: var(--btn-hover-bg);
  opacity: 1;
}

.filter-pill.active {
  background-color: var(--btn-hover-bg);
  border-color: var(--border-color);
  color: var(--text-color);
  opacity: 1;
}

.entry-card {
  background-color: var(--card-bg);
  border: 1px solid var(--border-color);
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
  color: var(--text-color);
  opacity: 0.6;
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
  color: var(--text-color);
  gap: 0.6rem;
}

.sentence,
.term-definition {
  color: var(--text-color);
  opacity: 0.7;
  font-size: 0.85rem;
}

.icon-button.favorite-button {
  position: absolute;
  top: 0;
  right: 0;
  background: none;
  border: none;
  color: #888;
  /* you can also theme this if needed */
  font-size: 1rem;
  cursor: pointer;
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
  filter: var(--flag-filter);
}

.search-and-types {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.6rem;
}

.search-bar-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.search-bar {
  font-family: Inter, sans-serif;
  height: 38px;
  padding: 0 2.2rem 0 1rem;
  border-radius: 999px;
  border: 1px solid var(--border-color);
  background-color: var(--card-bg);
  color: var(--secondary-text-color);
  outline: none;
  transition: all 0.3s ease;
  width: 180px;
  max-width: 100%;
  line-height: 1;
  display: flex;
  align-items: center;
  box-shadow: 0 0 0 0 transparent;
}

.search-bar::placeholder {
  font-family: Inter, sans-serif;
  color: var(--text-color);
  opacity: 0.5;
  transition: opacity 0.2s ease;
}

.search-bar:hover {
  background-color: var(--btn-hover-bg);
  border-color: var(--border-color);
}

.search-bar:focus {
  width: 260px;
  background-color: var(--card-bg);
  border-color: var(--accent-color, #409eff);
  box-shadow: 0 0 0 3px rgba(64, 158, 255, 0.15);
}

.search-bar:focus::placeholder {
  opacity: 0.3;
}

.clear-button {
  position: absolute;
  top: 50%;
  /* Position from top */
  transform: translateY(-50%);
  /* Pull up by 50% of its height */

  right: 1rem;
  background: none;
  border: none;
  /* color: var(--text-color); */
  color: #ff6b81;
  font-size: 1.4rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.2s ease;
  user-select: none;
}

.clear-button:hover {
  color: var(--text-color);
}

.clear-button i {
  transform: translateY(1px);
}
</style>
