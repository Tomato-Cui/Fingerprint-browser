<script setup lang="ts">
import { ref, watch } from 'vue';
import { AlertModel } from '@/components/alert-model';
import { environment_batch_delete } from '@/commands/environment';
import { useBrowserStatusStore } from "@/stores/browser";
import { toast } from 'vue-sonner';
import { itemFromKind } from '@tauri-apps/api/menu';

const browserStatusStore = useBrowserStatusStore();
const props = defineProps<{
    open: Boolean,
    data: any[],
    isChange: Boolean,
}>()
const emit = defineEmits(['close', 'update:isChange'])

const cancel = () => {
    emit('close')
}
const commit = async () => {
    
    try {
        // 检查是否有环境正在使用中
        for (const e of props.data) {
            if (browserStatusStore.getStatus(e.uuid)) {
                toast.warning('有环境正在使用中，无法删除');
                return; // 如果有环境正在使用，直接退出
            }
        }

        // 如果所有环境都未使用，执行删除操作
        await environment_batch_delete(props.data.map(e => e.uuid));
        toast.success('删除环境成功');

        // 删除成功后触发事件
        emit('update:isChange', !props.isChange);
        emit('close');
    } catch (error) {
        console.error('删除环境失败:', error);
        toast.error('删除环境失败，请重试');
    }
};
</script>

<template>
    <AlertModel :open="props.open" title="删除环境" @close="emit('close')" @submit="commit" @cancel="cancel">
        <div class="px-7">确定删除已选的{{ props.data.length }}个环境？</div>
    </AlertModel>
</template>