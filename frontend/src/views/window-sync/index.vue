<template>
    <div class=" h-main p-4">
        <div class="flex flex-col-reverse lg:flex-row gap-8 h-full overflow-hidden">
            <!-- Left Section -->
            <div class="lg:w-3/5 rounded-lg shadow-sm bg-white flex flex-col h-full">
                <div class="p-4">
                    <button @click="() => console.log(checkIDs)"
                        class="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2">
                        <MonitorIcon class="h-5 w-5" />
                        开始同步
                    </button>
                </div>

                <div class="flex-auto overflow-y-auto">
                    <!-- Table -->
                    <table class="w-full flex-auto">
                        <thead class="bg-gray-50 z-10 sticky top-0">
                            <tr>
                                <th class="w-12 p-4">
                                    <input @click="checkAllHandle" type="checkbox" class="rounded border-gray-300" />
                                </th>
                                <th class="text-left p-4 text-gray-400 text-sm font-light">序号</th>
                                <th class="text-left p-4 text-gray-400 text-sm font-light">名称</th>
                                <th class="text-left p-4 text-gray-400 text-sm font-light">分组</th>
                                <th class="text-left p-4 text-gray-400 text-sm font-light">状态</th>
                                <th class="text-left p-4 text-gray-400 text-sm font-light">操作</th>
                            </tr>
                        </thead>

                        <!-- class="border-t border-gray-100 text-sm text-gray-500" -->
                        <tbody class="h-90">
                            <tr v-for="(item, index) in tableData" :key="index" :class="{
                                'border-t border-gray-100': true,
                                'bg-blue-50 hover:bg-blue-100': item.selected,
                                'hover:bg-blue-100': !item.selected,
                            }">
                                <td class="p-4">
                                    <input type="checkbox" class="rounded border-gray-300"
                                        @click="() => checkHandle(item.id)" :value="item.id" v-model="checkIDs" />
                                </td>
                                <td class="p-4">{{ item.id }}</td>
                                <td class="p-4">{{ item.name }}</td>
                                <td class="p-4 ">{{ item.group }}</td>
                                <td class="p-4">
                                    <div class="flex items-center gap-2">
                                        <Chrome v-if="item.status" class="h-5 w-5 text-green-500" />
                                        <span>/</span>
                                    </div>
                                </td>
                                <td class="p-4">
                                    <div class="flex gap-2">
                                        <TooltipButton title="排列" @click="() => console.log('abc')"
                                            class="p-1 hover:bg-gray-100 rounded">
                                            <LayoutDashboardIcon class="h-5 w-5 text-gray-600" />
                                        </TooltipButton>
                                        <TooltipButton title="全屏" @click="() => console.log('abc')"
                                            class="p-1 hover:bg-gray-100 rounded">
                                            <MonitorIcon class="h-5 w-5 text-gray-600" />
                                        </TooltipButton>
                                        <TooltipButton title="关闭" @click="() => console.log('abc')"
                                            class="p-1 hover:bg-gray-100 rounded">
                                            <XIcon class="h-5 w-5 text-gray-600" />
                                        </TooltipButton>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            <!-- Right Section -->
            <div class="lg:w-2/5 rounded-xl overflow-hidden h-full bg-white">
                <div class="flex justify-between items-center mb-4 bg-sidebar p-6">
                    <div>
                        <h2 class="text-lg font-medium">操作台</h2>
                        <div class="text-sm text-gray-500">共{{ tableData.length }}个环境，已选择<span class="text-blue-600">{{
                            checkIDs.length }}</span>个环境</div>
                    </div>
                    <div class="flex gap-2">
                        <TooltipButton title="设置" class="p-1 bg-gray-100 rounded" @click="settingOpen = true">
                            <SettingsIcon class="h-10 w-10 text-gray-600" />
                        </TooltipButton>
                        <Model class="sm:max-w-[675px]" title="设置" :open="settingOpen"
                            @close="() => settingOpen = false">
                            <div class="mx-8 p-6">
                                <form @submit.prevent="handleSubmit">
                                    <!-- Sync Section -->
                                    <div class="mb-8">
                                        <h2 class="text-lg font-medium mb-4">同步</h2>

                                        <div class="space-y-4">
                                            <!-- Keyboard -->
                                            <div class="flex items-start gap-8">
                                                <label class="w-20 text-gray-600">键盘</label>
                                                <label class="flex items-center gap-2">
                                                    <input type="checkbox" v-model="form.keyboardSync"
                                                        class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500">
                                                    <span>同步键入</span>
                                                </label>
                                            </div>

                                            <!-- Mouse -->
                                            <div class="flex items-start gap-8">
                                                <label class="w-20 text-gray-600">鼠标</label>
                                                <div class="flex gap-6">
                                                    <label class="flex items-center gap-2">
                                                        <input type="checkbox" v-model="form.mouseClick"
                                                            class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500">
                                                        <span>同步点击</span>
                                                    </label>
                                                    <label class="flex items-center gap-2">
                                                        <input type="checkbox" v-model="form.mouseScroll"
                                                            class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500">
                                                        <span>同步滚动</span>
                                                    </label>
                                                    <label class="flex items-center gap-2">
                                                        <input type="checkbox" v-model="form.mouseTrack"
                                                            class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500">
                                                        <span>同步轨迹</span>
                                                    </label>
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    <!-- General Section -->
                                    <div class="mb-8">
                                        <h2 class="text-lg font-medium mb-4">通用</h2>

                                        <div class="space-y-4">
                                            <!-- Operation Platform -->
                                            <div class="flex items-start gap-8">
                                                <label class="w-20 text-gray-600">操作台</label>
                                                <label class="flex items-center gap-2">
                                                    <input type="checkbox" v-model="form.autoHide"
                                                        class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500">
                                                    <span>再不使用操作台时自动收起</span>
                                                </label>
                                            </div>

                                            <!-- Child Window -->
                                            <div class="flex items-start gap-8">
                                                <label class="w-20 flex whitespace-nowrap text-gray-600">
                                                    子窗口
                                                    <span
                                                        class="text-xs flex justify-center items-center border p-0.5 ml-1 rounded">Beta</span>
                                                </label>
                                                <div class="flex items-center gap-2">
                                                    <label class="flex items-center gap-2">
                                                        <input type="checkbox" v-model="form.useChildWindow"
                                                            class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500">
                                                        <span>使用标签页打开子窗口</span>
                                                    </label>
                                                    <button type="button" class="text-gray-400 hover:text-gray-600">
                                                        <InfoIcon class="w-4 h-4" />
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    <!-- Simulation Operations Section -->
                                    <div class="mb-8">
                                        <h2 class="text-lg font-medium mb-4">仿真操作</h2>

                                        <div class="space-y-4">
                                            <!-- Input Speed 1 -->
                                            <div class="flex items-center gap-2">
                                                <span class="text-gray-600">输入速度</span>
                                                <span class="text-gray-600">在</span>

                                                <Select v-model="form.inputSpeed1.min">
                                                    <SelectTrigger class="w-32">
                                                        <SelectValue placeholder="200"
                                                            class="w-full p-2 rounded-lg outline-none" />
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        <SelectGroup>
                                                            <SelectItem value="300">
                                                                300
                                                            </SelectItem>
                                                            <SelectItem value="200">
                                                                200
                                                            </SelectItem>
                                                            <SelectItem value="100">
                                                                100
                                                            </SelectItem>
                                                        </SelectGroup>
                                                    </SelectContent>
                                                </Select>
                                                <span class="text-gray-600">-</span>
                                                <Select v-model="form.inputSpeed1.max">
                                                    <SelectTrigger class="w-32">
                                                        <SelectValue placeholder="400"
                                                            class=" p-2 rounded-lg outline-none" />
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        <SelectGroup>
                                                            <SelectItem value="400">
                                                                400
                                                            </SelectItem>
                                                            <SelectItem value="500">
                                                                500
                                                            </SelectItem>
                                                            <SelectItem value="600">
                                                                600
                                                            </SelectItem>
                                                        </SelectGroup>
                                                    </SelectContent>
                                                </Select>
                                                <span class="text-gray-600">ms之间随机</span>
                                            </div>

                                            <!-- Input Speed 2 -->
                                            <div class="flex items-center gap-2">
                                                <span class="text-gray-600">输入速度</span>
                                                <span class="text-gray-600">在</span>

                                                <Select v-model="form.inputSpeed2.min">
                                                    <SelectTrigger class="w-32">
                                                        <SelectValue placeholder="200"
                                                            class="w-full p-2 rounded-lg outline-none" />
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        <SelectGroup>
                                                            <SelectItem value="100">
                                                                100
                                                            </SelectItem>
                                                            <SelectItem value="200">
                                                                200
                                                            </SelectItem>
                                                            <SelectItem value="300">
                                                                300
                                                            </SelectItem>
                                                        </SelectGroup>
                                                    </SelectContent>
                                                </Select>
                                                <span class="text-gray-600">-</span>
                                                <Select v-model="form.inputSpeed2.max">
                                                    <SelectTrigger class="w-32">
                                                        <SelectValue placeholder="400"
                                                            class=" p-2 rounded-lg outline-none" />
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        <SelectGroup>
                                                            <SelectItem value="400">
                                                                400
                                                            </SelectItem>
                                                            <SelectItem value="500">
                                                                500
                                                            </SelectItem>
                                                            <SelectItem value="600">
                                                                600
                                                            </SelectItem>
                                                        </SelectGroup>
                                                    </SelectContent>
                                                </Select>
                                                <span class="text-gray-600">ms之间随机</span>
                                            </div>
                                        </div>
                                    </div>

                                </form>
                            </div>
                            <div class="flex items-center gap-4 border-t p-6 px-14">
                                <PrimaryButton @click="() => settingOpen = false">
                                    保存
                                </PrimaryButton>
                                <CancelButton @click="() => settingOpen = false">
                                    取消
                                </CancelButton>
                            </div>
                        </Model>
                    </div>
                </div>

                <!-- Tab Buttons -->
                <div class="flex gap-4 mb-6 bg-[#F5F6F9FF] mx-6 p-4 rounded-md">
                    <button class="w-1/2 flex items-center gap-2 px-6 py-2 rounded-lg justify-center"
                        :class="activeTab === 'window' ? 'bg-white text-blue-600' : 'text-gray-600'"
                        @click="activeTab = 'window'">
                        <MonitorIcon class="h-5 w-5" />
                        窗口管理
                    </button>
                    <button class="w-1/2 flex items-center gap-2 px-6 py-2 rounded-lg justify-center"
                        :class="activeTab === 'text' ? 'bg-white text-blue-600' : 'text-gray-600'"
                        @click="activeTab = 'text'">
                        <FileTextIcon class="h-5 w-5" />
                        文本管理
                    </button>
                </div>

                <!-- Window Management Panel -->
                <div v-if="activeTab === 'window'" class="space-y-6 mx-6">
                    <!-- Action Buttons -->
                    <div class="grid grid-cols-2 gap-4">
                        <button
                            class="flex items-center justify-center gap-2 px-6 py-3 bg-white rounded-lg border border-gray-200 hover:border-blue-600">
                            <MinimizeIcon class="h-5 w-5" />
                            统一大小
                        </button>
                        <button
                            class="flex items-center justify-center gap-2 px-6 py-3 bg-white rounded-lg border border-gray-200 hover:border-blue-600">
                            <MonitorIcon class="h-5 w-5" />
                            显示窗口
                        </button>
                    </div>

                    <!-- Window Settings -->
                    <div class=" rounded-lg">
                        <h3 class="text-lg mb-4">窗口排列</h3>

                        <div class="bg-[#F5F6F9FF] p-8 rounded-lg mb-6">
                            <div class="mb-4 flex items-center gap-x-4">
                                <label class="block text-sm mb-2 whitespace-nowrap">显示器</label>
                                <Select v-model="screenSize" class="">
                                    <SelectTrigger>
                                        <SelectValue placeholder="内置屏幕 (1280x720)"
                                            class="w-full p-2 rounded-lg outline-none" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectGroup>
                                            <SelectItem value="扩展屏幕 (1920x1080)">
                                                扩展屏幕 (1920x1080)
                                            </SelectItem>
                                            <SelectItem value="内置屏幕 (1280x720)">
                                                内置屏幕 (1280x720)
                                            </SelectItem>
                                        </SelectGroup>
                                    </SelectContent>
                                </Select>
                            </div>

                            <div class="mb-4">
                                <div class="flex items-center justify-between">
                                    <label class="block text-sm">根据显示器的分辨率自动排版所选窗口</label>
                                    <div class="relative inline-block w-10 mr-2 align-middle select-none">
                                        <input type="checkbox" v-model="autoLayout"
                                            class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer" />
                                        <label
                                            class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"></label>
                                    </div>
                                </div>
                            </div>

                            <div class="mb-4">
                                <label class="block text-sm mb-2">排序方式</label>
                                <div class="flex gap-4">
                                    <label class="flex items-center gap-2 ">
                                        <input type="radio" v-model="sortType" value="palace" class="text-blue-600 " />
                                        宫格平铺
                                    </label>
                                    <label class="flex items-center gap-2">
                                        <input type="radio" v-model="sortType" value="cascade" class="text-blue-600" />
                                        重叠平铺
                                    </label>
                                </div>
                            </div>
                        </div>
                        <button
                            class="w-full py-3 bg-blue-50 text-blue-600 rounded-lg flex items-center justify-center gap-2">
                            <LayoutGridIcon class="h-5 w-5" />
                            一键排列
                        </button>
                    </div>
                </div>

                <!-- Text Management Panel -->
                <div v-if="activeTab === 'text'" class="space-y-6 mx-8">
                    <div class="grid grid-cols-2 gap-4">
                        <button
                            class="flex items-center justify-center gap-2 px-6 py-3 bg-white rounded-lg border border-gray-200 hover:border-blue-600">
                            <Eraser2Icon class="h-5 w-5" />
                            清空内容
                        </button>
                        <button
                            class="flex items-center justify-center gap-2 px-6 py-3 bg-white rounded-lg border border-gray-200 hover:border-blue-600">
                            <ClipboardIcon class="h-5 w-5" />
                            粘贴内容
                        </button>
                    </div>

                    <div class="bg-white rounded-lg space-y-6">
                        <div>
                            <h3 class="text-lg mb-4">仿真操作</h3>
                            <div class="flex justify-between items-center gap-4">
                                <div class="flex items-center gap-2">
                                    <span class="text-sm">延迟操作</span>
                                    <div class="relative inline-block w-10 align-middle select-none">
                                        <input type="checkbox" v-model="delayOperation"
                                            class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer" />
                                        <label
                                            class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"></label>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-sm">延迟输入</span>
                                    <div class="relative inline-block w-10 align-middle select-none">
                                        <input type="checkbox" v-model="delayInput"
                                            class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer" />
                                        <label
                                            class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"></label>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div>
                            <h3 class="text-lg mb-4">随机数字</h3>
                            <div class="flex items-center gap-4 bg-[#F5F6F9FF] p-4 rounded-lg">
                                <span class="text-sm">范围</span>
                                <input type="number" v-model="rangeStart" class="w-24 p-2 border rounded-lg" />
                                <span>-</span>
                                <input type="number" v-model="rangeEnd" class="w-24 p-2 border rounded-lg" />
                            </div>
                        </div>

                        <div>
                            <h3 class="text-lg mb-4">相同文本</h3>
                            <input type="text" placeholder="请输入" class="w-full p-2 border rounded-lg" />
                        </div>

                        <div>
                            <h3 class="text-lg mb-4">指定文本</h3>
                            <div class="space-y-4">
                                <h4 class="text-sm font-medium">文本组1</h4>
                                <div class="flex gap-4">
                                    <label class="flex items-center gap-2">
                                        <input type="radio" v-model="textInputType" value="sequential"
                                            class="text-blue-600" />
                                        顺利输入
                                    </label>
                                    <label class="flex items-center gap-2">
                                        <input type="radio" v-model="textInputType" value="random"
                                            class="text-blue-600" />
                                        随机输入
                                    </label>
                                    <label class="flex items-center gap-2">
                                        <input type="radio" v-model="textInputType" value="specified"
                                            class="text-blue-600" />
                                        指定输入
                                    </label>
                                </div>
                                <textarea placeholder="请输入位版本内容，每行一个文本，示例：&#10;文本内容一&#10;文本内容二"
                                    class="w-full h-32 p-3 border rounded-lg resize-none"></textarea>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select'
import { ref, computed } from 'vue'
import {
    Monitor as MonitorIcon,
    Activity as ActivityIcon,
    LayoutDashboard as LayoutDashboardIcon,
    X as XIcon,
    Settings as SettingsIcon,
    FileText as FileTextIcon,
    Minimize as MinimizeIcon,
    LayoutGrid as LayoutGridIcon,
    NotebookPen as Eraser2Icon,
    Clipboard as ClipboardIcon, Chrome, X
} from 'lucide-vue-next'
import TooltipButton from '@/components/tooltip-button.vue'
import { Info as InfoIcon } from 'lucide-vue-next'
import { Model } from '@/components/model/index'
import { PrimaryButton, CancelButton } from '@/components/button'

const baseData = [...new Array(23).keys()].map(item => ({
    id: item + 1,
    name: '新建环境' + item,
    group: item % 2 == 0 ? '/' : '星期天',
    status: true,
    selected: false
}));

const activeTab = ref('window')
const autoLayout = ref(true)
const sortType = ref('palace')
const delayOperation = ref(false)
const delayInput = ref(true)
const rangeStart = ref(1)
const rangeEnd = ref(2)
const checkIDs = ref([])
const checkAll = ref(false);
const textInputType = ref('sequential')
const screenSize = ref('内置屏幕 (1280x720)')
const settingOpen = ref(false);

const tableData = computed(() => {
    return baseData
});

const checkAllHandle = () => {
    checkAll.value = !checkAll.value;
    if (checkAll.value) {
        checkIDs.value = tableData.value.map(item => item.id);
        tableData.value.map(item => item.selected = true)
    } else {
        checkIDs.value = [];
        tableData.value.map(item => item.selected = false)
    }
}

const checkHandle = (id) => {
    let exist = checkIDs.value.includes(id);
    if (!exist) {
        checkIDs.value = [...checkIDs.value, id];
        tableData.value.map(item => {
            if (item.id == id) item.selected = true;
            item
        })
    } else {
        checkIDs.value = checkIDs.value.filter(item => item != id);
        tableData.value.map(item => {
            if (item.id == id) item.selected = false;
            item
        })
    }
}


const form = ref({
    keyboardSync: true,
    mouseClick: true,
    mouseScroll: true,
    mouseTrack: true,
    autoHide: true,
    useChildWindow: false,
    inputSpeed1: {
        min: '300',
        max: '400'
    },
    inputSpeed2: {
        min: '100',
        max: '400'
    }
})

const handleSubmit = () => {
    console.log('Form submitted:', form.value)
}

</script>

<style scoped>
.toggle-checkbox:checked {
    @apply right-0 border-blue-600;
}

.toggle-checkbox:checked+.toggle-label {
    @apply bg-blue-600;
}
</style>