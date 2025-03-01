<template>
    <div
        class="flex flex-col items-center justify-center w-full relative"
        ref="drop-zone-ref"
    >
        <div class="self-start mb-2">{{ label }}</div>
        <div class="relative w-full">
            <button
                type="button"
                @click.prevent="clearFiles"
                class="text-red-400 absolute start-4 top-4 cursor-pointer hover:bg-red-600/20 p-0.5 rounded-lg"
                v-if="preview && !multiple && selectedFilesPreview.length > 0"
            >
                <IconTrash :size="25" color="currentColor" />
            </button>
            <label
                class="flex flex-col items-center justify-center w-full min-h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
            >
                <div class="max-w-full max-h-full" v-if="!isOverDropZone">
                    <div
                        class="max-w-full max-h-full"
                        v-if="
                            preview &&
                            !multiple &&
                            selectedFilesPreview.length > 0
                        "
                    >
                        <img
                            width="500"
                            height="400"
                            :src="selectedFilesPreview[0]"
                            class="max-h-3/4 max-w-3/4 m-auto object-contain object-center p-2 rounded-2xl"
                        />
                    </div>
                    <div
                        class="flex flex-col items-center justify-center pt-5 pb-6"
                        v-else
                    >
                        <svg
                            class="w-8 h-8 mb-3 text-gray-500 dark:text-gray-400"
                            aria-hidden="true"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 20 16"
                        >
                            <path
                                stroke="currentColor"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
                            />
                        </svg>
                        <p
                            class="mb-2 text-sm text-gray-500 dark:text-gray-400 text-center"
                        >
                            <span class="font-semibold"
                                >برای انتخاب کلیک کنید</span
                            ><br />
                            <span class="text-xs"
                                >و یا فایل را در این مکان رها کنید</span
                            >
                        </p>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                            SVG, PNG, JPG و یا GIF (حداکثر 800x400px)
                        </p>
                    </div>
                </div>
                <div
                    class="flex flex-col items-center justify-center pt-5 pb-6"
                    v-else
                >
                    <svg
                        class="w-8 h-8 mb-4 text-action-primary dark:text-dark-action-primary animate-pulse-quick"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 20 16"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
                        />
                    </svg>
                    <p
                        class="mb-2 text-sm text-gray-500 dark:text-gray-400 text-center"
                    >
                        <span class="font-semibold">رها کنید</span>
                    </p>
                </div>
                <input
                    ref="file-input-ref"
                    type="file"
                    class="hidden"
                    :name="name"
                    @change.prevent="handleInputSelect"
                />
            </label>
        </div>
    </div>
</template>

<script lang="ts" setup>
import type { PhysicalPosition } from "@tauri-apps/api/dpi";
import { listen } from "@tauri-apps/api/event";
import { readFile } from "@tauri-apps/plugin-fs";

const { multiple, name, label, preview } = defineProps({
    multiple: {
        type: Boolean,
        default: false,
        required: false,
    },
    name: {
        type: String,
        required: true,
    },
    label: {
        type: String,
        required: false,
        default: "",
    },
    preview: {
        type: Boolean,
        required: false,
        default: false,
    },
});

const dropZoneRef = useTemplateRef("drop-zone-ref");
const fileInputRef = useTemplateRef("file-input-ref");
const selectedFilesPreview = ref<string[]>([]);
const unlistenOver = ref();
const unlistenDrop = ref();
const isOverDropZone = ref(false);

const selectedFiles = defineModel<File[]>("value", {
    default: [],
});

watch(selectedFilesPreview, (_, oldVal) => {
    if (oldVal) {
        for (const eachOldPreview of oldVal) {
            window.URL.revokeObjectURL(eachOldPreview);
        }
    }
});
watch(selectedFiles, (newVal) => {
    if (preview) {
        const newPreviews: string[] = [];
        for (const eachFile of newVal) {
            newPreviews.push(window.URL.createObjectURL(eachFile));
        }
        selectedFilesPreview.value = newPreviews;
    } else {
        selectedFilesPreview.value = [];
    }
});
watch(unlistenOver, (_, old) => {
    if (old) {
        old();
    }
});
watch(unlistenDrop, (_, old) => {
    if (old) {
        old();
    }
});
onUnmounted(() => {
    if (unlistenOver.value) {
        unlistenOver.value();
    }
    if (unlistenDrop.value) {
        unlistenDrop.value();
    }
});
onMounted(async () => {
    unlistenOver.value = await listen("tauri://drag-over", (e) => {
        const { position } = e.payload as {
            type: "over";
            position: PhysicalPosition;
        };
        if (dropZoneRef.value) {
            const { bottom, right, width, height } =
                dropZoneRef.value.getBoundingClientRect();
            if (
                position.x >= right - width &&
                position.x <= right &&
                position.y >= bottom - height &&
                position.y <= bottom
            ) {
                isOverDropZone.value = true;
            } else {
                isOverDropZone.value = false;
            }
        }
    });
    unlistenDrop.value = await listen("tauri://drag-drop", async (e) => {
        const { position, paths } = e.payload as {
            type: "drop";
            paths: string[];
            position: PhysicalPosition;
        };
        if (dropZoneRef.value) {
            const { bottom, right, width, height } =
                dropZoneRef.value.getBoundingClientRect();
            if (
                position.x >= right - width &&
                position.x <= right &&
                position.y >= bottom - height &&
                position.y <= bottom
            ) {
                isOverDropZone.value = false;
                for (const file of paths) {
                    const icon = new File(
                        [new Blob([await readFile(file)])],
                        "temp"
                    );
                    if (!multiple) {
                        selectedFiles.value = [icon];
                    } else {
                        selectedFiles.value.push(icon);
                    }
                }
            }
        }
    });
});

const clearFiles = () => {
    selectedFiles.value = [];
    if (fileInputRef.value) {
        fileInputRef.value.value = "";
    }
};
const handleInputSelect = (e: Event) => {
    const files: FileList | null = (<HTMLInputElement>e?.target)?.files;
    if (!files || !files?.length || files.length < 1) return;
    selectedFiles.value = [];
    let val: File[] = [];
    for (const eachFile of files) {
        val.push(eachFile);
        if (!multiple) break;
    }
    selectedFiles.value = val;
};
</script>

<style></style>
