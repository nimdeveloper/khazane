<template></template>

<script lang="ts" setup>
import { useStorage } from "@vueuse/core";
import { Order } from "~/interfaces/order";
import { ProductUnit } from "~/interfaces/product";
import { useMyWarehouseStore } from "~/stores/warehouse";

definePageMeta({ layout: "new-order" });

const wareHouseStore = useMyWarehouseStore();

const newWarehouseModalOpen = inject<{ open: boolean; preferred_name: string }>(
    "newWarehouseModalOpen"
);

const order = ref(new Order(""));

const { storage } = useTempOrder();

watch(order, (newVal) => {
    storage.value = newVal;
});
watch(
    () => newWarehouseModalOpen,
    (val) => {
        if (val && !val.open) {
            wareHouseStore.loadWareHouses();
        }
    }
);
onMounted(async () => {
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
