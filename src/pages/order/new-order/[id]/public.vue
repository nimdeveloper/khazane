<template>
    <div class="flex w-full gap-4 my-3">
        <div class="grow-1 w-1/2">
            <FormSelect
                :options="selectableItems"
                :selected="order.delivery"
                :searchable="true"
                :addable="false"
                @change="(e) => (order.delivery = e)"
                @toggled="
                    (e) => {
                        if (!e) {
                            warehouseStore.loadWareHouses();
                            personStore.loadPersons();
                        }
                    }
                "
                name="order_delivery"
                label="تحویل دهنده"
                class="grow-1"
            />
        </div>
        <div class="grow-1 w-1/2">
            <FormSelect
                :options="selectableItems"
                :selected="order.recipient"
                :searchable="true"
                :addable="true"
                @change="(e) => (order.recipient = e)"
                @toggled="
                    (e) => {
                        if (!e) {
                            warehouseStore.loadWareHouses();
                            personStore.loadPersons();
                        }
                    }
                "
                name="order_recipient"
                label="تحویل گیرنده"
                class="grow-1"
            />
        </div>
    </div>
    <div class="flex w-full mt-3 gap-4 my-3">
        <div class="grow-1 w-1/2">
            <FormInput
                name="order_type"
                label="نوع استناد"
                v-model:value="order.type"
                class="grow-1"
            />
        </div>
        <div class="grow-1 w-1/2">
            <FormInput
                name="order_document_number"
                label="شماره استناد"
                v-model:value="order.document_number"
                class="grow-1"
            />
        </div>
    </div>
    <div class="flex w-full mt-3 gap-4 my-3">
        <div class="grow-1 w-1/2">
            <FormDateInput
                name="order_document_date"
                label="تاریخ استناد"
                v-model:value="order.document_date"
                class="grow-1"
            />
        </div>
        <div class="grow-1 w-1/2">
            <FormSelect
                :options="personStore.persons"
                :selected="order.manager"
                :searchable="true"
                :addable="true"
                @change="(e) => (order.manager = e)"
                @toggled="(e) => !e && personStore.loadPersons()"
                @add="
                    (e) =>
                        (newUserModalOpen = { open: true, preferred_name: e })
                "
                name="order_manager"
                label="رابط"
                class="grow-1"
            />
        </div>
    </div>
    <div class="flex w-full mt-3 gap-4 my-3">
        <div class="grow-1">
            <FormTextArea
                name="order_description"
                label="توضیحات"
                v-model:value="order.description"
                class="grow-1"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { Order } from "~/interfaces/order";
import type { Person } from "~/interfaces/person";
import type { WareHouse } from "~/interfaces/warehouse";
import { useMyPersonStore } from "~/stores/person";
import { useMyWarehouseStore } from "~/stores/warehouse";

definePageMeta({ layout: "new-order" });

const personStore = useMyPersonStore();
const warehouseStore = useMyWarehouseStore();

const order = ref(new Order(""));

const newUserModalOpen = inject<{ open: boolean; preferred_name: string }>(
    "newUserModalOpen"
);

const { storage } = useTempOrder();

const selectableItems = computed(() => {
    const { persons } = personStore;
    const { warehouses } = warehouseStore;
    return (<(WareHouse | Person)[]>[]).concat(persons, warehouses);
});
watch(order, (newVal) => {
    storage.value = newVal;
});
onMounted(() => {
    personStore.loadPersons();
    warehouseStore.loadWareHouses();
    if (storage.value) {
        order.value = storage.value;
    }
});
</script>

<style></style>
