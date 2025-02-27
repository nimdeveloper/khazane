<template>
    <div
        class="flex w-full gap-4 my-3 flex-col md:flex-row border-t-1 pt-3 mt-5 md:mt-3 md:pt-0 md:border-0"
        v-for="(item, index) of order.approvers"
    >
        <div class="grow-1 md:w-1/2">
            <FormSelect
                :options="personStore.persons"
                :selected="item.person"
                :searchable="true"
                :addable="true"
                @toggled="(e) => !e && personStore.loadPersons()"
                @change="(e) => (item.person = e)"
                @add="(name: string) => newUserModalOpen = { open: true, preferred_name: name }"
                :name="`order_approvers_${index}_person`"
                label="کاربر"
                class="grow-1"
            />
        </div>
        <div class="grow-1 md:w-1/2">
            <FormInput
                :name="`order_approvers_${index}_name`"
                label="مسئولیت  "
                v-model:value="item.name"
                class="grow-1"
            />
        </div>
        <button
            @click.prevent="handleRemoveUser(index)"
            class="mt-auto mx-auto mb-2 text-red-400 hover:bg-red-400/20 cursor-pointer p-1 rounded-xl w-full md:w-auto text-center justify-center flex items-center"
        >
            <IconTrash :size="25" color="currentColor" />
            <div class="ms-1 md:hidden">حذف</div>
        </button>
    </div>
    <div>
        <button
            @click.prevent="handleNewUser"
            class="cursor-pointer text-action-primary dark:text-dark-action-primary hover:bg-action-primary/10 hover:dark:bg-dark-action-primary/10 p-3 rounded-xl"
        >
            <IconPlus
                :size="12"
                color="currentColor"
                class="inline me-2"
            />افزودن
        </button>
    </div>
</template>

<script lang="ts" setup>
import { Order } from "~/interfaces/order";
import { useMyPersonStore } from "~/stores/person";

definePageMeta({ layout: "new-order" });

const personStore = useMyPersonStore();

const newUserModalOpen = inject<{ open: boolean; preferred_name: string }>(
    "newUserModalOpen"
);

const order = ref(new Order(""));

const { storage } = useTempOrder();

watch(order, (newVal) => {
    storage.value = newVal;
});

onMounted(async () => {
    if (storage.value) {
        order.value = storage.value;
    }
});

const handleNewUser = () => {
    order.value.approvers.push({
        name: "",
        person: null,
    });
};
const handleRemoveUser = (index: number) => {
    order.value.approvers.splice(index, 1);
};
</script>

<style></style>
