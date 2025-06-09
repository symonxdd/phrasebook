<template>
  <div class="dashboard">

    <div class="search-bar-wrapper">
      <input class="search-bar" placeholder="Search..." v-model="searchQuery" @input="performSearch" />
      <button v-if="searchQuery" class="clear-search-btn" @click="clearSearch" aria-label="Clear search">
        <i class="bi bi-x"></i>
      </button>
    </div>

    <TermsView v-if="isSearching" :searchResults="searchResults" />

    <template v-if="!isSearching">
      <!-- Categories Section -->
      <div v-if="categories.length" class="section">
        <div class="section-content">
          <h2 class="section-title">Categories</h2>
          <div class="cards-container">
            <div v-for="category in categories" :key="'category-' + category.name" class="card category-card"
              @click="navigateToTerms(category.name, 'category')">
              <div class="card-title">{{ category.name }}</div>
              <ul class="card-content">
                <li class="card-item" v-for="term in category.terms_json" :key="term">{{ term }}</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <!-- Groups Section -->
      <div v-if="groups.length" class="section">
        <div class="section-content">
          <h2 class="section-title">Groups</h2>
          <div class="cards-container">
            <div v-for="group in groups" :key="'group-' + group.name" class="card group-card"
              @click="navigateToTerms(group.name, 'group')">
              <div class="card-title">{{ group.name }}</div>
              <ul class="card-content">
                <li class="card-item" v-for="term in group.terms_json" :key="term">{{ term }}</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <!-- Unsorted Section -->
      <div v-if="unsorted.length" class="section">
        <div class="section-content">
          <h2 class="section-title">Unsorted</h2>
          <div class="cards-container">
            <div class="card unsorted-card" @click="navigateToTerms('Unsorted', 'unsorted')">
              <ul class="card-content">
                <li class="card-item" v-for="item in unsorted" :key="item.id">{{ item.term }}</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

    </template>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import TermsView from "./TermsView.vue";


const categories = ref([]);
const groups = ref([]);
const searchQuery = ref("");
const searchResults = ref([]);
const unsorted = ref([]);

const router = useRouter();

const isSearching = computed(() => searchQuery.value.trim() !== "");

onMounted(async () => {
  // categories.value = await invoke("load_non_empty_categories");
  // groups.value = await invoke("load_non_empty_groups");
  // unsorted.value = await invoke("load_non_empty_unsorted");

  // console.log("categories:", categories.value);
  // console.log("groups:", groups.value);
  // console.log("unsorted:", unsorted.value);
});

async function navigateToTerms(name, type) {
  const routeParams = { name: "terms-view", query: { name, type } };

  try {
    const terms = await invoke("load_terms_by_" + type, { name });

    router.push(routeParams);
    searchResults.value = terms;
  } catch (error) {
    console.error("Failed to load terms:", error);
  }
}

async function performSearch() {
  if (!isSearching.value) {
    searchResults.value = [];
    return;
  }

  const results = await invoke("search_terms", { query: searchQuery.value });

  searchResults.value = results;

  console.log("Search results:", searchResults.value);
}

// Clear search input when the button is clicked
function clearSearch() {
  searchQuery.value = "";
  searchResults.value = []; // Optionally clear the search results as well
}
</script>

<style scoped>
.dashboard {
  width: 100%;
  max-width: 100vw;
  margin-top: 20px;
}

.cards-container {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
}

.card {
  min-width: 143px;
  background-color: #1A1A1A;
  border: 1px solid #2a2a2a;
  border-radius: 12px;
  padding: 12px 15px;
  transition: transform 0.2s, background-color 0.3s, border 0.3s;
}

.card:hover {
  background-color: #222;
  border-color: #444;
}

.card:active {
  transform: scale(0.98);
}

.card-title {
  font-size: 0.98rem;
  font-weight: 500;
  color: #adadad;
  margin-bottom: 0px;
  /* text-align: center; */
}

.card-item {
  color: #737373;
}

.card-content {
  list-style: none;
  padding: 0;
  color: #bbb;
  font-size: 0.90rem;
  line-height: normal;
}

.card-content li {
  padding: 5px 0;
}

.group-card {
  width: 200px;
}

.search-bar-wrapper {
  position: relative;
  width: 350px;
  margin: 0 auto 0;
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

.search-bar {
  height: 40px;
  display: block;
  border-radius: 6px;
  border: 1px solid transparent;
  background-color: #222;
  color: #ddd;
  outline: none;
  font-size: 1rem;
  padding: 8px 12px;
  transition: border 0.2s ease-in-out;
}

.search-bar:focus {
  border: 1px solid #555;
  background-color: #2b2b2b;
}

/* Clear Search Button (inside the search bar) */
.clear-search-btn {
  position: absolute;
  right: 12px;
  /* Position the button 12px from the right */
  background: transparent;
  border: none;
  color: #ddd;
  font-size: 1.2rem;
  cursor: pointer;
  transition: color 0.2s ease-in-out;
  /* Smooth color change on hover */
  display: flex;
  align-items: center;
  /* Vertically center the icon */
}

.clear-search-btn:hover {
  color: #bbb;
  /* Lighter color when hovered */
}

.clear-search-btn:focus {
  outline: none;
  /* Remove outline on focus */
}

.section {
  width: 100%;
  margin-bottom: 20px;
  /* display: flex;
  flex-direction: column;
  align-items: center; */
  /* Centers the whole group */
}

.section-content {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  /* Aligns the title and cards together */
}

.section-title {
  font-weight: normal;
  font-size: 1.10rem;
  margin-bottom: 5px;
  text-align: left;
  padding-left: 10px;
  color: #adadad;
}
</style>
