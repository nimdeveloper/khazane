<template>
    <div class="mb-2" v-if="label">{{ label }}</div>
    <div
        class="relative flex rounded-2xl row h-14 bg-active-item-bg cursor-pointer items-center outline-0"
        v-bind="$attrs"
        ref="current-select-ref"
    >
        <div class="w-full h-full">
            <label
                v-if="searchable"
                class="rounded-xl bg-input-bg w-full block text-text-primary relative h-full"
                @click.stop
            >
                <IconMagnify
                    :size="20"
                    color="currentColor"
                    class="absolute start-3 top-1/2 -translate-y-1/2 rotate-90"
                />
                <input
                    class="w-full bg-transparent border-0 ps-10 outline-0 shadow-[none] py-1 h-full"
                    type="text"
                    placeholder="جستجو"
                    v-model="searchValue"
                    @focus="toggleSelect"
                />
            </label>
        </div>
        <div
            @click.stop.prevent
            class="absolute left-0 w-full shadow shadow-gray-700/50 border-border-1 border-2 bg-active-item-bg rounded-2xl overflow-hidden transition-all z-10 cursor-default"
            :class="{
                'bottom-full -translate-y-0.5': position === 'top',
                'top-full translate-y-0.5': position === 'bottom',
                'max-h-56 opacity-100': isOpen,
                'max-h-0 p-0 opacity-0': !isOpen,
            }"
            ref="current-select-dropdown-ref"
        >
            <div class="px-4 max-h-50 mt-3" v-if="options.length > 0">
                <Simplebar
                    data-simplebar-direction="rtl"
                    class="max-h-48 w-full py-2.5 my-3"
                >
                    <button
                        v-for="(item, index) of options"
                        :key="index"
                        class="py-2 px-3 hover:bg-primary-bg bg-secondary-bg rounded-xl flex items-center w-full cursor-pointer"
                        :class="{
                            'border border-primary': selected?.find(
                                (each) => each.value === item.value
                            ),
                        }"
                        @click.stop.prevent="onItemClick(item)"
                    >
                        <span v-if="item.prefix">{{ item.prefix }}:&nbsp;</span
                        >{{ item.label }}&nbsp;
                        <div
                            class="p-2 bg-active-item-bg dark:hover:bg-gray-700 dark:border-gray-600 rounded-lg ms-auto me-1"
                        >
                            <IconPlus
                                v-if="
                                    !selected?.find(
                                        (each) => each.value === item.value
                                    )
                                "
                                :size="18"
                                color="currentColor"
                                class="text-primary"
                            />
                            <IconClose
                                v-else
                                :size="18"
                                color="currentColor"
                                class="text-text-secondary"
                            />
                        </div>
                    </button>
                </Simplebar>
            </div>
            <div class="py-3 px-9 my-3" v-if="options.length < 1">
                {{ label || "گزینه ای برای انتخاب" }}
                {{ searchable ? "پیدا نشد" : "موجود نیست" }}!
            </div>
            <div
                v-if="searchValue.length > 0 && options.length < 1 && addable"
                class="text-primary border-t-2 border-border-1 py-4 px-9 hover:bg-primary/10 text-nowrap overflow-hidden text-ellipsis cursor-pointer"
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

let isSelected: boolean;

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
    selected: SelectedType[] | null;
    options: Array<SelectedType>;
    label?: string;
    searchable?: boolean;
    addable?: boolean;
}>();
const emit = defineEmits(["change", "toggled", "add", "new", "remove"]);
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
function onItemClick(item: SelectedType) {
    if (!selected?.find((each) => each.value === item.value)) {
        onItemSelect(item);
    } else {
        onItemRemove(item);
    }
}
function onItemSelect(item: SelectedType) {
    emit("new", item);
    toggleSelect();
}
function onItemRemove(item: SelectedType) {
    emit("remove", item);
    toggleSelect();
}
function toggleSelect() {
    if (!isOpen.value) calculatePosition();
    isOpen.value = !isOpen.value;
    emit("toggled", !isOpen.value);
}
</script>

<style></style>
