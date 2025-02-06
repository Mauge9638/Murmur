<template>
  <div>
    <ul>
      <RouterLink to="/">
        <SidebarMenuItem
          text="Chat"
          :icon="{
            name: 'ChatBubbleLeftRightIcon',
            size: 6,
          }"
        />
      </RouterLink>
    </ul>
  </div>
</template>

<script setup lang="ts">
import SidebarMenuItem from "./SidebarMenuItem.vue";
import { RouterLink } from "vue-router";
</script>

<style scoped></style>
