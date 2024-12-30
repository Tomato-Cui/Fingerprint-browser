<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineEmits, defineProps, reactive } from 'vue'
import { UserPlus2Icon, UsersIcon } from 'lucide-vue-next'
import { IconCreateTeam, IconJoinTeam } from "@/assets/icons";
import { environment_proxies_modify } from "@/commands/environment-proxy"

const props = defineProps({
    editProxyDialog: Boolean,
    environmentUuid: String
})
const emit = defineEmits(['update:editProxyDialog'])

const joinForm = reactive({
    teamCode: "",
    teamLink: "",
    description: ""
})
const createForm = reactive({
    teamName: "",
    remark: ""
})
//确认
const subMit = () => {
    emit('update:editProxyDialog', false)
}

const environmentNumber = ref(7)  // 环境序号
const currentTab = ref('custom')  // 当前选项卡
const activeDropdown = ref(null)  // 当前下拉框

const tabs = [  // 选项卡
    { id: 'custom', name: '自定义' },
    { id: 'added', name: '已添加代理' }
]

const proxyTypes = ['NO Proxy (本地直连)', 'HTTP', 'SOCKS5']  // 代理类型
const selectedProxyType = ref('NO Proxy (本地直连)')   // 默认选择本地直连

const ipChannels = ['IP2Location', 'IPInfo', 'MaxMind']  // IP查询通道
const selectedIpChannel = ref('IP2Location')  // 默认选择IP2Location

const toggleDropdown = (dropdown) => {  // 切换下拉框
    activeDropdown.value = activeDropdown.value === dropdown ? null : dropdown
}

const selectProxyType = (type) => {  // 选择代理类型
    selectedProxyType.value = type
    activeDropdown.value = null
}

const selectIpChannel = (channel) => {  // 选择IP查询通道
    selectedIpChannel.value = channel
    activeDropdown.value = null
}

// Close dropdown when clicking outside
const handleClickOutside = (event) => {  // 点击外部关闭下拉框
    if (!event.target.closest('.relative')) {
        activeDropdown.value = null
    }
}

</script>

<template>
    <Model class="min-w-[700px]" :title="'修改代理'" :open="props.editProxyDialog" @close="() => emit('update:editProxyDialog', false)">
        <div class="space-x-4 p-4 flex flex-col">
            <!-- 在这里书写弹出框主题内容代码 -->

            <!-- 主体 -->
            <div class="px-6 pb-8 h-[400px]">
                <!-- Environment Number -->
                <div class="mb-6">
                    <label class="text-gray-600">环境序号</label>
                    <div class="inline-block ml-4 bg-blue-50 px-4 py-1 rounded">
                        <span class="text-blue-600">{{ props.environmentUuid }}</span>
                    </div>
                </div>

                <!-- Platform Tabs -->
                <div class="mb-6">
                    <label class="text-gray-600 mb-2 block">账号平台</label>
                    <div class="bg-gray-100 inline-flex rounded-lg p-1">
                        <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id" :class="[
                            'px-4 py-2 rounded-md transition-colors',
                            currentTab === tab.id
                                ? 'bg-white text-blue-600'
                                : 'text-gray-600 hover:text-gray-900'
                        ]">
                            {{ tab.name }}
                        </button>
                    </div>
                </div>

                <!-- Custom Configuration Form -->
                <div v-if="currentTab === 'custom'" class="space-y-4">
                    <!-- Proxy Type Dropdown -->
                    <div class="relative">
                        <label class="text-gray-600 mb-2 block">代理类型</label>
                        <div class="flex items-center">
                            <div class="relative flex-1">
                                <button @click="toggleDropdown('proxyType')"
                                    class="w-full px-4 py-2 text-left border rounded-lg flex items-center justify-between hover:border-gray-400">
                                    <span>{{ selectedProxyType }}</span>
                                    <svg class="w-5 h-5 text-gray-400"
                                        :class="{ 'transform rotate-180': activeDropdown === 'proxyType' }" fill="none"
                                        stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M19 9l-7 7-7-7" />
                                    </svg>
                                </button>

                                <div v-if="activeDropdown === 'proxyType'"
                                    class="absolute w-full mt-1 bg-white border rounded-lg shadow-lg z-10">
                                    <div v-for="type in proxyTypes" :key="type" @click="selectProxyType(type)"
                                        class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                        {{ type }}
                                    </div>
                                </div>
                            </div>
                            <div class="ml-2">
                                <div class="w-8 h-8 text-blue-500">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" class="w-full h-full">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- IP Lookup Channel Dropdown -->
                    <div class="relative">
                        <label class="text-gray-600 mb-2 block">IP查询通道</label>
                        <div class="relative">
                            <button @click="toggleDropdown('ipChannel')"
                                class="w-full px-4 py-2 text-left border rounded-lg flex items-center justify-between hover:border-gray-400">
                                <span>{{ selectedIpChannel }}</span>
                                <svg class="w-5 h-5 text-gray-400"
                                    :class="{ 'transform rotate-180': activeDropdown === 'ipChannel' }" fill="none"
                                    stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M19 9l-7 7-7-7" />
                                </svg>
                            </button>

                            <div v-if="activeDropdown === 'ipChannel'"
                                class="absolute w-full mt-1 bg-white border rounded-lg shadow-lg z-10">
                                <div v-for="channel in ipChannels" :key="channel" @click="selectIpChannel(channel)"
                                    class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                    {{ channel }}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Added Proxy Form -->
                <div v-else-if="currentTab === 'added'" class="space-y-4">
                    <div class="relative">
                        <label class="text-gray-600 mb-2 block">选择代理</label>
                        <div class="flex items-center">
                            <div class="relative flex-1">
                                <button @click="toggleDropdown('proxySelect')"
                                    class="w-full px-4 py-2 text-left border rounded-lg flex items-center justify-between hover:border-gray-400">
                                    <span class="text-gray-400">请选择</span>
                                    <svg class="w-5 h-5 text-gray-400"
                                        :class="{ 'transform rotate-180': activeDropdown === 'proxySelect' }"
                                        fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M19 9l-7 7-7-7" />
                                    </svg>
                                </button>
                            </div>
                            <div class="ml-2">
                                <div class="w-8 h-8 text-blue-500">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" class="w-full h-full">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex justify-center border-t">
                <div class="flex justify-start py-8 gap-x-4">
                    <PrimaryButton class="px-8" @click="subMit">确定
                    </PrimaryButton>
                    <CancelButton class="px-8" @click="() => emit('update:editProxyDialog', false)">取消
                    </CancelButton>
                </div>
            </div>
        </div>
    </Model>
</template>