<template>
  <div class="canister-detail">
    <h2>Canister Detail</h2>
    <p>Principal ID: {{ principalId }}</p>
    <p>Details for this canister will be displayed here.</p>
    <q-btn label="Back" color="negative" @click="goBack" />
  </div>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";
import { ref } from "vue";

// 获取路由实例和当前路由
const route = useRoute();
const router = useRouter();

// 从路由参数中获取 principalId
const principalId = ref(route.params.principalId as string);

// 返回上一页
const goBack = () => {
  router.back();
};
</script>

<style lang="scss" scoped></style>
