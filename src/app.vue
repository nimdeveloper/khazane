<template>
    <div
        class="bg-primary-bg print:bg-white print:text-active-item-bg h-full text-text-primary font-vazir"
    >
        <NuxtRouteAnnouncer />
        <NuxtLayout>
            <NuxtPage />
        </NuxtLayout>
    </div>
</template>

<script setup lang="ts">
import "simplebar-vue/dist/simplebar.min.css";
</script>

<style lang="scss">
html {
    background-color: var(--color-primary-bg);
}

body {
    padding: 10px;
    height: 100dvh;
    box-sizing: border-box;
    @media print {
        padding: 0;
    }
}
#app,
#__nuxt {
    height: 100%;
}
* {
    direction: rtl;
}
</style>
