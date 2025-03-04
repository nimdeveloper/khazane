<template>
    <Transition
        @onBeforeEnter="onBeforeEnter"
        @onBeforeAppear="onBeforeAppear"
        @onEnter="onEnter"
        @onAppear="onAppear"
        @onLeave="onLeave"
        @onEnterCancelled="onEnterCancelled"
        @onAppearCancelled="onAppearCancelled"
        @onLeaveCancelled="onLeaveCancelled"
        appear
        :css="false"
        ><slot
    /></Transition>
</template>

<script lang="ts" setup>
const onBeforeEnter = (el: Element) => {};
const onBeforeAppear = (el: Element) => {};
const onEnter = (el: Element, done: () => void) => {};
const onAppear = (el: Element, done: () => void) => {};
const onLeave = (
    el: Element & { _isLeaving?: boolean; _enterCancelled?: boolean },
    done: () => void
) => {};
const onEnterCancelled = (el: Element) => {};
const onAppearCancelled = (el: Element) => {};
const onLeaveCancelled = (el: Element) => {};
</script>
