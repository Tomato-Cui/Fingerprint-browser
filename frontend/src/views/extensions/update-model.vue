<script setup>
import { PrimaryButton, CancelButton } from '@/components/button'
import { Model } from '@/components/model/index'
import { Input } from '@/components/ui/input'

const props = defineProps({
    open: false,
    title: '',
});

const emit = defineEmits(['close']);
</script>

<template>
    <Model class="max-w-[1125px]" :title="props.title" :open="props.open"
        @close="() => emit('close')">
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
            <div class="flex justify-start pb-4 gap-x-4">
                <PrimaryButton class="px-8" @click="() => emit('close')">确定</PrimaryButton>
                <CancelButton class="px-8" @click="() => emit('close')">取消</CancelButton>
            </div>
        </div>
    </Model>
</template>