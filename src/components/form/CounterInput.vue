<template>
    <div class="inline-flex h-full flex-col w-full">
        <div>{{ label }}</div>
        <div
            class="relative flex items-center rounded-xl border-2 border-border-1 mt-2 w-full grow-1 min-h-12"
        >
            <button
                type="button"
                @click.prevent="setValue(val - 1)"
                class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-full mx-1 p-2 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none cursor-pointer"
            >
                <IconMinus :size="10" color="currentColor" />
            </button>
            <input
                type="text"
                aria-describedby="helper-text-explanation"
                class="bg-transparent border-0 h-full font-medium text-center outline-0 shadow-[none] text-gray-900 text-sm w-full dark:border-gray-600 dark:placeholder-gray-400 dark:text-white relative bottom-1.5"
                :placeholder="placeholder"
                :value="val"
                @change="(e) => setValue(e.target ? Number((<HTMLInputElement>(e.target)).value) : 0)"
                required
                :name="name"
            />
            <div
                class="absolute bottom-1 start-1/2 translate-x-1/2 flex items-center text-xs text-gray-400 space-x-1 rtl:space-x-reverse"
            >
                <!-- <IconPlus :size="10" color="currentColor" /> -->
                در دسترس: {{ max - val }}
            </div>
            <button
                type="button"
                @click.prevent="setValue(val + 1)"
                class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-full p-2 mx-1 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none cursor-pointer"
            >
                <IconPlus :size="10" color="currentColor" />
            </button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { IconPlus } from "#components";

const {
    name,
    placeholder,
    label,
    min = Number.MIN_SAFE_INTEGER,
    max = Number.MAX_SAFE_INTEGER,
} = defineProps({
    name: {
        type: String,
        required: true,
    },
    placeholder: {
        type: String,
        required: false,
        default: "",
    },
    label: {
        type: String,
        required: false,
        default: "",
    },
    min: {
        type: Number,
        required: false,
    },
    max: {
        type: Number,
        required: false,
    },
});
const val = defineModel<number>("value", {
    default: 0,
});
watch(val, (newVal, oldVal) => {
    if (Number.isNaN(Number(newVal))) {
        val.value = oldVal;
    }
    if (!newVal && newVal !== 0) {
        val.value = 0;
    }
    if (newVal < min) {
        val.value = min;
    }
    if (max < min) return;
    if (newVal > max) {
        val.value = max;
    }
});
watch(
    () => ({ min, max }),
    ({ min, max }, _) => {
        if (val.value < min) {
            val.value = min;
        }
        if (val.value > max) {
            val.value = max;
        }
    }
);
const setValue = (newVal: number) => {
    if (Number.isNaN(Number(newVal))) {
        return;
    }
    if (!newVal) {
        val.value = 0;
        return;
    }
    if (newVal < min) {
        val.value = min;
    }
    if (max >= min) {
        if (newVal > max) {
            val.value = max;
            return;
        }
    }
    val.value = newVal;
};
</script>

<style></style>
