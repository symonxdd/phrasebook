<template>
  <div class="dashboard">
    <div class="title title-categories">Categories</div>
    <div class="pill-container">
      <div v-for="category in categories" :key="category.name" class="pill category-pill"
        @click="navigateToTerms(category.name, 'category')">
        {{ category.name }}
      </div>
    </div>

    <div class="title title-groups">Groups</div>
    <div class="pill-container">
      <div v-for="group in groups" :key="group.name" class="pill group-pill"
        @click="navigateToTerms(group.name, 'group')">
        {{ group.name }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from 'vue-router';
import { useFilterStore } from '../stores/filters';

const filters = useFilterStore();
const categories = ref([]);
const groups = ref([]);
const router = useRouter();

onMounted(async () => {
  categories.value = await invoke("load_categories");
  groups.value = await invoke("load_groups");
  console.log("categories: ", categories.value);
  console.log("groups:", groups.value);
});

function navigateToTerms(name, type) {
  router.push({ name: 'terms-view', query: { name, type } });
}
</script>

<style scoped>
.dashboard {
  width: 100%;
  max-width: 100vw;
  padding: 10px;
}

.title {
  text-align: center;
  font-size: 1.75rem;
  margin-bottom: 21px;
  color: #979797;
}

.title-categories {
  margin-top: 50px;
}

.title-groups {
  margin-top: 80px;
}

.pill-container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 10px;
  /* margin-bottom: 50px; */
}

.pill {
  background-color: #1f1f1f;
  border: 1px solid #1f1f1f;
  color: #a5a5a5;
  border-radius: 21px;
  transition: 0.2s;
}

.pill:hover {
  border: 1px solid #4b4b4b;
}

.category-pill {
  padding: 8px 12px;
}

.group-pill {
  padding: 8px 12px;
}
</style>
