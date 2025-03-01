<template>
    <Simplebar data-simplebar-direction="rtl" class="px-3 py-2 flex-1 w-full">
        <div class="flex gap-3 flex-row flex-wrap justify-around pt-4">
            <ProductListItem
                v-if="view === 'list'"
                v-for="(product, index) of products"
                :key="`product_${product.key}_${index}`"
                :title="product.title"
                :basePrice="product.basePrice"
                :inventory="product.inventory"
                :unit="product.unit"
                :image="product.image"
                :category="product.category"
                :initialInventory="product.initialInventory"
            />
            <ProductGridItem
                v-if="view === 'grid'"
                v-for="(product, index) of products"
                :key="`product_${product.key}_${index}`"
                :title="product.title"
                :basePrice="product.basePrice"
                :inventory="product.inventory"
                :unit="product.unit"
                :image="product.image"
                :category="product.category"
                :initialInventory="product.initialInventory"
            />
            <div
                class="pt-10 flex flex-col items-center text-center w-full justify-center text-lg"
                v-if="products.length < 1"
            >
                <div>
                    <IconFileRemove
                        :size="30"
                        color="currentColor"
                        class="text-action-primary dark:text-dark-action-primary me-1.5 inline"
                    />محصولی یافت نشد!
                </div>
                <div class="text-sm mt-4">
                    همین حالا اضافه کنید
                    <div class="inline ms-1 text-nowrap">
                        <NuxtLink
                            :to="{ name: 'inventory-new-product' }"
                            class="border-action-primary dark:border-dark-action-primary border-2 rounded-lg px-2 py-0.5 text-primary dark:text-dark-primary cursor-pointer hover:bg-action-primary/40 hover:dark:bg-dark-action-primary/40 min-w-full"
                        >
                            <IconPlusSquare
                                color="currentColor"
                                :size="20"
                                class="inline opacity-90 me-0.5"
                            />محصول جدید
                        </NuxtLink>
                    </div>
                </div>
            </div>
            <div
                v-else-if="products.length < 1"
                class="pt-10 flex flex-col items-center text-center w-full justify-center text-lg"
            >
                <div>
                    <IconZoomExclamation
                        :size="35"
                        color="currentColor"
                        class="text-action-primary dark:text-dark-action-primary me-1.5 inline"
                    />محصولی یافت نشد!
                </div>
            </div>
        </div>
    </Simplebar>
</template>

<script lang="ts" setup>
import { useIntervalFn } from "@vueuse/core";
import Simplebar from "simplebar-vue";
import { useMyProductStore } from "~/stores/product";

const productStore = useMyProductStore();

const { filteredProducts: products } = storeToRefs(productStore);

const { view } = defineProps<{ view: "list" | "grid" }>();

useIntervalFn(() => {
    productStore.loadProducts();
}, 5000);

onMounted(() => {
    productStore.loadProducts();
});
</script>

<style></style>
