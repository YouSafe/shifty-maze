import "./assets/main.css";

import { createApp } from "vue";
import App from "./App.vue";
createApp(App).mount("#app");
/*
globalThis.window.addEventListener("beforeunload", (e) => {
  const message = "Are you sure you want to leave?";
  e.preventDefault();
  (e as any).returnValue = message;
  return message;
});
*/
