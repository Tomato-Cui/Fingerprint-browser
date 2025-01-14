<script setup>
const props = defineProps({
    title: '',
});
</script>
<template>

    <div class="space-y-2 flex items-center">
        <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">{{ props.title }}
        </p>
        <div class="w-full">
            <div class="flex gap-4 ">
                <slot />
            </div>
        </div>
    </div>
</template>