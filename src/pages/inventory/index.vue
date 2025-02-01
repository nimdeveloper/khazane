<template>
  <div class="w-full flex">
    <div class="flex flex-col" :style="{ width: `${sidebarWidth}%` }">
      <div class="flex flex-row mt-2">
        <h2 class="text-xl">
          محصولات
        </h2>
        <div class="border border-border-1 rounded-2xl py-1 px-2 text-xs ms-2">
          <span>{{ totalProducts }}</span> <span class="text-text-secondary">محصول</span>
        </div>
      </div>
      <div class="flex w-full rounded-2xl py-4 px-2 mt-4 bg-secondary-bg flex-col overflow-y-auto"
        style="height: calc(100dvh - 132px);">
        <div class="text-text-secondary ps-1">
          نوع محصولات
        </div>
        <div class="flex flex-wrap mt-3">
          <div v-for="item, index of productTypeChoices" :key="index" class="w-full lg:w-1/2 ps-1 pb-1">
            <button class="cursor-pointer rounded-2xl py-2 px-4 border border-border-2 w-full inline-flex items-center"
              :class="{
                'border-primary bg-active-item-bg': activeproductType === item.value
              }" @click="changeProductTypeFilter(item.value)">
              <div class="text-nowrap text-ellipsis overflow-hidden text-sm">{{ item.label }}</div>
              <div class="rounded-lg py-1 ms-auto text-sm px-2 " :class="{
                'border-primary bg-primary/20 text-primary': activeproductType === item.value,
                'bg-text-secondary/20': activeproductType !== item.value,
              }">1000</div>
            </button>
          </div>
        </div>
        <div class="mt-8 text-text-secondary ps-1">
          مرتب سازی
        </div>
        <div class="ps-1 mt-3">
          <FormSelect :options="sortChoices" :selected="activeSort" @change="(val) => activeSort = val">
            <template #icon>
              <IconSort :size="25" color="currentColor" />
            </template>
          </FormSelect>
        </div>
        <div class="mt-8">دسته بندی</div>
        <div class="ps-1 mt-3">
          <FormSelect :options="categoryChoices" :selected="activeCategory" @change="(val) => activeCategory = val">
            <template #icon>
              <IconCategory :size="25" color="currentColor" />
            </template>
          </FormSelect>
        </div>
        <div class="mt-8">انبار</div>
        <div class="ps-1 mt-3">
          <FormSelect :options="inventoryChoices" :selected="activeInvetory" @change="(val) => activeInvetory = val">
            <template #icon>
              <IconInventory :size="25" color="currentColor" />
            </template>
          </FormSelect>
        </div>
      </div>
    </div>
    <div class="p-4 flex" :style="{ width: `${100 - sidebarWidth}%` }">

    </div>
  </div>
</template>

<script lang="ts" setup>

const productTypeChoices = [
  {
    label: "همه",
    value: "all",
  },
  {
    label: "فعال",
    value: "active",
  },
  {
    label: "غیر فعال",
    value: "inactive",
  },
  {
    label: "پیش نویس",
    value: "draft",
  },
]
const sortChoices = [
  {
    prefix: "الفبایی",
    label: "آ-ی",
    value: "alphabetical-des"
  },
  {
    prefix: "الفبایی",
    label: "ی-آ",
    value: "alphabetical-asc"
  },
]
const categoryChoices = [
  {
    prefix: "",
    label: "همه",
    value: "all",
  },
  {
    prefix: "",
    label: "دسته بندی 1",
    value: "group1",
  },
  {
    prefix: "",
    label: "دسته بندی 2",
    value: "group2",
  },
]
const inventoryChoices = [
  {
    prefix: "",
    label: "همه",
    value: "all",
  },
  {
    prefix: "",
    label: "دسته بندی 1",
    value: "group1",
  },
  {
    prefix: "",
    label: "دسته بندی 2",
    value: "group2",
  },
]
const sidebarWidth = ref(35)
const totalProducts = ref(1000)

// Filters
const activeproductType = ref("all")
const activeSort = ref(sortChoices[0])
const activeCategory = ref(categoryChoices[0])
const activeInvetory = ref(inventoryChoices[0])

// Function
function changeProductTypeFilter(val: string) {
  activeproductType.value = val
}
</script>

<style></style>