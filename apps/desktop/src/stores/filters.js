import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useFilterStore = defineStore('filters', () => {
  const searchQuery = ref('')
  const activeGroups = ref([])

  function toggleGroup(groupName) {
    if (activeGroups.value.includes(groupName)) {
      activeGroups.value = activeGroups.value.filter(g => g !== groupName)
    } else {
      activeGroups.value.push(groupName)
    }
  }

  function clearGroups() {
    activeGroups.value = []
  }

  return { searchQuery, activeGroups, toggleGroup, clearGroups }
})
