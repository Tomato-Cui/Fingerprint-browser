<template>
    <!-- 设计的弹窗 -->
    <!-- <Model :open="props.open" title="导出环境" @close="emit('close')" class="min-w-[700px]">
        <div class="p-6 space-y-6">
            <div class="bg-white rounded-lg p-3 flex items-start gap-3 border shadow-xl">
                <InfoCircleIcon class="w-5 h-5 text-blue-500 flex-shrink-0 mt-0.5" />
                <span class="text-sm text-gray-700">Cookie 信息过多时可使用「文本文件」导出</span>
            </div>

            <div class="text-sm text-gray-600">已选择项目 {{ selectedCount }}</div>

            <div class="space-x-10 flex items-center">
                <label class="text-gray-700 font-[600]">文件类型</label>
                <div class="space-y-3">
                    <RadioGroup defaultValue="option-one" class="flex items-center space-x-4">
                        <div class="flex items-center space-x-2">
                            <RadioGroupItem value="option-one" id="option-one"
                                class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                            <Label htmlFor="option-one" class="font-[500]">Excel 文件 (*.xlsx)</Label>
                        </div>
                        <div class="flex items-center space-x-2">
                            <RadioGroupItem value="option-two" id="option-two"
                                class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                            <Label htmlFor="option-two" class="font-[500]">文本文件 (*.txt</Label>
                        </div>
                    </RadioGroup>
                </div>
            </div>

            <div class="flex items-center space-x-10">
                <span class="text-gray-700 font-[600]">同步密码</span>
                <div class="flex items-center space-x-4">
                    <Switch />
                    <span>同时导出密码</span>
                </div>
            </div>

            <div class="flex space-x-10">
                <label class="text-gray-700 font-[600]">代理类型</label>
                <div>
                    <div class="text-sm text-gray-600 mb-2">
                        验证码发送邮箱：jojolin@gmail.com
                    </div>
                    <div class="flex gap-2 border rounded-lg">
                        <input v-model="verificationCode" type="text" placeholder="输入验证码"
                            class="flex-1 px-3 py-2 rounded-md focus:outline-none" />
                        <button @click="sendVerificationCode"
                            class="px-4 py-2 text-blue-600 hover:bg-blue-50 transition-colors border-l">
                            发送验证码
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <div class="flex justify-end gap-3 p-4 border-t">
            <cancelButton @click="close">
                取消
            </cancelButton>
            <primaryButton @click="confirm">
                确认导出
            </primaryButton>
        </div>
    </Model> -->

    <!--  -->
      <AlertModel :open="props.open" title="导出文件" @close="close" @submit="confirm" @cancel="close">
        确定导出吗
      </AlertModel>
</template>

<script setup lang="ts">
import { AlertModel } from '@/components/alert-model';
import { Model } from '@/components/model'
import primaryButton from '@/components/button/primary-button.vue'
import cancelButton from '@/components/button/cancel-button.vue'
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import Switch from '@/components/ui/switch/Switch.vue';
import { ref } from 'vue'
import { InfoCircleIcon } from '@/assets/icons/system-operation'
import { convertToCSV, downloadCSV } from "@/util/lib";
import { toast } from 'vue-sonner';

const props = defineProps({
    open: Boolean,
    data: <any>[],
    selectedCount: {
        type: Number,
        default: 1
    }
})

const emit = defineEmits(['close', 'confirm'])

const fileType = ref('excel')
const syncPassword = ref(false)
const verificationCode = ref('')

const sendVerificationCode = () => {
    // Implement verification code sending logic
    console.log('Sending verification code...')
}

const close = () => {
    emit('close')
}

const confirm = () => {
    // emit('confirm', {
    //     fileType: fileType.value,
    //     syncPassword: syncPassword.value,
    //     verificationCode: verificationCode.value
    // })
    downloadCSV(convertToCSV(props.data));
    emit('close')
    toast("导出成功")
}
</script>