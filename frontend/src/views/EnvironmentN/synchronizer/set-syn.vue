<template>
    <Model :open="props.open" title="设置" class="min-w-[900px]" @close="emit('close')">
        <div class="bg-white w-full rounded-2xl overflow-hidden">
            <!-- Content -->
            <div class="p-6 space-y-8">
                <!-- Sync Settings -->
                <div class="flex items-center justify-between font-medium">
                    <span class="w-[300px] flex pl-[100px] font-semibold">同步开关</span>

                    <div class="flex-1 flex flex-col space-y-3">
                        <div class="flex flex-1 items-center gap-8">
                            <span>同步键盘输入</span>
                            <mySwitch v-model="keyboardSync" class="bg-blue-600" />
                        </div>

                        <div class="flex flex-1 items-center gap-8">
                            <span>同步鼠标事件</span>
                            <div class="flex gap-6 flex-1">
                                <label class="flex items-center gap-2">
                                    <input type="checkbox" v-model="mouseEvents.click" />
                                    <span>点击</span>
                                </label>
                                <label class="flex items-center gap-2">
                                    <input type="checkbox" v-model="mouseEvents.scroll" />
                                    <span>滚动</span>
                                </label>
                                <label class="flex items-center gap-2">
                                    <input type="checkbox" v-model="mouseEvents.track" />
                                    <span>轨迹</span>
                                </label>
                            </div>
                        </div>
                    </div>

                </div>



                <!-- Input Delay Settings -->
                <div class="flex items-center justify-between font-medium">
                    <span class="w-[300px] flex pl-[100px] font-semibold">仿真操作</span>

                    <div class="flex-1 space-y-3">
                        <div class="flex items-center gap-4">
                            <span class="w-24">文本输入延迟</span>
                            <div class="flex items-center gap-2">
                                <!-- <input type="number" v-model="textDelay.min" class="w-16 px-3 py-1 border rounded-lg" /> -->
                                <div class="flex border w-[100px] h-[40px] rounded-lg pl-2">
                                    <input type="text" v-model="textDelay.min" class="outline-none w-full" />
                                    <div class=" w-[40px] flex flex-col">
                                        <span class="size-[20px] border-l border-b ">
                                            <AltArrowDownIcon class="size-[20px] rotate-180" />
                                        </span>
                                        <span class="size-[20px] border-l">
                                            <AltArrowDownIcon class="size-[20px]" />
                                        </span>
                                    </div>
                                </div>
                                <span>至</span>
                                <div class="flex border w-[100px] h-[40px] rounded-lg pl-2">
                                    <input type="text" v-model="textDelay.max" class="outline-none w-full" />
                                    <div class=" w-[40px] flex flex-col">
                                        <span class="size-[20px] border-l border-b">
                                            <AltArrowDownIcon class="size-[20px] rotate-180" />
                                        </span>
                                        <span class="size-[20px] border-l">
                                            <AltArrowDownIcon class="size-[20px]" />
                                        </span>
                                    </div>
                                </div>
                                <span>秒</span>
                            </div>
                        </div>

                        <div class="flex items-center gap-4">
                            <span class="w-24">鼠标输入延迟</span>
                            <div class="flex items-center gap-2">
                                <!-- <input type="number" v-model="mouseDelay.min"
                                    class="w-16 px-3 py-1 border rounded-lg" /> -->
                                <div class="flex border w-[100px] h-[40px] rounded-lg pl-2">
                                    <input type="text" v-model="mouseDelay.min" class="outline-none w-full" />
                                    <div class=" w-[40px] flex flex-col">
                                        <span class="size-[20px] border-l border-b">
                                            <AltArrowDownIcon class="size-[20px] rotate-180" />
                                        </span>
                                        <span class="size-[20px] border-l">
                                            <AltArrowDownIcon class="size-[20px]" />
                                        </span>
                                    </div>
                                </div>
                                <span>至</span>
                                <!-- <input type="number" v-model="mouseDelay.max"
                                    class="w-16 px-3 py-1 border rounded-lg" /> -->
                                <div class="flex border w-[100px] h-[40px] rounded-lg pl-2">
                                    <input type="text" v-model="mouseDelay.max" class="outline-none w-full" />
                                    <div class=" w-[40px] flex flex-col">
                                        <span class="size-[20px] border-l border-b">
                                            <AltArrowDownIcon class="size-[20px] rotate-180" />
                                        </span>
                                        <span class="size-[20px] border-l">
                                            <AltArrowDownIcon class="size-[20px]" />
                                        </span>
                                    </div>
                                </div>
                                <span>秒</span>
                            </div>
                        </div>

                        <div class="flex items-center gap-4">
                            <span class="w-24">鼠标输入延迟</span>
                            <div class="flex items-center gap-4">
                                <label class="flex items-center gap-2">
                                    <input type="radio" v-model="mouseInputType" value="parallel"
                                        class="text-blue-600" />
                                    <span>并行</span>
                                </label>
                                <label class="flex items-center gap-2">
                                    <input type="radio" v-model="mouseInputType" value="serial" class="text-blue-600" />
                                    <span>串行</span>
                                </label>
                            </div>
                        </div>
                    </div>

                </div>

                <!-- Window Order -->
                <div class="flex items-center justify-between font-medium">
                    <span class="w-[300px] flex pl-[100px] font-semibold">环境窗口排序</span>
                    <div class="flex flex-1 gap-6">
                        <label v-for="option in orderOptions" :key="option.value" class="flex items-center gap-2">
                            <input type="radio" v-model="windowOrder" :value="option.value" class="text-blue-600" />
                            <span>{{ option.label }}</span>
                        </label>
                    </div>
                </div>

                <!-- Hotkeys -->
                <div class="flex items-center justify-between font-medium">
                    <span class="w-[300px] flex pl-[100px] font-semibold">快捷键</span>
                    <div class="flex-1 space-y-3">
                        <div v-for="(hotkey, action) in hotkeys" :key="action" class="flex items-center gap-4">
                            <span class="w-32 text-gray-500">{{ action }}</span>
                            <input type="text" v-model="hotkey.key" readonly
                                class="px-4 py-2 border rounded-lg w-48 bg-gray-50" />
                        </div>
                    </div>
                </div>
            </div>

            <!-- Footer -->
            <div class="px-6 py-4 border-t flex justify-end gap-4">
                <button @click="cancel" class="px-6 py-2 border rounded-lg hover:bg-gray-50">
                    取消
                </button>
                <button @click="submit" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                    保存
                </button>
            </div>
        </div>
    </Model>
    <!-- <div v-if="props.open" class="fixed inset-0 bg-black bg-opacity-25 flex items-center justify-center p-4 z-20"> -->

    <!-- </div> -->
</template>

<script setup>
import model from '@/components/model/model.vue'
import mySwitch from '@/components/switch.vue'
import { ref, defineProps, defineEmits } from 'vue'
import { XIcon } from 'lucide-vue-next'
import Model from '@/components/model/model.vue'
import { AltArrowDownIcon } from '@/assets/icons/environment'
import { CloseIcon } from '@/assets/icons/synchronizer'

const props = defineProps({
    open: false
})
const emit = defineEmits(['close'])

const keyboardSync = ref(true)
const mouseEvents = ref({
    click: true,
    scroll: true,
    track: true
})

const textDelay = ref({ min: 3 , max: 3 })
const mouseDelay = ref({ min: 3, max: 3 })
const mouseInputType = ref('parallel')

const windowOrder = ref('startup')
const orderOptions = [
    { value: 'startup', label: '按启动顺序' },
    { value: 'descending', label: '按序号由大到小' },
    { value: 'ascending', label: '按序号由小到大' },
    { value: 'random', label: '随机' }
]

const hotkeys = ref({
    '开始同步': { key: 'Option+S' },
    '停止同步': { key: 'Option+D' },
    '自动铺平': { key: 'Option+Z' },
    '打开/关闭 点击延迟': { key: 'Option+E' },
    '中断仿真执行': { key: 'Option+X' }
})

const cancel = () => {
    emit('close')
}
const submit = () => {
    emit('close')
}
</script>