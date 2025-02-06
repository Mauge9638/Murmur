<template>
  <main class="flex h-screen w-screen overflow-hidden bg-slate-800 text-white">
    <div class="flex-1 overflow-hidden">
      <RouterView />
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>
