<template>
    <div class="hidden flex-col text-right min-h-[90dvh] print:flex">
        <div>
            <h5 class="font-bold inline me-2">تحویل دهنده:</h5>
            <div class="inline">{{ order.delivery?.name }}</div>
        </div>
        <div>
            <h5 class="font-bold inline me-2">تحویل گیرنده:</h5>
            <div class="inline">{{ order.recipient?.name }}</div>
        </div>
        <div class="flex gap-10 my-5">
            <div class="inline">نوع استناد: {{ order.type }}</div>
            <div class="inline">شماره استناد: {{ order.document_number }}</div>
            <div class="inline">تاریخ استناد: {{ order.document_date }}</div>
            <div class="inline">رابط: {{ order.manager?.name }}</div>
        </div>
        <table class="border border-border-1 w-full text-center mb-auto">
            <thead class="print:bg-active-item-bg/20 text-sm">
                <tr>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        ردیف
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        کد استقرار
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        شناسه کالا
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        شرح کالا
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        کد فنی
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        واحد
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        مقدار (عدد)
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        مقدار (حروف)
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        قیمت واحد
                    </th>
                    <th class="border border-black/60 p-0.5 text-nowrap">
                        قیمت کل
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr
                    v-for="(item, index) of order.goods.filter(
                        (each) => !!each.product
                    )"
                    :key="index"
                >
                    <td class="border border-black/60 p-0.5">
                        {{ index + 1 }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{ item.product?.key }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{ item.product?.code }}
                    </td>
                    <td class="border border-black/60 p-0.5 px-2">
                        {{ item.product?.title }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{ item.product?.code }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{ item.product?.unit?.label || "" }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{ addCommas(item.quantity || 0) }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{ numberToWords(item.quantity) }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{ addCommas(item.product?.basePrice || 0) }}
                    </td>
                    <td class="border border-black/60 p-0.5">
                        {{
                            addCommas(
                                (item.product?.basePrice || 0) *
                                    (item.quantity || 0)
                            )
                        }}
                    </td>
                </tr>
            </tbody>
        </table>
        <div class="border-active-item-bg border mt-5">
            <div class="min-h-20">
                <h5 class="font-bold inline me-2">توضیحات:</h5>
                <div class="inline">
                    {{ order.description }}
                </div>
            </div>
            <div class="flex">
                <div
                    class="text-center print:bg-active-item-bg/20 p-1 border-e border-s-active-item-bg"
                    style="writing-mode: vertical-rl"
                >
                    تایید کنندگان
                </div>
                <div class="flex flex-1 flex-wrap grow-1" style="row-gap: 10px">
                    <div
                        v-for="item of order.approvers"
                        class="grow-1 text-nowrap min-w-40 text-center flex flex-col"
                    >
                        <div class="border border-primary-bg border-e-0">
                            {{ item.name }}
                        </div>
                        <div class="border-s border-primary-bg h-16 grow-1">
                            {{ item.person?.label }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div class="print:hidden">
        <button class="cursor-pointer" @click.prevent.stop="handlePrint">
            پرینت
        </button>
    </div>
</template>

<script lang="ts" setup>
import { Order } from "~/interfaces/order";
import { useMyMeasureStore } from "~/stores/measure";
import { addCommas, numberToWords } from "@persian-tools/persian-tools";

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
function handlePrint() {
    window?.print();
}
</script>

<style></style>
