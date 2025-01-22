<script setup lang="ts">
import { ref } from 'vue';
import { AlertModel } from '@/components/alert-model';

const props = defineProps<{
    open: Boolean,
    data: any[],
}>()
const emit = defineEmits(['close'])

const cancel = () => {
    emit('close')
}
const commit = () => {
    emit('close')
}
</script>

<template>
    <AlertModel :open="props.open" title="删除环境" @close="emit('close')" @submit="commit" @cancel="cancel">
        <div class="px-7">确定删除已选的{{ props.data.length }}个环境？</div>
    </AlertModel>
</template>