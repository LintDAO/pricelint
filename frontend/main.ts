import { Dialog, Notify, Quasar } from "quasar";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

import "@/assets/css/main.scss";
// Import icon libraries
import "@quasar/extras/bootstrap-icons/bootstrap-icons.css";
import "@quasar/extras/material-icons/material-icons.css";
// Import Quasar css
import "quasar/src/css/index.sass";

import { createPinia } from "pinia"; //类似于vuex的状态管理

const app = createApp(App);
const pinia = createPinia();

app.use(router).use(pinia).use(Quasar, {
  plugins: { Notify, Dialog }, // import Quasar plugins and add here
});

app.mount("#app");
