<template>
    <Transition
        @beforeEnter="onBeforeEnter"
        @beforeAppear="onBeforeAppear"
        @enter="onEnter"
        @appear="onAppear"
        @leave="onLeave"
        @enterCancelled="onEnterCancelled"
        @appearCancelled="onAppearCancelled"
        @leaveCancelled="onLeaveCancelled"
        appear
        type="animation"
        :css="false"
        ><slot
    /></Transition>
</template>

<script lang="ts" setup>
import anime, { type AnimeInstance } from "animejs";

const { appear, enter, leave } = defineProps<{
    appear?: Partial<anime.AnimeParams>;
    enter?: Partial<anime.AnimeParams>;
    leave?: Partial<anime.AnimeParams>;
}>();

const animationInstance = ref<AnimeInstance | null>(null);
const onBeforeEnter = (el: Element) => {
    console.log("Before Enter");
};
const onBeforeAppear = (el: Element) => {
    console.log("Before Appear");
};
const onEnter = async (el: Element, done: () => void) => {
    console.log("Enter");
    if (animationInstance.value) {
        animationInstance.value.pause();
        animationInstance.value = null;
    }
    let anim;
    if (enter) {
        anim = enter;
    }
    if (anim) {
        anim.targets = [el];
        anim.autoplay = false;
        anim.loop = false;
        animationInstance.value = anime({
            ...anim,
        });
        animationInstance.value.play();
        animationInstance.value.finished.then(() => {
            done();
            animationInstance.value?.pause();
            animationInstance.value = null;
        });
    }
};
const onAppear = async (el: Element, done: () => void) => {
    console.log("Appear");
    if (animationInstance.value) {
        animationInstance.value.pause();
        animationInstance.value = null;
    }
    let anim;
    if (appear) {
        anim = appear;
    } else if (enter) {
        anim = enter;
    }
    if (anim) {
        anim.targets = [el];
        anim.autoplay = false;
        anim.loop = false;
        animationInstance.value = anime({
            ...anim,
        });
        animationInstance.value.play();
        animationInstance.value.finished.then(() => {
            done();
            animationInstance.value?.pause();
            animationInstance.value = null;
        });
    }
};
const onLeave = async (el: Element, done: () => void) => {
    console.log("Leave");
    if (animationInstance.value) {
        animationInstance.value.pause();
        animationInstance.value.seek(0);
        animationInstance.value = null;
    }
    let anim;
    let reversePlay = false;
    if (leave) {
        anim = leave;
    } else if (appear) {
        anim = appear;
        reversePlay = true;
    } else if (enter) {
        anim = enter;
        reversePlay = true;
    }
    if (anim) {
        anim.targets = [el];
        anim.autoplay = false;
        anim.loop = false;
        animationInstance.value = anime({
            ...anim,
        });
        animationInstance.value.play();
        if (reversePlay) {
            animationInstance.value.reverse();
            animationInstance.value.seek(1);
        }
        await animationInstance.value.finished;
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
