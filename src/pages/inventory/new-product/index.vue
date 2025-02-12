<template>
    <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-2">
        در حال بارگذاری
    </h3>
</template>

<script lang="ts" setup>
import { useStorage } from "@vueuse/core";
import { ProductUnit } from "~/interfaces/product";

definePageMeta({ layout: "new-product" });

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

onBeforeMount(() => {
    if (!storage.value?.key) {
        const id = random(10);
        storage.value.key = id;
        navigateTo({
            name: "inventory-new-product-id-public",
            params: { id },
        });
    } else {
        navigateTo({
            name: "inventory-new-product-id-public",
            params: { id: storage.value.key },
        });
    }
});
</script>

<style></style>
