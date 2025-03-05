<template>
    <TransitionGroup
        @beforeEnter="onBeforeEnter"
        @beforeAppear="onBeforeAppear"
        @enter="onEnter"
        @leave="onLeave"
        @enterCancelled="onEnterCancelled"
        @appearCancelled="onAppearCancelled"
        @leaveCancelled="onLeaveCancelled"
        type="animation"
        :css="false"
        ><slot
    /></TransitionGroup>
</template>

<script lang="ts" setup>
import anime, { type AnimeInstance } from "animejs";

const { enter, leave } = defineProps<{
    enter?: Partial<anime.AnimeParams>;
    leave?: Partial<anime.AnimeParams>;
}>();

const animationInstances = ref<Map<Element, AnimeInstance>>(new Map());

const onBeforeEnter = (el: Element) => {
    console.log("Before Enter");
};
const onBeforeAppear = (el: Element) => {
    console.log("Before Appear");
};
const onEnter = (el: Element, done: () => void) => {
    console.log("Enter");
    const oldAnim = animationInstances.value?.get(el);
    if (oldAnim) {
        oldAnim.pause();
        animationInstances.value?.delete(el);
    }
    let anim;
    if (enter) {
        anim = enter;
    }
    if (anim) {
        anim.targets = [el];
        anim.autoplay = false;
        anim.loop = false;
        const newAnim = anime({
            ...anim,
        });
        newAnim.play();
        if (animationInstances.value) {
            animationInstances.value.set(el, newAnim);
        }
        newAnim.finished.then(() => {
            done();
            animationInstances.value.delete(el);
        });
    }
};
const onLeave = async (el: Element, done: () => void) => {
    console.log("Leave");
    const oldAnim = animationInstances.value?.get(el);
    if (oldAnim) {
        oldAnim.pause();
        animationInstances.value?.delete(el);
    }

    let anim;
    let reversePlay = false;
    if (leave) {
        anim = leave;
    } else if (enter) {
        anim = enter;
        reversePlay = true;
    }
    if (anim) {
        anim.targets = [el];
        anim.autoplay = false;
        anim.loop = false;
        const newAnim = anime({
            ...anim,
        });
        newAnim.play();
        if (reversePlay) {
            newAnim.reverse();
            newAnim.seek(0);
        }
        if (animationInstances.value) {
            animationInstances.value.set(el, newAnim);
        }
        newAnim.finished.then(() => {
            done();
            animationInstances.value.delete(el);
        });
    }
};
const onEnterCancelled = (el: Element) => {
    console.log("Enter Canceled");
};
const onAppearCancelled = (el: Element) => {
    console.log("Appear Cancel");
};
const onLeaveCancelled = (el: Element) => {
    console.log("Leave Cancel");
};
</script>
