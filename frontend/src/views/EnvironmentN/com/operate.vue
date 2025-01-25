<template>
    <Transition name="modal-fade">
        <div v-show="props.open"
            class="mt-16 absolute border mr-4 w-[320px] rounded-lg bg-white p-4 shadow-lg z-40 right-0 top-0">
            <div class="mb-4 flex items-center justify-between border-b pb-1">
                <h3 class="text-lg font-medium">快捷设置</h3>
                <TooltipButton title="重置">
                    <RefreshSquareIcon class="cursor-pointer size-6" @click="refresh" />
                </TooltipButton>
            </div>
            <div class="space-y-2 pr-2" ref="divHeight">
                <div class="flex items-center justify-between gap-4">
                    <FrameIcon class="size-5 text-gray-300" />
                    <div class="w-full h-[40px] bg-[#7744ff] flex rounded-lg items-center">
                        <RoundArrowRight class="size-6 mx-3" />
                        <span class="text-white">启动</span>
                        <div class="flex-1 flex justify-end">
                            <HelfGlobalIcon class="size-[51px] opacity-10" />
                        </div>
                    </div>
                    <label class="relative inline-flex cursor-pointer items-center">
                        <input type="checkbox" v-model="allActions[0].visible" disabled="true" class="peer sr-only">
                        <div
                            class=" cursor-not-allowed h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-400 peer-checked:after:translate-x-full">
                        </div>
                    </label>
                </div>
                <TransitionGroup name="list" tag="div">
                    <div v-for="(action, index) in allActions" :key="action.key"
                        class="flex items-center justify-between gap-4 transition-all duration-300 ease-in-out space-y-2"
                        v-show="action.key !== 'start'"
                        :class="{ 'opacity-50': draggedIndex === index, 'border-t-2 border-blue-500': dragOverIndex === index }"
                        draggable="true" @dragstart="handleDragStart($event, index)"
                        @dragenter="handleDragEnter($event, index)" @dragover.prevent
                        @dragleave="handleDragLeave($event, index)" @drop="handleDrop($event, index)"
                        @dragend="handleDragEnd">
                        <FrameIcon class="size-5 text-gray-700 cursor-move" />
                        <div class="flex items-center bg-[#EDEDFF80] w-full h-[40px] pl-[10px] rounded-lg">
                            <component :is="action.icon" class="mr-2 h-4 w-4" />
                            <span class="text-sm flex-1">{{ action.label }}</span>
                        </div>
                        <label class="relative inline-flex cursor-pointer items-center">
                            <input type="checkbox" v-model="action.visible" class="peer sr-only">
                            <div
                                class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full">
                            </div>
                        </label>
                    </div>
                </TransitionGroup>
            </div>
            <div class="mt-4 flex justify-end space-x-2">
                <button @click="cancelSet" class="rounded-md px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-50">
                    取消
                </button>
                <button @click="saveQuickSettings"
                    class="rounded-md bg-blue-600 px-3 py-1.5 text-sm text-white hover:bg-blue-700">
                    确认
                </button>
            </div>
        </div>
    </Transition>

</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, defineProps, defineEmits } from 'vue'
import { RefreshSquareIcon, StopCircleIcon, HelfGlobalIcon, RoundArrowRight, BookmarkCircleIcon, FileTextIcon, FileRightIcon, RulerPenIcon, RuleCrossPenIcon, PlateIcon, HomeSmileIcon, RoundTransferHorizontal, ForbidRoundTransferHorizontal, ClearCacheIcon, TrashBinTrashIcon, FrameIcon } from '@/assets/icons/environment/index'
import { environment_group_query } from '@/commands/environment-group'
import TooltipButton from "@/components/tooltip-button.vue";

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
    if (draggedIndex.value === null || draggedIndex.value === index) return

    // 清除之前的超时
    if (dragTimeout.value !== null) {
        clearTimeout(dragTimeout.value)
    }

    // 设置一个新的超时来执行交换
    dragTimeout.value = setTimeout(() => {
        const rect = (event.target as HTMLElement).getBoundingClientRect()
        const y = event.clientY - rect.top
        const height = rect.bottom - rect.top

        if (y > height / 2) {
            // 如果鼠标在元素的下半部分，就把当前元素移到拖动元素的后面
            if (index < draggedIndex.value!) {
                return
            }
        } else {
            // 如果鼠标在元素的上半部分，就把当前元素移到拖动元素的前面
            if (index > draggedIndex.value!) {
                return
            }
        }

        const draggedItem = allActions.value[draggedIndex.value!]
        allActions.value.splice(draggedIndex.value!, 1)
        allActions.value.splice(index, 0, draggedItem)
        draggedIndex.value = index
        dragOverIndex.value = index
    }, 1) // 100毫秒的延迟
}

const handleDragLeave = (event: DragEvent, index: number) => {
    if (dragOverIndex.value === index) {
        dragOverIndex.value = null
    }
}

const handleDrop = (event: DragEvent, index: number) => {
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

onMounted(() => {
    emit('select', allActions.value)
    copyActions.value = JSON.parse(JSON.stringify(allActions.value))  //数组深拷贝
})

const props = defineProps<{
    open: boolean,
    groupData: Array<any>,
}>()
const emit = defineEmits(['select', 'close', 'startAll', 'stopAll', 'addLabel', 'exportEnv', 'editProxy', 'editEnvInfo', 'transferEnv', 'untransferEnv', 'cleanCache', 'delEnv', 'addLabel', 'resetLabel', 'cleanLabel', 'addGroup', 'editStartPage', 'editUa'])

const allActions = ref([
    { key: 'start', label: '启动', icon: StopCircleIcon, visible: true, action: () => emit('startAll') },
    { key: 'stop', label: '停止', icon: StopCircleIcon, visible: true, action: () => emit('stopAll') },
    { key: 'tag', label: '设置标签', icon: BookmarkCircleIcon, visible: true, action: void (0), children: [{ key: 'addTab', label: '新增标签', active: () => emit('addLabel') }, { key: 'reset', label: '重设标签', active: () => emit('resetLabel') }, { key: 'clean', label: '清空标签', active: () => emit('cleanLabel') }] },
    { key: 'group', label: '设置分组', icon: FileTextIcon, visible: true, action: void (0), children: [{ key: 'addGroup', label: '增加分组', active: () => emit('addGroup') }] },
    { key: 'export', label: '导出环境', icon: FileRightIcon, visible: true, action: () => emit('exportEnv') },
    // , children: [{ key: 'export1', label: '导出已选', active: () => emit('exportEnv') }, { key: 'export2', label: '导出最近50条', active: () => emit('exportEnv') }] 
    { key: 'edit', label: '修改启动页', icon: RulerPenIcon, visible: true, action: () => emit('editStartPage') },
    { key: 'ua', label: '修改UA', icon: RuleCrossPenIcon, visible: true, action: () => emit('editUa') },
    { key: 'proxy', label: '修改代理', icon: PlateIcon, visible: true, action: () => emit('editProxy') },
    { key: 'env', label: '修改环境信息', icon: HomeSmileIcon, visible: true, action: () => emit('editEnvInfo') },
    { key: 'transfer', label: '转移', icon: RoundTransferHorizontal, visible: true, action: () => emit('transferEnv') },
    { key: 'cancelTransfer', label: '取消转移', icon: ForbidRoundTransferHorizontal, visible: true, action: () => emit('untransferEnv') },
    { key: 'clearCache', label: '清除缓存', icon: ClearCacheIcon, visible: true, action: () => emit('cleanCache') },
    { key: 'delEnvironment', label: '删除环境', icon: TrashBinTrashIcon, visible: true, action: () => emit('delEnv') },
])
const refresh = () => {
    allActions.value = [
        { key: 'start', label: '启动', icon: StopCircleIcon, visible: true, action: () => emit('startAll') },
        { key: 'stop', label: '停止', icon: StopCircleIcon, visible: true, action: () => emit('stopAll') },
        { key: 'tag', label: '设置标签', icon: BookmarkCircleIcon, visible: true, action: void (0), children: [{ key: 'addTab', label: '新增标签', active: () => emit('addLabel') }, { key: 'reset', label: '重设标签', active: () => emit('resetLabel') }, { key: 'clean', label: '清空标签', active: () => emit('cleanLabel') }] },
        { key: 'group', label: '设置分组', icon: FileTextIcon, visible: true, action: void (0), children: [{ key: 'addGroup', label: '增加分组', active: () => emit('addGroup') }] },
        { key: 'export', label: '导出环境', icon: FileRightIcon, visible: true, action: () => emit('exportEnv') },
        // , children: [{ key: 'export1', label: '导出已选', active: () => emit('exportEnv') }, { key: 'export2', label: '导出最近50条', active: () => emit('exportEnv') }] 
        { key: 'edit', label: '修改启动页', icon: RulerPenIcon, visible: true, action: () => emit('editStartPage') },
        { key: 'ua', label: '修改UA', icon: RuleCrossPenIcon, visible: true, action: () => emit('editUa') },
        { key: 'proxy', label: '修改代理', icon: PlateIcon, visible: true, action: () => emit('editProxy') },
        { key: 'env', label: '修改环境信息', icon: HomeSmileIcon, visible: true, action: () => emit('editEnvInfo') },
        { key: 'transfer', label: '转移', icon: RoundTransferHorizontal, visible: true, action: () => emit('transferEnv') },
        { key: 'cancelTransfer', label: '取消转移', icon: ForbidRoundTransferHorizontal, visible: true, action: () => emit('untransferEnv') },
        { key: 'clearCache', label: '清除缓存', icon: ClearCacheIcon, visible: true, action: () => emit('cleanCache') },
        { key: 'delEnvironment', label: '删除环境', icon: TrashBinTrashIcon, visible: true, action: () => emit('delEnv') },
    ]
}

watch(() => props.groupData.length, (_) => {
    if (!props.groupData) return;
    if (props.groupData.length === 0) return;
    allActions.value = allActions.value.map((item: any) => {  //找到分组的操作
        if (item.key === 'group') {
            //处理groupData的字段
            let list = props.groupData.map((e: any) => {
                return {
                    key: e.id,
                    label: e.value,
                }
            })
            return { ...item, children: [...list, { key: 'addGroup', label: '增加分组', active: () => emit('addGroup') }] }
        } else {
            return item
        }
    })
    emit('select', allActions.value)
})

const copyActions = ref([])

const saveQuickSettings = () => {
    //返回数组
    emit("select", allActions.value)
    copyActions.value = JSON.parse(JSON.stringify(allActions.value))
    emit("close")
}
const cancelSet = () => {
    allActions.value = JSON.parse(JSON.stringify(copyActions.value))
    emit('select', allActions.value)
    emit('close')
}

const screenHeight = ref(window.innerHeight);
const divHeight = ref<HTMLElement | null>(null)
const updateHeight = () => {
    screenHeight.value = window.innerHeight;
    if (screenHeight.value <= 1140) {
        if (divHeight.value) {
            divHeight.value.style.height = screenHeight.value - 500 + 'px';
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
    if (screenHeight.value <= 1140) {
        if (divHeight.value) {
            divHeight.value.style.height = screenHeight.value - 500 + 'px';
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
onMounted(() => {
    environment_group_query(1, 1000).then((res: any) => {
        allActions.value.find((item: any) => {  //找到分组的操作
            if (item.key === 'group') {
                res.data.data.reverse().forEach((e: any) => {
                    //
                    let groupV = { key: '', label: '', active: void (0) }
                    groupV.key = e.id
                    groupV.label = e.name
                    item.children = [groupV, ...item.children];
                })
            }
        })
    })
})
onUnmounted(() => {
    window.removeEventListener('resize', updateHeight);
});
</script>
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