<script setup lang="ts">
import { Model } from '@/components/model';
import primaryButton from '@/components/button/primary-button.vue';

const emit = defineEmits(['close'])
const props = defineProps<{
    open: Boolean,
    data: any[],
}>()
</script>

<template>
    <Model :open="props.open" @close="emit('close')" title="提示">
        <div class="px-7">勾选的环境不处于“转移中”状态，无法取消转移。</div>
        <!-- footer -->
        <div class="px-4 py-2 w-full flex items-center justify-end space-x-3">
            <primaryButton @click="emit('close')">我知道了</primaryButton>
        </div>
    </Model>
    <Model :open="props.open" @close="emit('close')" title="勾选项目正在转移">
        <div class="px-7">您已选择 1 个代理，该环境正在转移中</div>
        <!-- footer -->
        <div class="px-4 py-2 w-full flex items-center justify-end space-x-3">
            <primaryButton @click="emit('close')">我知道了</primaryButton>
        </div>
    </Model>
</template>