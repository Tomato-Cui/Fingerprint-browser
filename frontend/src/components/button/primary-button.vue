<script setup>
const props = defineProps({
    class: '',
});

const emit = defineEmits(['click', 'submit'])
</script>

<template>
    <button @click="() => emit('click')" @submit="() => emit('submit')"
        :class="['bg-primary text-white py-2 px-4 rounded-sm hover:bg-blue-700 transform ease-in-out delay-150 duration-300', props.class]">
        <slot />
    </button>
</template>