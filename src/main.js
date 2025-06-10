import { createApp } from "vue";
import { createPinia } from 'pinia'
import App from "./App.vue";
import "./styles.css";
import { router } from "./router";
import "bootstrap-icons/font/bootstrap-icons.css";
import masonry from 'vue-next-masonry';
import { useThemeStore } from './stores/themeStore'

const pinia = createPinia()
const app = createApp(App);
app.use(router);
app.use(pinia);
app.use(masonry);
app.mount("#app");

// Apply theme after mount
const themeStore = useThemeStore();
themeStore.initTheme();