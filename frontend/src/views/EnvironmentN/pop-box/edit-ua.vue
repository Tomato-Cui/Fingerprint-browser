<script setup lang="ts">
import { ref } from 'vue';
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

const props = defineProps<{
    open: Boolean
}>()
const emit = defineEmits(['close'])

const selectUA = ref()
const UAList = ref([
    { key: '130', name: '130' },
    { key: '129', name: '129' },
    { key: '128', name: '128' },
    { key: '127', name: '127' },
    { key: '126', name: '126' },
])

const cancel = () => {
    emit('close')
}
const commit = () => {
    emit('close')
}
</script>

<template>
    <Model :open="props.open" title="修改UA" @close="emit('close')" class="min-w-[600px]">

        <div class="w-full py-6 px-[30px] space-y-8">
            <p>已选择项目 1</p>
            <div class="flex w-full space-x-[100px]">
                <span class="font-[600]">UA类型</span>
                <div class="flex-1">
                    <Select>
                        <SelectTrigger>
                            <SelectValue v-model="selectUA" placeholder="所有环境"
                                class="p-2 w-full rounded-lg outline-none" />
                        </SelectTrigger>

                        <SelectContent>
                            <SelectGroup>
                                <SelectItem v-for="item in UAList" :value="item.key">{{ item.name }}</SelectItem>
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