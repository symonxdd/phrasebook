<template>
  <nav class="sidebar">
    <TitleLogo />

    <input v-model="filters.searchQuery" class="sidebar-search-bar" placeholder="Search terms..." />

    <ul class="nav-list">
      <li class="nav-item" :class="{ active: $route.path === '/' }" @click="$router.push('/')">
        <i class="bi bi-house-door nav-icon"></i>
        <span class="nav-text">Home</span>
      </li>

      <li class="nav-item" :class="{ active: $route.path === '/favorites' }" @click="$router.push('/favorites')">
        <i class="bi bi-star nav-icon"></i>
        <span class="nav-text">Favorites</span>
      </li>

      <!-- <div class="nav-separator"></div> -->
    </ul>

    <div class="nav-bottom">
      <li class="nav-item" :class="{ active: $route.path === '/settings' }" @click="$router.push('/settings')">
        <i class="bi bi-gear nav-icon"></i>
        <span class="nav-text">Settings</span>
      </li>
    </div>
  </nav>
</template>

<script setup>
import TitleLogo from './TitleLogo.vue';
import { useFilterStore } from '../stores/filters'
const filters = useFilterStore()
</script>

<style scoped>
.sidebar {
  width: 250px;
  min-width: 250px;
  background-color: #121212;
  height: 100vh;
  display: flex;
  flex-direction: column;
  padding: 10px;
}

.nav-list {
  flex-grow: 1;
  list-style: none;
  padding: 0;
  margin-top: 20px;
}

.nav-bottom {
  margin-top: auto;
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  padding: 10px 24px;
  color: #ddd;
  font-size: 0.9rem;
  border-radius: 6px;
  transition: background 0.2s, color 0.2s;
  margin-bottom: 8px;
}

.nav-item:hover {
  background: #2a2a2a;
  color: white;
}

.nav-item:active {
  background: #333;
}

.nav-item.active {
  color: #fff;
  font-weight: bold;
  background: transparent;
}

.nav-item::before {
  content: "";
  position: absolute;
  left: 6px;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  /* Start with 0 width */
  height: 60%;
  background-color: #FF3214;
  border-radius: 6px;
  opacity: 0;
  /* Start with 0 opacity */
  transition: width 0.3s ease, opacity 0.3s ease;
  /* Smooth transition */
}

.nav-item.active::before {
  opacity: 1;
  /* End with full opacity */
}

.nav-icon {
  font-size: 1rem;
  margin-right: 8px;
}

.nav-text {
  flex-grow: 1;
}

.nav-separator {
  height: 1px;
  background: rgba(255, 255, 255, 0.1);
  margin: 10px 0;
  width: 100%;
}

.sidebar-search-bar {
  width: 100%;
  margin: 20px 0 10px 0;
  padding: 8px 10px;
  border-radius: 4px;
  border: none;
  background: #222;
  color: #ddd;
  outline: none;
}

.sidebar-search-bar:focus {
  background: #2b2b2b;
}
</style>
