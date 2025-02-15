<template>
    <div class="flex w-full">
        <FormInput
            name="product_name"
            label="موجودی اولیه"
            v-model:value="product.initialInventory"
            class="grow-1"
        />
    </div>
    <div class="flex w-full mt-3">
        <FormInput
            name="product_name"
            label="موجودی کل"
            v-model:value="product.inventory"
            class="grow-1"
        />
    </div>
    <div
        class="flex w-full gap-4 my-3 flex-col md:flex-row border-t-1 pt-3 mt-5 md:mt-3 md:pt-0 md:border-0"
        v-for="(item, index) of product.ware_houses"
    >
        <div class="grow-1 md:w-1/2">
            <FormSelect
                :options="wareHouseStore.warehouses"
                :selected="item.warehouse"
                :searchable="true"
                :addable="true"
                @toggled="(e) => !e && wareHouseStore.loadWareHouses()"
                @change="(e) => (item.warehouse = e)"
                @add="(name: string) => newWarehouseModalOpen = { open: true, preferred_name: name }"
                :name="`product_warehouse_${index}_warehouse`"
                label="انبار"
                class="grow-1"
            />
        </div>
        <div class="grow-1 md:w-1/2">
            <FormCounterInput
                :name="`product_warehouse_${index}_quantity`"
                label="موجودی"
                v-model:value="item.quantity"
                :max="Number(remainingInventory) + item.quantity"
                :min="0"
                class="grow-1"
            />
        </div>
        <button
            @click.prevent="handleRemoveWareHouse(index)"
            class="mt-auto mx-auto mb-2 text-red-400 hover:bg-red-400/20 cursor-pointer p-1 rounded-xl w-full md:w-auto text-center justify-center flex items-center"
        >
            <IconTrash :size="25" color="currentColor" />
            <div class="ms-1 md:hidden">حذف</div>
        </button>
    </div>
    <div>
        <button
            @click.prevent="handleNewWareHouse"
            class="cursor-pointer text-primary hover:bg-primary/10 p-3 rounded-xl"
        >
            <IconPlus
                :size="12"
                color="currentColor"
                class="inline me-2"
            />افزودن انبار
        </button>
    </div>
</template>

<script lang="ts" setup>
import { ProductUnit } from "~/interfaces/product";
import { useMyWarehouseStore } from "~/stores/warehouse";

definePageMeta({ layout: "new-product" });

const wareHouseStore = useMyWarehouseStore();

const newWarehouseModalOpen = inject<{ open: boolean; preferred_name: string }>(
    "newWarehouseModalOpen"
);
const product = ref(new ProductUnit(""));

const { storage } = useTempProduct();

const remainingInventory = computed(() => {
    if (product.value) {
        const used = product.value.ware_houses.reduce((prev, current) => {
            return current.quantity + prev;
        }, 0);
        return product.value.inventory - used;
    }
    return 0;
});
watch(product, (newVal) => {
    if (newVal.ware_houses.length < 1) {
        newVal.ware_houses = Array.from([
            {
                quantity: 0,
                warehouse: null,
            },
        ]);
    }
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
onMounted(async () => {
    if (storage.value) {
        product.value = storage.value;
    }
});
const handleNewWareHouse = () => {
    product.value.ware_houses.push({
        warehouse: null,
        quantity: 0,
    });
};
const handleRemoveWareHouse = (index: number) => {
    product.value.ware_houses.splice(index, 1);
};
</script>

<style></style>
