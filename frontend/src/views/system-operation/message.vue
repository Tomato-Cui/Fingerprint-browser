<template>
    <!-- <div v-if="props.open" class="fixed inset-0 bg-black/0 flex items-center justify-center z-[100]"
    @click="() => emit('close')"></div> -->
    <Transition name="modal-fade">
        <div v-if="props.open" ref="targetDiv" class="bg-white rounded-2xl max-w-md w-[350px] mx-4 shadow-xl z-[150] border absolute top-[45px] right-[220px]">
            <!-- Tabs -->
            <div class="flex border-b px-2 text-sm justify-between">
                <div class="w-1/2 flex gap-x-2">
                    <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id as 'system' | 'team'"
                        class="flex-1 pt-3 pb-2 text-center relative"
                        :class="currentTab === tab.id ? 'text-[#5B5BFA]' : 'text-gray-500'">
                        {{ tab.name }}
                        <div class="absolute bottom-0 left-0 w-full h-0.5 bg-[#5B5BFA] transition-transform duration-300"
                            :class="currentTab === tab.id ? 'scale-x-100' : 'scale-x-0'"></div>
                    </button>
                </div>

                <button class="text-gray-400 hover:text-[#5B5BFA] transition-colors" @click="markAllRead">
                    <span class="flex items-center gap-x-1 text-xs">
                        全部已读
                        <CheckIcon class="w-4 h-4" />
                    </span>
                </button>

            </div>

            <!-- Content -->
            <div class="p-2 max-h-[400px] pb-12 overflow-y-auto overflow-x-hidden">
                <div v-for="message in currentMessages" :key="message.id" v-if="currentMessages.length!=0"
                    class="flex items-center gap-x-2 rounded-lg hover:bg-gray-100  transition-colors p-2 text-sm  group/wyc">
                    <div class="flex-shrink-0">
                        <img :src="message.icon" alt="" class="w-8 h-8 rounded-lg" />
                    </div>
                    <div class="flex-grow min-w-0 text-xs gap-y-2">
                        <h3 class="font-medium text-gray-900 truncate text-sm">{{ message.title }}</h3>
                        <p class="text-gray-500 text-xs">{{ message.date }}</p>
                    </div>
                    <button v-if="message.type == 'system'" class="flex-shrink-0 text-[#5B5BFA] text-sm hover:text-[#4A4AE5] transition-colors"
                        @click="lookMessage(message.id)">
                        查看详情
                    </button>
                    <div v-else class="flex gap-x-1">
                        <button  class="flex-shrink-0 text-[#5B5BFA] hover:text-[#4A4AE5] border-red-200 border text-xs p-1 rounded-md hidden group-hover/wyc:block"
                            @click="lookMessage(message.id)">
                            <XIcon class="w-4 h-4 text-red-400" />
                        </button>
                        <button  class="flex-shrink-0 text-[#5B5BFA] hover:text-[#4A4AE5] transition-colors border-gray-200 border text-xs p-1 rounded-md"
                            @click="acceptHandle(message.id, message.teamId as number)">
                            <CheckIcon class="w-4 h-4" />
                        </button>
                    </div>
                </div>
                <div v-else class="flex flex-col items-center justify-center">
                    <EmptyInboxIcon class="size-[150px]"/>
                    <p>暂无消息</p>
                </div>
            </div>
        </div>
    </Transition>
    <MessageInfo :open="mesInfoModel" :message-id="mesId" @close="mesInfoModel = false" class="fixed"/>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { CheckIcon,XIcon } from 'lucide-vue-next'
import avatar from '@/assets/icons/system-operation/Frame 2053141771.png'
import { EmptyInboxIcon } from '@/assets/icons/system-operation'
import MessageInfo from './message-info.vue'
import { user_receive_query, user_allow } from '@/commands/user-team-temp'
import { toast } from 'vue-sonner'

interface Message {
    id: number
    title: string
    date: string
    icon: string
    type: 'system' | 'team'
    read: boolean
    teamId?: number
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

const acceptHandle = (id: number, teamId: number) => {
    user_allow(id, teamId).then((res: any) => {
        if(res.code == 1){
            toast.success("加入成功")
        }else{
            toast.warning("加入失败")
        }
    })
}

// 在组件挂载时添加事件监听
onMounted(() => {
  document.addEventListener('click', handleClick);
});

watch([() => props.open], ()=>{
    if (!props.open) return;
    console.log(props.open);
    
    user_receive_query(1, 10).then(res=>{
        let {code, data} = res;
        if (code != -1){
            data = data.data;
            messages.value = data.map((item:any)=>({
                id: item.id,
                teamId: item.team_id,
                title: '收到' + item.team_name + '团队的邀请',
                date: item.created_at,
                icon: avatar,
                type: 'team',
                read: false
            }))
        }
        console.log(res);
    })
})

onUnmounted(() => {
  document.removeEventListener('click', handleClick);
});
</script>

<style scoped>
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