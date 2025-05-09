<template>
  <div class="comment-container bg-grey-2 q-py-xl">
    <div class="container">
      <div class="text-h4 text-weight-medium q-my-lg">
        <span class="text-blue-10">Users</span> have said
      </div>
      <div class="row items-start q-col-gutter-md">
        <div class="col-12 col-md-4" v-for="comment in comments">
          <q-card class="comment-card flat-card">
            <q-card-section>
              <div class="row items-center from">
                <q-icon name="star" color="positive" class="green-star" />
                <div class="text-subtitle1">
                  {{ comment.from }}
                </div>
              </div>
              <div class="text-weight-medium text-grey-9 description">
                {{ comment.description }}
              </div>
              <div class="row justify-between items-center author">
                <q-icon name="thumb_up" color="positive" size="xs" />
                <div class="name">{{ comment.author }}</div>
              </div>
            </q-card-section>
          </q-card>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
const comments = ref([
  {
    author: "Adam K.",
    from: "X",
    description: `TallyLint is going to save me at least 10 hours on filing my taxes this year.
    You can sort by ICP wallet and get your cost basis in USD using FIFO as required by the IRS.`,
  },
  {
    author: "wpb",
    from: "Developer Forum",
    description: `This kind of app is sorely needed for ICP.`,
  },
  {
    author: "Jerry",
    from: "Builder",
    description: `Holding multiple wallets makes it difficult for me to manage all my possessions at once, 
    and using TallyLint makes it much easier.`,
  },
])
</script>

<style lang="scss" scoped>
.comment-container {
  padding: 100px 0px;
  .comment-card {
    .q-card__section {
      padding: 25px 40px;
      box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
    }
    .from {
      .text-subtitle1 {
        font-weight: bold;
      }
    }
    .description {
      margin-top: 10px;
      margin-bottom: 10px;
    }
    .author {
      font-weight: bold;
    }
  }
}
</style>
