import { createRouter, createWebHistory } from "vue-router";
import Dashboard from "./components/Dashboard.vue";
import AddItem from "./components/AddItem.vue";

const routes = [
  { path: "/", component: Dashboard },
  { path: "/add", component: AddItem }
];

export const router = createRouter({
  history: createWebHistory(),
  routes
});
