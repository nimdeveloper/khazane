<template>
    <div class="w-full flex relative">
        <div
            class="h-[calc(100dvh-90px)] z-20 relative max-w-lg"
            :style="{
                width: lgAndLarger ? `${sidebarWidth}%` : '0',
            }"
        >
            <OrderFilters />
        </div>
        <div
            class="z-10 bg-action-secondary/30 dark:bg-dark-action-secondary/30 absolute w-full h-full top-0 left-0"
            :class="{
                hidden: !globalState.SidebarOpen || lgAndLarger,
            }"
        ></div>

        <div
            class="p-4 flex flex-col flex-1"
            :style="{ width: lgAndLarger ? `${100 - sidebarWidth}%` : '100%' }"
        >
            <div class="flex w-full flex-col md:flex-row">
                <div
                    class="grow-1 bg-glob-dark dark:bg-dark-glob-dark rounded-2xl md:me-2 justify-between flex items-center px-3 py-2 relative text-primary dark:text-dark-primary"
                >
                    <IconMagnify
                        :size="20"
                        color="currentColor"
                        class="absolute start-5 top-1/2 -translate-y-1/2 rotate-90"
                    />
                    <input
                        class="w-full bg-transparent border-0 ps-10 outline-0 shadow-[none] py-1"
                        type="text"
                        placeholder="جستجو"
                    />
                    <div
                        class="h-2/3 w-0.5 bg-secondary/10 dark:bg-dark-secondary/10 mx-1 my-auto"
                        v-if="scanSupported"
                    ></div>
                    <button
                        class="inline-flex text-xs align-middle items-center cursor-pointer active:bg-action-secondary/50 active:dark:bg-dark-action-secondary/50 h-full px-2 rounded transition-all"
                        v-if="scanSupported"
                    >
                        <IconScan
                            :size="25"
                            color="currentColor"
                            class="text-action-primary dark:text-dark-action-primary me-1.5"
                        />
                        اسکن
                    </button>
                </div>
                <div class="flex my-3 md:my-0 items-center">
                    <div
                        class="inline-flex rounded-xl bg-action-secondary dark:bg-dark-action-secondary justify-between grow-0"
                    >
                        <button
                            class="bg-action-secondary dark:bg-dark-action-secondary p-2 border-2 text-2xl cursor-pointer rounded-xl transition-colors w-11 h-11 my-auto"
                            :class="{
                                'border-action-primary dark:border-dark-action-primary text-action-primary dark:text-dark-action-primary':
                                    activeProductsView === 'list',
                                'border-transparent text-secondary dark:text-dark-secondary':
                                    activeProductsView !== 'list',
                            }"
                            @click.prevent="activeProductsView = 'list'"
                        >
                            <IconList
                                :size="18"
                                color="currentColor"
                                class="m-auto"
                            />
                        </button>
                        <button
                            class="bg-action-secondary dark:bg-dark-action-secondary px-3 py-2 border-2 text-2xl cursor-pointer rounded-xl transition-colors w-11 h-11 my-auto"
                            :class="{
                                'border-action-primary dark:border-dark-action-primary text-action-primary dark:text-dark-action-primary':
                                    activeProductsView === 'grid',
                                'border-transparent text-secondary dark:text-dark-secondary':
                                    activeProductsView !== 'grid',
                            }"
                            @click.prevent="activeProductsView = 'grid'"
                        >
                            <IconGrid
                                :size="14"
                                color="currentColor"
                                class="m-auto"
                            />
                        </button>
                    </div>
                    <div class="self-stretch flex">
                        <div
                            class="h-2/3 w-0.5 bg-secondary/30 dark:bg-dark-secondary/30 mx-2 my-auto"
                        ></div>
                    </div>
                    <div class="inline-flex self-stretch">
                        <button
                            class="rounded-xl bg-action-secondary dark:bg-dark-action-secondary py-1.5 px-2 text-secondary dark:text-dark-secondary hover:text-amber-50 hover:border-amber-50 transition-colors cursor-pointer min-w-full min-h-full"
                        >
                            <IconThreeDots :size="28" color="currentColor" />
                        </button>
                    </div>
                    <div class="grow-1 ms-2 text-nowrap">
                        <NuxtLink
                            :to="{ name: 'order-new-order' }"
                            class="bg-action-primary dark:bg-dark-action-primary rounded-2xl px-4 py-2.5 text-glob-dark dark:text-dark-glob-dark cursor-pointer hover:bg-action-primary/80 hover:dark:bg-dark-action-primary/80 font-bold min-w-full inline-block text-center"
                        >
                            گردش جدید
                        </NuxtLink>
                    </div>
                </div>
            </div>
            <InventoryOrderList
                class="max-h-[calc(100%-116px)] md:max-h-[calc(100%-48px)]"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { IconMagnify } from "#components";
import { breakpointsTailwind } from "@vueuse/core";
import { useMyGlobalStore } from "~/stores/global";

const globalState = useMyGlobalStore();

const breakpoints = useBreakpoints(breakpointsTailwind);
const lgAndLarger = breakpoints.greaterOrEqual("lg");

const sidebarWidth = ref(35);
const activeProductsView = ref<"list" | "grid">("list");
const scanSupported = ref<boolean>(true);

onMounted(() => {
    globalState.withFilter();
});
onBeforeUnmount(() => {
    globalState.noFilter();
});
</script>

<style></style>
