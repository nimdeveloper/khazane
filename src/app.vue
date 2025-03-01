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

const themeMode = useLocalStorage("theme", "system", {
    initOnMounted: true,
    mergeDefaults(storageValue, _) {
        return !storageValue ? "system" : storageValue;
    },
});

watch(themeMode, (val) => {
    // On page load or when changing themes, best to add inline in `head` to avoid FOUC
    document.documentElement.classList.toggle(
        "dark",
        val === "dark" ||
            (val === "system" &&
                window.matchMedia("(prefers-color-scheme: dark)").matches)
    );
});
onMounted(() => {
    // On page load or when changing themes, best to add inline in `head` to avoid FOUC
    document.documentElement.classList.toggle(
        "dark",
        themeMode.value === "dark" ||
            (themeMode.value === "system" &&
                window.matchMedia("(prefers-color-scheme: dark)").matches)
    );
});
</script>

<style lang="scss">
html {
    background-color: var(--color-glob-primary);
    &.dark {
        background-color: var(--color-dark-glob-primary);
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
