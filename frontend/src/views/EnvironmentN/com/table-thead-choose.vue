<script setup lang="ts">
import { ref, onMounted, defineEmits, defineProps, watch, onUnmounted } from 'vue'
import { FrameIcon } from '@/assets/icons/environment'

onMounted(() => {
    copyColumns.value = JSON.parse(JSON.stringify(allColumns.value))
    emit('select', allColumns.value)
})

const props = defineProps({
    open: Boolean
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

const screenHeight = ref(window.innerHeight);
const divHeight = ref<HTMLElement | null>(null)
const updateHeight = () => {
    screenHeight.value = window.innerHeight;
    if (screenHeight.value <= 1060) {
        if (divHeight.value) {
            divHeight.value.style.height = screenHeight.value - 550 + 'px';
            divHeight.value.style.overflow = 'auto';
        }
    }
    else {
        if (divHeight.value) {
            divHeight.value.style.height = 'auto'
        }
    }
};
watch(() => props.open, (_) => {
    screenHeight.value = window.innerHeight;
    if (screenHeight.value <= 1060) {
        if (divHeight.value) {
            divHeight.value.style.height = screenHeight.value - 550 + 'px';
            divHeight.value.style.overflow = 'auto';
        }
    }
    else {
        if (divHeight.value) {
            divHeight.value.style.height = 'auto'
        }
    }
})
onMounted(() => {
    window.addEventListener('resize', updateHeight);
});
onUnmounted(() => {
    window.removeEventListener('resize', updateHeight);
});

// 拖拽排序逻辑
let draggedIndex = ref<number | null>(null)
let dragOverIndex = ref<number | null>(null)
let dragTimeout = ref<any>(null)

const handleDragStart = (event: DragEvent, index: number) => {
    draggedIndex.value = index
    if (event.dataTransfer) {
        event.dataTransfer.effectAllowed = 'move'
        event.dataTransfer.setData('text/plain', index.toString())
    }
}

const handleDragEnter = (event: DragEvent, index: number) => {
    event.preventDefault()
    if (draggedIndex.value === null || draggedIndex.value === index || !isDraggable(allColumns.value[index])) return
    
    if (dragTimeout.value !== null) {
        clearTimeout(dragTimeout.value)
    }

    dragTimeout.value = setTimeout(() => {
        const draggedItem = allColumns.value[draggedIndex.value!]
        const newIndex = index > draggedIndex.value! ? index - 1 : index
        allColumns.value.splice(draggedIndex.value!, 1)
        allColumns.value.splice(newIndex, 0, draggedItem)
        draggedIndex.value = newIndex
        dragOverIndex.value = newIndex
    }, 100)
}

const isDraggable = (column: { key: string }) => {
    return column.key !== 'id'
}

const handleDragLeave = (index: number) => {
    if (dragOverIndex.value === index) {
        dragOverIndex.value = null
    }
}

const handleDrop = (event: DragEvent) => {
    event.preventDefault()
    if (dragTimeout.value !== null) {
        clearTimeout(dragTimeout.value)
    }
    dragOverIndex.value = null
}

const handleDragEnd = () => {
    if (dragTimeout.value !== null) {
        clearTimeout(dragTimeout.value)
    }
    draggedIndex.value = null
    dragOverIndex.value = null
}

</script>

<template>
    <Transition name="modal-fade">
        <div v-show="props.open" class="mt-16 mr-4 min-w-72 rounded-lg bg-white p-4 shadow-lg border">
            <div class="mb-4 flex items-center justify-between">
                <h3 class="text-lg font-medium">自定义表格字段</h3>
            </div>
            <div class="space-y-3" ref="divHeight">
                <TransitionGroup name="list" tag="div">
                    <div v-for="(column, index) in allColumns" :key="column.key"
                        class="flex items-center justify-between pr-2 space-y-3" :draggable="isDraggable(column)"
                        :class="{ 'opacity-50': draggedIndex === index, 'border-t-2 border-blue-500': dragOverIndex === index }"
                        @dragstart="handleDragStart($event, index)" @dragenter="handleDragEnter($event, index)"
                        @dragover.prevent @dragleave="handleDragLeave($event, index)" @drop="handleDrop($event, index)"
                        @dragend="handleDragEnd">
                        <FrameIcon class="size-5"
                            :class="{ 'cursor-move': isDraggable(column), 'text-gray-300': column.key === 'id', 'text-gray-700': column.key !== 'id' }" />
                        <span class="text-sm w-full bg-[#EDEDFF80] h-[40px] pl-4 rounded-lg flex items-center mx-4">
                            {{ column.label }}
                        </span>
                        <label class="relative inline-flex items-center"
                            :class="{ 'cursor-not-allowed': column.key == 'id', 'cursor-pointer': column.key != 'id' }">
                            <input type="checkbox" v-model="column.visible" :disabled="column.key == 'id'"
                                class="peer sr-only">
                            <div class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-'' peer-checked:bg-blue-600 peer-checked:after:translate-x-full"
                                :class="{ 'peer-checked:bg-blue-300': column.key == 'id' }">
                            </div>
                        </label>
                    </div>
                </TransitionGroup>
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
/* 滚动条整体样式 */
::-webkit-scrollbar {
    width: 5px;
    /* 滚动条宽度 */
}

/* 滚动条轨道 */
::-webkit-scrollbar-track {
    background-color: #ffffff;
    /* 轨道背景色 */
    border-radius: 6px;
    /* 圆角 */
}

/* 滚动条滑块 */
::-webkit-scrollbar-thumb {
    background-color: #be4242;
    /* 滑块颜色 */
    border-radius: 6px;
    /* 圆角 */
    border: 3px solid #c8c8c8;
    /* 滑块边框 */
}

/* 滚动条滑块悬停状态 */
::-webkit-scrollbar-thumb:hover {
    background-color: #2c5a8b;
    /* 悬停时滑块颜色 */
}

.modal-fade-enter-active,
.modal-fade-leave-active {
    transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
    opacity: 0;
}

.list-move,
.list-enter-active,
.list-leave-active {
    transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
    opacity: 0;
    transform: translateX(30px);
}

.list-leave-active {
    position: absolute;
}
</style>