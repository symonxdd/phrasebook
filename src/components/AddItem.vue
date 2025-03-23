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
        <label for="category">Group</label>
        <select v-model="newTerm.category" id="category">
          <option disabled value="">Select group</option>
          <option v-for="group in groups" :key="group.title" :value="group.title">
            {{ group.title }}
          </option>
          <option value="__create_new__">+ Create new group...</option>
        </select>
      </div>
      <div v-if="newTerm.category === '__create_new__'">
        <label for="newGroup">New group name</label>
        <input id="newGroup" v-model="newGroupName" @blur="createNewGroup" />
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
import { ref, onMounted } from 'vue';

const emit = defineEmits(['term-added', 'cancel']);

const newTerm = ref({
  term: '',
  meaning: '',
  extra: '',
  category: 'Terms',
  tags: []
});
const newGroupName = ref('');
const groups = ref([]);

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

async function createNewGroup() {
  if (newGroupName.value.trim() !== '') {
    await invoke('save_group', { group: { title: newGroupName.value.trim() } })
    groups.value = await invoke('load_groups')
    newTerm.value.category = newGroupName.value.trim()
    newGroupName.value = ''
  }
}

onMounted(async () => {
  groups.value = await invoke('load_groups')
})
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

button {
  width: fit-content;
  padding: 6px 12px;
}

button:hover {
  background-color: #6c3483;
}

.add-item form {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 400px;
}

input,
select {
  width: 100%;
  padding: 6px;
  background: #1a1a1a;
  border: 1px solid #333;
  color: #ddd;
  border-radius: 4px;
}
</style>
