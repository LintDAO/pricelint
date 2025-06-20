<template>
  <div>
    You don't have any canister.
    <q-btn color="primary" :loading="createLoading" @click="createNew()">
      Create New
    </q-btn>
  </div>
</template>

<script setup lang="ts">
import { createNewCanister } from "@/api/icp";
import { ref } from "vue";

const createLoading = ref(false);

const createNew = async () => {
  createLoading.value = true;
  await createNewCanister();
  createLoading.value = false;
};
</script>

<style lang="scss" scoped></style>
