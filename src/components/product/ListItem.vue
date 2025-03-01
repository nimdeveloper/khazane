<template>
    <div
        class="flex bg-action-secondary dark:bg-dark-action-secondary rounded-2xl p-1.5 my-2.5 max-w-2xl grow-1"
    >
        <div class="h-16 w-16 rounded-2xl bg-white overflow-hidden">
            <img
                :src="processedImage"
                width="64"
                height="64"
                class="h-16 w-16 object-center object-cover"
            />
        </div>
        <div class="flex grow-1 px-2">
            <div class="flex flex-col items-start justify-around ps-3.5">
                <div class="flex">
                    {{ title }}
                    <div
                        class="h-2/3 w-0.5 bg-secondary/10 dark:bg-dark-secondary/10 mx-2 md:mx-3 my-auto hidden sm:flex lg:hidden"
                    ></div>
                    <div
                        class="justify-start gap-5 grow-1 hidden sm:flex lg:hidden text-xs"
                    >
                        <div class="flex justify-start items-center gap-2">
                            <div
                                class="text-secondary dark:text-dark-secondary text-xs"
                            >
                                قیمت واحد
                            </div>
                            <div>{{ basePrice }}</div>
                        </div>
                        <div class="flex justify-start items-center gap-2">
                            <div
                                class="text-secondary dark:text-dark-secondary text-xs"
                            >
                                موجودی اولیه
                            </div>
                            <div>{{ initialInventory }}</div>
                        </div>
                    </div>
                </div>
                <div
                    class="flex items-center align-middle text-secondary dark:text-dark-secondary text-xs"
                >
                    <span
                        v-if="variantCount > 0"
                        class="bg-blue-100 text-secondary dark:text-dark-secondary font-medium px-2.5 py-0.5 rounded-md dark:bg-blue-400 h-5 text-nowrap"
                    >
                        5 نوع </span
                    >&nbsp;<span class="text-2xl pb-2" v-if="variantCount > 0"
                        >.</span
                    >&nbsp;<span class="text-nowrap" v-if="category">{{
                        category.label
                    }}</span
                    >&nbsp;<span class="text-2xl pb-2">.</span
                    >&nbsp;<IconBookmarkOpen
                        :size="16"
                        color="currentColor"
                    />&nbsp; <span>موجودی:</span>&nbsp;<span
                        class="text-primary dark:text-dark-primary text-nowrap"
                        >{{ inventory }} {{ unit ? unit.title : "-" }}</span
                    >
                </div>
            </div>
            <div
                class="h-2/3 w-0.5 bg-secondary/10 dark:bg-dark-secondary/10 mx-5 my-auto hidden lg:flex"
            ></div>
            <div class="justify-start gap-5 grow-1 hidden lg:flex">
                <div class="flex flex-col justify-center items-start">
                    <div
                        class="text-secondary dark:text-dark-secondary text-xs mb-1"
                    >
                        قیمت واحد
                    </div>
                    <div>{{ basePrice }}</div>
                </div>
                <div class="flex flex-col justify-center items-start">
                    <div
                        class="text-secondary dark:text-dark-secondary text-xs mb-1"
                    >
                        موجودی اولیه
                    </div>
                    <div>{{ initialInventory }}</div>
                </div>
            </div>
            <div class="p-2 flex ms-auto ps-4">
                <button
                    class="m-auto rounded-xl border-secondary dark:border-dark-secondary border-2 p-1.5 text-secondary dark:text-dark-secondary hover:text-amber-50 hover:border-amber-50 transition-colors cursor-pointer"
                >
                    <IconThreeDots :size="18" color="currentColor" />
                </button>
            </div>
        </div>
        <div class=""></div>
    </div>
</template>

<script lang="ts" setup>
import { IconThreeDots } from "#components";
import { BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
import type { IProductUnit } from "~/interfaces/product";

const defaultImage = "/images/no-photo.jpg";

const variantCount = ref(0);
const processedImage = ref("");
const { title, basePrice, inventory, unit, initialInventory, category, image } =
    defineProps<
        Pick<
            IProductUnit,
            | "title"
            | "basePrice"
            | "inventory"
            | "unit"
            | "initialInventory"
            | "image"
            | "category"
        >
    >();

watch(processedImage, (_, oldVal) => {
    if (oldVal) {
        try {
            window.URL.revokeObjectURL(oldVal);
        } catch {}
    }
});
onMounted(() => {
    if (image) {
        processRawFile(image);
        return;
    }
    processedImage.value = defaultImage;
});
watch(
    () => image,
    async (val) => {
        if (val) {
            processRawFile(val);
            return;
        }
        processedImage.value = defaultImage;
    }
);
const processRawFile = async (file: string | File) => {
    if (file instanceof File) {
        processedImage.value = window.URL.createObjectURL(file);
        return;
    } else if (file) {
        const openFile = new File(
            [
                new Blob([
                    await readFile(file, {
                        baseDir: BaseDirectory.AppCache,
                    }),
                ]),
            ],
            "temp"
        );
        processedImage.value = window.URL.createObjectURL(openFile);
        return;
    }
};
</script>

<style></style>
