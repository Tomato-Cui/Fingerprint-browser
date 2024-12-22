<script setup>
import {
    CircleAlert as CircleAlertIcon
} from 'lucide-vue-next'
import {
    AlertDialog,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { X } from 'lucide-vue-next'
import { PrimaryButton, CancelButton } from '@/components/button';
import { computed } from 'vue';

const props = defineProps({
    type: '',
    class: '',
    title: '',
    open: false,
})
const emit = defineEmits(['close', 'cancel', 'submit'])

const icon = computed(() => {
    if (props.type === 'warn') {
        return CircleAlertIcon
    }
    return CircleAlertIcon
});
const color = computed(() => {
    if (props.type === 'warn') {
        return 'text-orange-400'
    }
    return 'text-orange-400'
});

</script>

<template>
    <AlertDialog :open="props.open">
        <AlertDialogContent :class="`p-0 ${props.class}`">
            <AlertDialogHeader class="border-b p-4 pt-6 flex flex-row justify-between">
                <AlertDialogTitle class="text-xl flex gap-x-2 items-center">
                    <component :is="icon" :class="[color]" /> {{ props.title || '&nbsp;' }}
                </AlertDialogTitle>
                <div @click="() => emit('close')" class="cursor-pointer">
                    <X class="w-8 h-8 text-gray-400" />
                </div>
            </AlertDialogHeader>
            <div class="px-4 py-2">
                <slot></slot>
                <div class="flex justify-end gap-x-4 text-sm">
                    <CancelButton @click="() => emit('cancel')">取消</CancelButton>
                    <PrimaryButton @click="() => emit('submit')">确定</PrimaryButton>
                </div>
            </div>
        </AlertDialogContent>
    </AlertDialog>
</template>