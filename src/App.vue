<template>
  <div class="app-layout">
    <Sidebar />
    <div class="app-content">
      <div class="content-fade" :class="{ visible: !loading }">
        <!-- <WelcomeScreen v-if="appStore.terms.length === 0" @add-first-item="addingFirstItem = true" /> -->

        <!-- <AddItem v-if="addingFirstItem" @cancel="addingFirstItem = false" @term-added="handleTermAdded" /> -->

        <router-view />
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
import { useLanguageStore } from './stores/languageStore'

const languageStore = useLanguageStore()
const appStore = useAppStore();
const addingFirstItem = ref(false);
const loading = ref(true);

async function handleTermAdded() {
  await loadTerms();
  addingFirstItem.value = false;
}

onMounted(async () => {
  await languageStore.loadLanguages();
  await loadTerms();
});

async function loadTerms() {
  loading.value = false;
}
</script>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
}

.app-content {
  flex-grow: 1;
  background-color: var(--bg-color);
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
