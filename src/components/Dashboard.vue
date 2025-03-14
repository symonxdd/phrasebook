<template>
  <div class="dashboard">
    <div class="top-bar">
      <input v-model="searchQuery" class="search-bar" placeholder="Search terms..." />
      <div class="view-toggle">
        <button @click="viewMode = 'masonry'" :class="{ active: viewMode === 'masonry' }">
          <i class="bi bi-grid-3x3-gap"></i>
        </button>
        <button @click="viewMode = 'list'" :class="{ active: viewMode === 'list' }">
          <i class="bi bi-list-ul"></i>
        </button>
      </div>
    </div>

    <masonry v-if="viewMode === 'masonry'" :cols="cols" :gutter="10">
      <TermItem v-for="term in filteredTerms" :key="term.term" :term="term" />
    </masonry>

    <div v-if="viewMode === 'list'" class="terms-list">
      <TermItem v-for="term in filteredTerms" :key="term.term" :term="term" />
    </div>

    <button @click="$router.push('/add')" class="add-button">
      <i class="bi bi-plus-lg"></i>
    </button>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TermItem from "./TermItem.vue";

const terms = ref([]);
const searchQuery = ref("");
const viewMode = ref("masonry");
const cols = ref(getCols()); // Dynamic column count

function getCols() {
  if (window.innerWidth > 2000) return 6;
  if (window.innerWidth > 1600) return 5;
  if (window.innerWidth > 1200) return 4;
  if (window.innerWidth > 800) return 3;
  if (window.innerWidth > 600) return 2;
  return 1;
}

function updateCols() {
  cols.value = getCols();
}

onMounted(async () => {
  terms.value = await invoke("load_terms");
  window.addEventListener("resize", updateCols);
});

onUnmounted(() => {
  window.removeEventListener("resize", updateCols);
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
}

.view-toggle button {
  color: #777;
  font-size: 1.2rem;
  cursor: default;
  transition: color 0.2s, transform 0.1s;
  padding: 0 10px;
  background: transparent;
}

.view-toggle button:hover {
  color: #bbb;
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