<template>
    <Simplebar data-simplebar-direction="rtl" class="h-full w-full">
        <ProductDetail />
        Test
    </Simplebar>
</template>
<script setup lang="ts">
import Simplebar from "simplebar-vue";
import { useMyOrderStore } from "~/stores/order";
import { useMyPersonStore } from "~/stores/person";
import { useMyProductStore } from "~/stores/product";
import { useMyWarehouseStore } from "~/stores/warehouse";

const productStore = useMyProductStore();
const warehouseStore = useMyWarehouseStore();
const orderStore = useMyOrderStore();
const personStore = useMyPersonStore();

const SECONDS = 1000;
const MINUTES = 60 * SECONDS;

useInterval(30 * MINUTES, {
    async callback() {
        await Promise.all([
            productStore.loadProducts(),
            productStore.loadCategories(),
            warehouseStore.loadWareHouses(),
            orderStore.loadOrders(),
            personStore.loadPersons(),
        ]);
    },
    immediate: true,
});
onMounted(async () => {
    await Promise.all([
        productStore.loadProducts(),
        productStore.loadCategories(),
        warehouseStore.loadWareHouses(),
        orderStore.loadOrders(),
        personStore.loadPersons(),
    ]);
});
</script>
