<template>
    <Modal
        v-model:modal="$modal"
        @close="onClose"
        @open="animationState = 'open'"
        :closable="true"
        v-bind="$attrs"
    >
        <template v-slot="{ modal }">
            <div
                class="relative p-4 w-full max-w-xl max-h-full ms-auto pe-0 pb-0 pt-4 h-dvh"
            >
                <div
                    class="relative bg-white rounded-s-lg shadow-sm bg-action-secondary dark:bg-dark-action-secondary h-full flex flex-col transition-transform"
                    :class="{
                        '-translate-x-full': animationState == 'close',
                        'translate-0': animationState == 'open',
                    }"
                >
                    <!-- Modal header -->
                    <div
                        class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600 border-gray-200"
                    >
                        <h3
                            class="text-xl font-semibold text-gray-900 dark:text-white"
                        >
                            ایجاد کاربر جدید
                        </h3>
                        <button
                            type="button"
                            class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer"
                            @click.prevent="modal?.hide()"
                        >
                            <IconClose :size="12" color="currentColor" />
                            <span class="sr-only">Close modal</span>
                        </button>
                    </div>
                    <!-- Modal body -->
                    <div class="grow-1 flex px-3 flex-col">
                        <div class="w-full py-4">
                            <div class="flex w-full">
                                <FormInput
                                    name="product_name"
                                    label="نام و نام خانوادگی"
                                    v-model:value="person.full_name"
                                    class="grow-1"
                                />
                            </div>
                        </div>
                    </div>
                    <div
                        class="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600 justify-end"
                    >
                        <button
                            @click.prevent="modal?.hide()"
                            type="button"
                            class="py-2.5 px-5 ms-auto text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 cursor-pointer"
                            :class="[
                                'focus:z-50 focus:ring-4 focus:ring-gray-100 focus:outline-none',
                                'dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700',
                            ]"
                        >
                            بستن
                        </button>
                        <button
                            @click.prevent="submit"
                            type="button"
                            class="text-white bg-green-700 hover:bg-green-800 font-medium rounded-lg text-sm px-5 py-2.5 ms-2 cursor-pointer disabled:cursor-not-allowed disabled:grayscale-100"
                            :class="[
                                'focus:outline-none focus:ring-4 focus:ring-green-300',
                                'dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800',
                            ]"
                        >
                            ذخیره
                        </button>
                    </div>
                </div>
            </div>
        </template>
    </Modal>
</template>

<script lang="ts" setup>
import type { ModalInterface } from "flowbite";
import { Person } from "~/interfaces/person";
import { useMyPersonStore } from "~/stores/person";

const { preferredName } = defineProps<{ preferredName?: string }>();
const $modal = ref<ModalInterface | null>(null);

const personStore = useMyPersonStore();

const animationState = ref<"close" | "open">("close");
const person = ref(new Person(random(20)));

watch(
    () => preferredName,
    (val) => {
        if (person.value && val) {
            person.value.full_name = val;
        }
    }
);
onMounted(() => {
    if (person.value && preferredName) {
        person.value.full_name = preferredName;
    }
});
const submit = () => {
    personStore.addPerson(person.value);
    $modal.value?.hide();
};
const onClose = () => {
    if (person.value) {
        person.value = new Person(random(20));
    }
    animationState.value = "close";
};
</script>

<style></style>
