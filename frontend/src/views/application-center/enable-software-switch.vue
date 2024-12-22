<script setup>
import { ref } from 'vue'

const switchStatus = ref(false);
const emit = defineEmits(['click']);

const switchOpenHandle = () => {
    if (!switchStatus.value) {
        emit('click');
    }
}
</script>

<template>
    <Switch v-model:checked="switchStatus" @click="switchOpenHandle" />
</template>