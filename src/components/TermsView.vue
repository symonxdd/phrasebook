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
// import { useFilterStore } from '../stores/filters';

// const filters = useFilterStore();
const route = useRoute();
const terms = ref([]);
const viewMode = ref("list");
const cols = ref(getCols()); // Dynamic column count

const props = defineProps({
  searchResults: Array
});

let dragMoved = false; // Track if movement occurs

const quickFilters = ref(null); // Reference for quick-filters
// let isDragging = false;
// let startX, scrollLeft;

// function handleFilterClick(title) {
//   if (!dragMoved) {
//     filters.toggleGroup(title);
//   }
// }

// Drag-to-scroll logic for quick-filters
// function startDrag(e) {
//   isDragging = true;
//   dragMoved = false; // Reset drag movement flag
//   startX = e.pageX - quickFilters.value.offsetLeft;
//   scrollLeft = quickFilters.value.scrollLeft;
// }

// function onDrag(e) {
//   if (!isDragging) return;
//   e.preventDefault();
//   dragMoved = true; // Dragging has started
//   const x = e.pageX - quickFilters.value.offsetLeft;
//   const walk = (x - startX) * 1; // scroll speed
//   quickFilters.value.scrollLeft = scrollLeft - walk;
// }

// function stopDrag() {
//   isDragging = false;
// }

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

  // if (quickFilters.value) {
  // quickFilters.value.addEventListener("mousedown", startDrag);
  // window.addEventListener("mousemove", onDrag); // Listen on window
  // window.addEventListener("mouseup", stopDrag); // Listen on window
  // }
});

onUnmounted(() => {
  window.removeEventListener("resize", updateCols);

  // if (quickFilters.value) {
  // quickFilters.value.removeEventListener("mousedown", startDrag);
  // }
  // window.removeEventListener("mousemove", onDrag); // Remove from window
  // window.removeEventListener("mouseup", stopDrag); // Remove from window
});

const filteredTerms = computed(() => {
  // If searchResults exist, return them directly (override filtering)
  if (props.searchResults && props.searchResults.length > 0) {
    return props.searchResults;
  }

  // Otherwise, apply category/group filtering
  const name = route.query.name;
  const type = route.query.type;

  console.log('name:', name, 'type:', type);

  return terms.value; // Fallback to category/group terms

  // return terms.value.filter(term => {
  //   const matchesSearch = term.term.toLowerCase().includes(filters.searchQuery.toLowerCase());

  //   const matchesType = type === 'category'
  //     ? term.category === name
  //     : term.group === name;

  //   return matchesSearch && matchesType;
  // });
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