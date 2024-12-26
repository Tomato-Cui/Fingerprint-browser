<script setup>
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { PrimaryButton, CancelButton } from '@/components/button';
import { ShuffleIcon, WaypointsIcon } from 'lucide-vue-next'
import ImportProxyModel from './components/import-proxy-model.vue'
const tabs = [
    { id: 'single', name: '单个创建', href: '/environment-action/create' },
    { id: 'batch', name: '批量创建', href: '/environment-action/creates' },
    { id: 'import', name: '窗口导入', href: '/environment-action/imports' }
]

const route = useRoute();
const router = useRouter();

const importProxyOpen = ref(false)
const proxyModelCloseHandle = () => importProxyOpen.value = false;
const proxyModelOpenHandle = () => importProxyOpen.value = true;

</script>
<template>
    <div class="h-main bg-white m-4 flex flex-col rounded-md">
        <div class="flex border-b mb-2 justify-between pr-4">
            <nav class="flex">
                <button v-for="tab in tabs" :key="tab.id" @click="() => router.push(tab.href)"
                    class="px-6 py-3 -mb-px font-medium"
                    :class="route.path === tab.href ? 'text-blue-600 border-b-2 border-blue-600' : 'text-gray-600'">
                    {{ tab.name }}
                </button>
            </nav>
            <PrimaryButton @click="proxyModelOpenHandle" v-if="route.path === '/environment-action/creates'"
                class="my-1 text-white px-4 flex justify-center items-center gap-x-2 rounded-lg">
                <WaypointsIcon />
                代理导入
            </PrimaryButton>
        </div>
        <div class="flex overflow-hidden gap-x-2 flex-auto">
            <div class="overflow-y-auto flex-auto ">
                <router-view />
            </div>

            <div
                class="hidden xl:flex flex-col bg-gray-50 xl:w-1/3 2xl:w-1/4 rounded-md border shadow-sm p-4 h-[calc(100%-1%)] text-xs">
                <div class="flex items-center justify-between mb-6">
                    <h3 class="text-base font-medium">概要</h3>
                    <button class="p-1 hover:bg-gray-100 rounded">
                        <Menu class="w-5 h-5 text-gray-400" />
                    </button>
                </div>

                <div class="space-y-4 flex-auto">
                    <div class="flex justify-between">
                        <span class="text-gray-600">浏览器</span>
                        <span>SunBrowser[智能匹配]</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">User-Agent</span>
                        <span class="text-right text-xs text-gray-600">Mozilla/5.0 (Windows NT 10.0; Win64; x64)
                            AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.6778.70 Safari/537.36</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">WebRTC</span>
                        <span>禁用</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">时区</span>
                        <span>基于 IP</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">地理位置</span>
                        <span>[询问]基于IP</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">语言</span>
                        <span>基于 IP</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">界面语言</span>
                        <span>基于语言</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">分辨率</span>
                        <span>基于 User-Agent</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">字体</span>
                        <span>默认</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">字体列表</span>
                        <span>默认</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">字体指纹</span>
                        <span>默认</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">WebRTC</span>
                        <span>默认</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">Canvas</span>
                        <span>默认</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">WebGL图像</span>
                        <span>默认</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-600">硬件并发数</span>
                        <span>默认</span>
                    </div>
                    <div class="mt-6 text-sm">
                        <span class="text-gray-600">可前往</span>
                        <a href="#" class="text-blue-600 hover:underline">偏好设置</a>
                        <span class="text-gray-600">自定义指纹默认值。</span>
                    </div>
                </div>

                <button
                    class="w-full mt-6 px-4 py-2 text-blue-500 rounded-md border-2 border-blue-600 hover:border-blue-600 flex items-center justify-center gap-2">
                    <ShuffleIcon class="w-4 h-4" />
                    一键随机生成
                </button>
            </div>
        </div>
        <div class="flex gap-x-4 pt-3 px-6 mb-8 bg-white border-t">
            <PrimaryButton>确定</PrimaryButton>
            <CancelButton @click="router.go(-1)">取消</CancelButton>
        </div>

        <ImportProxyModel :open="importProxyOpen" @close="proxyModelCloseHandle" />
    </div>
</template>