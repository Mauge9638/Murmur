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
        <ChatOutput class="row-span-4 min-h-0 p-2" />
        <ChatInput />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ChatInput from "../components/ChatInput.vue";
import ChatOutput from "../components/ChatOutput.vue";
import SidebarMenu from "../components/SidebarMenu.vue";
</script>

<style scoped></style>
