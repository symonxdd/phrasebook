<template>
  <div class="dashboard">
    <div class="top-bar">
      <input v-model="searchQuery" class="search-bar" placeholder="Search terms..." />
      <div class="view-toggle">
        <button @click="viewMode = 'list'" :class="{ active: viewMode === 'list' }">
          <i class="bi bi-list-ul"></i>
        </button>
        <button @click="viewMode = 'grid'" :class="{ active: viewMode === 'grid' }">
          <i class="bi bi-grid-3x3-gap"></i>
        </button>
      </div>
    </div>

    <div :class="viewMode === 'list' ? 'terms-list' : 'terms-grid'">
      <TermItem v-for="term in filteredTerms" :key="term.term" :term="term" />
    </div>

    <button @click="$router.push('/add')" class="add-button">
      <i class="bi bi-plus-lg"></i>
    </button>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TermItem from "./TermItem.vue";

const terms = ref([]);
const searchQuery = ref("");
const viewMode = ref("grid"); // Default to list view

onMounted(async () => {
  terms.value = await invoke("load_terms");
});

const filteredTerms = computed(() =>
  terms.value.filter(term => term.term.toLowerCase().includes(searchQuery.value.toLowerCase()))
);
</script>

<style scoped>
.dashboard {
  width: 100%;
  max-width: 100vw;
  padding: 10px;
}

.top-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.search-bar {
  flex-grow: 1;
  padding: 10px;
  font-size: 1rem;
  border: none;
  border-radius: 4px;
  background: #222;
  color: #ddd;
  outline: none;
}

.view-toggle {
  display: flex;
  gap: 5px;
}

.view-toggle button {
  background: transparent;
  border: none;
  color: #777;
  font-size: 1.2rem;
  cursor: pointer;
  transition: color 0.2s, transform 0.1s;
}

.view-toggle button:hover {
  color: #bbb;
  /* transform: scale(1.1); */
}

.view-toggle button:active {
  transform: scale(0.95);
}

.view-toggle button.active {
  color: white;
}

/* List View */
.terms-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

/* Grid View */
.terms-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 10px;
  align-items: start;
  /* This ensures that each card only spans the height it needs */
}

.add-button {
  position: fixed;
  bottom: 20px;
  right: 20px;
  background: #6a11cb;
  color: white;
  border-radius: 50%;
  width: 50px;
  height: 50px;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 24px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
  border: none;
  cursor: pointer;
}

.add-button:hover {
  background: #570fc5;
}
</style>