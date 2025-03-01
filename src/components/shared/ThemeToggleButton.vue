<template>
    <button
        class="text-nowrap cursor-pointer"
        @click.prevent.stop="toggleTheme"
    >
        <component
            :is="
                themeMode === 'system'
                    ? IconBrightness
                    : themeMode === 'light'
                    ? IconLightMode
                    : IconMoon
            "
            :size="16"
            color="currentColor"
            class="inline me-1.5"
        />
        {{
            themeMode === "system"
                ? "سیستم"
                : themeMode === "light"
                ? "روشن"
                : "تاریک"
        }}
    </button>
</template>

<script lang="ts" setup>
import { IconBrightness, IconLightMode, IconMoon } from "#components";

const themeMode = useLocalStorage("theme", "system", {
    initOnMounted: true,
    mergeDefaults(storageValue, _) {
        return !storageValue ? "system" : storageValue;
    },
});

const toggleTheme = () => {
    if (themeMode.value === "system") {
        // Whenever the user explicitly chooses dark mode
        themeMode.value = "dark";
    } else if (themeMode.value == "dark") {
        // Whenever the user explicitly chooses light mode
        themeMode.value = "light";
    } else if (themeMode.value == "light") {
        // Whenever the user explicitly chooses to respect the OS preference
        themeMode.value = undefined;
    } else {
        themeMode.value = undefined;
    }
};
</script>

<style lang="scss"></style>
