<template>
    <OrderPrint :order="toPrintOrder" />
    <Simplebar
        data-simplebar-direction="rtl"
        force-visible="x"
        class="px-3 py-2 flex-1 w-full print:hidden h-full"
    >
        <div
            class="flex gap-3 flex-row flex-wrap justify-around pt-4 print:p-0"
        >
            <OrderListItem
                @print="
                    toPrintOrder = order;
                    handlePrint();
                "
                v-for="(order, index) of orders"
                :key="`product_${order.key}_${index}`"
                :order="order"
            />
            <div
                class="pt-10 flex flex-col items-center text-center w-full justify-center text-lg print:hidden"
                v-if="orders.length < 1"
            >
                <div>
                    <IconFileRemove
                        :size="30"
                        color="currentColor"
                        class="text-action-primary dark:text-dark-action-primary me-1.5 inline"
                    />موردی یافت نشد!
                </div>
                <div class="text-sm mt-4">
                    همین حالا اضافه کنید
                    <div class="inline ms-1 text-nowrap">
                        <NuxtLink
                            :to="{ name: 'order-new-order' }"
                            class="border-action-primary dark:border-dark-action-primary border-2 rounded-lg px-2 py-0.5 text-primary dark:text-dark-primary cursor-pointer hover:bg-action-primary/40 hover:dark:bg-dark-action-primary/40 min-w-full"
                        >
                            <IconPlusSquare
                                color="currentColor"
                                :size="20"
                                class="inline opacity-90 me-0.5"
                            />گردش جدید
                        </NuxtLink>
                    </div>
                </div>
            </div>
            <div
                v-else-if="orders.length < 1"
                class="pt-10 flex flex-col items-center text-center w-full justify-center text-lg print:hidden"
            >
                <div>
                    <IconZoomExclamation
                        :size="35"
                        color="currentColor"
                        class="text-action-primary dark:text-dark-action-primary me-1.5 inline"
                    />محصولی یافت نشد!
                </div>
            </div>
        </div>
    </Simplebar>
</template>

<script lang="ts" setup>
import { OrderListItem } from "#components";
import { useIntervalFn } from "@vueuse/core";
import Simplebar from "simplebar-vue";
import { Order } from "~/interfaces/order";
import { useMyOrderStore } from "~/stores/order";

const orderStore = useMyOrderStore();

const toPrintOrder = ref(new Order(""));

const { filteredOrders: orders } = storeToRefs(orderStore);

// const { view } = defineProps<{ view: "list" | "grid" }>();

useIntervalFn(() => {
    orderStore.loadOrders();
}, 5000);

onMounted(() => {
    orderStore.loadOrders();
});

const handlePrint = () => {
    window?.print();
};
</script>

<style></style>
