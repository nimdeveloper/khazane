<template>
    <div class="w-full mt-3">
        <FormSelect
            :options="measurementStore.units"
            :selected="product.unit"
            :searchable="true"
            :addable="true"
            @change="(e) => (product.unit = e)"
            @add="(e) => measurementStore.addUnit(e)"
            @toggled="(e) => !e && measurementStore.loadUnits()"
            name="product_warehouse_measurement-unit"
            label="واحد اندازه گیری"
            class="grow-1"
        />
    </div>
</template>

<script lang="ts" setup>
import { ProductUnit } from "~/interfaces/product";
import { useMyMeasureStore } from "~/stores/measure";

definePageMeta({ layout: "new-product" });

const measurementStore = useMyMeasureStore();

const product = ref(new ProductUnit(""));

const { storage } = useTempProduct();

watch(product, (val) => {
    if (val) {
        storage.value = val;
    }
});
onMounted(async () => {
    measurementStore.loadUnits();
    if (storage.value) {
        product.value = storage.value;
    }
});
</script>

<style></style>
