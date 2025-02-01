<template>
  <div class="relative flex rounded-2xl row px-2.5 p-3 bg-active-item-bg cursor-pointer items-center"
    @click="toggleSelect()" ref="current-select-ref">
    <div class="me-2 text-text-secondary">
      <slot name="icon" />
    </div>
    <div>
      <span class="text-text-secondary me-2" v-if="selected.prefix">{{ selected.prefix }}:</span><span>{{ selected.label
        }}</span>
    </div>
    <IconAltArrow :size="20" class="ms-auto text-text-secondary" color="currentColor"
      :direction="isOpen ? 'up' : 'down'" />
    <div
      class="absolute left-0 w-full shadow-lg border-border-1 border-2 bg-active-item-bg rounded-2xl overflow-hidden transition-all z-10"
      :class="{
        'bottom-full -translate-y-0.5': position === 'top',
        'top-full translate-y-0.5': position === 'bottom',
        'h-initial px-5 py-3 opacity-100': isOpen,
        'h-0 p-0 opacity-0': !isOpen
      }" ref="current-select-dropdown-ref">
      <button v-for="item, index of options" :key="index"
        class="py-2 px-3 hover:bg-primary-bg rounded-xl flex items-center w-full cursor-pointer" :class="{
          'border border-primary': selected.value === item.value
        }" @click.prevent="onItemSelect(item)">
        <IconCheckSquare v-if="selected.value === item.value" :size="26" color="currentColor"
          class="text-primary me-1" /><span v-if="item.prefix">{{
            item.prefix }}:&nbsp;</span>{{
            item.label }}
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
// Defines
interface SelectedType {
  value: string;
  prefix: string;
  label: String;
}
const { options, selected } = defineProps({
  selected: {
    type: Object,
    required: true
  },
  options: {
    type: Array<SelectedType>,
    required: true,
  }
})
const emit = defineEmits(['change',])
const selectHolder = useTemplateRef('current-select-ref')
const selectDropdownHolder = useTemplateRef('current-select-dropdown-ref')

// State
const isOpen = ref(false)
const position = ref("top")
const dropdownHeight = ref(0)

function calculatePosition() {
  if (selectHolder.value && selectDropdownHolder.value) {
    const pos = selectHolder.value.getBoundingClientRect();
    const elementPos = pos.y
    const windowHeight = window.innerHeight
    if (elementPos > windowHeight) return
    let dropDownHeight = selectDropdownHolder.value.clientHeight
    if (dropdownHeight.value < dropDownHeight) dropdownHeight.value = dropDownHeight
    else dropDownHeight = dropdownHeight.value
    if (dropDownHeight <= 0) return
    console.log(dropDownHeight, elementPos, windowHeight)
    if (windowHeight - elementPos <= dropDownHeight) position.value = "top"
    else position.value = "bottom"
  }
}

onUpdated(() => {
  if (isOpen.value) calculatePosition()
})
onMounted(() => {
  window.addEventListener("animationend", calculatePosition)
  document.addEventListener("resize", calculatePosition)
  document.addEventListener("scroll", calculatePosition)
})
onUnmounted(() => {
  window.removeEventListener("animationend", calculatePosition)
  document.removeEventListener("resize", calculatePosition)
  document.removeEventListener("scroll", calculatePosition)
})
function onItemSelect(item: SelectedType) {
  emit("change", item)
}
function toggleSelect() {
  if (!isOpen.value) calculatePosition()
  isOpen.value = !isOpen.value
}
</script>

<style></style>