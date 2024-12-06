import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";

import "./tailwind.css";

const app = createApp(App);

// Load user state from localStorage when the app starts
store.dispatch("loadUserState");
store.dispatch("appStart");
app.use(store);
app.use(router);

app.mount("#app");
