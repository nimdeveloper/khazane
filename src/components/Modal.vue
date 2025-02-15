<template>
    <!-- Main modal -->
    <div
        data-modal-backdrop="static"
        tabindex="-1"
        aria-hidden="true"
        class="hidden overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-[50] justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"
        ref="current-page-modal"
    >
        <!-- Modal content -->
        <slot :modal="$modalInstance">
            <div class="relative p-4 w-full max-w-2xl max-h-full">
                <div
                    class="relative bg-white rounded-lg shadow-sm dark:bg-gray-700"
                >
                    <!-- Modal header -->
                    <slot name="Header">
                        <div
                            class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600 border-gray-200"
                        >
                            <h3
                                class="text-xl font-semibold text-gray-900 dark:text-white"
                            >
                                {{ title }}
                            </h3>
                            <button
                                type="button"
                                class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                                v-if="closable"
                                @click.prevent="closeModal"
                            >
                                <svg
                                    class="w-3 h-3"
                                    aria-hidden="true"
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 14 14"
                                >
                                    <path
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                                    />
                                </svg>
                                <span class="sr-only">Close modal</span>
                            </button>
                        </div>
                    </slot>
                    <!-- Modal body -->
                    <slot name="Content"></slot>
                    <!-- Modal footer -->
                    <slot name="Footer">
                        <div
                            class="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600"
                        >
                            <button
                                v-if="closable"
                                @click.prevent="closeModal"
                                type="button"
                                class="py-2.5 px-5 ms-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-50 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"
                            >
                                بستن
                            </button>
                        </div>
                    </slot>
                </div>
            </div>
        </slot>
    </div>
</template>

<script lang="ts" setup>
import { initFlowbite, Modal, type ModalInterface } from "flowbite";

const { title, closable } = defineProps({
    title: {
        type: String,
        default: "",
        required: false,
    },
    closable: {
        type: Boolean,
        default: true,
        required: false,
    },
});

const emit = defineEmits(["close", "open"]);

const modalModel = defineModel<ModalInterface | null>("modal", {
    default: null,
});
const isOpen = defineModel<boolean>("is-open", {
    default: false,
});

const $modalInstance = ref<ModalInterface | null>(null);
const $modalElement = useTemplateRef("current-page-modal");

onMounted(() => {
    useFlowbite(() => {
        initFlowbite();
    });
});
watch($modalElement, () => {
    if ($modalElement.value) {
        const modal: ModalInterface = new Modal($modalElement.value, {
            onHide: () => {
                if (document.activeElement) {
                    (<HTMLElement>document.activeElement).blur();
                }
                isOpen.value = false;
                emit("close");
            },
            onShow: () => {
                isOpen.value = true;
                emit("open");
            },
        });
        $modalInstance.value = modal;
        modalModel.value = modal;
    }
});
const openModal = () => {
    if ($modalInstance.value) {
        $modalInstance.value.show();
    }
};
const closeModal = () => {
    if (document.activeElement) {
        (<HTMLElement>document.activeElement).blur();
    }
    if ($modalInstance.value) {
        $modalInstance.value.hide();
    }
};

watch(isOpen, (newVal, _) => {
    if (!modalModel.value) return;
    if (!newVal) {
        if (modalModel.value.isVisible()) {
            if (document.activeElement) {
                (<HTMLElement>document.activeElement).blur();
            }
            modalModel.value.hide();
        }
        return;
    }
    if (modalModel.value.isHidden()) {
        modalModel.value.show();
    }
});
</script>

<style></style>
