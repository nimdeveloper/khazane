<template>
    <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-2">
        در حال بارگذاری
    </h3>
</template>

<script lang="ts" setup>
definePageMeta({ layout: "new-product" });

const { storage } = useTempOrder();

onBeforeMount(() => {
    if (!storage.value?.key) {
        const id = random(30);
        storage.value.key = id;
        navigateTo({
            name: "order-new-order-id-public",
            params: { id },
        });
    } else {
        navigateTo({
            name: "order-new-order-id-public",
            params: { id: storage.value.key },
        });
    }
});
</script>

<style></style>
