<script setup lang="ts">
import { ref, watch } from 'vue';
import { Model } from '@/components/model';
import primaryButton from '@/components/button/primary-button.vue';
import cancelButton from '@/components/button/cancel-button.vue';
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/select";
import { environment_fingerprint_modify_ua } from '@/commands/environment-fingerprint';
import { generateRandomUserAgent } from '@/util/lib'
import { toast } from 'vue-sonner';

const props = defineProps<{
    open: Boolean,
    data: any[],
}>()
const emit = defineEmits(['close'])

const selectUA = ref()
const UAList = ref<{key: string, name: string, value: string}[]>([
    { key: '130', name: '130', value: '' },
    { key: '129', name: '129', value: '' },
    { key: '128', name: '128', value: '' },
    { key: '127', name: '127', value: '' },
    { key: '126', name: '126', value: '' },
])
watch(() => props.open, (newVal) => {
    if (newVal) {
        for (let i = 0; i < 5; i++) {
            UAList.value[i].value = generateRandomUserAgent()
        }
    }
})

const cancel = () => {
    emit('close')
}
const commit = async () => {
    try {
        for (let i = 0; i < props.data.length; i++) {
            await environment_fingerprint_modify_ua(props.data[i]?.fp_id, selectUA.value)
        }
        console.log('修改UA:', selectUA.value, props.data);
        
        toast.success('修改UA成功');
        emit('close')
    } catch (error) {
        console.log('修改UA:', error);
        toast.error('修改UA失败，请重试:', error || '未知错误');
    }
}
</script>

<template>
    <Model :open="props.open" title="修改UA" @close="emit('close')" class="min-w-[600px]">
        <div class="w-full py-6 px-[30px] space-y-8">
            <p>已选择项目 {{ props.data.length }}</p>
            <div class="flex w-full space-x-[100px]">
                <span class="font-[600]">UA类型</span>
                <div class="flex-1">
                    <Select v-model="selectUA">
                        <SelectTrigger>
                            <SelectValue placeholder="请选择"
                                class="p-2 w-full rounded-lg outline-none" />
                        </SelectTrigger>

                        <SelectContent>
                            <SelectGroup>
                                <SelectItem v-for="item in UAList" :value="item.value">{{ item.name }}</SelectItem>
                            </SelectGroup>
                        </SelectContent>
                    </Select>
                    <p class="text-gray-400 text-[13px]">内容将根据UA智能匹配</p>
                </div>
            </div>
        </div>

        <!-- footer -->
        <div class="px-4 py-2 w-full flex items-center justify-end border-t space-x-3">
            <cancelButton @click="cancel">取消</cancelButton>
            <primaryButton @click="commit">确认</primaryButton>
        </div>
    </Model>

</template>