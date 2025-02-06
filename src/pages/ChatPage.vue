<template>
  <div class="grid h-full w-full grid-cols-8">
    <div class="col-span-1 p-2 pl-0">
      <div
        class="h-full rounded-lg rounded-l-none border-2 border-l-0 border-cyan-800 bg-gray-600/40 p-2"
      >
        <SidebarMenu />
      </div>
    </div>
    <div class="col-span-7 min-h-0 p-2">
      <div class="grid h-full grid-rows-5 rounded-lg bg-slate-900/80">
        <ChatOutput class="row-span-4 min-h-0 p-2" :messages="messages" />
        <ChatInput @enter-prompt="sendPrompt" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import ChatInput from "../components/ChatInput.vue";
import ChatOutput from "../components/ChatOutput.vue";
import SidebarMenu from "../components/SidebarMenu.vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { Message, MessageSender } from "../types/MessageTypes";

//const response = ref<string>("");
let unlisten: UnlistenFn | null = null;
const timestampForResponse = ref<number>(0);

const messages = ref<Message[]>([]);

onMounted(async () => {
  // Set up the event listener when component mounts
  unlisten = await listen("generate_response", (event) => {
    if (messages.value != null) {
      messages.value.find(
        (message) =>
          message.timestampInMs === timestampForResponse.value &&
          message.agent === MessageSender.Agent,
      )!.message += event.payload as string;
    }
  });
});

onUnmounted(() => {
  // Clean up the event listener when component unmounts
  if (unlisten) unlisten();
});

const sendPrompt = async (message: string) => {
  messages.value.push({
    agent: MessageSender.User,
    message: message,
    timestampInMs: new Date().getTime(),
  });
  timestampForResponse.value = new Date().getTime();
  messages.value.push({
    agent: MessageSender.Agent,
    message: "",
    timestampInMs: timestampForResponse.value,
  });
  console.log(messages.value);
  await invoke("generate", { message: message });
};
</script>
