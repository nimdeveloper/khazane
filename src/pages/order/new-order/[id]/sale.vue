<template></template>

<script lang="ts" setup>
import { Order } from "~/interfaces/order";

definePageMeta({ layout: "new-order" });

const order = ref(new Order(""));

const { storage } = useTempOrder();

watch(order, (newVal) => {
    storage.value = newVal;
});

onMounted(async () => {
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
