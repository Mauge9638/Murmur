<template>
  <div class="flex h-full flex-col overflow-y-auto" ref="containerRef">
    <div
      v-for="message in messages"
      :key="message.agent + message.timestampInMs"
      :class="
        message.agent === 'User'
          ? 'justify-end self-end p-4 pl-40'
          : 'p-4 pr-40'
      "
    >
      <div
        v-if="message.agent === MessageSender.User"
        class="w-fit rounded-lg bg-cyan-700/50 p-4"
      >
        <UserIcon class="size-6" />
        <div>
          {{ message.message }}
        </div>
      </div>
      <div
        v-else-if="message.agent === MessageSender.Agent"
        class="self-start rounded-lg bg-slate-700/50 p-4"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          aria-hidden="true"
          data-slot="icon"
          fill="none"
          class="size-6"
        >
          <RobotIcon />
        </svg>
        <MarkdownRenderer :markdown="message.message" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import MarkdownRenderer from "./MarkdownRenderer.vue";
import RobotIcon from "./RobotIcon.vue";
import { UserIcon } from "@heroicons/vue/24/solid";
import { Message, MessageSender } from "../types/MessageTypes";

defineProps<{
  messages: Message[];
}>();

const realTimeMarkdown = ref<string>("");
const containerRef = ref<HTMLDivElement | null>(null);
const stopAutoScroll = ref(false);

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const makeRealTimeMarkdown = async () => {
  for (const entry of testMarkdown.split("")) {
    realTimeMarkdown.value += entry;
    await delay(10);
  }
};

watch(realTimeMarkdown, () => {
  if (containerRef.value && !stopAutoScroll.value) {
    containerRef.value.scrollTop = containerRef.value.scrollHeight;
  }
});

onMounted(() => {
  containerRef.value?.addEventListener("wheel", () => {
    stopAutoScroll.value = true;
    if (containerRef.value) {
      if (
        containerRef.value.scrollTop + containerRef.value.clientHeight ===
        containerRef.value.scrollHeight
      ) {
        stopAutoScroll.value = false;
      }
    }
  });
  makeRealTimeMarkdown();
});

const testMarkdown = `
# Main Documentation

## Installation
### Quick Start

Here's a **bold statement** about our project. It's *really* important to understand the basics.

Some ***bold and italic*** text for emphasis.

## Features

1. First feature
2. Second feature
   - Sub-feature A
   - Sub-feature B
3. Third feature

> Important note: This is a blockquote with some critical information.
> It can span multiple lines.

## Code Examples

Inline code: \`npm install package-name\`

\`\`\`javascript
function example() {
  console.log("This is a code block");
  return true;
}
\`\`\`

## Links and Tables

[Visit our website](https://example.com)

| Feature | Status | Notes |
|---------|--------|-------|
| Auth    | ✅     | Done  |
| API     | 🚧     | WIP   |

---

## Final Notes

- First point
  - Nested point
  - Another nested point
- Second point
- Third point

Text with \`inline code\` and _italic emphasis_.
`;
</script>

<!-- <style>
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
 -->
