<template>
  <svg
    viewBox="-0.5 -0.5 16 16"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    :height="size"
    :width="size"
  >
    <path
      d="M4.14 2.4562500000000003h10.08"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
    ></path>
    <path
      d="m0.78 2.4646874999999997 0.008375 -0.009375"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2.5"
    ></path>
    <path
      d="m0.78 7.5046875 0.008375 -0.009375"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2.5"
    ></path>
    <path
      d="m0.78 12.5446875 0.008375 -0.009375"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2.5"
    ></path>
    <path
      d="M4.14 7.49625h10.08"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
    ></path>
    <path
      d="M4.14 12.536249999999999h10.08"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
    ></path>
  </svg>
</template>

<script>
export default {
  name: "List",
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
