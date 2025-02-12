<template>
    <div class="flex flex-row mt-2">
        <h2 class="text-xl">محصولات</h2>
        <div class="border border-border-1 rounded-2xl py-1 px-2 text-xs ms-2">
            <span>{{ productStore.productsCount["all"] }}</span>
            <span class="text-text-secondary ms-1">محصول</span>
        </div>
    </div>
    <div
        class="flex w-full rounded-2xl py-4 px-2 mt-4 bg-secondary-bg flex-col grow overflow-hidden"
    >
        <Simplebar data-simplebar-direction="rtl" class="h-full pe-3">
            <div class="text-text-secondary ps-1">نوع محصولات</div>
            <div class="flex flex-wrap mt-3">
                <div
                    v-for="(item, index) of productTypeChoices"
                    :key="index"
                    class="w-full lg:w-1/2 ps-1 pb-1"
                >
                    <button
                        class="cursor-pointer rounded-2xl py-2 px-4 border border-border-2 w-full inline-flex items-center"
                        :class="{
                            'border-primary bg-active-item-bg':
                                (productStore.filters.status || 'all') ===
                                item.value,
                        }"
                        @click="changeProductTypeFilter(item.value)"
                    >
                        <div
                            class="text-nowrap text-ellipsis overflow-hidden text-sm"
                        >
                            {{ item.label }}
                        </div>
                        <div
                            class="rounded-lg py-1 ms-auto text-sm px-2"
                            :class="{
                                'border-primary bg-primary/20 text-primary':
                                    (productStore.filters.status || 'all') ===
                                    item.value,
                                'bg-text-secondary/20':
                                    (productStore.filters.status || 'all') !==
                                    item.value,
                            }"
                        >
                            {{ productStore.productsCount[item.value] }}
                        </div>
                    </button>
                </div>
            </div>
            <div class="mt-8 text-text-secondary ps-1">مرتب سازی</div>
            <div class="ps-1 mt-3">
                <FormSelect
                    :options="sortChoices"
                    :selected="productStore.filters.sort || sortChoices[0]"
                    @change="(val) => (productStore.filters.sort = val)"
                >
                    <template #icon>
                        <IconSort :size="25" color="currentColor" />
                    </template>
                </FormSelect>
            </div>
            <div class="mt-8">دسته بندی</div>
            <div class="ps-1 mt-3">
                <FormSelect
                    :options="categoryChoices"
                    :selected="productStore.filters.category || defaultCategory"
                    @change="(val) => (productStore.filters.category = val)"
                    @toggled="(e) => !e && productStore.loadCategories()"
                >
                    <template #icon>
                        <IconCategory :size="25" color="currentColor" />
                    </template>
                </FormSelect>
            </div>
            <div class="mt-8">انبار</div>
            <div class="ps-1 mt-3">
                <FormSelect
                    :options="inventoryChoices"
                    :selected="
                        productStore.filters.warehouse || defaultInventory
                    "
                    @change="(val) => (productStore.filters.warehouse = val)"
                    @toggled="(e) => !e && wareHouseStore.loadWareHouses()"
                >
                    <template #icon>
                        <IconInventory :size="25" color="currentColor" />
                    </template>
                </FormSelect>
            </div>
        </Simplebar>
    </div>
</template>

<script lang="ts" setup>
import Simplebar from "simplebar-vue";
import { ProductCategory, type IProductUnit } from "~/interfaces/product";
import { WareHouse } from "~/interfaces/warehouse";
import { useMyProductStore } from "~/stores/product";
import { useMyWarehouseStore } from "~/stores/warehouse";

const defaultInventory = ref(new WareHouse("", "همه"));
const defaultCategory = ref(new ProductCategory("", "همه"));

const productStore = useMyProductStore();
const wareHouseStore = useMyWarehouseStore();

// Filter data
const sortChoices = [
    {
        prefix: "الفبایی",
        label: "آ-ی",
        value: "alphabetical-des",
        sorter: (a: IProductUnit, b: IProductUnit) =>
            -b.title.localeCompare(a.title),
    },
    {
        prefix: "الفبایی",
        label: "ی-آ",
        value: "alphabetical-asc",
        sorter: (a: IProductUnit, b: IProductUnit) =>
            b.title.localeCompare(a.title),
    },
];
const categoryChoices = computed(() => [
    defaultCategory.value,
    ...productStore.categories,
]);
const inventoryChoices = computed(() => [
    defaultInventory.value,
    ...wareHouseStore.warehouses,
]);
const productTypeChoices = [
    {
        label: "همه",
        value: "all",
    },
    {
        label: "فعال",
        value: "active",
    },
    {
        label: "غیر فعال",
        value: "inactive",
    },
    {
        label: "پیش نویس",
        value: "draft",
    },
] as { label: string; value: "all" | "draft" | "inactive" | "active" }[];

// Filters

const totalProducts = ref(1000);

// Function
function changeProductTypeFilter(val: string) {
    productStore.filters.status = val;
}

onMounted(() => {
    productStore.filters.sort = sortChoices[0];
    productStore.loadCategories();
    wareHouseStore.loadWareHouses();
});
</script>

<style></style>
