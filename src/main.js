import { createApp } from "vue";
import "./assets/tailwind.css";
import "./style.css";
import App from "./App.vue";
import { createPinia } from "pinia";

/**
 * Fontawesome
 * The “Library” is the way to subset or reduce file sizes and reference icons easily.  
 */
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { library } from "@fortawesome/fontawesome-svg-core";
import { faChevronLeft, faChevronRight, faRocket, faCopy, faSun, faMoon } from "@fortawesome/free-solid-svg-icons";
library.add(faChevronLeft, faChevronRight, faRocket, faCopy, faSun, faMoon);


createApp(App)
.component('font-awesome-icon', FontAwesomeIcon)
.use(createPinia() )
.mount("#app");
