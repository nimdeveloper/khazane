<template>
    <NuxtLayout name="default">
        <WarehouseNewModal
            v-model:is-open="newWarehouseModalOpen.open"
            :preferred-name="newWarehouseModalOpen.preferred_name"
        />
        <div class="flex flex-col w-full">
            <NuxtLink
                class="inline-flex items-center mt-2 lg:mb-6 gap-0 hover:gap-1 transition-all lg:ms-4 text-secondary dark:text-dark-secondary"
                to="/inventory/"
            >
                <IconNavArrowRight
                    :size="20"
                    color="currentColor"
                    class="me-2"
                />
                باز گشت به لیست محصولات
            </NuxtLink>
            <div class="flex w-full">
                <div class="flex flex-col lg:flex-row w-full">
                    <ul
                        class="flex lg:block space-y space-y-4 grow-1 lg:grow-0 lg:w-96 lg:max-w-96 text-sm font-medium text-gray-500 dark:text-gray-400 lg:me-4 lg:mb-0 z-[2]"
                    >
                        <li
                            v-for="(step, index) of steps"
                            :key="index"
                            class="relative pt-3 lg:ps-6 lg:pt-0 grow"
                        >
                            <div
                                class="hidden sm:block absolute h-2 w-1/3 -start-1/2 -translate-x-full -translate-y-1/2 lg:w-9 bg-gradient-to-r lg:bg-action-primary lg:dark:bg-dark-action-primary lg:start-0 top-1/2 rounded-full transition-transform z-[1]"
                                :class="{
                                    'lg:translate-x-1/2': step.active,
                                    'lg:translate-x-full': !step.active,
                                    'hidden lg:block': index == 0,
                                    'bg-action-primary dark:bg-dark-action-primary':
                                        index !== 0 && step.active,
                                    'bg-glob-secondary dark:bg-dark-glob-secondary':
                                        index !== 0 && !step.active,
                                }"
                            ></div>
                            <NuxtLink
                                :to="step.link"
                                class="inline-flex flex-col lg:flex-row items-center px-4 lg:ps-4 lg:pe-6 py-3 rounded-lg w-full text-nowrap hover:bg-action-primary/10 hover:dark:bg-dark-action-primary/10 text-center lg:text-start z-[2] relative"
                                :class="{
                                    'bg-action-primary/5 dark:bg-dark-action-primary/5 text-action-primary dark:text-dark-action-primary':
                                        step.active,
                                    'text-primary dark:text-dark-primary':
                                        !step.active,
                                }"
                            >
                                <component
                                    :is="step.icon"
                                    :size="24"
                                    color="currentColor"
                                    class="me-2"
                                    :class="{
                                        'text-action-primary dark:text-dark-action-primary':
                                            step.active,
                                        'text-secondary dark:text-dark-secondary':
                                            !step.active,
                                    }"
                                />
                                <div class="flex flex-col">
                                    <div
                                        class="text-secondary dark:text-dark-secondary text-sm hidden lg:block"
                                    >
                                        مرحله {{ index + 1 }}
                                    </div>
                                    <div class="mt-1 text-xs lg:text-sm">
                                        {{ step.name }}
                                    </div>
                                </div>
                            </NuxtLink>
                        </li>
                    </ul>
                    <div
                        class="pt-6 ps-6 pe-3 pb-16 text-medium rounded-xl grow-1 bg-action-secondary dark:bg-dark-action-secondary h-[calc(100dvh-225px)] lg:h-[calc(100dvh-138px)] flex flex-col relative w-auto lg:w-[calc(100%-384px)]"
                    >
                        <Simplebar
                            data-simplebar-direction="rtl"
                            class="h-full pe-3"
                        >
                            <slot />
                        </Simplebar>
                        <div
                            class="absolute bottom-0 left-0 w-full h-16 py-3 pe-5 flex justify-end"
                        >
                            <button
                                @click="saveDraft"
                                type="button"
                                class="py-2.5 px-5 ms-auto text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 cursor-pointer"
                                :class="[
                                    'focus:z-50 focus:ring-4 focus:ring-gray-100 focus:outline-none',
                                    'dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700',
                                ]"
                            >
                                ذخیره پیش نویس
                            </button>
                            <button
                                v-if="hasPrevious"
                                type="button"
                                class="text-white bg-yellow-400 hover:bg-yellow-500 font-medium rounded-lg text-sm px-5 py-2.5 ms-2 cursor-pointer disabled:cursor-not-allowed disabled:grayscale-100"
                                :class="[
                                    'focus:outline-none focus:ring-4 focus:ring-yellow-300',
                                    'dark:focus:ring-yellow-900',
                                ]"
                                @click.prevent="handlePrevious"
                            >
                                قبلی
                            </button>
                            <button
                                v-if="hasNext"
                                type="button"
                                class="text-white bg-green-700 hover:bg-green-800 font-medium rounded-lg text-sm px-5 py-2.5 ms-2 cursor-pointer disabled:cursor-not-allowed disabled:grayscale-100"
                                :class="[
                                    'focus:outline-none focus:ring-4 focus:ring-green-300',
                                    'dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800',
                                ]"
                                @click.prevent="handleNext"
                            >
                                بعدی
                            </button>
                            <button
                                v-else-if="storage?.valid()"
                                type="button"
                                class="text-white bg-green-700 hover:bg-green-800 font-medium rounded-lg text-sm px-5 py-2.5 ms-2 cursor-pointer disabled:cursor-not-allowed disabled:grayscale-100"
                                :class="[
                                    'focus:outline-none focus:ring-4 focus:ring-green-300',
                                    'dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800',
                                ]"
                                @click.prevent="handleNext"
                            >
                                ذخیره
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </NuxtLayout>
</template>

<script lang="ts" setup>
import { useStorage } from "@vueuse/core";
import Simplebar from "simplebar-vue";
import { IconBox, IconDocument, IconRuler, IconTagFill } from "#components";
import { ProductUnit } from "~/interfaces/product";
import { useMyProductStore } from "~/stores/product";

const productStore = useMyProductStore();

const newWarehouseModalOpen = ref({ open: false, preferred_name: "" });

provide(/* key */ "newWarehouseModalOpen", /* value */ newWarehouseModalOpen);

const { storage } = useTempProduct();

const route = useRoute();
const steps = ref([
    {
        name: "اطلاعات عمومی",
        link: {
            name: "inventory-new-product-id-public",
            params: { id: "-1" },
        },
        icon: shallowRef(IconDocument),
        active: false,
    },
    {
        name: "اطلاعات مالی",
        link: { name: "inventory-new-product-id-sale", params: { id: "-1" } },
        icon: shallowRef(IconTagFill),
        active: false,
    },
    {
        name: "اطلاعات موجودی",
        link: {
            name: "inventory-new-product-id-quantity",
            params: { id: "-1" },
        },
        icon: shallowRef(IconBox),
        active: false,
    },
    {
        name: "اندازه گیری",
        link: {
            name: "inventory-new-product-id-measure",
            params: { id: "-1" },
        },
        icon: shallowRef(IconRuler),
        active: false,
    },
]);
const hasPrevious = computed(() => {
    let previous_step = -1;
    for (const [index, step] of steps.value.entries()) {
        if (step.active) {
            previous_step = index - 1;
            break;
        }
    }
    return previous_step > -1 && previous_step < steps.value.length;
});
const hasNext = computed(() => {
    let next_step = -1;
    for (const [index, step] of steps.value.entries()) {
        if (step.active) {
            next_step = index + 1;
            break;
        }
    }
    return next_step !== -1 && next_step < steps.value.length;
});

function handlePrevious() {
    let previous_step = -1;
    for (const [index, step] of steps.value.entries()) {
        if (step.active) {
            previous_step = index - 1;
            break;
        }
    }
    if (previous_step > -1 && previous_step < steps.value.length) {
        navigateTo(steps.value[previous_step].link);
    }
}
function saveDraft() {
    storage.value.status = "draft";
    productStore.addProduct(storage.value).then(() => {
        storage.value = null;
        navigateTo({
            name: "inventory",
        });
    });
}
function handleNext() {
    let next_step = -1;
    for (const [index, step] of steps.value.entries()) {
        if (step.active) {
            next_step = index + 1;
            break;
        }
    }
    if (next_step !== -1 && next_step < steps.value.length) {
        navigateTo(steps.value[next_step].link);
    } else {
        if (storage.value.valid()) {
            storage.value.status = "active";
            productStore.addProduct(storage.value).then(() => {
                storage.value = null;
                navigateTo({
                    name: "inventory",
                });
            });
        }
    }
}
function updateActiveStatus() {
    steps.value.forEach((each) => {
        if (each.link.name === route.name) {
            each.active = true;
        } else {
            each.active = false;
        }
    });
}

onMounted(() => {
    steps.value.forEach((each) => {
        const id = route.params.id;
        if (id) {
            each.link.params = { id: String(id) };
        }
    });
    updateActiveStatus();
});
watch(route, (val) => {
    steps.value.forEach((each) => {
        const id = val.params.id;
        if (id) {
            each.link.params = { id: String(id) };
        }
    });
    updateActiveStatus();
});
</script>

<style></style>
