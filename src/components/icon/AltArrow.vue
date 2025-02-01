<template>
  <svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" :height="size" :width="size"
    :style="{ transform: svgTransform }" class="transition-transform">
    <path
      d="m8.246933333333333 10.556733333333334 4.286466666666667 -4.420286666666666C12.800866666666666 5.860626666666667 12.638733333333334 5.333333333333333 12.286466666666666 5.333333333333333H3.713533333333333c-0.3522733333333333 0 -0.5144066666666667 0.5272933333333333 -0.24693333333333334 0.8031133333333332l4.286466666666667 4.420286666666666c0.14206666666666667 0.1466 0.35179999999999995 0.14653333333333332 0.4938666666666667 0Z"
      :fill="color" stroke-width="0.6667"></path>
  </svg>
</template>

<script>

export default {
  name: "AltArrow",
  props: {
    size: {
      type: Number,
      default: 80
    },
    color: {
      type: String,
      default: "#343C54"
    },
    direction: {
      type: String,
      default: "down"
    }
  },
  computed: {
    svgTransform() {
      if (this.direction === "up") {
        return "rotate(180deg)"
      }
      if (this.direction === "left") {
        return "rotate(90deg)"
      }
      if (this.direction === "right") {
        return "rotate(-90deg)"
      }
      if (this.direction === "down") {
        return "rotate(0deg)"
      }
      return "none"
    }
  }
};
</script>

<style></style>