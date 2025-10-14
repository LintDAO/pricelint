<template>
  <div class="q-mt-md q-pr-md" style="display: flex; flex-direction: column">
    <q-btn
      v-for="btn in buttons"
      :key="btn.label"
      :label="btn.label"
      :to="getButtonLink(btn)"
      :icon="btn.icon"
      flat
      no-caps
      dense
      align="left"
      class="text-grey-8 rounded-borders q-px-md q-py-sm"
      :ripple="false"
      :class="{
        'bg-grey-3': isActive(btn),
        'text-grey-10': isActive(btn),
      }"
      unelevated
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";

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
const router = useRouter();

// 使用路由 meta 或 props 或默认按钮
const buttons = computed(() => {
  return props.buttons ?? (route.meta.sidebar as Button[] | undefined);
});

// 计算按钮的实际链接
const getButtonLink = (btn: Button) => {
  return btn.to.startsWith("/")
    ? btn.to
    : `${route.path.substring(0, route.path.lastIndexOf("/") + 1)}${btn.to}`;
};

// 判断按钮是否为当前路由
const isActive = (btn: Button) => {
  const link = getButtonLink(btn);
  // 精确匹配完整路径
  const resolvedPath = router.resolve(link).path;
  // 支持子路径匹配（例如 /table 匹配 /canisters/:canisterId/insights/table）
  const linkSegment = link.split("/").pop() || "";
  const currentSegment = route.path.split("/").pop() || "";
  return (
    route.path === resolvedPath ||
    (linkSegment && currentSegment === linkSegment) ||
    route.matched.some((r) => r.path === resolvedPath)
  );
};
</script>
