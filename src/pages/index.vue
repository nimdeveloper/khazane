<template>
    <Simplebar data-simplebar-direction="rtl" class="h-full w-full">
        <ProductDetail />
        <div
            class="min-h-[80dvh] flex flex-col items-center justify-center rounded-xl"
        >
            <h1
                class="text-5xl text-gray-900 dark:text-white font-bold mb-8 animate-pulse"
            >
                Coming Soon
            </h1>
            <p class="text-gray-900 dark:text-white text-lg mb-8">
                We're working hard to bring you something amazing. Stay tuned!
            </p>
        </div>
    </Simplebar>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
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

// useInterval(30 * MINUTES, {
//     async callback() {
//         await Promise.all([
//             productStore.loadProducts(),
//             productStore.loadCategories(),
//             warehouseStore.loadWareHouses(),
//             orderStore.loadOrders(),
//             personStore.loadPersons(),
//         ]);
//     },
//     immediate: true,
// });
onMounted(async () => {
    // setTimeout(() => {
    //     invoke("list_locations", {
    //         filters: { sort_by: "id", sort_order: "desc" },
    //     }).then((data) => console.log(data));
    // invoke("create_location", {
    //     location: { name: "test" },
    // }).then((data) => {
    //     console.log(data);
    //     invoke("list_locations", {
    //         filters: { sort_by: "id", sort_order: "desc" },
    //     }).then((data) => console.log(data));
    // });
    // }, 1000);
    // await Promise.all([
    //     productStore.loadProducts(),
    //     productStore.loadCategories(),
    //     warehouseStore.loadWareHouses(),
    //     orderStore.loadOrders(),
    //     personStore.loadPersons(),
    // ]);
});
</script>
