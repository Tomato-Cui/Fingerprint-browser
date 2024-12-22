<script setup>
const props = defineProps({
    class: '',
});

const emit = defineEmits(['click', 'submit'])
</script>

<template>
    <button @click="() => emit('click')" @submit="() => emit('submit')"
        :class="['py-2 px-4 rounded-sm hover:bg-gray-200 transform ease-in-out delay-150 duration-200', props.class]">
        <slot />
    </button>
</template>