import { defineStore } from 'pinia'
import { ref, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useLanguageStore = defineStore('language', () => {
  const languages = ref([])
  const visibleLanguages = ref([])
  const exploreVisibleLanguages = ref([])
  const isInitializing = ref(true)

  const loadLanguages = async () => {
    const savedOrder = localStorage.getItem('languageOrder')
    const savedVisibility = localStorage.getItem('visibleLanguages')
    const savedExploreVisibility = localStorage.getItem('exploreVisibleLanguages')

    const result = await invoke('get_all_languages')

    let sorted = result
    if (savedOrder) {
      try {
        const order = JSON.parse(savedOrder)
        sorted = order.map(code => result.find(l => l.code === code)).filter(Boolean)
      } catch (e) {
        console.error('Invalid languageOrder in localStorage', e)
      }
    }

    const missing = result.filter(l => !sorted.some(s => s.code === l.code))
    languages.value = [...sorted, ...missing]

    visibleLanguages.value = savedVisibility
      ? JSON.parse(savedVisibility).filter(code => languages.value.some(l => l.code === code))
      : languages.value.map(l => l.code)

    exploreVisibleLanguages.value = savedExploreVisibility
      ? JSON.parse(savedExploreVisibility).filter(code => languages.value.some(l => l.code === code))
      : [...visibleLanguages.value]

    await nextTick()
    isInitializing.value = false
  }

  // Save visibleLanguages and sync exploreVisibleLanguages
  watch(visibleLanguages, (newVisible) => {
    if (isInitializing.value) return
    localStorage.setItem('visibleLanguages', JSON.stringify(newVisible))
    exploreVisibleLanguages.value = exploreVisibleLanguages.value.filter(code => newVisible.includes(code))
  }, { deep: true })

  // Save exploreVisibleLanguages
  watch(exploreVisibleLanguages, (newExplore) => {
    localStorage.setItem('exploreVisibleLanguages', JSON.stringify(newExplore))
  }, { deep: true })

  const toggleLanguageVisibility = (code) => {
    const idx = visibleLanguages.value.indexOf(code)
    if (idx !== -1) {
      visibleLanguages.value.splice(idx, 1)
    } else {
      visibleLanguages.value.push(code)
    }
  }

  const toggleExploreLanguageVisibility = (code) => {
    const idx = exploreVisibleLanguages.value.indexOf(code)
    if (idx !== -1) {
      exploreVisibleLanguages.value.splice(idx, 1)
    } else {
      exploreVisibleLanguages.value.push(code)
    }
  }

  return {
    languages,
    visibleLanguages,
    exploreVisibleLanguages,
    loadLanguages,
    toggleLanguageVisibility,
    toggleExploreLanguageVisibility,
  }
})
