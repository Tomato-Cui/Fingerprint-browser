<template>
    <!-- <div v-if="props.open" class="fixed inset-0 bg-black/0 flex items-center justify-center z-[100]"
    @click="() => emit('close')"></div> -->
    <Transition name="modal-fade">
        <div v-if="props.open" ref="targetDiv" class="bg-white rounded-2xl w-[500px] max-w-md mx-4 shadow-xl z-[150] border absolute top-[45px] right-[220px]">
            <!-- Header -->
            <div class="flex justify-between items-center p-4">
                <h2 class="text-xl font-medium">信息中心</h2>
                <button class="text-gray-400 hover:text-[#5B5BFA] transition-colors" @click="markAllRead">
                    <span class="flex items-center gap-2">
                        全部已读
                        <CheckIcon class="w-5 h-5" />
                    </span>
                </button>
            </div>

            <!-- Tabs -->
            <div class="flex border-b">
                <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id as 'system' | 'team'"
                    class="flex-1 py-3 text-center relative"
                    :class="currentTab === tab.id ? 'text-[#5B5BFA]' : 'text-gray-500'">
                    {{ tab.name }}
                    <div class="absolute bottom-0 left-0 w-full h-0.5 bg-[#5B5BFA] transition-transform duration-300"
                        :class="currentTab === tab.id ? 'scale-x-100' : 'scale-x-0'"></div>
                </button>
            </div>

            <!-- Content -->
            <div class="p-4 h-[400px] overflow-y-auto">
                <TransitionGroup name="list" tag="div" class="space-y-4 h-full" :class="{'flex items-center justify-center': currentMessages.length==0}">
                    <div v-for="message in currentMessages" :key="message.id" v-if="currentMessages.length!=0"
                        class="flex items-center gap-4 p-4 rounded-lg hover:bg-gray-50 transition-colors">
                        <div class="flex-shrink-0">
                            <img :src="message.icon" alt="" class="w-12 h-12 rounded-lg" />
                        </div>
                        <div class="flex-grow min-w-0">
                            <h3 class="font-medium text-gray-900 truncate">{{ message.title }}</h3>
                            <p class="text-gray-500 text-sm">{{ message.date }}</p>
                        </div>
                        <button class="flex-shrink-0 text-[#5B5BFA] text-sm hover:text-[#4A4AE5] transition-colors"
                            @click="lookMessage(message.id)">
                            查看详情
                        </button>
                    </div>
                    <div v-else class="flex flex-col items-center justify-center">
                        <EmptyInboxIcon class="size-[150px]"/>
                        <p>暂无消息</p>
                    </div>
                </TransitionGroup>
            </div>
        </div>
    </Transition>
    <MessageInfo :open="mesInfoModel" :message-id="mesId" @close="mesInfoModel = false" class="fixed"/>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { CheckIcon } from 'lucide-vue-next'
import avatar from '@/assets/icons/system-operation/Frame 2053141771.png'
import { EmptyInboxIcon } from '@/assets/icons/system-operation'
import MessageInfo from './message-info.vue'

interface Message {
    id: number
    title: string
    date: string
    icon: string
    type: 'system' | 'team'
    read: boolean
}

const props = defineProps<{
    open: boolean
}>()

const emit = defineEmits(['close', 'markAllRead', 'viewDetails'])

const currentTab = ref<'system' | 'team'>('system')

const tabs = [
    { id: 'system', name: '系统信息' },
    { id: 'team', name: '团队通知' }
]

const messages = ref<Message[]>([
    {
        id: 1,
        title: 'XXy 官方通知',
        date: '2025-01-13',
        icon:  avatar,
        type: 'system',
        read: false
    },
    {
        id: 2,
        title: 'XXy 官方通知',
        date: '2025-01-13',
        icon: avatar,
        type: 'system',
        read: false
    },
    {
        id: 2,
        title: 'XXy 官方通知',
        date: '2025-01-13',
        icon: avatar,
        type: 'team',
        read: false
    },
    // Add more messages as needed
])

const mesInfoModel = ref(false)
const mesId = ref(0)
const lookMessage = (id: number) => {
    mesInfoModel.value = true,
    mesId.value = id
}

const currentMessages = computed(() => {
    return messages.value.filter(message => message.type === currentTab.value)
})

const markAllRead = () => {
    messages.value = messages.value.map(msg => ({ ...msg, read: true }))
    emit('markAllRead')
}

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

/* List Transitions */
.list-move,
.list-enter-active,
.list-leave-active {
    transition: all 0.3s ease;
}

.list-enter-from {
    opacity: 0;
    transform: translateX(30px);
}

.list-leave-to {
    opacity: 0;
    transform: translateX(-30px);
}

.list-leave-active {
    position: absolute;
}
</style>