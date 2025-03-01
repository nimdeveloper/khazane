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
                            ایجاد انبار جدید
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
                                    label="نام انبار"
                                    v-model:value="warehouse.name"
                                    class="grow-1"
                                />
                            </div>
                        </div>
                        <div class="w-full py-4">
                            <div class="flex w-full">
                                <FormInput
                                    name="product_name"
                                    label="نام کوتاه"
                                    v-model:value="warehouse.shorthand"
                                    class="grow-1"
                                />
                            </div>
                        </div>
                        <div class="w-full py-4">
                            <div class="flex w-full flex-col">
                                <div class="grow-1 w-full">رنگ</div>
                                <div class="flex my-2">
                                    <div
                                        v-for="(color, index) of colors"
                                        :key="`${color.key}_${index}`"
                                        class="w-8 h-8 mx-2 rounded-full cursor pointer cursor-pointer box-border relative inline"
                                        :style="{ backgroundColor: color.code }"
                                        @click="warehouse.color = color"
                                    >
                                        <div
                                            class="border-action-primary dark:border-dark-action-primary w-full h-full scale-135 left-1/2 top-1/2 -translate-1/2 border-2 absolute rounded-full"
                                            v-if="
                                                color.key ===
                                                warehouse.color?.key
                                            "
                                        ></div>
                                    </div>
                                </div>
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
import { WareHouse } from "~/interfaces/warehouse";
import { useMyWarehouseStore } from "~/stores/warehouse";

const colors = WareHouse.COLOR_PALATES;

const { preferredName } = defineProps<{ preferredName?: string }>();
const $modal = ref<ModalInterface | null>(null);

const wareHouseStore = useMyWarehouseStore();

const animationState = ref<"close" | "open">("close");
const warehouse = ref(new WareHouse(random(20)));

watch(
    () => preferredName,
    (val) => {
        if (warehouse.value && val) {
            warehouse.value.name = val;
        }
    }
);
onMounted(() => {
    if (warehouse.value && preferredName) {
        warehouse.value.name = preferredName;
    }
});
const submit = () => {
    console.log(warehouse.value);
    wareHouseStore.addWareHouse(warehouse.value);
    $modal.value?.hide();
};
const onClose = () => {
    if (warehouse.value) {
        warehouse.value = new WareHouse(random(20));
    }
    animationState.value = "close";
};
</script>

<style></style>
