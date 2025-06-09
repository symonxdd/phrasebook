import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAppStore = defineStore('app', () => {
  const entries = ref([]);
  // const sentences = ref([]);
  // const categories = ref([]);
  return { entries };
});
