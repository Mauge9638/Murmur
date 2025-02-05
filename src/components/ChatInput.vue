<template>
  <div class="flex items-start justify-start align-top">
    <textarea
      class="h-full w-full bg-slate-800 text-white"
      type="text"
      v-model="prompt"
      placeholder="..."
      @keydown.enter.meta.prevent="prompt = ''"
    />
    <button
      class="absolute right-10 bottom-10 rounded-xl border-2 border-teal-700 bg-cyan-800 p-2 text-white"
    >
      Submit
    </button>
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
