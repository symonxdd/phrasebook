<template>
  <div class="add-item">
    <h2>Add new item</h2>
    <form @submit.prevent="addTerm">
      <div>
        <label for="term">Term</label>
        <input v-model="newTerm.term" id="term" type="text" required />
      </div>
      <div>
        <label for="meaning">Meaning</label>
        <input v-model="newTerm.meaning" id="meaning" type="text" required />
      </div>
      <div>
        <label for="extra">Extra Information</label>
        <input v-model="newTerm.extra" id="extra" type="text" />
      </div>
      <div>
        <label for="category">Category</label>
        <select v-model="newTerm.category" id="category">
          <option value="Terms">Terms</option>
          <option value="Concepts">Concepts</option>
          <option value="Sentencology">Sentencology</option>
        </select>
      </div>
      <div>
        <label for="tags">Tags (comma separated)</label>
        <input v-model="newTerm.tags" id="tags" type="text" />
      </div>
      <button type="submit">Add Term</button>
      <button type="button" @click="$emit('cancel')">Cancel</button>
    </form>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from 'vue';

const emit = defineEmits(['term-added', 'cancel']);

const newTerm = ref({
  term: '',
  meaning: '',
  extra: '',
  category: 'Terms',
  tags: []
});

async function addTerm() {
  try {
    await invoke('save_term', { term: newTerm.value });
    console.log('Term added successfully:', newTerm.value);
    emit('term-added');
  } catch (error) {
    console.log("Failed to load terms:", error);
  }

  newTerm.value = { term: '', meaning: '', extra: '', category: 'Terms', tags: [] }; // Reset form
}
</script>

<style scoped>
.add-item {
  padding: 20px;
  color: white;
  border-radius: 8px;
}

form div {
  margin-bottom: 15px;
}

label {
  display: block;
}

input,
select {
  margin-top: 5px;
}

button {
  background-color: #202020;
  color: white;
  cursor: pointer;
}

button:hover {
  background-color: #6c3483;
}
</style>
