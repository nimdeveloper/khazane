<template>
    <div class="flex flex-col h-full overflow-hidden">
        <HeaderNavbar />
        <div class="pt-5 flex grow-1 overflow-hidden print:p-0.5">
            <slot />
        </div>
    </div>
</template>
