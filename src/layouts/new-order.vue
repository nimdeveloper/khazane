<template>
    <NuxtLayout name="default">
        <UserNewModal
            v-model:is-open="newUserModalOpen.open"
            :preferred-name="newUserModalOpen.preferred_name"
        />
        <LocationNewModal
            v-model:is-open="newLocationModalOpen.open"
            :preferred-name="newLocationModalOpen.preferred_name"
        />
        <div class="flex flex-col w-full">
            <NuxtLink
                class="inline-flex items-center mt-2 lg:mb-6 gap-0 hover:gap-1 transition-all lg:ms-4 text-secondary dark:text-dark-secondary print:hidden"
                to="/order/"
            >
                <IconNavArrowRight
                    :size="20"
                    color="currentColor"
                    class="me-2"
                />
                باز گشت به گردش انبار
            </NuxtLink>
            <div class="flex w-full">
                <div class="flex flex-col lg:flex-row w-full">
                    <ul
                        class="flex lg:block space-y space-y-4 grow-1 lg:grow-0 lg:w-96 lg:max-w-96 text-sm font-medium text-gray-500 dark:text-gray-400 lg:me-4 lg:mb-0 z-[2] print:hidden"
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
                        class="pt-6 ps-6 pe-3 pb-16 text-medium rounded-xl grow-1 bg-action-secondary dark:bg-dark-action-secondary h-[calc(100dvh-225px)] lg:h-[calc(100dvh-138px)] print:h-auto print:min-h-[95dvh] print:bg-transparent print:p-0.5 flex flex-col relative w-auto lg:w-[calc(100%-384px)]"
                    >
                        <div class="h-full pe-3 hidden print:block print:p-0">
                            <slot />
                        </div>
                        <Simplebar
                            data-simplebar-direction="rtl"
                            class="h-full pe-3 print:hidden"
                        >
                            <slot />
                        </Simplebar>
                        <div
                            class="absolute bottom-0 left-0 w-full h-16 py-3 pe-5 flex justify-end print:hidden"
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
import Simplebar from "simplebar-vue";
import { IconBox, IconDocument, IconReportFile, IconUsers } from "#components";
import { useMyOrderStore } from "../stores/order";

const newUserModalOpen = ref({ open: false, preferred_name: "" });
const newLocationModalOpen = ref({ open: false, preferred_name: "" });

provide(/* key */ "newUserModalOpen", /* value */ newUserModalOpen);
provide(/* key */ "newLocationModalOpen", /* value */ newLocationModalOpen);

const orderStore = useMyOrderStore();

const { storage } = useTempOrder();
const route = useRoute();
const steps = ref([
    {
        name: "اطلاعات عمومی",
        link: {
            name: "order-new-order-id-public",
            params: { id: "-1" },
        },
        icon: shallowRef(IconDocument),
        active: false,
    },
    {
        name: "افراد",
        link: { name: "order-new-order-id-users", params: { id: "-1" } },
        icon: shallowRef(IconUsers),
        active: false,
    },
    {
        name: "محصولات",
        link: {
            name: "order-new-order-id-products",
            params: { id: "-1" },
        },
        icon: shallowRef(IconBox),
        active: false,
    },
    {
        name: "فاکتور",
        link: {
            name: "order-new-order-id-invoice",
            params: { id: "-1" },
        },
        icon: shallowRef(IconReportFile),
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
    orderStore
        .addOrder(storage.value, isNaN(Number(storage.value.key)))
        .then(() => {
            storage.value = null;
            navigateTo({
                name: "order",
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

            orderStore
                .addOrder(storage.value, isNaN(Number(storage.value.key)))
                .then(() => {
                    storage.value = null;
                    navigateTo({
                        name: "order",
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
