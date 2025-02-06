<template>
  <div class="flex items-start justify-start p-3 align-top">
    <textarea
      class="h-full w-full rounded-lg border-2 border-cyan-500 bg-slate-800/70 p-2 text-white"
      type="text"
      v-model="prompt"
      placeholder="Press Enter to send a message and Shift+Enter for a new line"
      @keydown.enter.exact.prevent="prompt = ''"
      @keydown.shift.enter.prevent="prompt += '\n'"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const prompt = ref<string>("");
</script>

<style scoped>
textarea {
  resize: none;
}
</style>
