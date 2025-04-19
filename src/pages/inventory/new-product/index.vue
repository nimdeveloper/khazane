<template>
    <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-2">
        در حال بارگذاری
    </h3>
</template>

<script lang="ts" setup>
definePageMeta({ layout: "new-product" });

const { storage } = useTempProduct();

onBeforeMount(() => {
    if (!storage.value?.id) {
        const id = 0;
        storage.value.id = id;
        navigateTo({
            name: "inventory-new-product-id-public",
            params: { id: id },
        });
    } else {
        navigateTo({
            name: "inventory-new-product-id-public",
            params: { id: `${storage.value.id}` },
        });
    }
});
</script>

<style></style>
