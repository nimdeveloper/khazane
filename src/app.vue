<template>
  <div class="bg-primary-bg min-h-dvh text-text-primary font-[Vazir]">
    <NuxtRouteAnnouncer />

    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>
<style lang="scss">
html {
  background-color: var(--color-primary-bg);
}

body {
  padding: 10px;
}

* {
  direction: rtl;
}
</style>