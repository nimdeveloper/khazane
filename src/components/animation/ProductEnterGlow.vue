<template>
    <SharedTransitionControl :enter="enterAnimation">
        <slot />
    </SharedTransitionControl>
</template>

<script lang="ts" setup>
import { type AnimeParams } from "animejs";

const enterAnimation: AnimeParams = {
    translateY: [20, 0],
    opacity: [0, 1],
    duration: 600,
    keyframes: [
        {
            borderWidth: "2px",
            borderStyle: "solid",
            borderColor: "transparent",
        },
        {
            borderWidth: "0px",
            borderStyle: "solid",
            duration: 1,
        },
    ],
};
</script>

<style lang="scss"></style>
