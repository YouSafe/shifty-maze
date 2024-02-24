import "./assets/main.css";

import { createApp } from "vue";
import { Notification } from "./notification";
import App from "./App.vue";
createApp(App).mount("#app");

globalThis.addEventListener("unhandledrejection", (event) => {
  Notification.error({
    title: "Unhandled Promise Rejection",
    content: event.reason,
  });
  console.error(event);
});
globalThis.addEventListener("error", (event) => {
  Notification.error({
    title: "Unhandled Error",
    content: event.message,
  });
  console.error(event);
});
