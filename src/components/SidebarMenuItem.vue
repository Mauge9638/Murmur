<template>
  <li class="p-2">
    <button
      type="button"
      class="flex w-full cursor-pointer flex-row items-center justify-start rounded-lg border-2 border-cyan-800 bg-slate-800/70 p-2 hover:border-cyan-500 hover:bg-slate-800/20"
      @click="props.onClick"
    >
      <span v-if="icon">
        <component
          :is="iconComponents[icon.name]"
          :class="`text-${icon.color} size-${icon.size} bg-${icon.bgColor}`"
        />
      </span>
      <span v-if="text" class="px-1">{{ props.text }}</span>
    </button>
  </li>
</template>

<script setup lang="ts">
import { BeakerIcon, ChatBubbleLeftRightIcon } from "@heroicons/vue/24/solid";

type IconName = "BeakerIcon" | "ChatBubbleLeftRightIcon";

const props = defineProps<{
  text?: string;
  icon?: { name: IconName; size?: number; color?: string; bgColor?: string };
  onClick: () => void;
}>();

const iconComponents = {
  BeakerIcon,
  ChatBubbleLeftRightIcon,
};
</script>

<style scoped></style>
