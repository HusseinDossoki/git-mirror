import { createApp } from "vue";
import "./assets/tailwind.css";
import "./assets/fontawesome-free-6.2.1-web/css/all.min.css";
import "./style.css";
import App from "./App.vue";
import { createPinia } from "pinia";

createApp(App)
.use(createPinia() )
.mount("#app");
