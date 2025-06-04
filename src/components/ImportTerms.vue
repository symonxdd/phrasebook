<template>
  <div class="import-container">
    <h2>Import Terms</h2>
    <textarea v-model="notes" placeholder="Paste your notes here..."></textarea>
    <button @click="importNotes">Import</button>
    <p v-if="message">{{ message }}</p>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const notes = ref("");
const message = ref("");

const importNotes = async () => {
  if (!notes.value.trim()) {
    message.value = "Paste some text first!";
    return;
  }

  try {
    const result = await invoke("import_terms_from_text", { notes: notes.value });
    message.value = result;
    notes.value = ""; // Clear input after success
  } catch (error) {
    message.value = "Failed to import terms.";
    console.error(error);
  }
};
</script>

<style scoped>
.import-container {
  width: 70%;
  margin: auto;
  /* text-align: center; */
}

h2 {
  margin-bottom: 20px;
}

textarea {
  width: 100%;
  height: 200px;
  margin-bottom: 10px;
  padding: 10px;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  border: 1px solid #3b3b3b;
  background-color: transparent;
  border-radius: 5px;
}

button {
  padding: 10px 15px;
  /* font-size: 16px; */
  background-color: #2973d4;
  color: white;
  border: none;
  border-radius: 5px;
  margin-bottom: 15px;
}

button:hover {
  background-color: #1d60b8;
}
</style>
