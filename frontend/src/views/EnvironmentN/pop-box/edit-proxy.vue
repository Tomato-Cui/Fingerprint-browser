<script setup lang="ts">
import { Model } from '@/components/model';
import primaryButton from '@/components/button/primary-button.vue';
import cancelButton from '@/components/button/cancel-button.vue';
import { ref, reactive, watch } from 'vue'
import { QuestionCircleIcon } from '@/assets/icons/environment/operate';
// import EmptyFolder from '@/assets/icons/environment/operate/Empty folder 1.png';
import { IconEmptyFolder } from '@/assets/icons/environment-trash/index';
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/select";
import BuyProxy from './buy-proxy.vue';
import Input from '@/components/input.vue';
import { environment_modify_proxy } from '@/commands/environment-proxy';
import { toast } from 'vue-sonner';

const props = defineProps({
    open: Boolean,
    data: <any>[],
    isChange: Boolean
})
const emit = defineEmits(['close', 'update:isChange'])
watch(() => props.open, (val) => {
    if (val) {
        activeTab.value = 'same'
        chooseProxy.value = 1
        forms.kind = ''
        forms.host = ''
        forms.port = ''
        forms.username = ''
        forms.password = ''
    }
})

const cancel = () => {
    emit('close')
}
const commit = () => {
    try {
        // 检查是否有环境正在使用中
        props.data.forEach((item: any) => {
            environment_modify_proxy(item.uuid, forms)
        })
        toast.success('修改代理成功');
        emit('update:isChange', !props.isChange)
        emit('close')
    } catch (error) {
        toast.error('删除环境失败，请重试');
    }
}

const tabs = [
    { id: 'same', name: '相同代理' },
    { id: 'different', name: '不同代理' }
]
const proxy = ref([
    { id: 0, value: 'NO Proxy (本地直连)' },
    { id: 1, value: 'HTTP' },
    { id: 2, value: 'HTTPS' },
    { id: 3, value: 'SSH' },
    { id: 4, value: 'Luminati（静态）' },
    { id: 5, value: 'SOCKS5' },
    { id: 6, value: 'Lumauto（Luminati动态）' },
    { id: 7, value: '360Proxy（动态）' },
    { id: 8, value: '922S5（动态）' },
    { id: 9, value: 'IPHTML（动态）' },
    { id: 10, value: 'PiaProxy（动态）' },
    { id: 11, value: 'ABCProxy（动态）' },
    { id: 12, value: 'Oxylabs（静态）' },
    { id: 13, value: 'Oxylabsauto（Oxylabs动态）' },
    { id: 14, value: 'Proxys.io' },
    { id: 15, value: 'Trojan' },
    { id: 16, value: 'Shadowsocks' },
])

const activeTab = ref('same')
const protocolType = ref('auto')
const ipChannel = ref('IP2location')
const chooseProxy = ref(1)
const buyProxyModel = ref(false)

interface Proxy {
    id: number
    value: string
}

const proxyList = ref<Proxy[]>([
    { id: 1, value: '' },
    { id: 2, value: '' },
    { id: 3, value: '' }
])
const protocolList = [
    { id: 1, value: 'HTTP' },
    { id: 2, value: 'HTTPS' },
    { id: 3, value: 'SOCKS5' },
    { id: 4, value: 'SSH' }
]
const IPFindList = [
    { id: 1, value: 'IP2Location' },
    { id: 2, value: 'MaxMind' },
    { id: 3, value: 'DB-IP' },
    { id: 4, value: 'IP-API' }
]

const clearProxies = () => {
    proxyList.value.forEach(proxy => proxy.value = '')
}

const testProxy = () => {
    console.log('Testing proxies...')
}
const proxyMenu = [
    { value: 1, label: '新代理' },
    { value: 2, label: '选择已有代理' }
]
const forms = reactive({
    kind: '',
    host: '',
    port: '',
    username: '',
    password: ''
})

</script>

<template>
    <Model :open="props.open" title="修改代理" @close="emit('close')" class="min-w-[750px]">

        <div class="p-6">
            <!-- Tabs -->
            <div class="w-full">
                <div class="flex items-center justify-between px-6 h-[35px] space-x-14">
                    <div class="space-x-2 rounded-md h-11 px-1 bg-gray-100 md:flex">
                        <button v-for="item in tabs" @click="activeTab = item.id"
                            class="px-3 my-1 rounded-md whitespace-nowrap"
                            :class="{ 'bg-white shadow-lg': item.id === activeTab }">
                            {{ item.name }}
                        </button>
                    </div>
                </div>
            </div>

            <!-- Form Content -->
            <transition name="fade" mode="out-in">
                <div :key="activeTab">
                    <div v-if="activeTab === 'different'" class="space-y-6">
                        <div class="flex items-center justify-between gap-4">
                            <div class="flex items-center gap-2">
                                <span class="text-gray-700">代理信息</span>
                                <QuestionCircleIcon class="w-4 h-4 text-gray-400" />
                            </div>
                            <div class="flex items-center gap-4">
                                <span class="text-gray-700">协议类型</span>
                                <div class="w-40">
                                    <Select>
                                        <SelectTrigger>
                                            <SelectValue v-model="protocolType" placeholder="选择协议类型"
                                                class="p-2 w-full rounded-lg outline-none" />
                                        </SelectTrigger>

                                        <SelectContent>
                                            <SelectGroup>
                                                <SelectItem v-for="item in protocolList" :value="item.value">
                                                    {{
                                                        item.value }}
                                                </SelectItem>
                                            </SelectGroup>
                                        </SelectContent>
                                    </Select>
                                </div>

                            </div>
                        </div>

                        <!-- Proxy Inputs -->
                        <div class="space-y-4 bg-white border rounded-lg p-4">
                            <TransitionGroup name="list" tag="div" class="space-y-4 max-h-[200px] overflow-auto">
                                <div v-for="(data, index) in props.data" :key="data.id"
                                    class="flex items-center gap-4 p-1">
                                    <span class="text-gray-500 w-8">{{ index + 1 }}</span>
                                    <span class="text-gray-700 w-[100px]">{{ data.name }}</span>
                                    <span class="text-gray-500">:</span>
                                    <!-- TODO 每一个环境的代理（绑定有问题） -->
                                    <Input v-model="data.proxy_kind" placeholder="输入或粘贴代理，支持多行" class="flex-1" />
                                </div>

                            </TransitionGroup>
                        </div>

                        <!-- Clear Button -->
                        <div class="flex justify-end">
                            <button @click="clearProxies" class="text-blue-600 text-sm hover:text-blue-700">
                                清空
                            </button>
                        </div>

                        <!-- IP Channel -->
                        <div class="flex items-center justify-between gap-5">
                            <span class="text-gray-700">IP查询渠道</span>
                            <div class="flex-1">
                                <!-- <Select >
                                    <SelectTrigger disabled>
                                        <SelectValue v-model="ipChannel" placeholder="IP2location"
                                            class="p-2 w-full rounded-lg outline-none" />
                                    </SelectTrigger>

                                    <SelectContent>
                                        <SelectGroup>
                                            <SelectItem v-for="item in IPFindList" :value="item.value">
                                                {{ item.value }}
                                            </SelectItem>
                                        </SelectGroup>
                                    </SelectContent>
                                </Select> -->
                                <Select>
                                    <SelectTrigger class="w" disabled>
                                        <SelectValue placeholder="IP2Location"
                                            class="w-full p-2 rounded-lg outline-none" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectGroup>
                                            <SelectItem value="IP2Location"> IP2Location </SelectItem>
                                        </SelectGroup>
                                    </SelectContent>
                                </Select>
                            </div>
                        </div>
                    </div>
                    <div v-if="activeTab === 'same'">
                        <p class="text-gray-500 mx-8 my-4">已选择代理 {{ props.data.length }}</p>
                        <div class="flex items-center gap-4 mb-4 mx-12">
                            <span class="w-[100px] font-[600]">代理</span>
                            <div class="flex items-center rounded-lg gap-4 border px-2 py-1">
                                <button @click="chooseProxy = 1" class="p-2 font-[600] transition-all duration-200"
                                    :class="{ 'rounded-lg bg-[#5050FA] text-white': chooseProxy == 1 }">
                                    新代理
                                </button>
                                <button @click="chooseProxy = 2" class="p-2 font-[600] transition-all duration-200"
                                    :class="{ 'rounded-lg bg-[#5050FA] text-white': chooseProxy == 2 }">
                                    选择已有代理
                                </button>
                            </div>
                            <!-- <div class="flex items-center gap-2 cursor-pointer" @click="buyProxyModel = true">
                                <ShoppingCartIcon class="size-5" />
                                <span class="text-[#5050FA]">购买代理</span>
                            </div> -->
                        </div>
                        <div v-if="chooseProxy == 1" class="flex flex-col gap-4 mx-12">
                            <div class="flex items-center gap-4">
                                <span class="w-[100px] font-[600]">代理类型 <span class="text-red-600">*</span></span>
                                <div class="flex flex-col flex-1">
                                    <div class="flex-1">
                                        <Select v-model="forms.kind">
                                            <SelectTrigger>
                                                <SelectValue placeholder="选择代理类型"
                                                    class="p-2 w-full rounded-lg outline-none" />
                                            </SelectTrigger>

                                            <SelectContent>
                                                <SelectGroup>
                                                    <SelectItem v-for="item in proxy" :value="item.value">{{ item.value
                                                        }}
                                                    </SelectItem>
                                                </SelectGroup>
                                            </SelectContent>
                                        </Select>
                                    </div>
                                </div>
                            </div>

                            <div className="flex flex-col space-y-4"
                                v-if="forms.kind !== 'NO Proxy (本地直连)' && forms.kind !== ''">
                                <div className="flex gap-4 items-center">
                                    <p class="w-[100px] text-left text-sm relative font-[600]">代理主机 <span
                                            class="text-red-600">*</span>
                                    </p>
                                    <Input v-model="forms.host" class="flex-1" placeholder="输入代理主机" />
                                </div>
                                <div className="flex gap-4 items-center">
                                    <p class="w-[100px] text-left text-sm relative font-[600]">
                                        代理端口 <span class="text-red-600">*</span>
                                    </p>
                                    <Input v-model="forms.port" class="flex-1" placeholder="输入代理端口" />
                                </div>
                                <div className="flex gap-4 items-center">
                                    <p class="w-[100px] text-left text-sm font-[600]">代理账号</p>
                                    <Input v-model="forms.username" class="flex-1" placeholder="输入代理账号" />
                                </div>
                                <div className="flex gap-4 items-center">
                                    <p class="w-[100px] text-left text-sm font-[600]">代理密码</p>
                                    <Input v-model="forms.password" class="flex-1" placeholder="输入代理密码" />
                                </div>
                            </div>
                        </div>
                        <div v-else class="flex flex-col items-center justify-center p-6">
                            <IconEmptyFolder class="size-[180px]" />
                            <p>无代理可选，您可选择『新代理』或『购买代理』</p>
                        </div>
                    </div>
                </div>

            </transition>
        </div>

        <!-- footer -->
        <div class="px-4 py-2 w-full flex items-center justify-end border-t space-x-3">
            <!-- <a href="#" class="text-blue-500 hover:text-blue-600">代理检测</a> -->
            <cancelButton @click="cancel">取消</cancelButton>
            <primaryButton @click="commit">确认</primaryButton>
        </div>
    </Model>
    <BuyProxy :open="buyProxyModel" @close="buyProxyModel = false" />
</template>

<style scoped>
.list-enter-active,
.list-leave-active {
    transition: all 0.3s ease;
}

.list-enter-from,
.list-leave-to {
    opacity: 0;
    transform: translateX(30px);
}

/* 定义过渡效果 */
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}
</style>