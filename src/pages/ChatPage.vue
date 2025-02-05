<template>
  <div
    class="grid h-full w-full grid-cols-8 rounded-lg border-2 border-cyan-800"
  >
    <div class="col-span-1 border-r-2 border-cyan-800 p-2">
      Placeholder for menu
    </div>
    <div class="col-span-7 grid min-h-0 grid-rows-5 border-r-2 border-cyan-800">
      <ChatOutput class="row-span-4 min-h-0 border-b-2 border-cyan-800 p-2" />
      <ChatInput />
    </div>
  </div>
</template>

<script setup lang="ts">
import ChatInput from "../components/ChatInput.vue";
import ChatOutput from "../components/ChatOutput.vue";
</script>

<style scoped></style>
