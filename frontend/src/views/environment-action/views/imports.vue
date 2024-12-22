<script setup>
import { IconLinuxOs, IconMacOs, IconWindowOs, IconFirefox, IconChrome } from '@/assets/icons/index'
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@/components/ui/accordion'
import { Switch } from '@/components/ui/switch'
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select'
import Input from '@/components/input.vue';
import { CopyIcon, FolderIcon, FolderPlusIcon } from 'lucide-vue-next'
import {
    Tabs,
    TabsContent,
    TabsList,
    TabsTrigger,
} from '@/components/ui/tabs'
import Alignment from '../components/alignment.vue'
</script>
<template>

    <div class="space-y-4 px-6 py-3">
        <p class="text-gray-600 text-sm">请点击下载窗口导入模板，一次性最多导入500个窗口</p>

        <div class="border-2 border-dashed border-gray-200 rounded-lg p-8 text-center"
            @dragover.prevent="handleDragOver" @dragleave.prevent="handleDragLeave" @drop.prevent="handleDrop"
            :class="{ 'bg-blue-50 border-blue-200': isDragging }">
            <div class="mb-4">
                <FolderIcon class="w-12 h-12 mx-auto text-gray-300" />
            </div>

            <p class="text-gray-600 mb-2">请拖拽到此处或点击上传</p>
            <p class="text-sm text-gray-500">请将编辑好的文件在此处上传，仅支持.xlsx类型文件，最大不超过5MB。</p>

            <input type="file" ref="fileInput" accept=".xlsx" class="hidden" @change="handleFileSelect">

            <div class="flex justify-center">
                <button
                    class="mt-4 px-6 py-2 text-sm border border-gray-200 rounded-md hover:bg-gray-50 flex  justify-center items-center">
                    <FolderPlusIcon class="w-6 h-6 mx-auto text-gray-300 mr-2" />
                    上传文件
                </button>
            </div>
        </div>
    </div>

    <Accordion type="single" class="w-full px-6 py-3" collapsible>
        <AccordionItem class="border-0" value="basic-setting">
            <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">基础设置</AccordionTrigger>
            <AccordionContent class="px-4 space-y-4">
                <div class="space-y-2 flex items-center">
                    <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">名称</p>
                    <div class="relative text-sm w-full">
                        <Input type="text" placeholder="请输入环境名称" class="w-full" />
                        <span class="absolute right-3 top-2 text-xs text-gray-400">0/100</span>
                    </div>
                </div>



                <div class="space-y-2 flex items-center">
                    <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">浏览器</p>
                    <div class="w-full">
                        <div class="w-1/2 flex gap-x-4">
                            <Select>
                                <SelectTrigger>
                                    <IconChrome class="w-6 h-6 " />
                                    <SelectValue placeholder="Chrome" class="w-full pl-4 rounded-lg outline-none" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="chrome">
                                            Chrome
                                        </SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                            <Select>
                                <SelectTrigger>
                                    <IconFirefox class="w-6 h-6 " />
                                    <SelectValue placeholder="FireFox" class="w-full pl-4 rounded-lg outline-none" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="firefox">
                                            FireFox
                                        </SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                </div>

                <div class="space-y-2 flex items-center">
                    <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">操作系统
                    </p>
                    <div class="w-full">
                        <div class="flex gap-4 ">
                            <button
                                class="flex items-center gap-2 px-4 py-2 border rounded-md bg-blue-50 border-blue-200  text-blue-600">
                                <Input type="checkbox" checked class="rounded" />
                                <IconWindowOs class="w-5 h-5" />
                                Windows
                            </button>
                            <button class="flex items-center gap-2 px-4 py-2 border rounded-md ">
                                <Input type="checkbox" class="rounded" />
                                <IconMacOs class="w-5 h-5" />
                                MacOs
                            </button>
                            <button class="flex items-center gap-2 px-4 py-2 border rounded-md ">
                                <Input type="checkbox" class="rounded" />
                                <IconLinuxOs class="w-5 h-5" />
                                Linux
                            </button>
                        </div>
                    </div>
                </div>

                <div class="space-y-2 flex items-center">
                    <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">User-Agent</p>
                    <div class="w-full">
                        <div class="flex gap-4 ">
                            <Select>
                                <SelectTrigger class="w-1/4">
                                    <SelectValue placeholder="全部" class="w-full p-2 rounded-lg outline-none" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="all">
                                            全部
                                        </SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                            <div class="flex-auto relative">
                                <Input type="text" value="Mozilla/5.0(Windows NT 10.0;Win64; x64)..."
                                    class="w-full text-sm text-gray-500" readonly />
                                <div class="absolute right-2 top-2 flex gap-2">
                                    <button class="p-1 hover:bg-gray-100 rounded">
                                        <CopyIcon class="w-4 h-4 text-gray-400" />
                                    </button>
                                    <button class="p-1 hover:bg-gray-100 rounded">
                                        <Link class="w-4 h-4 text-blue-600" />
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>



                <div class="space-y-2 flex items-center">
                    <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">分组
                    </p>
                    <div class="w-full">
                        <div class="flex gap-x-4">
                            <Select>
                                <SelectTrigger class="w-full">
                                    <SelectValue placeholder="全部" class="w-full p-2 rounded-lg outline-none" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="all">
                                            全部
                                        </SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                </div>


                <div class="space-y-2 flex items-center">
                    <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">Cookie</p>
                    <div class="w-full">
                        <div class="flex gap-x-4">
                            <Input type="text" value="支持格式：JSON、Netscape、Name=Value"
                                class="text-gray-500 text-sm flex-auto" readonly />
                            <button class="px-4 py-2 hover:text-blue-600 border rounded-md hover:border-blue-200">
                                合并
                            </button>
                        </div>
                    </div>
                </div>


                <!-- Notes -->
                <div class="space-y-2 flex items-center">
                    <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">备注</p>
                    <div class="w-full">
                        <div class="flex gap-x-4 relative">
                            <Input placeholder="请输入备注" rows="3" class="w-full px-3 py-2 border rounded-md"></Input>
                            <span class="absolute right-3 top-2 text-gray-400 text-xs">0/1500</span>
                        </div>
                    </div>
                </div>
            </AccordionContent>
        </AccordionItem>
    </Accordion>


    <!-- Fingerprint Configuration -->
    <Accordion type="single" class="w-full px-6 py-3" collapsible default-value="webrtc-setting">
        <AccordionItem value="webrtc-setting" class="border-0">
            <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">高级指纹信息
            </AccordionTrigger>
            <AccordionContent class="px-4 space-y-4">
                <!-- WebRTC -->
                <Alignment title="WebRTC">
                    <button class="px-4 py-1.5 rounded-md text-sm"
                        :class="webRTC === '转发' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'">转发</button>
                    <button class="px-4 py-1.5 rounded-md text-sm"
                        :class="webRTC === '替换' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'">替换</button>
                    <button class="px-4 py-1.5 rounded-md text-sm"
                        :class="webRTC === '真实' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'">真实</button>
                    <button class="px-4 py-1.5 rounded-md text-sm"
                        :class="webRTC === '禁用' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'">禁用</button>
                </Alignment>


                <!-- Time Zone -->
                <Alignment title="时区">
                    <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">基于
                        IP</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">真实</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">自定义</button>
                </Alignment>

                <!-- Geolocation -->
                <Alignment title="地理位置">
                    <label class="flex items-center gap-2">
                        <Input type="radio" checked class="text-blue-600" />
                        <span class="text-sm">每次询问</span>
                    </label>
                    <label class="flex items-center gap-2">
                        <Input type="radio" class="text-blue-600" />
                        <span class="text-sm">始终允许</span>
                    </label>
                </Alignment>

                <!-- Language -->
                <Alignment title="语言">
                    <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">基于
                        IP</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">自定义</button>
                </Alignment>

                <!-- Interface Language -->
                <Alignment title="界面语言">
                    <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">基于语言</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">真实</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">自定义</button>
                </Alignment>


                <!-- Resolution -->
                <Alignment title="分辨率">
                    <div class="flex gap-2">
                        <button class="px-4 py-1.5 rounded-md text-sm text-gray-600 bg-blue-50">随机</button>
                        <button class="px-4 py-1.5 rounded-md text-sm">预定义</button>
                        <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">自定义</button>
                    </div>
                    <div class="relative">
                        <Select>
                            <SelectTrigger class="w-full">
                                <SelectValue placeholder="基于 User-Agent" class="w-full p-2 rounded-lg outline-none" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    <SelectItem value="user-agent">
                                        基于 User-Agent
                                    </SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>
                </Alignment>


                <!-- Font -->
                <Alignment title="界面语言">
                    <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">默认</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">自定义</button>
                </Alignment>

                <!-- Hardware Noise -->
                <Alignment title="硬件噪音">
                    <div class="space-y-4">
                        <label class="block text-sm font-medium text-gray-700"></label>
                        <div class="flex items-center gap-6">
                            <label class="flex items-center gap-2">
                                <Switch />
                                <span class="text-sm">Canvas</span>
                            </label>
                            <label class="flex items-center gap-2">
                                <Switch />
                                <span class="text-sm">AudioContext</span>
                            </label>
                        </div>
                        <div class="flex items-center gap-6">
                            <label class="flex items-center gap-2">
                                <Switch />
                                <span class="text-sm">WebGL图像</span>
                            </label>
                            <label class="flex items-center gap-2">
                                <Switch />
                                <span class="text-sm">SpeechVoices</span>
                            </label>
                        </div>

                        <div class="flex items-center gap-6">
                            <label class="flex items-center gap-2">
                                <Switch />
                                <span class="text-sm">ClientRects</span>
                            </label>
                            <label class="flex items-center gap-2">
                                <Switch />
                                <span class="text-sm">媒体设备[Auto]</span>
                                <button class="text-blue-600 text-sm hover:underline">编辑</button>
                            </label>
                        </div>
                    </div>
                </Alignment>

                <!-- WebGL Data -->
                <div class="space-y-2 ">
                    <div class="flex">
                        <p class="w-32 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">操作系统
                        </p>
                        <div class="flex gap-2 ">
                            <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">默认</button>
                            <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">自定义</button>
                        </div>
                    </div>
                    <div class="w-full ml-32 space-y-2 pt-4">
                        <label class="block text-sm font-medium text-gray-700">厂商</label>
                        <div class="relative">
                            <Select>
                                <SelectTrigger class="w-full">
                                    <SelectValue placeholder="ANGLE (Intel, Intel(R) UHD Graphics 610 Dir..."
                                        class="p-2 rounded-lg outline-none" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="user-agent">
                                            Google Inc. (Intel)
                                        </SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>

                        <label class="block text-sm font-medium text-gray-700">渲染器</label>
                        <div class="relative">
                            <Select>
                                <SelectTrigger class="w-full">
                                    <SelectValue placeholder="ANGLE (Intel, Intel(R) UHD Graphics 610 Dir..."
                                        class="p-2 rounded-lg outline-none" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="user-agent">
                                            ANGLE (Intel, Intel(R) UHD Graphics 610 Dir...
                                        </SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                </div>


                <!-- WebGPU -->
                <Alignment title="WebGPU">
                    <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">基于
                        WebGL</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">真实</button>
                    <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">禁用</button>
                </Alignment>

            </AccordionContent>
        </AccordionItem>
    </Accordion>



    <!-- Advanced Settings -->
    <Accordion type="single" class="w-full px-6 py-3" collapsible>
        <AccordionItem value="software-setting" class="border-0">
            <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">高级设置
            </AccordionTrigger>
            <AccordionContent class="px-4">
                <Alignment title="应用">
                    <Select>
                        <SelectTrigger class="w-full">
                            <SelectValue placeholder="跟随团队应用" class="p-2 rounded-lg outline-none" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectGroup>
                                <SelectItem value="user-agent">
                                    跟随团队应用
                                </SelectItem>
                            </SelectGroup>
                        </SelectContent>
                    </Select>
                </Alignment>
                <p class="text-sm text-gray-500 ml-[8.25rem] mt-1">[应用中心-团队应用] 中已开启的应用将安装到浏览器。</p>
            </AccordionContent>
        </AccordionItem>
    </Accordion>
</template>