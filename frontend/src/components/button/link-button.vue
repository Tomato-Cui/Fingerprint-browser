<script setup>
const props = defineProps({
    class: '',
});

const emit = defineEmits(['click', 'submit'])
</script>

<template>
    <button @click="() => emit('click')" @submit="() => emit('submit')"
        :class="['text-blue-600 py-2 px-4 hover:text-blue-400 hover:underline transform ease-in-out delay-150 duration-300', props.class]">
        <slot />
    </button>
</template>