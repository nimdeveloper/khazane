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
                <div class="flex">
                    <div class="rounded-xl overflow-hidden m-1">
                        <img
                            :src="imagesMap.get(item.product!.key) || defaultImage"
                            width="64"
                            height="64"
                            class="h-16 w-16 object-center object-cover"
                        />
                    </div>
                    <div class="flex">
                        {{ item.product?.label }}
                        {{ item.quantity }}
                    </div>
                </div>
                <div class="bg-secondary-bg m-1 rounded-2xl p-1">
                    <div class="grow-1 md:w-1/2">
                        <FormCounterInput
                            :name="`order_product_${index}_quantity`"
                            v-model:value="item.quantity"
                            :max="100"
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
import { ProductUnit, type IProductUnit } from "~/interfaces/product";
import { useMyProductStore } from "~/stores/product";
import { useMyWarehouseStore } from "~/stores/warehouse";

definePageMeta({ layout: "new-order" });

const defaultImage = "/images/no-photo.jpg";

const wareHouseStore = useMyWarehouseStore();
const productStore = useMyProductStore();

const newWarehouseModalOpen = inject<{ open: boolean; preferred_name: string }>(
    "newWarehouseModalOpen"
);

const order = ref(new Order(""));
const imagesMap = ref<Map<string, string>>(new Map());

const { storage } = useTempOrder();

watch(order, (newVal) => {
    storage.value = newVal;
    for (const eachGood of newVal.goods) {
        const product = eachGood.product;
        if (product) {
            if (product.image) {
                console.log(product.image);
                processRawFile(product.image).then((file) => {
                    console.log(file);
                    if (file) {
                        imagesMap.value.set(product.key, file);
                    }
                });
            } else {
                const oldImage = imagesMap.value.get(product.key);
                if (oldImage) {
                    window.URL.revokeObjectURL(oldImage);
                }
                imagesMap.value.delete(product.key);
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
watch(
    () => newWarehouseModalOpen,
    (val) => {
        if (val && !val.open) {
            wareHouseStore.loadWareHouses();
        }
    }
);

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

onMounted(async () => {
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
