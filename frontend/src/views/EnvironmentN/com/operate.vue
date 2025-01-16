<template>
    <Transition name="modal-fade">
        <div v-show="props.open" class="mt-16 mr-4 min-w-72 rounded-lg bg-white p-4 shadow-lg z-40">
            <div class="mb-4 flex items-center justify-between border-b pb-1">
                <h3 class="text-lg font-medium">快捷设置</h3>
                <RefreshSquareIcon class="size-6" />
            </div>
            <div class="space-y-3 pr-2" ref="divHeight">
                <div class="flex items-center justify-between gap-4">
                    <FrameIcon class="size-5 text-gray-200" />
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
                <div v-for="action in allActions" :key="action.key" class="flex items-center justify-between gap-4"
                    v-show="action.key !== 'start'">
                    <FrameIcon class="size-5" />
                    <div class="flex items-center bg-[#EDEDFF80] w-full h-[40px] pl-[10px] rounded-lg">
                        <component :is="action.icon" class="mr-2 h-4 w-4" />
                        <span class="text-sm flex-1">{{ action.label }}</span>
                        <AltArrowDownIcon v-show="action.children" class="size-5" />
                    </div>
                    <label class="relative inline-flex cursor-pointer items-center">
                        <input type="checkbox" v-model="action.visible" class="peer sr-only">
                        <div
                            class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full">
                        </div>
                    </label>
                </div>
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
import { ref, computed, watch, onMounted, onUnmounted, defineProps, defineEmits } from 'vue'
import { AltArrowDownIcon, RefreshSquareIcon, StopCircleIcon, HelfGlobalIcon, RoundArrowRight, BookmarkCircleIcon, FileTextIcon, FileRightIcon, RulerPenIcon, RuleCrossPenIcon, PlateIcon, HomeSmileIcon, RoundTransferHorizontal, ForbidRoundTransferHorizontal, ClearCacheIcon, TrashBinTrashIcon, FrameIcon } from '@/assets/icons/environment/index'

onMounted(() => {
    emit('select', allActions.value)
    copyActions.value = JSON.parse(JSON.stringify(allActions.value))  //数组深拷贝
})

const props = defineProps<{
    open: false,
}>()
const emit = defineEmits(['select', 'close', 'startAll', 'stopAll'])


const showQuickSettings = ref(false)
const allActions = ref([
    { key: 'start', label: '启动', icon: StopCircleIcon, visible: true, action: () => emit('startAll') },
    { key: 'stop', label: '停止', icon: StopCircleIcon, visible: true, action: () => emit('stopAll') },
    { key: 'tag', label: '设置标签', icon: BookmarkCircleIcon, visible: true, action: void (0), children: [{ name: '标签1' }] },
    { key: 'group', label: '设置分组', icon: FileTextIcon, visible: true, action: void (0), children: [{ name: '分组1' }] },
    { key: 'export', label: '导出环境', icon: FileRightIcon, visible: true, action: void (0), children: [{ name: '导出1' }] },
    { key: 'edit', label: '修改启动页', icon: RulerPenIcon, visible: true, action: void (0) },
    { key: 'ua', label: '修改UA', icon: RuleCrossPenIcon, visible: true, action: void (0) },
    { key: 'proxy', label: '修改代理', icon: PlateIcon, visible: true, action: void (0) },
    { key: 'env', label: '修改环境信息', icon: HomeSmileIcon, visible: true, action: void (0) },
    { key: 'transfer', label: '转移', icon: RoundTransferHorizontal, visible: true, action: void (0) },
    { key: 'cancelTransfer', label: '取消转移', icon: ForbidRoundTransferHorizontal, visible: true, action: void (0) },
    { key: 'clearCache', label: '清除缓存', icon: ClearCacheIcon, visible: true, action: void (0) },
    { key: 'delEnvironment', label: '删除环境', icon: TrashBinTrashIcon, visible: true, action: void (0) },
])
const copyActions = ref([])
// watch(() => props.open, (v) => {
//     if (v) copyActions.value = JSON.parse(JSON.stringify(allActions.value))  //数组深拷贝
// })

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
</style>