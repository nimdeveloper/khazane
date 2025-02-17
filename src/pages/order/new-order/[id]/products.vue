<template>
    <InventoryProductSelect
        :searchable="true"
        :selected="
            order.goods.map((each) => each.product).filter((each) => !!each)
        "
        :addable="true"
        :options="[{ label: 'test', value: 'x' }]"
        @new="addItem"
        @remove="removeItem"
    />
    <div v-for="(item, index) of order.goods">
        {{ item.product?.label }}
        {{ item.quantity }}
    </div>
</template>

<script lang="ts" setup>
import { useStorage } from "@vueuse/core";
import { Order } from "~/interfaces/order";
import { ProductUnit, type IProductUnit } from "~/interfaces/product";
import { useMyWarehouseStore } from "~/stores/warehouse";

definePageMeta({ layout: "new-order" });

const wareHouseStore = useMyWarehouseStore();

const newWarehouseModalOpen = inject<{ open: boolean; preferred_name: string }>(
    "newWarehouseModalOpen"
);

const order = ref(new Order(""));

const { storage } = useTempOrder();

watch(order, (newVal) => {
    storage.value = newVal;
});
watch(
    () => newWarehouseModalOpen,
    (val) => {
        if (val && !val.open) {
            wareHouseStore.loadWareHouses();
        }
    }
);

function addItem(item: ProductUnit) {
    console.log("HER");
    order.value.goods.push({ quantity: 0, product: item });
}
function removeItem(item: ProductUnit) {
    const index = order.value.goods.findIndex(
        (each) => each.product?.key === item.key
    );
    if (index > -1) {
        order.value.goods.splice(index, 1);
    }
}

onMounted(async () => {
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
