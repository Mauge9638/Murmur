import { createRouter, createWebHistory } from "vue-router";
import ChatPage from "@/pages/ChatPage.vue";

export default createRouter({
  history: createWebHistory("/"),
  routes: [
    {
      path: "/",
      name: "Chat",
      component: ChatPage,
    },
  ],
});
