<template>
    <Simplebar data-simplebar-direction="rtl" class="h-full w-full">
        <ProductDetail />
        <div
            class="min-h-[80dvh] flex flex-col items-center justify-center rounded-xl"
        >
            <h1
                class="text-5xl text-gray-900 dark:text-white font-bold mb-8 animate-pulse"
            >
                Coming Soon
            </h1>
            <p class="text-gray-900 dark:text-white text-lg mb-8">
                We're working hard to bring you something amazing. Stay tuned!
            </p>
        </div>
        داشبورد
        <div class="flex flex-row gap-2">
            <div
                class="rounded-xl bg-action-secondary dark:bg-dark-action-secondary p-4 flex flex-col w-full md:w-xl"
            >
                <div class="w-full">وضعیت انبار</div>
                <div class="flex flex-col sm:flex-row">
                    <div class="mx-auto">
                        <VueApexCharts
                            width="300"
                            :options="options"
                            :series="series"
                        ></VueApexCharts>
                    </div>
                    <div class="flex flex-col grow-1">
                        <div
                            v-for="(each, index) of data"
                            class="my-1.5"
                            :key="index"
                        >
                            <div
                                class="text-xs text-secondary dark:text-dark-secondary"
                            >
                                {{ each.label }}
                            </div>
                            <div
                                class="w-full bg-gray-200 rounded-full h-2 my-2 dark:bg-gray-700"
                            >
                                <div
                                    class="h-2 rounded-full transition-all"
                                    :style="{
                                        width: `${
                                            (each.value * 100) / totalProducts
                                        }%`,
                                        background: each.color,
                                    }"
                                ></div>
                            </div>
                            <div
                                class="text-primary dark:text-dark-primary text-sm"
                            >
                                {{ addCommas(each.value) }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div
                class="rounded-xl bg-action-secondary dark:bg-dark-action-secondary p-4 flex flex-col grow-1"
            >
                <div class="w-full">آخرین محصولات</div>
                <div class="flex flex-col sm:flex-row">
                    <div class="flex flex-col grow-1"></div>
                </div>
            </div>
        </div>
        <div>
            <SampleDashboard />
        </div>
    </Simplebar>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import VueApexCharts from "vue3-apexcharts";
import Simplebar from "simplebar-vue";
import { useMyOrderStore } from "~/stores/order";
import { useMyPersonStore } from "~/stores/person";
import { useMyProductStore } from "~/stores/product";
import { useMyWarehouseStore } from "~/stores/warehouse";
import type { ApexOptions } from "apexcharts";
import { breakpointsTailwind } from "@vueuse/core";
import { addCommas, numberToWords } from "@persian-tools/persian-tools";

const productStore = useMyProductStore();
const warehouseStore = useMyWarehouseStore();
const orderStore = useMyOrderStore();
const personStore = useMyPersonStore();

const SECONDS = 1000;
const MINUTES = 60 * SECONDS;

// useInterval(30 * MINUTES, {
//     async callback() {
//         await Promise.all([
//             productStore.loadProducts(),
//             productStore.loadCategories(),
//             warehouseStore.loadWareHouses(),
//             orderStore.loadOrders(),
//             personStore.loadPersons(),
//         ]);
//     },
//     immediate: true,
// });
onMounted(async () => {
    // setTimeout(() => {
    //     invoke("list_locations", {
    //         filters: { sort_by: "id", sort_order: "desc" },
    //     }).then((data) => console.log(data));
    // invoke("create_location", {
    //     location: { name: "test" },
    // }).then((data) => {
    //     console.log(data);
    //     invoke("list_locations", {
    //         filters: { sort_by: "id", sort_order: "desc" },
    //     }).then((data) => console.log(data));
    // });
    // }, 1000);
    // await Promise.all([
    //     productStore.loadProducts(),
    //     productStore.loadCategories(),
    //     warehouseStore.loadWareHouses(),
    //     orderStore.loadOrders(),
    //     personStore.loadPersons(),
    // ]);
});

const data = ref([
    { label: "موجودی بالا", value: 17, color: "#3adb7e" },
    { label: "کالا های رو به اتمام", value: 41, color: "#c4eb53" },
    { label: "موجودی کم", value: 55, color: "#f8ef33" },
    { label: "اتمام موجودی", value: 44, color: "#ed3e73" },
]);

const options = ref<ApexOptions>({
    dataLabels: {
        enabled: false,
    },
    legend: {
        show: false,
    },
    chart: {
        stacked: true,
        fontFamily: "Vazir",

        width: "300",
        type: "donut",
    },
    labels: data.value.map((e) => e.label),
    fill: {
        colors: data.value.map((e) => e.color),
        opacity: 1,
    },
    stroke: {
        show: true,
        curve: "straight",
        lineCap: "round",
        colors: ["transparent"],
        width: 2,
        dashArray: 0,
    },
    yaxis: {
        show: false,
    },
    responsive: [
        {
            breakpoint: breakpointsTailwind.lg,
            options: {
                chart: {
                    width: "300",
                },
            },
        },
        {
            breakpoint: breakpointsTailwind.md,
            options: {
                chart: {
                    width: "300",
                },
            },
        },
    ],
    plotOptions: {
        polarArea: {
            rings: {
                strokeWidth: 0,
            },
            spokes: {
                strokeWidth: 0,
            },
        },

        pie: {
            donut: {
                labels: {
                    show: true,
                    name: {
                        show: true,
                        offsetY: 25,
                        formatter: (val) =>
                            val.toLowerCase() === "total" ? "کل محصولات" : val,
                    },
                    value: {
                        show: true,
                        offsetY: -25,
                    },
                    total: {
                        show: true,
                    },
                },
            },
        },
    },
    theme: {
        monochrome: {
            enabled: true,
            shadeTo: "light",
            shadeIntensity: 0.6,
        },
    },
});

const series = computed(() => data.value.map((e) => e.value));
const totalProducts = computed(() => {
    return series.value.reduce((p, c) => p + c, 0);
});
</script>
