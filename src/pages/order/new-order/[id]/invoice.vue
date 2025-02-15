<template></template>

<script lang="ts" setup>
import { Order } from "~/interfaces/order";
import { useMyMeasureStore } from "~/stores/measure";

definePageMeta({ layout: "new-order" });

const measurementStore = useMyMeasureStore();

const order = ref(new Order(""));

const { storage } = useTempOrder();

onMounted(async () => {
    measurementStore.loadUnits();
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
