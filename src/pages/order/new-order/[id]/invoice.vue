<template>
    <table>
        <tbody>
            <tr
                v-for="(item, index) of order.goods.filter(
                    (each) => !!each.product
                )"
                :key="index"
            >
                <td>{{ index + 1 }}</td>
                <td>{{ item.product?.key }}</td>
                <td>{{ item.product?.code }}</td>
                <td>{{ item.product?.title }}</td>
                <td>{{ item.product?.code }}</td>
                <td>{{ item.product?.unit?.label }}</td>
            </tr>
        </tbody>
    </table>
</template>

<script lang="ts" setup>
import { Order } from "~/interfaces/order";
import { useMyMeasureStore } from "~/stores/measure";

definePageMeta({ layout: "new-order" });

const measurementStore = useMyMeasureStore();

const order = ref(new Order(""));

const { storage } = useTempOrder();

onMounted(async () => {
    measurementStore.loadUnits();
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
