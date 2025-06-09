<template>
  <div class="terms-view">
    <div class="top-bar">
      <div class="view-toggle">
        <button @click="viewMode = 'list'" :class="{ active: viewMode === 'list' }">
          <i class="bi bi-list-ul"></i>
        </button>
        <button @click="viewMode = 'masonry'" :class="{ active: viewMode === 'masonry' }">
          <i class="bi bi-grid-3x3-gap"></i>
        </button>
      </div>
    </div>

    <div v-if="viewMode === 'list'" class="terms-list">
      <template v-if="filteredTerms.length > 0">
        <TermItem v-for="term in filteredTerms" :key="term.term" :term="term" />
      </template>
      <div v-else class="no-results">No results found</div>
    </div>

    <masonry v-if="viewMode === 'masonry'" :cols="cols" :gutter="10">
      <template v-if="filteredTerms.length > 0">
        <TermItem v-for="term in filteredTerms" :key="term.term" :term="term" />
      </template>
      <div v-else class="no-results">No results found</div>
    </masonry>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRoute } from 'vue-router';
import TermItem from "./TermItem.vue";

const route = useRoute();
const terms = ref([]);
const viewMode = ref("list");
const cols = ref(getCols()); // Dynamic column count

const props = defineProps({
  searchResults: Array
});

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
  const { name, type } = route.query;
  if (name && type) {
    terms.value = await invoke("load_terms_by_" + type, { name });
  }

  window.addEventListener("resize", updateCols);

});

onUnmounted(() => {
  window.removeEventListener("resize", updateCols);
});

const filteredTerms = computed(() => {
  if (props.searchResults && props.searchResults.length > 0) {
    return props.searchResults;
  }

  const name = route.query.name;
  const type = route.query.type;

  console.log('name:', name, 'type:', type);

  return terms.value;
});
</script>

<style scoped>
.terms-view {
  width: 100%;
  max-width: 100vw;
  padding: 10px;
}

.top-bar {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 10px;
  /* margin-bottom: 10px; */
  overflow: hidden;
  position: relative;
  height: 33.75px;
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
  /* gap: 6px; */
}

.quick-filters {
  display: flex;
  gap: 6px;
  overflow-x: auto;
  white-space: nowrap;
  scrollbar-width: none;
  -ms-overflow-style: none;
  flex-grow: 1;
  user-select: none;
}

.quick-filters::-webkit-scrollbar {
  display: none;
}

.filter-pill {
  background: #222;
  border: 1px solid #333;
  padding: 5px 10px;
  border-radius: 50px;
  color: #bbb;
  transition: all 0.2s;
  font-size: 0.85rem;
}

.filter-pill:hover {
  background: #333;
  color: #fff;
}

.filter-pill.active {
  background: #FF3214;
  color: #fff;
  border-color: #FF3214;
}

.no-results {
  text-align: center;
  font-size: 1.2rem;
  color: gray;
  margin-top: 20px;
}
</style>