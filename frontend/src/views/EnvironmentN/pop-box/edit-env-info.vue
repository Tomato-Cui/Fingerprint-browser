<template>
    <Model :open="props.open" title="修改环境信息" @close="emit('close')" class="min-w-[700px]">
        <!-- 内容区域 -->
        <div class="p-6 space-y-6">
            <!-- 提示信息 -->
            <div class="rounded-lg p-2 space-y-2 border shadow-xl">
                <div class="flex gap-2 items-center">
                    <InfoIcon class="w-5 h-5 text-blue-500 flex-shrink-0" />
                    <div class="text-sm text-gray-600">
                        <p>1.仅支持更新导出Profile文件的信息，单个文件环境数不能超过500个</p>
                        <p>2.不想更新某列的信息，请勿改动或保持为"空"</p>
                    </div>
                </div>
            </div>

            <!-- 文件上传区域 -->
            <div class="border-2 border-dashed border-[#C5CEE0] rounded-lg px-8 py-12 bg-[#F7F9FC]"
                :class="{ 'border-blue-500 bg-blue-50': isDragging }" @dragenter.prevent="isDragging = true"
                @dragleave.prevent="isDragging = false" @dragover.prevent @drop.prevent="handleDrop"
                @click="triggerFileInput">
                <div class="flex flex-col items-center justify-center space-y-4">
                    <div class="w-16 h-16 bg-emerald-100 rounded-lg flex items-center justify-center">
                        <FormatIcon class="w-8 h-8 text-emerald-500" />
                    </div>
                    <div class="text-center">
                        <span class="text-blue-500 cursor-pointer">点击上传</span>
                        <span class="text-gray-500">或将文件拖拽至此</span>
                    </div>
                    <input type="file" ref="fileInput" class="hidden" @change="handleFileSelect"
                        accept=".xlsx,.xls,.csv" />
                </div>
            </div>

            <!-- 已上传文件 -->
            <div v-if="uploadedFile" class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                <div class="flex items-center gap-3">
                    <FileIcon class="w-5 h-5 text-gray-500" />
                    <span class="text-sm text-gray-700">{{ uploadedFile.name }}</span>
                </div>
                <button @click="removeFile" class="text-gray-400 hover:text-gray-600">
                    <XIcon class="w-5 h-5" />
                </button>
            </div>

            <!-- 注意事项 -->
            <div class="space-y-2 text-sm text-gray-600">
                <h3 class="font-medium">注意:</h3>
                <ul class="space-y-1 list-inside">
                    <li>1、需先导出需要修改信息的环境</li>
                    <li>2、将需要修改的信息填写在对应的表格中</li>
                    <li>3、填写保存后，上传文件后即可更新现盖原信息「端对端加密」属性暂不支持修改</li>
                </ul>
            </div>
        </div>

        <!-- 底部按钮 -->
        <div class="px-4 py-2 w-full flex items-center justify-end border-t space-x-3">
            <cancelButton @click="cancel">取消</cancelButton>
            <primaryButton @click="commit">确认</primaryButton>
        </div>
    </Model>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Model } from '@/components/model';
import primaryButton from '@/components/button/primary-button.vue';
import cancelButton from '@/components/button/cancel-button.vue';
import { XIcon, InfoIcon, FileIcon } from 'lucide-vue-next'
import { FormatIcon } from '@/assets/icons/environment/operate';

const props = defineProps<{
    open: Boolean,
    data: any[],
}>()
const emit = defineEmits(['close', 'confirm'])
const isDragging = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)
const uploadedFile = ref<File | null>(null)

const triggerFileInput = () => {
    fileInput.value?.click()
}

const handleFileSelect = (event: Event) => {
    const input = event.target as HTMLInputElement
    if (input.files && input.files[0]) {
        uploadedFile.value = input.files[0]
    }
}

const handleDrop = (event: DragEvent) => {
    isDragging.value = false
    if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
        uploadedFile.value = event.dataTransfer.files[0]
    }
}

const removeFile = () => {
    uploadedFile.value = null
    if (fileInput.value) {
        fileInput.value.value = ''
    }
}

const cancel = () => {
    emit('close')
}
const commit = () => {
    emit('close')
}
</script>