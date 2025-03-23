import { createApp } from "vue";
import { createPinia } from 'pinia'
import App from "./App.vue";
import "./styles.css";
import { router } from "./router";
import "bootstrap-icons/font/bootstrap-icons.css";
import masonry from 'vue-next-masonry';

const pinia = createPinia()
const app = createApp(App);
app.use(router);
app.use(masonry);
app.use(pinia);
app.mount("#app");
