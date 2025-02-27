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
        </div>
        <div
            class="flex w-full rounded-2xl py-4 px-2 mt-4 bg-glob-secondary dark:bg-dark-glob-secondary flex-col overflow-hidden"
        >
            <Simplebar data-simplebar-direction="rtl" class="h-full pe-3">
                <div class="text-secondary dark:text-dark-secondary ps-1">
                    نوع محصولات
                </div>
                <div class="flex flex-wrap mt-3"></div>
            </Simplebar>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { breakpointsTailwind, onClickOutside } from "@vueuse/core";
import Simplebar from "simplebar-vue";
import { useMyGlobalStore } from "~/stores/global";

const breakpoints = useBreakpoints(breakpointsTailwind);
const lgAndLarger = breakpoints.greaterOrEqual("lg");

const globalState = useMyGlobalStore();

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

onMounted(() => {});
</script>

<style></style>
