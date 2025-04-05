<template>
    <div
        class="flex bg-action-secondary dark:bg-dark-action-secondary rounded-2xl p-1.5 my-2.5 max-w-2xl grow-1 min-w-80 print:hidden"
    >
        <div
            class="flex grow-1 px-2 relative"
            :class="{ ' ps-6': status === 'draft' }"
        >
            <div
                class="inline text-center text-sm bg-action-primary absolute -right-1.5 -top-1.5 -bottom-1.5 px-1 py-2 rounded-r-lg"
                style="writing-mode: vertical-rl"
                v-if="status === 'draft'"
            >
                پیش نویس
            </div>
            <div
                class="flex flex-col items-start justify-around ps-3.5 print:hidden"
            >
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
                        class="bg-blue-100 text-secondary dark:text-dark-glob-secondary font-medium px-2.5 py-0.5 rounded-md dark:bg-blue-400 h-5 text-nowrap"
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
            <div
                class="p-2 flex ms-auto ps-4 relative print:hidden"
                ref="order-item-popup"
            >
                <button
                    class="m-auto rounded-xl border-secondary dark:border-dark-secondary border-2 p-1.5 text-secondary dark:text-dark-secondary hover:text-amber-50 hover:border-amber-50 transition-colors cursor-pointer"
                    @click.stop.prevent="menuOpen = !menuOpen"
                >
                    <IconThreeDots :size="18" color="currentColor" />
                </button>
                <div
                    class="absolute end-0 top-full bg- max-w-[90dvw] w-36 bg-glob-secondary dark:bg-dark-glob-secondary p-2 rounded-2xl z-10"
                    :class="{ hidden: !menuOpen }"
                >
                    <button
                        class="p-2 cursor-pointer w-full text-start hover:dark:bg-dark-glob-dark hover:bg-glob-dark rounded-xl"
                        @click.prevent.stop="handlePrint()"
                    >
                        پرینت
                    </button>
                    <button
                        class="p-2 cursor-pointer w-full text-start hover:dark:bg-dark-glob-dark hover:bg-glob-dark rounded-xl block"
                        @click.prevent.stop="handleComplete"
                    >
                        تکمیل
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import type { Order } from "~/interfaces/order";
import { instanceOfIWareHouse, WareHouse } from "~/interfaces/warehouse";

const orderItemDropdown = useTemplateRef("order-item-popup");
const { storage } = useTempOrder();
const emit = defineEmits(["print"]);

onClickOutside(orderItemDropdown, () => {
    menuOpen.value = false;
});
const menuOpen = ref(false);

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
const { order } = defineProps<{
    order: Order;
}>();
const {
    document_date,
    document_number,
    goods,
    delivery,
    recipient,
    key,
    manager,
    type,
    status,
} = order;
const handlePrint = () => {
    emit("print");
};
const handleComplete = () => {
    storage.value = order;
    navigateTo({ name: "order-new-order-id-public", params: { id: key } });
};
</script>

<style lang="scss"></style>
