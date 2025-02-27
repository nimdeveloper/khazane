<template>
    <div
        class="h-dvh bg-glob-secondary/30 dark:bg-dark-glob-secondary/30 absolute left-0 top-0 lg:hidden w-dvw -z-[1] print:hidden"
        :class="{ hidden: !menuOpen }"
    ></div>
    <button
        class="cursor-pointer p-2 rounded-xl transition-colors lg:hidden ms-auto bg-action-secondary dark:bg-dark-action-secondary"
        :class="{
            'text-action-primary dark:text-dark-action-primary': menuOpen,
            'text-secondary dark:text-dark-secondary': !menuOpen,
        }"
        @click.stop.prevent="menuOpen = !menuOpen"
    >
        <IconMenu :size="24" color="currentColor" />
    </button>
    <div
        ref="menu_holder"
        class="flex justify-between align-middle content-between items-center text-sm absolute top-full w-full flex-col bg-glob-secondary dark:bg-dark-glob-secondary z-20 rounded-xl translate-y-3 gap-2 overflow-hidden transition-all ms-auto"
        :class="[
            {
                'max-h-dvh py-2 px-2': menuOpen,
                'max-h-0 p-0': !menuOpen,
            },
            'lg:static lg:top-0 lg:w-[unset] lg:flex-row lg:bg-[transparent!important] lg:gap-0 lg:max-h-[unset] lg:p-0 lg:translate-0',
        ]"
    >
        <NuxtLink
            v-for="(item, id) in routes"
            :key="id"
            :to="item.path"
            class="cursor-pointer inline-flex rounded-2xl px-4 py-5 w-full justify-start gap-2 transition-colors duration-300"
            :class="[
                {
                    'bg-action-secondary dark:bg-dark-action-secondary text-action-primary dark:text-dark-action-primary lg:shadow-lg':
                        item.active,
                    'text-secondary dark:text-dark-secondary': !item.active,
                },
                'lg:w-[unset] lg:rounded-4xl lg:px-4.5 lg:py-2 lg:mx-1 lg:justify-around lg:items-center lg:gap-0',
                'hover:bg-action-secondary hover:dark:bg-dark-action-secondary lg:hover:shadow-lg',
            ]"
        >
            <component :is="item.icon" :size="20" color="currentColor" />
            <span
                class="ms-1"
                :class="{
                    'text-primary dark:text-dark-primary': item.active,
                }"
                >{{ item.title }}</span
            >
        </NuxtLink>
    </div>
</template>

<script lang="ts" setup>
import { onClickOutside } from "@vueuse/core";
import {
    IconFileReport,
    IconFolder,
    IconGauge,
    IconMenu,
    IconTransferVan,
    IconWarehouse,
} from "#components";

const route = useRoute();

const menuOpen = ref(false);
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
const menuHolder = useTemplateRef("menu_holder");

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

onClickOutside(
    menuHolder,
    () => {
        if (menuOpen.value) {
            menuOpen.value = false;
        }
    },
    {
        ignore: () =>
            menuHolder.value && menuHolder.value.previousSibling
                ? [menuHolder.value.previousSibling as HTMLElement]
                : [],
    }
);
</script>

<style lang="scss"></style>
