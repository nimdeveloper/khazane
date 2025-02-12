<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="-0.5 -0.5 16 16"
    fill="none"
    :stroke="color"
    stroke-linecap="round"
    stroke-linejoin="round"
    :height="size"
    :width="size"
  >
    <path
      d="M5 2.5h0.625l1.875 1.875h4.375a1.25 1.25 0 0 1 1.25 1.25v5m-1.25 1.25H3.125a1.25 1.25 0 0 1 -1.25 -1.25V3.75a1.25 1.25 0 0 1 0.743125 -1.143125"
      stroke-width="1"
    ></path>
    <path d="m1.875 1.875 11.25 11.25" stroke-width="1"></path>
  </svg>
</template>

<script>
export default {
  name: "FolderOff",
  props: {
    size: {
      type: Number,
      default: 80,
    },
    color: {
      type: String,
      default: "#343C54",
    },
  },
};
</script>

<style></style>
