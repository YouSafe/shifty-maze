import { createDiscreteApi } from "naive-ui";

const { message, notification, dialog, loadingBar } = createDiscreteApi([
  "message",
  "dialog",
  "notification",
  "loadingBar",
]);

export {
  message as Message,
  notification as Notification,
  dialog as Dialog,
  loadingBar as LoadingBar,
};
