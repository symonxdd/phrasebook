<template>
  <div class="edit-entry-wrapper">
    <div v-if="error" class="error">{{ error }}</div>
    <component v-else :is="currentEditor" :entryData="entryData" @saved="handleSaved" />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

import EditTerm from './EditTerm.vue'
import EditConcept from './EditConcept.vue'
import EditSentence from './EditSentence.vue'

const route = useRoute()
const router = useRouter()

const entryId = Number(route.params.entryId)

const error = ref(null)
const entryData = ref(null)
const entryType = ref(null)

// map entry types to components
const editors = {
  term: EditTerm,
  concept: EditConcept,
  sentence: EditSentence,
}

const currentEditor = computed(() => {
  return editors[entryType.value] || null
})

async function fetchEntry() {
  try {
    error.value = null
    entryData.value = await invoke('get_entry_by_id_command', { entry_id: entryId })
    entryType.value = entryData.value.type
  } catch (e) {
    error.value = e.toString()
  }
}

function handleSaved() {
  router.push('/explore')
}

onMounted(() => {
  fetchEntry()
})
</script>

<style scoped>
.edit-entry-wrapper {
  padding: 25px 0 150px 0;
}

.error {
  color: #c21c1c;
}
</style>
