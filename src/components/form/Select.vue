<template>
    <div class="mb-2" v-if="label">{{ label }}</div>
    <div
        class="relative flex rounded-2xl row px-2.5 p-3 bg-action-secondary dark:bg-dark-action-secondary cursor-pointer items-center border-2 border-border-1 outline-0 focus:border-secondary focus:dark:border-dark-secondary"
        @click="toggleSelect"
        v-bind="$attrs"
        ref="current-select-ref"
    >
        <div class="me-2 text-secondary dark:text-dark-secondary">
            <slot name="icon" />
        </div>
        <div v-if="selected">
            <span
                class="text-secondary dark:text-dark-secondary me-2"
                v-if="selected.prefix"
                >{{ selected.prefix }}:</span
            ><span>{{ selected.label }}</span>
        </div>
        <div v-else>
            <span class="text-secondary dark:text-dark-secondary me-2"
                >انتخاب کنید</span
            >
        </div>
        <IconAltArrow
            :size="20"
            class="ms-auto text-secondary dark:text-dark-secondary"
            color="currentColor"
            :direction="isOpen ? 'up' : 'down'"
        />
        <div
            @click.stop.prevent
            class="absolute left-0 w-full shadow shadow-gray-700/20 border-border-1 border-2 bg-action-secondary dark:bg-dark-action-secondary rounded-2xl overflow-hidden transition-all z-10 cursor-default"
            :class="{
                'bottom-full -translate-y-0.5': position === 'top',
                'top-full translate-y-0.5': position === 'bottom',
                'max-h-56 opacity-100': isOpen,
                'max-h-0 p-0 opacity-0': !isOpen,
            }"
            ref="current-select-dropdown-ref"
        >
            <div class="px-5 pt-3 mb-3">
                <label
                    v-if="searchable"
                    class="rounded-xl bg-glob-dark dark:bg-dark-glob-dark w-full block text-primary dark:text-dark-primary relative"
                    @click.stop
                >
                    <IconMagnify
                        :size="20"
                        color="currentColor"
                        class="absolute start-3 top-1/2 -translate-y-1/2 rotate-90"
                    />
                    <input
                        class="w-full bg-transparent border-0 ps-10 outline-0 shadow-[none] py-1"
                        type="text"
                        placeholder="جستجو"
                        v-model="searchValue"
                    />
                    <div
                        class="h-2/3 w-0.5 bg-secondary/10 dark:bg-dark-secondary/10 mx-1 my-auto"
                    ></div>
                </label>
            </div>
            <div class="px-5 max-h-50" v-if="options.length > 0">
                <Simplebar
                    data-simplebar-direction="rtl"
                    class="max-h-48 pe-3 w-full pb-3 mb-3"
                >
                    <button
                        v-for="(item, index) of options"
                        :key="index"
                        class="py-2 px-3 hover:bg-glob-primary hover:dark:bg-dark-glob-primary rounded-xl flex items-center w-full cursor-pointer"
                        :class="{
                            'border border-action-primary dark:border-dark-action-primary':
                                selected?.value === item.value,
                        }"
                        @click.stop.prevent="onItemSelect(item)"
                    >
                        <span v-if="item.prefix">{{ item.prefix }}:&nbsp;</span
                        >{{ item.label }}&nbsp;
                        <IconCheckSquare
                            v-if="selected?.value === item.value"
                            :size="20"
                            color="currentColor"
                            class="text-action-primary dark:text-dark-action-primary me-1"
                        />
                    </button>
                </Simplebar>
            </div>
            <div class="py-3 px-9 mb-3" v-if="options.length < 1">
                {{ label || "گزینه ای برای انتخاب" }}
                {{ searchable ? "پیدا نشد" : "موجود نیست" }}!
            </div>
            <div
                v-if="searchValue.length > 0 && options.length < 1 && addable"
                class="text-action-primary dark:text-dark-action-primary border-t-2 border-border-1 py-4 px-9 hover:bg-action-primary/10 hover:dark:bg-dark-action-primary/10 text-nowrap overflow-hidden text-ellipsis cursor-pointer"
                @click.stop="addNewItem"
            >
                <IconPlus
                    :size="12"
                    color="currentColor"
                    class="inline me-2"
                />اضافه کردن "{{ searchValue }}"
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onClickOutside } from "@vueuse/core";
import Simplebar from "simplebar-vue";

interface SelectedType {
    value: string;
    prefix?: string;
    label: string;
}

// Defines
const {
    options: defaultOptions,
    selected,
    label,
    searchable = false,
    addable = false,
} = defineProps<{
    selected: SelectedType | null;
    options: Array<SelectedType>;
    label?: string;
    searchable?: boolean;
    addable?: boolean;
}>();
const emit = defineEmits(["change", "toggled", "add"]);
const selectHolder = useTemplateRef("current-select-ref");
const selectDropdownHolder = useTemplateRef("current-select-dropdown-ref");

// State
const isOpen = ref(false);
const position = ref("top");
const dropdownHeight = ref(0);
const searchValue = ref("");

const options = computed(() => {
    if (!searchable) return defaultOptions;
    return defaultOptions.filter((each) => {
        const searchAble = each.label.replaceAll(" ", "");
        const searchAble2 = each.prefix?.replaceAll(" ", "") || "";
        return (
            searchAble.startsWith(
                searchValue.value.trim().replaceAll(" ", "")
            ) ||
            searchAble2.startsWith(searchValue.value.trim().replaceAll(" ", ""))
        );
    });
});
watch(isOpen, (_, oldVal) => {
    if (oldVal && selectDropdownHolder.value) {
        calculateDropDownHeight();
    }
});
watch(selectDropdownHolder, (newVal, oldVal) => {
    if (!newVal && oldVal) {
        oldVal.removeEventListener("transitionend", () =>
            calculateDropDownHeight()
        );
    }
    if (!!(!oldVal && newVal)) {
        newVal.addEventListener("transitionend", () =>
            calculateDropDownHeight()
        );
    }
});
watch(dropdownHeight, (newVal, _) => {
    calculatePosition(newVal);
});

function calculateDropDownHeight() {
    if (selectDropdownHolder.value) {
        const selectDropdownPosition =
            selectDropdownHolder.value.getBoundingClientRect();
        let dropDownHeight = Math.max(
            selectDropdownHolder.value.clientHeight,
            selectDropdownPosition.bottom - selectDropdownPosition.top
        );
        if (dropdownHeight.value < dropDownHeight)
            dropdownHeight.value = dropDownHeight;
    }
}

function calculatePosition(dropdownHeightValue = dropdownHeight.value) {
    if (selectHolder.value && selectDropdownHolder.value) {
        const selectBoxPosition = selectHolder.value.getBoundingClientRect();
        const windowHeight = window.innerHeight;
        if (selectBoxPosition.bottom > windowHeight) return;
        calculateDropDownHeight();
        if (dropdownHeightValue <= 0) return;
        if (windowHeight - selectBoxPosition.bottom <= dropdownHeightValue + 26)
            position.value = "top";
        else position.value = "bottom";
    }
}
onMounted(() => {
    window.addEventListener("resize", () => calculatePosition());
    document.addEventListener("scroll", () => calculatePosition());
});
onUnmounted(() => {
    window.removeEventListener("resize", () => calculatePosition());
    document.removeEventListener("scroll", () => calculatePosition());
});
onClickOutside(selectHolder, () => {
    if (isOpen.value) {
        isOpen.value = false;
        emit("toggled", true);
    }
});

function addNewItem() {
    if (!addable) return;
    const label = searchValue.value;
    emit("add", label);
}
function onItemSelect(item: SelectedType) {
    emit("change", item);
    toggleSelect();
}
function toggleSelect() {
    if (!isOpen.value) calculatePosition();
    isOpen.value = !isOpen.value;
    emit("toggled", !isOpen.value);
}
</script>

<style></style>
