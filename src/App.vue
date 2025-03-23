<template>
  <div class="app-layout">
    <Sidebar />
    <div class="app-content">
      <div class="content-fade" :class="{ visible: !loading }">
        <WelcomeScreen v-if="appStore.terms.length === 0" @add-first-item="addingFirstItem = true" />

        <AddItem v-if="addingFirstItem" @cancel="addingFirstItem = false" @term-added="handleTermAdded" />

        <router-view v-else-if="appStore.terms.length > 0" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import WelcomeScreen from "./components/WelcomeScreen.vue";
import AddItem from './components/AddItem.vue';
import Sidebar from "./components/Sidebar.vue";
import { useAppStore } from './stores/app';

const appStore = useAppStore();
const addingFirstItem = ref(false);
const loading = ref(true);

async function handleTermAdded() {
  await loadTerms();
  addingFirstItem.value = false;
}

onMounted(async () => {
  await loadTerms();
});

async function loadTerms() {
  try {
    appStore.terms = await invoke('load_terms');
    appStore.categories = await invoke("load_categories");
    appStore.groups = await invoke("load_groups");
    console.log("Loaded terms:", appStore.terms);
  } catch (error) {
    console.log("Failed to load terms:", error);
  } finally {
    loading.value = false;
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
  transition: opacity 0.3s ease;
}

/* Fade-in magic */
.content-fade {
  opacity: 0;
  transition: opacity 0.4s ease;
}

.content-fade.visible {
  opacity: 1;
}
</style>
