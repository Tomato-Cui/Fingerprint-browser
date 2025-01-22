<template>
    <Model :open="props.open" title="购买代理" @close="emit('close')" class="min-w-[1000px]">
        <!-- 内容区域 -->
        <div class="p-6 space-y-6">
            <!-- 购买须知 -->
            <div class="bg-blue-50 p-4 rounded-lg flex items-start gap-3" v-if="isWarning">
                <InfoIcon class="w-5 h-5 text-blue-500 flex-shrink-0 mt-0.5" />
                <div class="flex-1">
                    <div class="flex items-center justify-between">
                        <span class="text-gray-700">购买须知:</span>
                        <div class="flex gap-2">
                            <button class="text-blue-500 hover:text-blue-600">
                                <ChevronDownIcon class="w-5 h-5" />
                            </button>
                            <button @click="isWarning = false" class="text-gray-400 hover:text-gray-500">
                                <XIcon class="w-5 h-5" />
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <!-- 搜索框 -->
            <div class="relative">
                <SearchIcon class="w-5 h-5 text-gray-400 absolute left-3 top-1/2 transform -translate-y-1/2" />
                <input type="text" placeholder="地域/地区"
                    class="w-full pl-10 pr-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" />
            </div>

            <!-- 代理类型 -->
            <div class="space-y-3">
                <h3 class="text-gray-700">代理类型</h3>
                <div class="flex flex-wrap gap-3">
                    <button v-for="type in proxyTypes" :key="type.id" @click="selectProxyType(type.id)"
                        class="px-4 py-2 rounded-lg border transition-all duration-200" :class="[
                            selectedProxyType === type.id
                                ? 'border-blue-500 bg-blue-50 text-blue-600'
                                : 'border-gray-200 hover:border-blue-500'
                        ]">
                        {{ type.name }}
                    </button>
                </div>
            </div>

            <!-- 地域 -->
            <div class="space-y-3">
                <h3 class="text-gray-700">地域</h3>
                <div class="flex flex-wrap gap-3">
                    <button v-for="region in regions" :key="region.id" @click="selectRegion(region.id)"
                        class="px-4 py-2 rounded-lg border transition-all duration-200" :class="[
                            selectedRegion === region.id
                                ? 'border-blue-500 bg-blue-50 text-blue-600'
                                : 'border-gray-200 hover:border-blue-500'
                        ]">
                        {{ region.name }}
                    </button>
                </div>
            </div>

            <!-- 地区 -->
            <div class="space-y-3">
                <h3 class="text-gray-700">地区</h3>
                <div class="flex flex-wrap gap-3">
                    <button v-for="area in areas" :key="area.id" @click="selectArea(area.id)"
                        class="px-4 py-2 rounded-lg border transition-all duration-200" :class="[
                            selectedArea === area.id
                                ? 'border-blue-500 bg-blue-50 text-blue-600'
                                : 'border-gray-200 hover:border-blue-500'
                        ]">
                        {{ area.name }}
                    </button>
                </div>
            </div>

            <!-- 服务商 -->
            <div class="space-y-3">
                <h3 class="text-gray-700">服务商</h3>
                <div class="flex flex-wrap gap-3">
                    <button v-for="provider in providers" :key="provider.id" @click="selectProvider(provider.id)"
                        class="px-4 py-2 rounded-lg border transition-all duration-200" :class="[
                            selectedProvider === provider.id
                                ? 'border-blue-500 bg-blue-50 text-blue-600'
                                : 'border-gray-200 hover:border-blue-500'
                        ]">
                        {{ provider.name }}
                    </button>
                </div>
            </div>

            <!-- 购买时长 -->
            <div class="space-y-3">
                <h3 class="text-gray-700">购买时长</h3>
                <div class="flex flex-wrap gap-3">
                    <button v-for="duration in durations" :key="duration.id" @click="selectDuration(duration.id)"
                        class="relative px-4 py-2 rounded-lg border transition-all duration-200" :class="[
                            selectedDuration === duration.id
                                ? 'border-blue-500 bg-blue-50 text-blue-600'
                                : 'border-gray-200 hover:border-blue-500'
                        ]">
                        {{ duration.name }}
                        <span v-if="duration.discount"
                            class="absolute -top-2 -right-2 bg-red-500 text-white text-xs px-1.5 py-0.5 rounded">
                            -{{ duration.discount }}%
                        </span>
                    </button>
                </div>
            </div>

            <!-- 购买数量 -->
            <div class="space-y-3">
                <h3 class="text-gray-700">购买数量</h3>
                <input v-model="quantity" type="number" min="1"
                    class="w-32 px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" />
            </div>

            <!-- 价格信息 -->
            <div class="flex items-center justify-between text-sm">
                <div class="flex items-center gap-6">
                    <div>
                        实付金额 <span class="text-orange-500 text-lg">$10.00</span>
                    </div>
                    <div class="text-gray-500">
                        订单金额: $10.00
                    </div>
                    <div class="text-gray-500">
                        优惠金额: -$0.00
                    </div>
                </div>
            </div>

            <!-- 协议同意 -->
            <div class="flex items-center gap-2">
                <input type="checkbox" id="agreement" v-model="agreementAccepted"
                    class="rounded text-blue-500 focus:ring-blue-500" />
                <label for="agreement" class="text-sm text-gray-600">
                    我已经阅读并同意
                    <a href="#" class="text-blue-500 hover:text-blue-600">《技术服务协议》</a>、
                    <a href="#" class="text-blue-500 hover:text-blue-600">《购买须知》</a>
                </label>
            </div>
        </div>

        <!-- 底部按钮 -->
        <div class="p-4 border-t">
            <button @click="submit" :disabled="!agreementAccepted"
                class="w-24 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed">
                提交
            </button>
        </div>
    </Model>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { XIcon, InfoIcon, SearchIcon, ChevronDownIcon } from 'lucide-vue-next'
import { Model } from '@/components/model';

const props = defineProps<{
    open: Boolean
}>()

const emit = defineEmits(['close'])

// 数据
const proxyTypes = [
    { id: 1, name: '宽带' },
    { id: 2, name: '小众' },
    { id: 3, name: '云平台' }
]

const regions = [
    { id: 1, name: 'Middle East' },
    { id: 2, name: 'South America' },
    { id: 3, name: 'Asia Pacific' },
    { id: 4, name: 'Europe' },
    { id: 5, name: 'North America' },
    { id: 6, name: 'Africa' }
]

const areas = [
    { id: 1, name: 'Bahrain' }
]

const providers = [
    { id: 1, name: 'Amazon Cloud' }
]

const durations = [
    { id: 1, name: '30天' },
    { id: 2, name: '90天', discount: 5 },
    { id: 3, name: '180天', discount: 10 },
    { id: 4, name: '360天', discount: 20 }
]

// 状态
const selectedProxyType = ref(3)
const selectedRegion = ref(1)
const selectedArea = ref(1)
const selectedProvider = ref(1)
const selectedDuration = ref(1)
const quantity = ref(1)
const agreementAccepted = ref(false)
const isWarning = ref(true)

// 方法
const selectProxyType = (id: number) => selectedProxyType.value = id
const selectRegion = (id: number) => selectedRegion.value = id
const selectArea = (id: number) => selectedArea.value = id
const selectProvider = (id: number) => selectedProvider.value = id
const selectDuration = (id: number) => selectedDuration.value = id

const submit = () => {
    if (!agreementAccepted.value) return
    console.log('提交订单')
}

const close = () => {
    // 关闭弹窗
}
</script>