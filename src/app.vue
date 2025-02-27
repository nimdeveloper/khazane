<template>
    <div
        class="bg-glob-primary dark:bg-dark-glob-primary print:bg-white print:text-dark-action-secondary h-full text-primary dark:text-dark-primary font-vazir"
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
    @media (prefers-color-scheme: dark) {
        background-color: var(--color-dark-glob-primary);
    }
    @media (prefers-color-scheme: light) {
        background-color: var(--color-glob-primary);
    }
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
.simplebar-content-wrapper {
    outline: none !important;
}
</style>
