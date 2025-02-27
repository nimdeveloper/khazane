<template>
    <button
        class="text-nowrap"
        :class="{
            'cursor-pointer': !updateLoading,
        }"
        @click.prevent.stop="handleUpdate"
    >
        <IconSync
            :size="16"
            color="currentColor"
            class="inline me-1.5"
            :class="{ 'animate-spin': updateLoading }"
        />{{ state }}
    </button>
</template>

<script lang="ts" setup>
import { check, Update } from "@tauri-apps/plugin-updater";

const updateLoading = ref(false);
const updateDownloaded = ref(false);
const updateInstance = ref<Update | null>(null);

const state = computed(() => {
    if (!updateInstance.value && !updateLoading.value) return "بروزرسانی";
    if (!updateInstance.value) return "در حال بررسی";
    if (updateInstance.value.available && !updateLoading.value)
        return `دانلود (${updateInstance.value.version})`;
    return "اعمال بروزرسانی";
});
async function handleUpdate() {
    if (updateLoading.value) return;
    updateLoading.value = true;
    updateDownloaded.value = false;
    try {
        if (updateInstance.value && !updateDownloaded.value) {
            await updateInstance.value.download();
            updateDownloaded.value = true;
        } else if (updateInstance.value && updateDownloaded.value) {
            try {
                await updateInstance.value.install();
                updateDownloaded.value = false;
                updateInstance.value = null;
            } finally {
            }
        } else {
            const update = await check();

            updateInstance.value = update;
        }
    } catch (e) {
        console.error(e);
    } finally {
        updateLoading.value = false;
    }
}
</script>

<style lang="scss"></style>
