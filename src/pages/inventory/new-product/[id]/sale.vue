<template>
    <div class="flex w-full">
        <FormInput
            name="product_name"
            label="قیمت واحد"
            v-model:value="product.basePrice"
            class="grow-1"
        />
    </div>
</template>

<script lang="ts" setup>
import { useStorage } from "@vueuse/core";
import { ProductUnit } from "~/interfaces/product";

definePageMeta({ layout: "new-product" });

const product = ref(new ProductUnit(""));
const storage = useStorage(
    "new-product-temp-item",
    new ProductUnit(""),
    sessionStorage,
    {
        serializer: {
            read: (v: any) => ProductUnit.read(v),
            write: (v: any) => v.write(),
        },
    }
);

watch(product, (newVal) => {
    storage.value = newVal;
});

onMounted(async () => {
    if (storage.value) {
        product.value = storage.value;
    }
});
</script>

<style></style>
