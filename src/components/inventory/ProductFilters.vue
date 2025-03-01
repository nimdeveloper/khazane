<template>
    <div
        class="bg-glob-primary dark:bg-dark-glob-primary px-2 py-2.5 shadow transition-transform border-2 border-border-2 h-full flex flex-col rounded-2xl top-[5px] w-[calc(100dvw-20px)] md:w-96 lg:w-full absolute"
        :class="[
            {
                'translate-x-[110%]': !globalState.SidebarOpen && !lgAndLarger,
                'translate-0': globalState.SidebarOpen,
            },
            'lg:static lg:py-0 lg:h-full lg:shadow-none lg:px-0 lg:border-0',
        ]"
        ref="filters-sidebar-ref"
    >
        <div class="flex flex-row mt-2">
            <h2 class="text-xl">محصولات</h2>
            <div
                class="border border-border-1 rounded-2xl py-1 px-2 text-xs ms-2"
            >
                <span>{{ productStore.productsCount["all"] }}</span>
                <span class="text-secondary dark:text-dark-secondary ms-1"
                    >محصول</span
                >
            </div>
        </div>
        <div
            class="flex w-full rounded-2xl py-4 px-2 mt-4 bg-glob-secondary dark:bg-dark-glob-secondary flex-col overflow-hidden"
        >
            <Simplebar data-simplebar-direction="rtl" class="h-full pe-3">
                <div class="text-secondary dark:text-dark-secondary ps-1">
                    نوع محصولات
                </div>
                <div class="flex flex-wrap mt-3">
                    <div
                        v-for="(item, index) of productTypeChoices"
                        :key="index"
                        class="w-full lg:w-1/2 ps-1 pb-1"
                    >
                        <button
                            class="cursor-pointer rounded-2xl py-2 px-4 border border-border-2 w-full inline-flex items-center"
                            :class="{
                                'border-action-primary dark:border-dark-action-primary bg-action-secondary dark:bg-dark-action-secondary':
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
                                    'border-action-primary dark:border-dark-action-primary bg-action-primary/20 dark:bg-dark-action-primary/20 text-action-primary dark:text-dark-action-primary':
                                        (productStore.filters.status ||
                                            'all') === item.value,
                                    'bg-secondary/20 dark:bg-dark-secondary/20':
                                        (productStore.filters.status ||
                                            'all') !== item.value,
                                }"
                            >
                                {{ productStore.productsCount[item.value] }}
                            </div>
                        </button>
                    </div>
                </div>
                <div class="mt-8 text-secondary dark:text-dark-secondary ps-1">
                    مرتب سازی
                </div>
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
                        :selected="
                            productStore.filters.category || defaultCategory
                        "
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
                        @change="
                            (val) => (productStore.filters.warehouse = val)
                        "
                        @toggled="(e) => !e && wareHouseStore.loadWareHouses()"
                    >
                        <template #icon>
                            <IconInventory :size="25" color="currentColor" />
                        </template>
                    </FormSelect>
                </div>
            </Simplebar>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { breakpointsTailwind, onClickOutside } from "@vueuse/core";
import Simplebar from "simplebar-vue";
import { ProductCategory, type IProductUnit } from "~/interfaces/product";
import { WareHouse } from "~/interfaces/warehouse";
import { useMyGlobalStore } from "~/stores/global";
import { useMyProductStore } from "~/stores/product";
import { useMyWarehouseStore } from "~/stores/warehouse";

const breakpoints = useBreakpoints(breakpointsTailwind);
const lgAndLarger = breakpoints.greaterOrEqual("lg");

const globalState = useMyGlobalStore();

const defaultInventory = ref(new WareHouse("", "همه"));
const defaultCategory = ref(new ProductCategory("", "همه"));

const productStore = useMyProductStore();
const wareHouseStore = useMyWarehouseStore();

const filtersRef = useTemplateRef("filters-sidebar-ref");

onClickOutside(
    filtersRef,
    () => {
        globalState.SidebarOpen = false;
    },
    {
        ignore: () =>
            Array.from(
                document.getElementsByClassName("sidebar-close-exception")
            ) as Array<HTMLElement>,
    }
);

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
