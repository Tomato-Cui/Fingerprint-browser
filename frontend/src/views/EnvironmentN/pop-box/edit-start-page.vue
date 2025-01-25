<script setup lang="ts">
import { ref, watch } from 'vue';
import { Model } from '@/components/model';
import primaryButton from '@/components/button/primary-button.vue';
import cancelButton from '@/components/button/cancel-button.vue';
import Input from '@/components/input.vue';
import { environment_modify_default_url } from '@/commands/environment'
import { toast } from 'vue-sonner';

const props = defineProps<{
    open: Boolean,
    data: any[],
}>()
const emit = defineEmits(['close'])

const cancel = () => {
    emit('close')
}
const commit = async () => {
    try {
        let urlStr = ''
        for (let i = 0; i < forms.value.length; i++) {
            if (forms.value[i].value) {
                let str = ''
                if(i === 0){
                    str = forms.value[i].value
                }else{
                    str = "," + forms.value[i].value
                }
                urlStr += str
            }
        }
        props.data.forEach(async (item: any) => {
            await environment_modify_default_url(item.uuid, urlStr).catch((res: any) => {
                toast.warning("修改失败：",res.message)
            })
        })
        toast.success('修改成功')
        emit('close')
    } catch {
        toast.error("修改错误")
    }
}

interface Form {
    id: number;
    value: string;
}
const forms = ref<Form[]>([
    { id: 1, value: '' }
])
watch(() => props.open, (_) => {
    forms.value = [
        { id: 1, value: '' }
    ]
})
</script>

<template>
    <Model :open="props.open" title="修改启动页" @close="emit('close')" class="min-w-[600px]">

        <!-- 原版 -->
        <!-- <div class="w-full py-6 px-[30px] space-y-3">
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
        </div> -->

        <div class="w-full px-6 space-y-3">
            <div><span class="text-gray-400">已选择项目 {{ props.data.length }}</span>
            </div>
            <div class="flex w-full space-x-4">
                <span class="font-[600]">启动打开网页</span>
                <div class="flex-1 space-y-3 max-h-[300px] overflow-auto p-1">
                    <Input v-for="item in forms" placeholder="输入网页地址" class="w-full" v-model="item.value"/>
                </div>
            </div>
            <a href="#" class="text-blue-300 hover:text-blue-600 w-full flex justify-end"
                @click="forms.push({ id: forms.length + 1, value: '' })">添加</a>
        </div>

        <!-- footer -->
        <div class="px-4 py-2 w-full flex items-center justify-end border-t space-x-3">
            <cancelButton @click="cancel">取消</cancelButton>
            <primaryButton @click="commit">确认</primaryButton>
        </div>
    </Model>
</template>