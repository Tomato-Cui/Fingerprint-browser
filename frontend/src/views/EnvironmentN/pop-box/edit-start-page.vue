<script setup lang="ts">
import { Model } from '@/components/model';
import primaryButton from '@/components/button/primary-button.vue';
import cancelButton from '@/components/button/cancel-button.vue';
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import Switch from '@/components/ui/switch/Switch.vue';

const props = defineProps<{
    open: Boolean
}>()
const emit = defineEmits(['close'])

const cancel = () => {
    emit('close')
}
const commit = () => {
    emit('close')
}
</script>

<template>
    <Model :open="props.open" title="修改启动页" @close="emit('close')" class="min-w-[600px]">

        <div class="w-full py-6 px-[30px] space-y-3">
            <div>
                <p class="flex items-center justify-between font-[600]"> <span>环境启动后将打开指定网页</span> <span
                        class="text-gray-400">已选择项目 1</span></p>
                <p class="text-gray-400"> 设置成功后启动环境将以当前配置的最新URL为准</p>
            </div>
            <div class="space-y-8">
                <div class="rounded-lg">
                    <div class="flex gap-[100px]">
                        <div class="">
                            <span class="font-[600]">启动后</span>
                        </div>
                        <RadioGroup defaultValue="option-one" class="m-0">
                            <div class="flex items-center space-x-2">
                                <RadioGroupItem value="option-one" id="option-one"
                                    class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                <Label htmlFor="option-one" class="font-[500]">继续浏览上次打开的网页</Label>
                            </div>
                            <div class="flex items-center space-x-2">
                                <RadioGroupItem value="option-two" id="option-two"
                                    class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                <Label htmlFor="option-two" class="font-[500]">打开指定网页</Label>
                            </div>
                        </RadioGroup>
                    </div>
                </div>
                <div class="rounded-lg">
                    <div class="gap-[80px] flex">
                        <div class="">
                            <span class="font-[600]">其他设置</span>
                        </div>
                        <div class="space-y-3">
                            <div class="flex items-center space-x-3">
                                <Switch /><span>同时打开「账号」已配置的平台网页</span> 
                            </div>
                            <div class="flex items-center space-x-3">
                                <Switch /><span>打开后默认停留在检测页</span> 
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- footer -->
        <div class="px-4 py-2 w-full flex items-center justify-end border-t space-x-3">
            <cancelButton @click="cancel">取消</cancelButton>
            <primaryButton @click="commit">确认</primaryButton>
        </div>
    </Model>
</template>