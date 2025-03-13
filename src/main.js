import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";
import { router } from "./router";
import "bootstrap-icons/font/bootstrap-icons.css";
import masonry from 'vue-next-masonry';

const app = createApp(App);
app.use(router);
app.use(masonry);
app.mount("#app");
