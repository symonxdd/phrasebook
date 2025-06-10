import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useLanguageStore = defineStore('language', () => {
  const languages = ref([])
  const visibleLanguages = ref([])
  const exploreVisibleLanguages = ref([])

  // Load languages and initialize visibleLanguages
  const loadLanguages = async () => {
    if (languages.value.length) return

    const savedOrder = localStorage.getItem('languageOrder')
    const savedVisibility = localStorage.getItem('visibleLanguages')
    const savedExploreVisibility = localStorage.getItem('exploreVisibleLanguages')
    console.log("localStorage exploreVisibleLanguages:", savedExploreVisibility);

    const result = await invoke('get_all_languages')

    // Sort by savedOrder if exists
    let sortedLanguages = result
    if (savedOrder) {
      try {
        const parsedOrder = JSON.parse(savedOrder)
        sortedLanguages = parsedOrder
          .map(code => result.find(lang => lang.code === code))
          .filter(Boolean)
      } catch (e) {
        console.error('Invalid JSON in localStorage for languageOrder', e)
      }
    }

    // Append missing languages
    const missing = result.filter(lang => !sortedLanguages.some(l => l.code === lang.code))
    languages.value = [...sortedLanguages, ...missing]

    // Load global visibility
    if (savedVisibility) {
      try {
        visibleLanguages.value = JSON.parse(savedVisibility)
      } catch {
        visibleLanguages.value = languages.value.map(l => l.code)
      }
    } else {
      visibleLanguages.value = languages.value.map(l => l.code)
    }

    if (savedExploreVisibility) {
      try {
        const parsed = JSON.parse(savedExploreVisibility)
        // Only keep codes that exist in loaded languages
        exploreVisibleLanguages.value = parsed.filter(code =>
          result.some(lang => lang.code === code)
        )
      } catch {
        exploreVisibleLanguages.value = result.map(l => l.code)
      }
    } else {
      exploreVisibleLanguages.value = result.map(l => l.code)
    }
  }

  // Watchers to persist to localStorage
  watch(languages, val => {
    const codes = val.map(lang => lang.code)
    localStorage.setItem('languageOrder', JSON.stringify(codes))
  }, { deep: true })

  watch(visibleLanguages, (newVisible) => {
    localStorage.setItem('visibleLanguages', JSON.stringify(newVisible))

    // Add new visible languages that aren't in exploreVisibleLanguages yet
    newVisible.forEach(code => {
      if (!exploreVisibleLanguages.value.includes(code)) {
        exploreVisibleLanguages.value.push(code)
      }
    })

    // Remove any codes from exploreVisibleLanguages that are no longer visible
    exploreVisibleLanguages.value = exploreVisibleLanguages.value.filter(code =>
      newVisible.includes(code)
    )
  }, { deep: true })

  watch(exploreVisibleLanguages, val => {
    console.log("setting exploreVisibleLanguages, new value:", val);

    localStorage.setItem('exploreVisibleLanguages', JSON.stringify(val))
  }, { deep: true })

  const toggleLanguageVisibility = (code) => {
    const idx = visibleLanguages.value.indexOf(code)
    if (idx !== -1) visibleLanguages.value.splice(idx, 1)
    else visibleLanguages.value.push(code)
  }

  const toggleExploreLanguageVisibility = (code) => {
    console.log("Toggling explore language visibility for code:", code);

    const idx = exploreVisibleLanguages.value.indexOf(code)
    if (idx !== -1) exploreVisibleLanguages.value.splice(idx, 1)
    else exploreVisibleLanguages.value.push(code)
  }

  return {
    languages,
    visibleLanguages,
    exploreVisibleLanguages,
    toggleLanguageVisibility,
    toggleExploreLanguageVisibility,
    loadLanguages,
  }
})
