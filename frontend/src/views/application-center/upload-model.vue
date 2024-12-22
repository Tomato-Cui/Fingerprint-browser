<script setup>
import {
    CloudUpload as CloudUploadIcon,
} from 'lucide-vue-next'
import { Model, ModelClose } from '@/components/model/index'
import { Input } from '@/components/ui/input'
import {
    Tabs,
    TabsContent,
    TabsList,
    TabsTrigger,
} from '@/components/ui/tabs'

const props = defineProps({
    open: false,
    title: '',
    disableTab: '',
});

const emit = defineEmits(['close', 'submit']);
</script>

<template>
    <Model class="max-w-[1125px]" :title="props.title" :open="props.open"
        @close="() => emit('close')">
        <Tabs default-value="chrome-software" class="w-full p-8">
            <div class="flex justify-start items-center gap-x-8">
                <p class="whitespace-nowrap flex justify-end items-center w-20">
                    上传类型
                </p>
                <div>
                    <TabsList class="grid w-full grid-cols-2">
                        <TabsTrigger value="package" :disabled="props.disableTab === 'package'">
                            安装包
                        </TabsTrigger>
                        <TabsTrigger value="chrome-software">
                            Chrome应用商店
                        </TabsTrigger>
                    </TabsList>
                </div>
            </div>
            <TabsContent value="chrome-software">
                <div class="flex gap-x-8 py-4">
                    <p class="whitespace-nowrap flex justify-end items-center w-24">
                        应用URL
                    </p>
                    <Input class="" />
                </div>
                <div class="w-full pl-28">
                    <div class="bg-gray-100 rounded-lg p-4 space-y-2 my-4">
                        <div class="text-gray-600 mb-2">温馨提示:</div>
                        <ol class="space-y-1 text-sm text-gray-600">
                            <li>1.此上传类型需使用非中国大陆网络。</li>
                            <li>2.应用中遇三方接收，需终解释权归属三方所有。</li>
                            <li class="flex items-center gap-1">
                                3.前往
                                <a href="#" class="text-blue-600 hover:underline">Chrome 应用商店</a>
                                ，复制/粘贴上传应用链接。
                            </li>
                        </ol>

                        <img src="@/assets/images/chrome-extensiions-help.png" class="rounded-md">
                    </div>
                </div>
            </TabsContent>
            <TabsContent value="package">
                <div class="flex flex-col justify-start gap-x-8 py-4">
                    <div class="flex flex-col gap-x-8 py-4">
                        <div class="flex gap-x-8 py-2">
                            <p class="whitespace-nowrap flex items-center justify-end w-20">
                                安装包
                            </p>
                            <button
                                class="flex border-blue-200 border-[1.5px] rounded-md px-1 py-1 text-xs items-center border-dashed">
                                <CloudUploadIcon class="text-blue-400" />
                                上传应用
                            </button>
                        </div>
                        <span class="text-gray-400 text-xs pl-28">
                            支持zip格式，限制60M<br />
                            若提供文件为 crx 格式，请先将文件压缩成为 zip 格式式，解压后重新压缩为 zip 格式
                        </span>
                    </div>
                    <div class="flex flex-col gap-x-8 py-4">
                        <div class="flex gap-x-8 py-2">
                            <p class="whitespace-nowrap flex items-center justify-end w-20">
                                图标
                            </p>
                            <button
                                class="flex flex-col w-32 h-32 justify-center border-blue-200 border-[1.5px] rounded-md px-1 py-1 text-xs items-center border-dashed">
                                <CloudUploadIcon class="text-blue-400" />
                                点击上传或将图片拖拽到此处
                            </button>
                        </div>
                        <span class="text-gray-400 text-xs pl-28">
                            选填，至少60*60像素, 比例1:1，支持jpg/jpeg/png格式，限制1M
                        </span>
                    </div>
                    <div class="flex gap-x-8 py-4">
                        <p class="whitespace-nowrap flex items-center justify-end w-24">
                            应用名称
                        </p>
                        <Input class="" placeholder="必填，请填写应用名称" />
                    </div>
                    <div class="flex gap-x-8 py-4">
                        <p class="whitespace-nowrap flex items-center justify-end w-24">
                            简介
                        </p>
                        <Input class="" placeholder="选填，请填写应用的简介说明" />
                    </div>
                </div>
            </TabsContent>
            <div class="pl-28">
                <div class="flex gap-4 mt-2">
                    <ModelClose>
                        <button @click="emit('submit')"
                            class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
                            确定
                        </button>
                    </ModelClose>
                    <ModelClose>
                        <button @click="emit('close')" class="px-6 py-2 text-gray-600 hover:text-gray-900">
                            取消
                        </button>
                    </ModelClose>
                </div>
            </div>
        </Tabs>
    </Model>
</template>