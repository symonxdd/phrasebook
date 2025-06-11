<template>
  <nav class="sidebar">
    <TitleLogo />

    <ul class="nav-list">
      <!-- <li class="nav-item" :class="{ active: $route.path === '/' }" @click="$router.push('/')">
        <i class="bi bi-house-door nav-icon"></i>
        <span class="nav-text">Home</span>
      </li> -->

      <li class="nav-item" :class="{ active: $route.path === '/explore' }" @click="$router.push('/explore')">
        <i class="bi bi-compass nav-icon"></i>
        <span class="nav-text">Explore</span>
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
  width: 254px;
  min-width: 254px;
  background-color: var(--bg-color);
  height: 100vh;
  display: flex;
  flex-direction: column;
  padding: 10px 12px;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.05);
  transition: background-color 0.3s ease;
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
  padding: 12px 20px;
  color: var(--nav-item-color);
  font-size: 0.95rem;
  font-weight: 500;
  border-radius: 8px;
  transition: background-color 0.25s ease, color 0.25s ease;
  margin-bottom: 8px;
  cursor: pointer;
}

.nav-item:hover {
  background-color: var(--nav-item-hover-bg);
  color: var(--nav-item-hover-color);
}

.nav-item:active {
  transform: scale(0.98);
  transition: transform 0.1s ease;
}

.nav-item.active {
  /* background-color: var(--nav-item-active-bg); */
  color: var(--nav-item-active-color);
  font-weight: 600;
}

.nav-item::before {
  content: "";
  position: absolute;
  left: 6px;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 60%;
  background-color: var(--nav-indicator-color);
  border-radius: 6px;
  opacity: 0;
  transition: width 0.3s ease, opacity 0.3s ease;
}

.nav-item.active::before {
  opacity: 1;
}

.nav-icon {
  font-size: 1.1rem;
  margin-right: 10px;
}

.nav-text {
  flex-grow: 1;
}

.nav-separator {
  height: 1px;
  background: var(--border-color);
  opacity: 0.3;
  margin: 10px 0;
  width: 100%;
}
</style>
