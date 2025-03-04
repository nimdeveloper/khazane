<template>
    <button
        class="text-nowrap relative"
        :class="{
            'cursor-pointer': !updateLoading,
        }"
        @click.prevent.stop="handleUpdate"
    >
        <span class="absolute top-1 right-1 flex size-2.5" v-if="hasUpdate">
            <span
                class="absolute inline-flex h-full w-full animate-ping rounded-full bg-action-primary opacity-75"
            ></span>
            <span
                class="relative inline-flex size-2.5 rounded-full bg-action-primary"
            ></span>
        </span>
        <component
            :is="
                updateInstance?.available && !updateLoading
                    ? IconDownload
                    : IconSync
            "
            :size="16"
            color="currentColor"
            class="inline me-1.5"
            :class="{ 'animate-spin': updateLoading }"
        />{{ state }}
        <div
            v-if="updateLoading && !!updateInstance"
            class="w-full bg-gray-200 rounded-full h-1.5 my-2 dark:bg-gray-700"
        >
            <div
                class="bg-blue-600 h-1.5 rounded-full dark:bg-blue-500 transition-all"
                :style="{
                    width: `${progressPercent}%`,
                }"
            ></div>
        </div>
    </button>
</template>

<script lang="ts" setup>
import { IconDownload, IconSync } from "#components";
import { Channel, invoke, Resource } from "@tauri-apps/api/core";
import { check, Update } from "@tauri-apps/plugin-updater";

const hasUpdate = defineModel<boolean>("has-update", {
    default: false,
});
const updateLoading = ref(false);
const updateDownloaded = ref(false);
const updateInstance = ref<Partial<Update> | null>(null);
const updateProgress = ref({ all: 0, downloaded: 0 });

const progressPercent = computed(() => {
    const { all, downloaded } = updateProgress.value;
    if (all <= 0) return 0;
    return Math.round((downloaded * 100) / all);
});
const state = computed(() => {
    if (!updateInstance.value && !updateLoading.value) return "بروزرسانی";
    if (!updateInstance.value) return "در حال بررسی";
    if (
        updateInstance.value.available &&
        !updateLoading.value &&
        !updateDownloaded.value
    )
        return `دانلود (${updateInstance.value.version})`;
    if (updateLoading.value) return "در حال دانلود";
    return "اعمال بروزرسانی";
});
async function handleDownload() {
    if (updateInstance.value) {
        const channel = new Channel();

        let downloaded = 0;
        channel.onmessage = ({ data, event }: any) => {
            console.log(event, data);
            let contentLength: number = 0;
            let chunkLength: number = 0;
            if (data) {
                contentLength = data.contentLength;
                chunkLength = data.chunkLength;
            }
            switch (event) {
                case "Finished":
                    updateProgress.value = {
                        all: 0,
                        downloaded: 0,
                    };
                    updateLoading.value = false;
                    updateDownloaded.value = true;
                    break;
                case "Progress":
                    downloaded += chunkLength;
                    updateProgress.value = {
                        ...updateProgress.value,
                        downloaded,
                    };
                    break;
                case "Started":
                    updateProgress.value = {
                        ...updateProgress.value,
                        all: contentLength,
                    };
                    updateLoading.value = true;
                    break;
            }
        };

        const downloadedBytesRid = await invoke<number>(
            "plugin:updater|download",
            {
                onEvent: channel,
                rid: updateInstance.value.rid,
            }
        );
        // @ts-ignore
        updateInstance.value.downloadedBytes = new Resource(downloadedBytesRid);
    }
}
/** Install downloaded updater package */
async function handleInstall() {
    if (updateInstance.value) {
        // @ts-ignore
        if (!updateInstance.value.downloadedBytes) {
            throw new Error("Update.install called before Update.download");
        }
        await invoke("plugin:updater|install", {
            updateRid: updateInstance.value.rid,
            // @ts-ignore
            bytesRid: updateInstance.value.downloadedBytes.rid,
        });
        // Don't need to call close, we did it in rust side already
        // @ts-ignore
        updateInstance.value.downloadedBytes = undefined;
    }
}
async function handleUpdate() {
    if (updateLoading.value) return;
    updateLoading.value = true;
    try {
        if (updateInstance.value && !updateDownloaded.value) {
            await handleDownload();
        } else if (updateInstance.value && updateDownloaded.value) {
            try {
                await handleInstall();
                updateDownloaded.value = false;
                updateInstance.value = null;
                hasUpdate.value = false;
            } catch {
            } finally {
            }
        } else {
            const update = await check();
            if (update) {
                hasUpdate.value = true;
                updateInstance.value = { ...update, rid: update?.rid };
            } else {
                hasUpdate.value = false;
            }
        }
    } catch (e) {
        console.error(e);
    } finally {
        updateLoading.value = false;
    }
}
useInterval(10 * 1000, {
    async callback() {
        console.log("HERE");
        const update = await check();
        if (update) {
            hasUpdate.value = true;
            updateInstance.value = { ...update, rid: update?.rid };
        } else {
            hasUpdate.value = false;
        }
    },
});
</script>

<style lang="scss"></style>
