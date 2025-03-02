<template>
    <InventoryProductSelect
        :searchable="true"
        :selected="
            order.goods.map((each) => each.product).filter((each) => !!each)
        "
        @toggled="(e) => !e && productStore.loadProducts()"
        :addable="true"
        :options="productStore.products"
        @new="addItem"
        @remove="removeItem"
    />
    <div class="flex flex-col my-2">
        <div
            v-for="(item, index) of order.goods"
            class="border-2 rounded-2xl border-border-1 my-2"
            :key="`${index}_${item.product?.key}`"
        >
            <div v-if="item.product">
                <div class="flex flex-col md:flex-row">
                    <div class="rounded-xl overflow-hidden m-1">
                        <img
                            :src="
                                imagesList.length > index && imagesList[index]
                                    ? imagesList[index]
                                    : defaultImage
                            "
                            width="64"
                            height="64"
                            class="w-full h-36 md:h-16 md:w-16 object-center object-cover rounded-xl"
                        />
                    </div>
                    <div class="flex flex-col ms-1">
                        <div>
                            {{ item.product?.label }}
                        </div>
                        <div
                            class="flex items-center align-middle text-secondary dark:text-dark-secondary text-xs"
                        >
                            <span
                                class="text-nowrap"
                                v-if="item.product?.category"
                                >{{ item.product.category.label }}</span
                            >&nbsp;<span class="text-2xl pb-2">.</span
                            >&nbsp;<IconBookmarkOpen
                                :size="16"
                                color="currentColor"
                            />&nbsp; <span>موجودی:</span>&nbsp;<span
                                class="text-primary dark:text-dark-primary text-nowrap"
                                >{{ item.product?.inventory }}
                                {{
                                    item.product?.unit
                                        ? item.product?.unit.title
                                        : "-"
                                }}</span
                            >
                        </div>
                    </div>
                    <button
                        @click.prevent="handleRemoveProduct(index)"
                        class="m-auto me-2 text-red-400 hover:bg-red-400/20 cursor-pointer p-1 rounded-xl w-full md:w-auto text-center justify-center flex items-center"
                    >
                        <IconTrash :size="25" color="currentColor" />
                        <div class="ms-1 md:hidden">حذف</div>
                    </button>
                </div>
                <div
                    class="bg-glob-secondary dark:bg-dark-glob-secondary m-1 rounded-2xl p-1 px-2"
                >
                    <div class="grow-1 md:w-1/2">
                        <FormCounterInput
                            :name="`order_product_${index}_quantity`"
                            v-model:value="item.quantity"
                            label="تعداد"
                            :max="item.product?.inventory || 0"
                            :min="0"
                            class="grow-1"
                        />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { BaseDirectory, readFile } from "@tauri-apps/plugin-fs";
import { Order } from "~/interfaces/order";
import { ProductUnit } from "~/interfaces/product";
import { useMyProductStore } from "~/stores/product";

definePageMeta({ layout: "new-order" });

const defaultImage = "/images/no-photo.jpg";

const productStore = useMyProductStore();

const order = ref(new Order(""));
const imagesList = ref<string[]>([]);

const { storage } = useTempOrder();

watch(order, (newVal) => {
    storage.value = newVal;
    for (const [index, eachGood] of newVal.goods.entries()) {
        const product = eachGood.product;
        if (product) {
            if (product.image) {
                console.log(product.image);
                processRawFile(product.image).then((file) => {
                    console.log(file);
                    if (file) {
                        if (imagesList.value[index]) {
                            try {
                                window.URL.revokeObjectURL(
                                    imagesList.value[index]
                                );
                            } catch (e) {}
                        }
                        imagesList.value[index] = file;
                    }
                });
            } else {
                if (imagesList.value[index]) {
                    try {
                        window.URL.revokeObjectURL(imagesList.value[index]);
                    } catch (e) {}
                }
                delete imagesList.value[index];
            }
        }
    }
});
const processRawFile = async (file: string | File) => {
    if (file instanceof File) {
        return window.URL.createObjectURL(file);
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
        return window.URL.createObjectURL(openFile);
    }
};

function addItem(item: ProductUnit) {
    order.value.goods.push({ quantity: 0, product: item });
}
function removeItem(item: ProductUnit) {
    const index = order.value.goods.findIndex(
        (each) => each.product?.key === item.key
    );
    if (index > -1) {
        order.value.goods.splice(index, 1);
    }
}
function handleRemoveProduct(index: number) {
    order.value.goods.splice(index, 1);
}

onMounted(async () => {
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
