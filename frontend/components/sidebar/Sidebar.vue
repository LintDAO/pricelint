<template>
  <div class="q-ml-md" style="display: flex; flex-direction: column">
    <q-btn
      v-for="btn in buttons"
      :key="btn.label"
      :label="btn.label"
      :to="btn.to"
      :icon="btn.icon"
      flat
      no-caps
      dense
      align="left"
      class="text-grey-8 rounded-borders q-mb-sm q-px-sm q-py-xs"
      :ripple="false"
      :class="{
        'bg-grey-3': $route.path === btn.to,
        'text-grey-10': $route.path === btn.to,
      }"
      unelevated
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

// 定义按钮接口
interface Button {
  label: string;
  to: string;
  icon?: string;
}

// 定义 props
const props = defineProps<{
  buttons?: Button[];
}>();

// 获取当前路由
const route = useRoute();

// 使用路由 meta 或 props 或默认按钮
const buttons = computed(() => {
  return props.buttons ?? (route.meta.sidebar as Button[] | undefined);
});
</script>
