<template>
    <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-2">
        در حال بارگذاری
    </h3>
</template>

<script lang="ts" setup>
definePageMeta({ layout: "new-product" });

const { storage } = useTempOrder();

onBeforeMount(() => {
    /**
     * TODO Fix pages check the order id from the URL
     */
    if (!storage.value?.id) {
        storage.value.id = 0;
        navigateTo({
            name: "order-new-order-id-public",
            params: { id: "new" },
        });
    } else {
        navigateTo({
            name: "order-new-order-id-public",
            params: { id: `${storage.value.id}` },
        });
    }
});
</script>

<style></style>
