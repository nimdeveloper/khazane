<template>
    <div class="inline-flex h-full flex-col w-full">
        <div>{{ label }}</div>
        <input
            v-model="val"
            :placeholder="placeholder"
            :name="name"
            class="rounded-xl border-2 border-border-1 py-2 px-3 outline-0 shadow-[none] focus:border-secondary focus:dark:border-dark-secondary mt-2 w-full grow-1"
        />
    </div>
</template>

<script lang="ts" setup>
const { name, placeholder, label } = defineProps({
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
});
const val = defineModel<string | number>("value", {
    default: "",
});
</script>

<style></style>
