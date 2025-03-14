<template>
  <div class="app-layout">
    <Sidebar />
    <div class="app-content">
      <WelcomeScreen v-if="terms.length === 0 && !addingFirstItem" @add-first-item="addingFirstItem = true" />
      <AddItem v-if="addingFirstItem" @cancel="addingFirstItem = false" @term-added="handleTermAdded" />
      <router-view v-else-if="terms.length > 0" />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import WelcomeScreen from "./components/WelcomeScreen.vue";
import AddItem from './components/AddItem.vue';
import Sidebar from "./components/Sidebar.vue";

const terms = ref([]);
const addingFirstItem = ref(false);

async function handleTermAdded() {
  await loadTerms();
  addingFirstItem.value = false;
}

onMounted(async () => {
  await loadTerms();
});

async function loadTerms() {
  try {
    terms.value = await invoke('load_terms');
  } catch (error) {
    console.log("Failed to load terms:", error);
  }
}
</script>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
}

.app-content {
  flex-grow: 1;
  background: #121212;
  color: white;
  padding: 10px;
  overflow-y: auto;
}
</style>
