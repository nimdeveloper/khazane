<template>
    <div
        class="hidden flex-col text-right min-h-[calc(100dvh-10px)] print:flex text-xs border-2 px-2 py-1 rounded relative z-10"
    >
        <div
            class="absolute -z-10 w-screen left-1/2 top-1/2 -translate-1/2 text-center -rotate-45 opacity-[8%]"
            style="font-size: 8rem"
        >
            <div>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    id="Layer_1"
                    data-name="Layer 1"
                    viewBox="0 0 24 24"
                    width="144px"
                    height="144px"
                    class="h-36 inline"
                >
                    <path
                        d="M12.016,5.731L3.025,2.724,10.427,.257c1.026-.342,2.136-.342,3.162,0l7.419,2.473-8.992,3.001Zm-3.182,5.623l2.175-3.624L3.063,5.081c-.617-.206-1.293,.045-1.628,.602L.308,7.563c-.667,1.112-.133,2.555,1.097,2.965l5.081,1.694c.889,.296,1.865-.065,2.347-.868ZM3.008,2.718v.011l.017-.006-.017-.006ZM11.008,23.905V11.617l-.46,.766c-.742,1.236-2.044,1.945-3.415,1.945-.425,0-.856-.067-1.28-.209l-3.845-1.281v4.558c0,2.152,1.377,4.063,3.419,4.743l4.435,1.478c.374,.121,.758,.22,1.146,.287ZM23.728,7.596l-1.148-1.913c-.334-.557-1.011-.808-1.628-.602l-7.945,2.648,2.175,3.624c.482,.804,1.458,1.165,2.347,.868l5.118-1.706c1.211-.404,1.737-1.825,1.08-2.92Zm-6.845,6.733c-1.371,0-2.673-.708-3.415-1.945l-.46-.766v12.282c.422-.074,.84-.182,1.236-.314h.01l4.335-1.446c2.042-.681,3.419-2.591,3.419-4.743v-4.559l-3.845,1.282c-.424,.142-.855,.209-1.28,.209Z"
                        fill="currentColor"
                    />
                </svg>
                خزانه
            </div>
        </div>
        <div class="flex">
            <div class="grow-1">
                <h1 class="text-center indent-[35%] font-bold text-md">
                    سند جابجایی (حواله انبار)
                </h1>
                <div>
                    <h5 class="font-bold inline me-2">تحویل دهنده:</h5>
                    <div class="inline">{{ order.delivery?.name }}</div>
                </div>
                <div>
                    <h5 class="font-bold inline me-2">تحویل گیرنده:</h5>
                    <div class="inline">{{ order.recipient?.name }}</div>
                </div>
                <div class="flex gap-10 my-5 flex-wrap" style="row-gap: 0">
                    <div class="inline">
                        <div class="font-bold inline me-2">نوع استناد:</div>
                        <div class="inline">{{ order.type }}</div>
                    </div>
                    <div class="inline">
                        <div class="font-bold inline me-2">شماره استناد:</div>
                        <div class="inline">{{ order.document_number }}</div>
                    </div>
                    <div class="inline">
                        <div class="font-bold inline me-2">تاریخ استناد:</div>
                        <div class="inline">{{ order.document_date }}</div>
                    </div>
                    <div class="inline">
                        <div class="font-bold inline me-2">رابط:</div>
                        <div class="inline">{{ order.manager?.full_name }}</div>
                    </div>
                    <div class="inline">
                        <div class="font-bold inline me-2">
                            به کار گیرندگان:
                        </div>
                        <div class="inline">
                            {{
                                order.users.map((each) => each?.name).join("،")
                            }}
                        </div>
                    </div>
                </div>
            </div>
            <div class="w-[25%] m-1 max-w-72">
                <div class="border rounded w-full h-full p-1">
                    <div>شماره سند:</div>
                    <div>تاریخ سند:</div>
                    <div></div>
                </div>
            </div>
        </div>
        <table class="border border-border-1 w-full text-center mb-auto">
            <thead
                class="print:bg-dark-action-secondary/20"
                style="font-size: 10px"
            >
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
        <div class="border-dark-action-secondary border mt-5">
            <div class="min-h-18">
                <h5 class="font-bold inline me-2">توضیحات:</h5>
                <div class="inline">
                    {{ order.description }}
                </div>
            </div>
            <div class="flex">
                <div
                    class="text-center print:bg-dark-action-secondary/20 p-1 border-e border-s-dark-action-secondary"
                    style="writing-mode: vertical-rl"
                >
                    تایید کنندگان
                </div>
                <div class="flex flex-1 flex-wrap grow-1">
                    <div
                        v-for="item of order.approvers"
                        class="grow-1 text-nowrap min-w-40 text-center flex flex-col"
                    >
                        <div
                            class="border border-glob-primary dark:border-dark-glob-primary-bg border-e-0 min-h-4"
                            style="font-size: 10px"
                        >
                            {{ item.name }}
                        </div>
                        <div
                            class="border-s border-glob-primary dark:border-dark-glob-primary-bg h-8 grow-1"
                        >
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
import { useMeasureStore } from "~/stores/measure";
import { addCommas, numberToWords } from "@persian-tools/persian-tools";

definePageMeta({ layout: "new-order" });

const measurementStore = useMeasureStore();

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
