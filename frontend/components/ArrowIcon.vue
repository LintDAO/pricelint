<!-- ArrowIcon.vue -->
<template>
  <q-icon :class="iconClass" :size="size" :name="iconName" />
</template>

<script setup lang="ts">
import { computed } from "vue";

// 定义 props 类型，根据传入Up还是Down字符来显示对应的箭头
interface Props {
  direction: "Up" | "Down";
  size?: "xs" | "sm" | "md" | "lg" | "xl" | string; // 支持 Quasar 预定义大小或自定义值
}

const props = defineProps<Props>();

// 根据 direction 计算图标名称
const iconName = computed(() => {
  return props.direction === "Up" ? "arrow_upward" : "arrow_downward";
});

// 根据 direction 计算图标颜色类
const iconClass = computed(() => {
  return props.direction === "Up" ? "text-red-5" : "text-green-6";
});
</script>
