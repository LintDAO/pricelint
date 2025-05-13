<template>
  <div class="container footer-container q-mt-xl">
    <div class="header">
      <div class="text-h4">TallyLint</div>
    </div>
    <div class="row justify-between items-center q-mt-sm">
      <div class="footer-left col-auto">
        <nav class="footer-nav row items-center q-gutter-md">
          <router-link to="/" class="footer-link">Home</router-link>
        </nav>
      </div>
      <div class="footer-right col-auto">
        <a :href="DOCS_URL" class="footer-link" target="_Blank">Gitbook</a>
      </div>
    </div>
    <div class="footer-bottom row justify-between items-center q-mt-md">
      <div class="footer-bottom-left col-auto">
        <p>© TallyLint 2023-2024. All Rights Reserved.</p>
      </div>
      <div class="footer-bottom-right col-auto row items-center q-gutter-md">
        <q-chip icon="bookmark">give us more suggestions!</q-chip>
        <a :href="DISCORD_URL" target="_Blank">
          <q-icon class="cursor-pointer" size="sm" name="discord" />
        </a>
        <a :href="X_URL" target="_Blank">
          <q-icon class="cursor-pointer" size="sm" name="bi-twitter-x" />
        </a>
        <a :href="GITHUB_URL" target="_Blank">
          <q-icon class="cursor-pointer" size="sm" name="bi-github" />
        </a>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { DISCORD_URL, DOCS_URL, GITHUB_URL, X_URL } from "@/api/constants/docs"
</script>

<style lang="scss" scoped>
.footer-container {
  width: 100%;
}

.footer-link {
  color: rgb(107 114 128);
  text-decoration: none;
  &:hover {
    text-decoration: underline;
  }
}

.footer-bottom {
  width: 100%;
  border-top: 1px solid #e0e0e0;
  padding-top: 1rem;
  font-size: 0.875rem;
  color: #666;
  a {
    color: rgba(0, 0, 0, 0.6);
    &:hover {
      color: rgba(0, 0, 0, 0.8);
    }
  }
}
</style>
