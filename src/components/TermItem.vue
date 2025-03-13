<template>
  <div class="term-card">
    <p class="term-line">
      <span class="term">{{ term.term }}</span>:
      <span class="meaning">{{ term.meaning }}</span>
    </p>
    <small class="extra" v-if="term.extra">{{ term.extra }}</small>
  </div>
</template>

<script setup>
defineProps(["term"]);
</script>

<style scoped>
.term-card {
  background-color: #1a1a1a;
  padding: 12px;
  border-radius: 6px;
  border: 1px solid #2a2a2a;
  transition: background 0.2s, border 0.2s;
  color: #ddd;
  margin-top: 10px;
}

.term-card:hover {
  background-color: #222;
  border-color: #444;
}

.term-line {
  font-weight: normal;
  line-height: 1.3;
}

.term {
  font-size: 0.85rem;
  font-weight: bold;
  color: #f1f1f1;
}

.meaning {
  font-size: 0.85rem;
  color: #bbb;
}

.extra {
  display: block;
  color: #888;
  font-size: 0.80rem;
  margin-top: 4px;
}
</style>