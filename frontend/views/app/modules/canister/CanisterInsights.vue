<template>
  <q-page class="row">
    <q-col class="col-auto" style="width: 200px">
      <SideBar />
    </q-col>
    <q-col class="col">
      <div class="q-pa-md">
        <h2>Content</h2>
      </div>
    </q-col>
  </q-page>
</template>

<script setup lang="ts">
import SideBar from "@/components/sidebar/Sidebar.vue";
</script>

<style lang="scss" scoped></style>
