import { createRouter, createWebHistory } from "vue-router";
import Dashboard from "./components/Dashboard.vue";
import AddItem from "./components/AddItem.vue";
import Settings from "./components/Settings.vue";

const routes = [
  { path: "/", component: Dashboard },
  { path: "/add", component: AddItem },
  { path: "/settings", component: Settings }
];

export const router = createRouter({
  history: createWebHistory(),
  routes
});
