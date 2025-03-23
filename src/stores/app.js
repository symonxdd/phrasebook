import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAppStore = defineStore('app', () => {
  const terms = ref([]);
  const categories = ref([]);
  const groups = ref([]);
  return { terms, categories, groups };
});
