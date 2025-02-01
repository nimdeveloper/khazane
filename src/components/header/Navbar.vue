<template>
  <div class="w-full flex justify-between align-middle content-between items-center">
    <div class="inline-flex items-center">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" width="40px" height="40px" class="h-10">
        <path fill="#c7d7e2"
          d="M57.2,59.7H18.8c-1.3,0-2.4-1.1-2.4-2.4V19.4c0-1.3,1.1-2.4,2.4-2.4h38.4c1.3,0,2.4,1.1,2.4,2.4v37.8 C59.7,58.6,58.6,59.7,57.2,59.7" />
        <path fill="#f4f7f9"
          d="M28.236,17h-3.672c-0.715,0.54-1.164,1.401-1.164,2.4v17.9c0,1.7,1.3,3,3,3c1.6,0,3-1.4,3-3V19.4 C29.4,18.401,28.951,17.54,28.236,17" />
        <path fill="#c7d7e2"
          d="M57.2,111H18.8c-1.3,0-2.4-1.1-2.4-2.4V70.7c0-1.3,1.1-2.4,2.4-2.4h38.4c1.3,0,2.4,1.101,2.4,2.4 v37.8C59.7,109.9,58.6,111,57.2,111" />
        <path fill="#f4f7f9"
          d="M28.235,68.3h-3.671c-0.715,0.54-1.165,1.401-1.165,2.4V88.6c0,1.7,1.3,3,3,3c1.6,0,3-1.399,3-3V70.7 C29.4,69.701,28.951,68.84,28.235,68.3" />
        <path fill="#c7d7e2"
          d="M109.2,111H70.8c-1.3,0-2.399-1.1-2.399-2.4V70.7c0-1.3,1.1-2.4,2.399-2.4h38.4 c1.3,0,2.399,1.101,2.399,2.4v37.8C111.6,109.9,110.5,111,109.2,111" />
        <path fill="#f4f7f9"
          d="M80.135,68.3h-3.671c-0.715,0.54-1.165,1.401-1.165,2.4V88.6c0,1.7,1.3,3,3,3c1.7,0,3-1.399,3-3V70.7 C81.3,69.701,80.851,68.84,80.135,68.3" />
        <path fill="#454b54"
          d="M57.2,62.7H18.8c-3,0-5.4-2.4-5.4-5.4V19.4c0-3,2.4-5.4,5.4-5.4h38.4c3,0,5.4,2.4,5.4,5.4v37.9 C62.7,60.3,60.2,62.7,57.2,62.7z M19.4,56.7h37.3V20H19.4V56.7z" />
        <path fill="#dadbdd"
          d="M69.7 16.4c-1.7 0-3 1.3-3 3v37.9c0 1.7 1.3 3 3 3 1.6 0 3-1.4 3-3V19.4C72.7 17.7 71.4 16.4 69.7 16.4M79.7 16.4c-1.7 0-3 1.3-3 3v37.9c0 1.7 1.3 3 3 3 1.6 0 3-1.4 3-3V19.4C82.7 17.7 81.4 16.4 79.7 16.4M89.7 16.4c-1.7 0-3 1.3-3 3v37.9c0 1.7 1.3 3 3 3 1.6 0 3-1.4 3-3V19.4C92.7 17.7 91.4 16.4 89.7 16.4M99.7 16.4c-1.7 0-3 1.3-3 3v37.9c0 1.7 1.3 3 3 3 1.6 0 3-1.4 3-3V19.4C102.7 17.7 101.4 16.4 99.7 16.4M109.7 16.4c-1.7 0-3 1.3-3 3v37.9c0 1.7 1.3 3 3 3 1.6 0 3-1.4 3-3V19.4C112.7 17.7 111.4 16.4 109.7 16.4" />
        <path fill="#454b54"
          d="M57.2 114H18.8c-3 0-5.4-2.4-5.4-5.4V70.7c0-3 2.4-5.4 5.4-5.4h38.4c3 0 5.4 2.4 5.4 5.4v37.9C62.7 111.6 60.2 114 57.2 114zM19.4 108h37.3V71.3H19.4V108zM109.2 114H70.8c-3 0-5.4-2.4-5.4-5.4V70.7c0-3 2.4-5.4 5.4-5.4h38.4c3 0 5.4 2.4 5.4 5.4v37.9C114.6 111.6 112.2 114 109.2 114zM71.3 108h37.3V71.3H71.3V108z" />
      </svg>
      <h1 class="font-bold text-xl ms-1">
        انبار دار
      </h1>
    </div>
    <div class="flex justify-between align-middle content-between items-center text-sm">
      <NuxtLink v-for="item, id in routes" :key="id" :to="item.path"
        class="cursor-pointer inline-flex rounded-4xl px-4.5 py-2 mx-1 justify-around items-center hover:bg-active-item-bg hover:shadow-lg"
        :class="{ 'bg-active-item-bg text-primary shadow-lg': item.active, 'text-text-secondary': !item.active }">
        <component :is="item.icon" :size="20" color="currentColor" />
        <span class="ms-1" :class="{ 'text-text-primary': item.active }">{{ item.title }}</span>
      </NuxtLink>
    </div>
  </div>
</template>
<script setup lang="ts">
import { IconBarChart, IconBooks, IconCart, IconFolder, IconGauge } from '#components';

const route = useRoute()

const routes = ref([
  {
    icon: shallowRef(IconGauge),
    title: "داشبورد",
    active: true,
    path: '/'
  },
  {
    icon: shallowRef(IconBooks),
    title: "انبار",
    active: false,
    path: '/inventory'
  },
  {
    icon: shallowRef(IconCart),
    title: "سفارش",
    active: false,
    path: '/order'
  },
  {
    icon: shallowRef(IconBarChart),
    title: "گزارش",
    active: false,
    path: '/reports'
  },
  {
    icon: shallowRef(IconFolder),
    title: "پرونده ها",
    active: false,
    path: '/documents'
  },
]);

function updateActiveStatus() {
  routes.value.forEach((each, index) => {
    if (each.path === "/") {
      if (route.path === each.path) {
        each.active = true
      } else {
        each.active = false
      }
    } else if (route.path.startsWith(each.path)) {
      each.active = true
    } else {
      each.active = false
    }
  })
}
watch(route, () => {
  updateActiveStatus()
})
onBeforeMount(() => {
  updateActiveStatus()
})

</script>
