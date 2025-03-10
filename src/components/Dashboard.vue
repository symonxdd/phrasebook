<template>
  <div class="dashboard">
    <input v-model="searchQuery" class="search-bar" placeholder="Search terms..." />
    <div class="terms-list">
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

onMounted(async () => {
  terms.value = await invoke("load_terms");
});

const filteredTerms = computed(() =>
  terms.value.filter(term => term.term.toLowerCase().includes(searchQuery.value.toLowerCase()))
);
</script>

<style scoped>
/* Add styling for the floating add button */
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
}
</style>
