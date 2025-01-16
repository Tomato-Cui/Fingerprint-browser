<script setup>
import { ref, onMounted, defineEmits, defineProps } from 'vue'
import { FrameIcon } from '@/assets/icons/environment'

onMounted(() => {
    copyColumns.value = JSON.parse(JSON.stringify(allColumns.value))
    emit('select', allColumns.value)
})

const props = defineProps({
    open: false
})
const emit = defineEmits(['close', 'select'])
const copyColumns = ref([])
const allColumns = ref([
    { key: 'id', label: '序号', visible: true },
    { key: 'name', label: '环境名称', visible: true },
    { key: 'action', label: '操作', visible: true },
    { key: 'status', label: '状态', visible: true },
    { key: 'account', label: '账号信息', visible: true },
    { key: 'proxy', label: '代理信息', visible: true },
    { key: 'description', label: '备注', visible: false },
    { key: 'tab', label: '标签', visible: false },
    { key: 'groupName', label: '分组', visible: false },
    { key: 'create_at', label: '创建信息', visible: false },
    { key: 'lastOpen', label: '最后启动信息', visible: false },
])

const saveChoose = () => {
    copyColumns.value = JSON.parse(JSON.stringify(allColumns.value))
    emit('select', allColumns.value)
    emit('close')
}
const cancelChoose = () => {
    allColumns.value = JSON.parse(JSON.stringify(copyColumns.value))
    emit('select', allColumns.value)
    emit('close')
}
</script>

<template>
    <Transition name="modal-fade">
        <div v-show="props.open" class="mt-16 mr-4 min-w-72 rounded-lg bg-white p-4 shadow-lg border">
            <div class="mb-4 flex items-center justify-between">
                <h3 class="text-lg font-medium">自定义表格字段</h3>
                <button @click="showColumnsModal = false">
                    <XIcon class="h-4 w-4" />
                </button>
            </div>
            <div class="space-y-3">
                <div v-for="column in allColumns" :key="column.key" class="flex items-center justify-between">
                    <FrameIcon class="size-5" />
                    <span class="text-sm w-full bg-[#EDEDFF80] h-[40px] pl-4 rounded-lg flex items-center mx-4">{{
                        column.label }}</span>
                    <label class="relative inline-flex items-center"
                        :class="{ 'cursor-not-allowed': column.key == 'id', 'cursor-pointer': column.key != 'id' }">
                        <input type="checkbox" v-model="column.visible" :disabled="column.key == 'id'"
                            class="peer sr-only">
                        <div class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-'' peer-checked:bg-blue-600 peer-checked:after:translate-x-full"
                            :class="{ 'peer-checked:bg-blue-300': column.key == 'id' }">
                        </div>
                    </label>
                </div>
            </div>
            <div class="mt-4 flex justify-end space-x-2">
                <button @click="cancelChoose" class="rounded-md px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-50">
                    取消
                </button>
                <button @click="saveChoose"
                    class="rounded-md bg-blue-600 px-3 py-1.5 text-sm text-white hover:bg-blue-700">
                    确认
                </button>
            </div>
        </div>
    </Transition>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
    transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
    opacity: 0;
}
</style>