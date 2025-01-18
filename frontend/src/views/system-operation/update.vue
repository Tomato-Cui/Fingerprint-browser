<template>
    <Transition name="modal-fade">
        <div v-if="props.open" ref="targetDiv"
            class="bg-white rounded-xl w-[500px] max-w-md mx-4 border shadow-xl z-[999]">
            <!-- Header -->
            <div class="flex justify-between items-center p-4 border-b">
                <h2 class="text-lg font-medium">更新</h2>
                <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600">
                    <!-- <XIcon class="w-5 h-5" /> -->
                    <div class="size-7 text-xl">X</div>
                </button>
            </div>

            <!-- Tabs -->
            <div class="flex border-b">
                <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id"
                    class="flex-1 py-3 text-center relative"
                    :class="currentTab === tab.id ? 'text-[#5B5BFA]' : 'text-gray-500'">
                    {{ tab.name }}
                    <div class="absolute bottom-0 left-0 w-full h-0.5 bg-[#5B5BFA] transition-transform duration-300"
                        :class="currentTab === tab.id ? 'scale-x-100' : 'scale-x-0'"></div>
                </button>
            </div>

            <!-- Content -->
            <div class="p-6 overflow-hidden">
                <TransitionGroup name="tab-fade" mode="out-in">
                    <div v-if="currentTab === 'client'" key="client">
                        <div v-if="false && clientList.length != 0" class="bg-gray-50 rounded-lg p-6">
                            <!-- Version Title -->
                            <div class="flex items-center gap-2 mb-4">
                                <span class="text-lg">补丁版本</span>
                                <span class="bg-red-50 text-red-500 text-xs px-2 py-0.5 rounded">NEW</span>
                                <span class="text-gray-400 ml-auto">发布时间: 2025.01.13</span>
                            </div>

                            <!-- Update List -->
                            <ul class="space-y-2 mb-6">
                                <li class="flex items-start gap-2">
                                    <span>1.</span>
                                    <span>本地设置:版本新增"自动更新"功能</span>
                                </li>
                                <li class="flex items-start gap-2">
                                    <span>2.</span>
                                    <span>本地设置:新增"内核卸载"功能</span>
                                </li>
                            </ul>

                            <!-- Warning -->
                            <div class="text-red-500 text-sm">
                                <p>注意:更新会关闭当前设备已打开的浏览器，</p>
                                <p>请注意保存!!!</p>
                            </div>
                        </div>
                        <div v-else class="flex flex-col items-center justify-center">
                            <ContentUploadIcon class="size-[150px]" />
                            <h2 class="font-[600]">当前版本：V1.12.1</h2>
                            <p class="text-gray-400 font-[300]">您的客户端已是最新版本</p>
                        </div>
                    </div>

                    <div v-else key="core" class="bg-gray-50 rounded-lg p-6">
                        <!-- Core Update Content -->
                        <div class="flex items-center gap-2 mb-4">
                            <span class="text-lg">内核更新</span>
                            <span class="text-gray-400 ml-auto">发布时间: 2025.01.13</span>
                        </div>
                        <ul class="space-y-2 mb-6">
                            <li class="flex items-start gap-2">
                                <span>1.</span>
                                <span>更新内核版本至最新版</span>
                            </li>
                        </ul>
                    </div>
                </TransitionGroup>
            </div>

            <!-- Footer -->
            <div class="p-4 flex gap-4 justify-end">
                <button @click="$emit('skip')" class="px-6 py-2 border rounded-lg hover:bg-gray-50 transition-colors">
                    跳过本版本
                </button>
                <button @click="$emit('update')"
                    class="px-6 py-2 bg-[#5B5BFA] text-white rounded-lg hover:bg-[#4A4AE5] transition-colors">
                    立即更新
                </button>
            </div>
        </div>
    </Transition>
</template>

<script setup lang="ts">
import { ref, defineEmits, defineProps, onMounted, onUnmounted } from 'vue'
import { ContentUploadIcon } from '@/assets/icons/system-operation';

const props = defineProps<{
    open: boolean
}>()
const currentTab = ref('client')
const tabs = [
    { id: 'client', name: '客户端升级' },
    { id: 'core', name: '内核升级' }
]
const clientList = ref([
    { name: 1 }
])

const emit = defineEmits(['close', 'skip', 'update'])

// 获取指定的 div 元素
const targetDiv = ref<any>(null);

// 监听点击事件
const handleClick = (event: MouseEvent) => {

    // 判断是否点击到了指定的 div
    if (targetDiv.value && !targetDiv.value.contains(event.target)) {
        if (props.open === true) emit('close');  //未点击到则关闭弹窗
    }
};

// 在组件挂载时添加事件监听
onMounted(() => {
    document.addEventListener('click', handleClick);
});

// 在组件卸载时移除事件监听
onUnmounted(() => {
    document.removeEventListener('click', handleClick);
});
</script>

<style scoped>
/* Modal Fade Animation */
.modal-fade-enter-active,
.modal-fade-leave-active {
    transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
    opacity: 0;
}

/* Tab Content Transition */
.tab-fade-enter-active,
.tab-fade-leave-active {
    transition: all 0.3s ease;
}

.tab-fade-enter-from {
    opacity: 0;
    transform: translateX(30px);
}

.tab-fade-leave-to {
    opacity: 0;
    transform: translateX(-30px);
}

.tab-fade-leave-active {
    position: absolute;
}
</style>