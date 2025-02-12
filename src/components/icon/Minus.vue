<template>
    <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 16 16"
        :height="size"
        :width="size"
    >
        <path
            d="M15.839999999999998 8c0 0.6671535714285713 -0.539 1.2061535714285714 -1.2061535714285714 1.2061535714285714H1.3661535714285713C0.699 9.20615357142857 0.16 8.667153571428571 0.16 8s0.539 -1.2061535714285714 1.2061535714285714 -1.2061535714285714h13.267692857142857c0.6671535714285713 0 1.2061535714285714 0.539 1.2061535714285714 1.2061535714285714Z"
            :fill="color"
            stroke-width="0.0357"
        ></path>
    </svg>
</template>

<script>
export default {
    name: "Minus",
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
