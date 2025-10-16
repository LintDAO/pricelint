<template>
  <div>
    <q-btn label="Get History" @click="getTokenHistoryData"></q-btn>
  </div>
</template>

<script setup lang="ts">
import { getTokenPriceHistory } from "@/api/token";

const getTokenHistoryData = async () => {
  const priceData = await getTokenPriceHistory();
  console.log("price", priceData);
};
</script>

<style lang="scss" scoped></style>
