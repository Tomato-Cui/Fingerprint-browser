<script setup>
const props = defineProps({
    class: '',
});

const emit = defineEmits(['click', 'submit'])
</script>

<template>
    <button @click="() => emit('click')" @submit="() => emit('submit')"
        :class="['border border-slate-200 text-black active:bg-blue-50 hover:border-primary hover:text-primary py-2 px-4 rounded-sm transform ease-in-out duration-100', props.class]">
        <slot />
    </button>
</template>