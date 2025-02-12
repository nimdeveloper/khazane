<template>
    <div class="flex w-full">
        <FormInput
            name="product_name"
            label="نام محصول"
            v-model:value="product.title"
            class="grow-1"
        />
    </div>
    <div class="flex w-full gap-4 my-3">
        <div class="grow-1 w-1/2">
            <FormSelect
                :options="productStore.categories"
                :selected="product.category"
                :searchable="true"
                :addable="true"
                @change="(e) => (product.category = e)"
                @toggled="(e) => !e && productStore.loadCategories()"
                @add="(name:string) => productStore.addCategory(name)"
                name="product_category"
                label="دسته بندی"
                class="grow-1"
            />
        </div>
        <div class="grow-1 w-1/2">
            <FormInput
                name="product_code"
                label="کد فنی"
                v-model:value="product.code"
                class="grow-1"
            />
        </div>
    </div>
    <div class="flex w-full gap-4 my-3">
        <div class="grow-1">
            <FormFileInput
                name="product_image"
                label="تصویر"
                v-model:value="productFiles"
                class="grow-1"
                :preview="true"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useStorage } from "@vueuse/core";
import { ProductUnit } from "~/interfaces/product";
import {
    writeFile,
    mkdir,
    BaseDirectory,
    readFile,
} from "@tauri-apps/plugin-fs";
import * as path from "@tauri-apps/api/path";
import { useMyProductStore } from "~/stores/product";

definePageMeta({ layout: "new-product" });

const productStore = useMyProductStore();

const product = ref(new ProductUnit(""));
const productFiles = ref<File[]>([]);
const storage = useStorage(
    "new-product-temp-item",
    new ProductUnit(""),
    sessionStorage,
    {
        serializer: {
            read: (v: any) => ProductUnit.read(v),
            write: (v: any) => v.write(),
        },
    }
);
watch(productFiles, async (newVal) => {
    const files: string[] = [];
    for (const each of newVal) {
        if (each instanceof File) {
            const folder_path = await path.join(
                "products",
                storage.value.key,
                "tmp"
            );
            await mkdir(folder_path, {
                baseDir: BaseDirectory.AppCache,
                recursive: true,
            });
            const file_path = await path.join(
                "products",
                storage.value.key,
                "tmp",
                `${random(5)}.file`
            );
            let buff = await each.arrayBuffer();
            const contents = new Uint8Array(buff); // fill a byte array
            await writeFile(file_path, contents, {
                baseDir: BaseDirectory.AppCache,
            });
            files.push(file_path);
        }
    }
    const item = product.value;
    if (files.length > 0) {
        item.image = files[0];
    } else {
        item.image = "";
    }
    product.value = item;
});
watch(product, (newVal) => {
    storage.value = newVal;
});
onMounted(async () => {
    productStore.loadCategories();
    if (storage.value) {
        product.value = storage.value;
        if (storage.value.image && !(storage.value.image instanceof File)) {
            productFiles.value = [
                new File(
                    [
                        new Blob([
                            await readFile(storage.value.image, {
                                baseDir: BaseDirectory.AppCache,
                            }),
                        ]),
                    ],
                    "temp"
                ),
            ];
        }
    }
});
</script>

<style></style>
