<template>
    <div v-if="props.open"
        class="absolute top-10 flex flex-col w-full max-w-md rounded-2xl overflow-hidden bg-white p-3 max-h-[500px] min-h-[400px] border shadow-xl z-10">
        <!-- Header -->
        <div class="flex items-center justify-between px-4 py-2 border-b">
            <h2 class="text-lg font-medium">同步器</h2>
            <div class="flex items-center gap-4">
                <button class="text-gray-400 hover:text-gray-600" @click="setSynModel = !setSynModel">
                    <SettingsMinimalisticIcon class="size-5" />
                </button>
                <button class="p-1 text-gray-400 hover:text-gray-600">
                    <CloseIcon class="w-5 h-5" @click="emit('close')" />
                </button>
            </div>
        </div>

        <div class="flex-1 overflow-auto">
            <div v-if="environmentData.length != 0" class="bg-[#F7F9FC] mt-2 rounded-xl">
                <!-- Selected count -->
                <div class="px-4 py-3 border-b">
                    <div class="flex items-center text-blue-600 px-2 gap-4">
                        <input type="checkbox" />
                        <span class="text-gray-500">已选 2 项</span>
                    </div>
                </div>

                <!-- List items -->
                <div class="px-4 py-2 space-y-3">
                    <div v-for="i in environmentData" :key="i.id"
                        class="flex items-center justify-between py-1 bg-white p-2 rounded-lg">
                        <div class="flex items-center">
                            <div class="w-5 h-5 rounded flex items-center justify-center mr-3">
                                <input type="checkbox" />
                            </div>
                            <span class="mr-3">{{ i.name }}</span>
                            <span class="px-3 py-1 bg-blue-50 text-blue-600 text-sm rounded-lg">主控</span>
                        </div>
                        <div class="flex items-center gap-3 text-gray-400">
                            <TooltipButton :title="'切换窗口'"
                                class="p-2.5 rounded border border-gray-200 hover:bg-gray-100" @click="void">
                                <component :is="SquareSortVerticalIcon" class="w-5 h-5 text-gray-600" />
                            </TooltipButton>
                            <TooltipButton :title="'设为主控'"
                                class="p-2.5 rounded border border-gray-200 hover:bg-gray-100" @click="void">
                                <component :is="ScreencastIcon" class="w-5 h-5 text-gray-600" />
                            </TooltipButton>
                            <!-- <SquareSortVerticalIcon class="w-5 h-5" />
                            <ScreencastIcon class="w-5 h-5" /> -->
                        </div>
                    </div>
                </div>
            </div>
            <div v-else class="flex flex-col items-center justify-center my-auto">
                <NoContentIcon class="size-[150px]" />
                <p>请至少选择两个窗口</p>
            </div>
        </div>

        <!-- Footer -->
        <div class="px-4 py-4 flex items-center justify-end gap-3 border-t">
            <button class="px-6 py-2 border rounded-lg hover:bg-gray-50" @click="cancel">
                取消
            </button>
            <button v-if="0" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                开始同步
            </button>
            <button v-else class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700" @click="cancel">
                确认
            </button>
        </div>
    </div>
    <Auth :open="authModel" @close="authModel = false"/>
    <SetSyn :open="setSynModel" @close="setSynModel = false"/>
</template>

<script setup>
import Auth from './auth.vue'
import TooltipButton from "@/components/tooltip-button.vue";
import SetSyn from './set-syn.vue';
import { ref, defineProps, defineEmits } from 'vue'
import { SettingsMinimalisticIcon, CloseIcon, SquareSortVerticalIcon, ScreencastIcon, NoContentIcon } from '@/assets/icons/synchronizer/index'
// import {
//     XIcon,
//     CheckIcon,
//     HexagonIcon,
//     ArrowUpIcon,
//     MonitorIcon
// } from 'lucide-vue-next'

const props = defineProps({
    open: false
})
const emit = defineEmits(['close'])

const authModel = ref(false)  //授权模态框
const setSynModel = ref(false) //设置模态框

const environmentData = ref([
    {id: 1, name: 'p-1'},
    {id: 2, name: 'p-2'},
    {id: 3, name: 'p-2'},
    {id: 4, name: 'p-2'},
    {id: 5, name: 'p-2'},
    {id: 6, name: 'p-2'},
    {id: 7, name: 'p-2'},
    {id: 8, name: 'p-2'},
    {id: 9, name: 'p-2'},
    {id: 10, name: 'p-2'},
    {id: 11, name: 'p-2'},
    {id: 12, name: 'p-2'},
])

const cancel = () => {
    emit('close')
}
</script>