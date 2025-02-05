<template>
  <div class="h-full overflow-y-auto">
    <div class="markdown-body" v-html="md.render(realTimeMarkdown)" />
  </div>
</template>

<script setup lang="ts">
import MarkdownIt from "markdown-it";
import { onMounted, ref } from "vue";

const md = new MarkdownIt();

const realTimeMarkdown = ref<string>("");

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const makeRealTimeMarkdown = async () => {
  for (const entry of testMarkdown.split("")) {
    realTimeMarkdown.value += entry;
    await delay(1);
  }
};

onMounted(() => {
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
