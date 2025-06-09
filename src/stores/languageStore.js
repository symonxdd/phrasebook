import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useLanguageStore = defineStore('language', () => {
  const languages = ref([])

  const loadLanguages = async () => {
    if (languages.value.length > 0) return

    const savedOrder = localStorage.getItem('languageOrder')
    const result = await invoke('get_all_languages') // assume full objects

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

    // Append missing langs from backend
    const missing = result.filter(lang => !sortedLanguages.some(l => l.code === lang.code))
    languages.value = [...sortedLanguages, ...missing]
  }

  watch(
    languages,
    (val) => {
      const codes = val.map(lang => lang.code)
      localStorage.setItem('languageOrder', JSON.stringify(codes))
    },
    { deep: true }
  )

  return {
    languages,
    loadLanguages
  }
})
