import { createRouter, createWebHistory } from "vue-router";
import Dashboard from "./components/Dashboard.vue";
import AddItem from "./components/AddItem.vue";
import Settings from "./components/Settings.vue";
import TermsView from "./components/TermsView.vue";
import ImportTerms from "./components/ImportTerms.vue";
import Favorites from "./components/Favorites.vue";

const routes = [
  { path: "/", component: Dashboard },
  { path: "/add", component: AddItem },
  { path: "/settings", component: Settings },
  { path: '/import', component: ImportTerms },
  { path: '/favorites', component: Favorites },
  {
    path: '/terms',
    name: 'terms-view',
    component: TermsView
  }
];

export const router = createRouter({
  history: createWebHistory(),
  routes
});
