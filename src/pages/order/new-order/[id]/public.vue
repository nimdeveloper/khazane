<template></template>

<script lang="ts" setup>
import { useMyProductStore } from "~/stores/product";
import { Order } from "~/interfaces/order";

definePageMeta({ layout: "new-order" });

const productStore = useMyProductStore();

const order = ref(new Order(""));
const productFiles = ref<File[]>([]);

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
