<template>
  <div class="feature-container bg-grey-2">
    <div class="container q-py-md">
      <div class="text-h4 text-weight-medium q-my-lg">
        What does <span class="text-blue-10">TallyLint</span> offer?
      </div>
      <div class="featrue-card row justify-around q-col-gutter-md q-mb-lg">
        <div class="col-6 col-md-3" v-for="(card, index) in cards" :key="index">
          <q-card class="flat-card">
            <q-card-section>
              <q-btn
                fab
                :color="card.color"
                :icon="card.icon"
                class="q-mr-xs"
              />
              <div class="text-h6 q-my-sm">{{ card.title }}</div>
              <div class="text-subtitle2">{{ card.subtitle }}</div>
            </q-card-section>
          </q-card>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
const cards = ref([
  {
    icon: "drafts",
    color: "primary",
    title: "Dashboard",
    subtitle: "Show multiple token variations of your current holdings",
  },
  {
    icon: "wallet",
    color: "primary",
    title: "Wallets",
    subtitle: "Statistics of all wallets",
  },
  {
    icon: "swap_horiz",
    color: "primary",
    title: "Transactions",
    subtitle: "Filter and customize transaction history",
  },
  {
    icon: "receipt_long",
    color: "primary",
    title: "Profit/loss report",
    subtitle: "Profit/loss report for all your historical data",
  },
])
</script>

<style lang="scss" scoped>
.feature-container {
  padding-bottom: 150px;
  .container {
    .featrue-card {
      .q-card {
        // width: 230px;
        min-height: 180px;
      }
    }
  }
}
</style>
