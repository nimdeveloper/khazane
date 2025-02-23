<template>
    <div
        class="h-dvh bg-secondary-bg/30 absolute left-0 top-0 md:hidden w-dvw z-[7] print:hidden"
        :class="{ hidden: !menuOpen }"
    ></div>
    <div
        class="w-full flex justify-between align-middle content-between items-start relative z-[30] print:hidden"
    >
        <div class="inline-flex items-center">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                id="Layer_1"
                data-name="Layer 1"
                viewBox="0 0 24 24"
                width="40px"
                height="40px"
                class="text-primary h-10"
            >
                <path
                    d="M12.016,5.731L3.025,2.724,10.427,.257c1.026-.342,2.136-.342,3.162,0l7.419,2.473-8.992,3.001Zm-3.182,5.623l2.175-3.624L3.063,5.081c-.617-.206-1.293,.045-1.628,.602L.308,7.563c-.667,1.112-.133,2.555,1.097,2.965l5.081,1.694c.889,.296,1.865-.065,2.347-.868ZM3.008,2.718v.011l.017-.006-.017-.006ZM11.008,23.905V11.617l-.46,.766c-.742,1.236-2.044,1.945-3.415,1.945-.425,0-.856-.067-1.28-.209l-3.845-1.281v4.558c0,2.152,1.377,4.063,3.419,4.743l4.435,1.478c.374,.121,.758,.22,1.146,.287ZM23.728,7.596l-1.148-1.913c-.334-.557-1.011-.808-1.628-.602l-7.945,2.648,2.175,3.624c.482,.804,1.458,1.165,2.347,.868l5.118-1.706c1.211-.404,1.737-1.825,1.08-2.92Zm-6.845,6.733c-1.371,0-2.673-.708-3.415-1.945l-.46-.766v12.282c.422-.074,.84-.182,1.236-.314h.01l4.335-1.446c2.042-.681,3.419-2.591,3.419-4.743v-4.559l-3.845,1.282c-.424,.142-.855,.209-1.28,.209Z"
                    fill="currentColor"
                />
            </svg>
            <h1 class="font-bold text-xl ms-1">خزانه</h1>
            <button
                class="cursor-pointer p-2 ms-3 border-2 rounded-xl transition-colors sidebar-close-exception"
                :class="{
                    'text-primary border-primary':
                        globalState.SidebarOpen && !lgAndLarger,
                    'text-text-secondary border-border-1':
                        !globalState.SidebarOpen && !lgAndLarger,
                    hidden: lgAndLarger,
                }"
                @click.stop.prevent="handleFilterButtonClick"
                v-if="globalState.HasFilter"
            >
                <IconFilter :size="16" color="currentColor" />
            </button>
        </div>
        <button
            class="cursor-pointer p-2 ms-3 border-2 rounded-xl transition-colors md:hidden"
            :class="{
                'text-primary': menuOpen,
                'text-text-secondary': !menuOpen,
            }"
            @click.prevent="menuToggle"
        >
            <IconMenu :size="24" color="currentColor" />
        </button>
        <div
            class="flex justify-between align-middle content-between items-center text-sm absolute top-full w-full flex-col bg-secondary-bg z-20 rounded-xl translate-y-3 gap-2 overflow-hidden transition-all"
            :class="[
                {
                    'max-h-dvh py-2 px-2': menuOpen,
                    'max-h-0 p-0': !menuOpen,
                },
                'md:static md:top-0 md:w-[unset] md:flex-row md:bg-transparent md:gap-0 md:max-h-[unset] md:p-0 md:translate-0',
            ]"
        >
            <NuxtLink
                v-for="(item, id) in routes"
                :key="id"
                :to="item.path"
                class="cursor-pointer inline-flex rounded-2xl px-4 py-5 w-full justify-start gap-2 transition-colors duration-300"
                :class="[
                    {
                        'bg-active-item-bg text-primary md:shadow-lg':
                            item.active,
                        'text-text-secondary': !item.active,
                    },
                    'md:w-[unset] md:rounded-4xl md:px-4.5 md:py-2 md:mx-1 md:justify-around md:items-center md:gap-0',
                    'hover:bg-active-item-bg md:hover:shadow-lg',
                ]"
            >
                <component :is="item.icon" :size="20" color="currentColor" />
                <span
                    class="ms-1"
                    :class="{ 'text-text-primary': item.active }"
                    >{{ item.title }}</span
                >
            </NuxtLink>
        </div>
    </div>
</template>
<script setup lang="ts">
import {
    IconFileReport,
    IconFolder,
    IconGauge,
    IconMenu,
    IconTransferVan,
    IconWarehouse,
} from "#components";
import { useMyGlobalStore } from "~/stores/global";
import { breakpointsTailwind } from "@vueuse/core";

const breakpoints = useBreakpoints(breakpointsTailwind);
const lgAndLarger = breakpoints.greaterOrEqual("lg");
const mdAndLarger = breakpoints.greaterOrEqual("md");

const globalState = useMyGlobalStore();
const { toggleSidebar } = globalState;
const route = useRoute();
const routes = ref([
    {
        icon: shallowRef(IconGauge),
        title: "داشبورد",
        active: true,
        path: "/",
    },
    {
        icon: shallowRef(IconWarehouse),
        title: "انبار",
        active: false,
        path: "/inventory",
    },
    {
        icon: shallowRef(IconTransferVan),
        title: "گردش انبار",
        active: false,
        path: "/order",
    },
    {
        icon: shallowRef(IconFileReport),
        title: "گزارش",
        active: false,
        path: "/reports",
    },
    {
        icon: shallowRef(IconFolder),
        title: "پرونده ها",
        active: false,
        path: "/documents",
    },
]);

const menuOpen = ref(false);

function menuToggle() {
    menuOpen.value = !menuOpen.value;
}
function updateActiveStatus() {
    routes.value.forEach((each, index) => {
        if (each.path === "/") {
            if (route.path === each.path) {
                each.active = true;
            } else {
                each.active = false;
            }
        } else if (route.path.startsWith(each.path)) {
            each.active = true;
        } else {
            each.active = false;
        }
    });
}
watch(route, () => {
    updateActiveStatus();
});
onBeforeMount(() => {
    updateActiveStatus();
});

const handleFilterButtonClick = () => {
    toggleSidebar();
};
</script>
