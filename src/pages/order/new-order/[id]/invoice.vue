<template>
    <OrderPrint :order="order" />
    <div class="print:hidden">
        <button class="cursor-pointer" @click.prevent.stop="handlePrint">
            پرینت
        </button>
    </div>
</template>

<script lang="ts" setup>
import { Order } from "~/interfaces/order";
import { useMeasureStore } from "~/stores/measure";

definePageMeta({ layout: "new-order" });

const measurementStore = useMeasureStore();

const order = ref(new Order(""));

const { storage } = useTempOrder();

onMounted(async () => {
    measurementStore.loadUnits();
    if (storage.value) {
        order.value = storage.value;
    }
});
function handlePrint() {
    window?.print();
}
</script>

<style></style>
