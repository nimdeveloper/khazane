<template>
    <div
        class="flex bg-action-secondary dark:bg-dark-action-secondary rounded-2xl p-1.5 my-2.5 max-w-2xl grow-1 min-w-96"
    >
        <div class="flex grow-1 px-2">
            <div class="flex flex-col items-start justify-around ps-3.5">
                <div class="flex">
                    {{ display_key }}
                    <div
                        class="h-2/3 w-0.5 bg-secondary/10 dark:bg-dark-secondary/10 mx-2 md:mx-3 my-auto hidden sm:flex lg:hidden"
                    ></div>
                    <div
                        class="justify-start gap-5 grow-1 hidden sm:flex lg:hidden text-xs"
                    >
                        {{ (delivery as any)?.label }} به
                        {{ (recipient as any)?.label }}
                    </div>
                </div>
                <div
                    class="flex items-center align-middle text-secondary dark:text-dark-secondary text-xs"
                >
                    <span
                        v-if="goods?.length > 0"
                        class="bg-blue-100 text-secondary dark:text-dark-secondary font-medium px-2.5 py-0.5 rounded-md dark:bg-blue-400 h-5 text-nowrap"
                    >
                        {{ goods?.length }} کالا </span
                    >&nbsp;<span class="text-2xl pb-2" v-if="goods?.length > 0"
                        >.</span
                    >&nbsp;<span class="text-nowrap">{{
                        manager?.full_name || "-"
                    }}</span
                    >&nbsp;<span class="text-2xl pb-2">.</span
                    >&nbsp;<IconBookmarkOpen
                        :size="16"
                        color="currentColor"
                    />&nbsp; <span>ش.استناد:</span>&nbsp;<span
                        class="text-primary dark:text-dark-primary text-nowrap"
                        >{{ document_number || "-" }}</span
                    >
                </div>
            </div>
            <div class="p-2 flex ms-auto ps-4">
                <button
                    class="m-auto rounded-xl border-secondary dark:border-dark-secondary border-2 p-1.5 text-secondary dark:text-dark-secondary hover:text-amber-50 hover:border-amber-50 transition-colors cursor-pointer"
                >
                    <IconThreeDots :size="18" color="currentColor" />
                </button>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import type { IOrderProduct } from "~/interfaces/order";
import type { IPerson } from "~/interfaces/person";
import {
    instanceOfIWareHouse,
    WareHouse,
    type IWareHouse,
} from "~/interfaces/warehouse";

const display_key = computed(() => {
    let res = "";
    if (delivery instanceof WareHouse || instanceOfIWareHouse(delivery)) {
        res += "W";
    } else {
        res += "P";
    }
    if (recipient instanceof WareHouse || instanceOfIWareHouse(recipient)) {
        res += "W";
    } else {
        res += "P";
    }
    res += "-";
    res += key;
    return res;
});
const {
    document_date,
    document_number,
    goods,
    delivery,
    recipient,
    order_key: key,
    manager,
    type,
} = defineProps<{
    document_date?: string;
    document_number?: string;
    goods: IOrderProduct[];
    delivery: IPerson | IWareHouse | null;
    recipient: IPerson | IWareHouse | null;
    order_key: string;
    manager: IPerson | null;
    type?: string;
}>();
</script>

<style lang="scss"></style>
