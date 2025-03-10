<template>
  <div class="term-card">
    <h3>{{ term.term }}</h3>
    <p>{{ term.meaning }}</p>
    <small v-if="term.extra">{{ term.extra }}</small>
  </div>
</template>

<script setup>
defineProps(["term"]);
</script>

<style scoped>
.term-card {
  background: #1e1e1e;
  padding: 15px;
  border-radius: 5px;
  margin-bottom: 10px;
  transition: background 0.2s;
}

.term-card:hover {
  background: #292929;
}
</style>
