<template>
    <div
        class="flex bg-action-secondary dark:bg-dark-action-secondary rounded-2xl p-1.5 my-2.5 max-w-3xs grow-1 flex-col"
    >
        <div class="h-full w-full overflow-hidden p-1.5">
            <img
                :src="processedImage"
                width="128"
                height="128"
                class="h-full w-full object-center object-cover bg-white rounded-2xl"
            />
        </div>
        <div class="flex grow-1 flex-col items-start justify-around">
            <div class="flex w-full mt-1 px-2">
                <div>{{ title }}</div>
                <button
                    class="ms-auto rounded-xl text-secondary dark:text-dark-secondary hover:text-amber-50 hover:border-amber-50 transition-colors cursor-pointer rotate-90"
                >
                    <IconThreeDots :size="24" color="currentColor" />
                </button>
            </div>
            <div
                class="flex items-center align-middle text-secondary dark:text-dark-secondary text-xs px-1"
            >
                <span
                    v-if="variantCount > 0"
                    class="bg-blue-100 text-secondary font-medium px-2.5 py-0.5 rounded-md dark:bg-blue-400 dark:text-gray-900 h-5 text-nowrap"
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
            <div class="justify-start gap-5 grow-1 flex px-2">
                <div class="flex flex-col justify-center items-start">
                    <div
                        class="text-secondary dark:text-dark-secondary text-xs mb-1"
                    >
                        قیمت واحد
                    </div>
                    <div>{{ basePrice }}</div>
                </div>
                <div
                    class="h-2/3 w-0.5 bg-secondary/10 dark:bg-dark-secondary/10 mx-1 md:mx-2 my-auto hidden sm:flex lg:hidden"
                ></div>
                <div class="flex flex-col justify-center items-start">
                    <div
                        class="text-secondary dark:text-dark-secondary text-xs mb-1"
                    >
                        موجودی اولیه
                    </div>
                    <div>{{ initialInventory }}</div>
                </div>
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

<style lang="scss"></style>
