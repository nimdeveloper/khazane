<template>
  <svg
    viewBox="-0.5 -0.5 16 16"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    id="Nav-Arrow-Right--Streamline-Iconoir"
    :height="size"
    :width="size"
  >
    <path
      d="M4.2333125 0.9666874999999999 10.7666875 7.5 4.2333125 14.0333125"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="1.5"
    ></path>
  </svg>
</template>

<script>
export default {
  name: "NavArrowRight",
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
