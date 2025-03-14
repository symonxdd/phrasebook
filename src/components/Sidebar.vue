<template>
  <nav class="sidebar">
    <TitleLogo />
    <ul class="nav-list">
      <template v-for="(item, index) in navItems" :key="item.path">
        <li class="nav-item" :class="{ active: $route.path === item.path }" @click="$router.push(item.path)">
          <i :class="item.icon" class="nav-icon"></i>
          <span class="nav-text">{{ item.label }}</span>
        </li>

        <div v-if="index === 1" class="nav-separator"></div>
      </template>
    </ul>
  </nav>
</template>

<script setup>
import TitleLogo from './TitleLogo.vue';

const navItems = [
  { path: '/', label: 'Home', icon: 'bi bi-house-door' },
  { path: '/favorites', label: 'Favorites', icon: 'bi bi-star' },
  { path: '/settings', label: 'Settings', icon: 'bi bi-gear' }
];
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
  list-style: none;
  padding: 0;
  margin-top: 20px;
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
</style>
