<template>
    <div class="error-404-container">
        <div class="tip">Sorry, the page you are visiting does not exist</div>
        <button @click="goBack(router)">Back</button>
    </div>
</template>

<script lang="ts" setup>
    import {goBack} from "@/router/routers";
    import {useRouter} from "vue-router";

    const router = useRouter();

</script>

<style lang="scss">
    .error-404-container {
        text-align: center;
    }
</style>
