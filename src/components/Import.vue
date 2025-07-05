<template>
  <div class="import">
    <h2>Bulk Import</h2>
    <textarea v-model="rawJson" placeholder="Paste JSON here..." rows="15" cols="80" />
    <button @click="importEntries">Import</button>

    <p v-if="status">{{ status }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const rawJson = ref('')
const status = ref('')

async function importEntries() {
  try {
    const parsed = JSON.parse(rawJson.value)

    if (!Array.isArray(parsed)) {
      status.value = 'JSON must be an array of entries'
      return
    }

    for (const entry of parsed) {
      await invoke('add_entry_command', {
        entryType: entry.type,
        groupId: entry.group_id ?? null,
        term: entry.type === 'term' ? entry.term : null,
        concept: entry.type === 'concept' ? entry.concept : null,
        sentence: entry.type === 'sentence' ? entry.sentence : null,
      })
    }

    status.value = 'Import successful!'
  } catch (err) {
    console.error(err)
    status.value = 'Failed to import: ' + err.message
  }
}
</script>

<style scoped>
textarea {
  width: 100%;
  font-family: monospace;
}

button {
  margin-top: 10px;
}
</style>
