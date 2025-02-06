<template>
  <div>
    <button
      type="button"
      ref="collapsible"
      class="cursor-pointer rounded-t-lg bg-slate-700/50 p-4 text-white"
      :class="collapsibleOpen && thinkPart ? 'rounded-t-lg' : 'rounded-lg'"
      @click="toggleCollapsible"
    >
      <span class="flex flex-row">
        See thought process
        <ChevronDownIcon v-if="!collapsibleOpen" class="size-6" />
        <ChevronUpIcon v-else class="size-6" />
      </span>
    </button>
    <div
      v-if="thinkPart"
      class="content rounded-tr-lg rounded-b-lg bg-slate-700/50 p-2 text-white transition-all duration-800 ease-in-out"
    >
      <!--  <p>{{ thinkPart }}</p> -->
      <div class="markdown-body" v-html="md.render(thinkPart)" />
    </div>
    <span v-if="thinkPart" class="p-4"></span>
    <div class="markdown-body" v-html="md.render(cleanedMarkdown)" />
  </div>
</template>

<script setup lang="ts">
import { ChevronDownIcon, ChevronUpIcon } from "@heroicons/vue/24/solid";
import MarkdownIt from "markdown-it";
import { computed, ref } from "vue";

const props = defineProps<{
  markdown: string;
}>();

const collapsible = ref<HTMLButtonElement | null>(null);
const collapsibleOpen = ref(false);

const toggleCollapsible = () => {
  collapsibleOpen.value = !collapsibleOpen.value;
  const button = collapsible.value;
  if (button) {
    button.classList.toggle("active");
    const content = button.nextElementSibling;
    if (content instanceof HTMLElement) {
      content.style.display =
        content.style.display === "block" ? "none" : "block";
      content.style.height = content.style.display === "block" ? "auto" : "0";
    }
  }
};

const thinkPart = computed(() => {
  const startToThink = props.markdown.split("</think>")[0];
  return startToThink.replace(/<think>/g, "").trim();
});

const cleanedMarkdown = computed(() => {
  const parts = props.markdown.split("</think>");
  return parts.length > 1 ? parts[1].trim() : "";
});

const md = new MarkdownIt();
</script>

<style scoped>
/* Style the collapsible content. Note: hidden by default */
.content {
  height: 0;
  display: none;
  overflow: hidden;
}
</style>

<style>
.markdown-body h1 {
  font-size: 2em;
  font-weight: bold;
  margin-bottom: 0.5em;
}

.markdown-body h2 {
  font-size: 1.5em;
  font-weight: bold;
  margin-bottom: 0.5em;
}

.markdown-body h3 {
  font-size: 1.25em;
  font-weight: bold;
  margin-bottom: 0.5em;
}

.markdown-body ul {
  list-style-type: disc;
  margin-left: 1.5em;
}

.markdown-body li {
  margin: 0.25em 0;
}

.markdown-body ol {
  list-style-type: decimal;
  margin-left: 1.5em;
}

.markdown-body {
  line-height: 1.6;
}

.markdown-body p {
  margin-bottom: 1em;
}

.markdown-body strong {
  font-weight: bold;
}

.markdown-body em {
  font-style: italic;
}

.markdown-body code {
  font-family: monospace;
  background-color: #1e293b;
  padding: 0.2em 0.4em;
  border-radius: 0.2em;
}

.markdown-body pre {
  background-color: #1e293b;
  padding: 1em;
  margin: 1em 0;
  border-radius: 0.5em;
  overflow-x: auto;
}

.markdown-body blockquote {
  border-left: 4px solid #475569;
  margin: 1em 0;
  padding-left: 1em;
  color: #94a3b8;
}

.markdown-body hr {
  border: 0;
  border-top: 1px solid #475569;
  margin: 2em 0;
}

.markdown-body a {
  color: #38bdf8;
  text-decoration: underline;
}

.markdown-body img {
  max-width: 100%;
  height: auto;
}

.markdown-body table {
  border-collapse: collapse;
  width: fit-content;
  margin: 1em 0;
  border: 1px solid #475569;
}

.markdown-body th,
.markdown-body td {
  border: 1px solid #475569;
  padding: 0.5em;
  text-align: left;
}

.markdown-body th {
  background-color: #1e293b;
  font-weight: bold;
}

.markdown-body tr:nth-child(even) {
  background-color: #1e293b;
}
</style>
