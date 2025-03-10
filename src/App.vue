<template>
  <div class="app-container">
    <WelcomeScreen v-if="terms.length === 0 && !addingFirstItem" @add-first-item="addingFirstItem = true" />
    <AddItem v-if="addingFirstItem" @cancel="addingFirstItem = false" @term-added="handleTermAdded" />
    <router-view v-else-if="terms.length > 0" />
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import WelcomeScreen from "./components/WelcomeScreen.vue";
import AddItem from './components/AddItem.vue';

const terms = ref([]);
const addingFirstItem = ref(false);

async function handleTermAdded() {
  await loadTerms();
  addingFirstItem.value = false; // Exit add item screen
}

onMounted(async () => {
  await loadTerms();
});

async function loadTerms() {
  try {
    terms.value = await invoke('load_terms');
    console.log('Terms loaded successfully:', terms.value.length);
  } catch (error) {
    console.log("Failed to load terms:", error);
  }
}
</script>

<style scoped>
.app-container {
  background: #121212;
  color: #fff;
  font-family: "Inter", sans-serif;
  height: 100vh;
  padding: 20px;
}
</style>
