<template>
    <div
        class="bg-primary-bg print:bg-white print:text-active-item-bg h-full text-text-primary font-vazir"
    >
        <NuxtRouteAnnouncer />
        <Modal v-model:modal="modal" />
        <NuxtLayout>
            <NuxtPage />
        </NuxtLayout>
    </div>
</template>

<script setup lang="ts">
import type { ModalInterface } from "flowbite";

import "simplebar-vue/dist/simplebar.min.css";

const modal = ref<any>(null);

const openModal = () => {
    if (modal.value) {
        modal.value.show();
    }
};
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
