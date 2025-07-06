import { defineStore } from 'pinia'
import { ref, watch, reactive } from 'vue'

export const useEntryFilterStore = defineStore('entryFilter', () => {
  const selectedTypes = ref(['term'])

  // Load from localStorage on init
  const saved = localStorage.getItem('selectedTypes')

  if (saved) {
    try {
      selectedTypes.value = JSON.parse(saved)
    } catch (e) {
      console.warn('Invalid selectedTypes localStorage data:', e)
    }
  }

  // Persist to localStorage when changed
  watch(selectedTypes, (newVal) => {
    localStorage.setItem('selectedTypes', JSON.stringify(newVal))
  }, { deep: true })

  return {
    selectedTypes
  }
})
