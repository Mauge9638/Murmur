<template>
  <div>
    <div class="markdown-body" v-html="md.render(props.markdown)" />
  </div>
</template>

<script setup lang="ts">
import MarkdownIt from "markdown-it";

const props = defineProps<{
  markdown: string;
}>();

const md = new MarkdownIt();
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
