import { createRouter, createWebHistory } from "vue-router";
import Dashboard from "./components/Dashboard.vue";
import AddItem from "./components/AddItem.vue";
import Settings from "./views/Settings.vue";
import TermsView from "./components/TermsView.vue";
import ImportTerms from "./components/ImportTerms.vue";
import Favorites from "./components/Favorites.vue";
import Explore from "./views/Explore.vue";
import AddEntry from "./components/AddEntry.vue";

const routes = [
  { path: '/', redirect: '/explore' },

  // { path: "/", component: Dashboard },
  { path: "/add", component: AddEntry },
  { path: '/import', component: ImportTerms },
  { path: "/settings", component: Settings },
  { path: '/favorites', component: Favorites },
  { path: '/explore', component: Explore },
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
