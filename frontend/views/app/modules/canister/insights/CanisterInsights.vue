<template>
  <q-page class="row" style="gap: 4rem">
    <div class="col-auto" style="width: 200px">
      <SideBar />
    </div>
    <div class="col">
      <div class="q-pa-md">
        <!-- 子路由内容将在这里渲染 -->
        <router-view />
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import SideBar from "@/components/sidebar/Sidebar.vue";
</script>

<style lang="scss" scoped></style>
