<template>
    <Model :open="props.open" title="转移环境" @close="emit('close')" class="min-w-[700px]">
        <!-- 内容区域 -->
        <div class="p-6 space-y-6">
            <!-- 警告信息 -->
            <div class="bg-amber-50 rounded-lg p-4 flex items-center gap-3">
                <AlertCircleIcon class="w-5 h-5 text-amber-500 flex-shrink-0" />
                <p class="text-sm text-gray-700">
                    「转移」操作将把您选择环境的所有权限移交给另一个MoreLogin团队。此操作无法撤销，请谨慎操作！
                </p>
            </div>

            <!-- 已选择项目 -->
            <div class="text-sm text-gray-600">
                已选择项目 <span class="text-blue-600">{{ selectedCount }}</span>
            </div>

            <!-- 同步代理设置 -->
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <span class="text-gray-700 w-[150px] font-[600]">同步代理</span>
                    <div class="flex-1 flex items-center gap-3">
                        <button
                            class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200"
                            :class="syncProxy ? 'bg-blue-600' : 'bg-gray-200'" @click="toggleSyncProxy">
                            <span
                                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200 shadow-sm"
                                :class="syncProxy ? 'translate-x-6' : 'translate-x-1'" />
                        </button>
                        <span class="font-[600]">同时代理信息</span>
                        <span class="text-amber-500 text-sm">(仅同步自有代理)</span>
                    </div>
                </div>
            </div>

            <!-- 接收方邮箱 -->
            <div class="space-y-2 flex items-center">
                <label class="text-gray-700 font-[600] w-[150px]">接受方邮箱地址</label>
                <Input v-model="recipientEmail" type="email" placeholder="请输入接受方邮箱地址"/>
            </div>

            <!-- 验证码 -->
            <div class="flex">
                <label class="text-gray-700 font-[600] w-[150px]">验证邮箱</label>
                <div class="flex-1">
                    <div class="text-sm text-gray-600">
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

        <!-- 底部按钮 -->
        <div class="px-4 py-2 w-full flex items-center justify-end border-t space-x-3">
            <cancelButton @click="cancel">取消</cancelButton>
            <primaryButton @click="commit">确认</primaryButton>
        </div>
    </Model>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Model } from '@/components/model';
import primaryButton from '@/components/button/primary-button.vue';
import cancelButton from '@/components/button/cancel-button.vue';
import Input from '@/components/input.vue';
import { XIcon, AlertCircleIcon } from 'lucide-vue-next'

const props = defineProps({
    open: Boolean,
    data: Array,
    selectedCount: {
        type: Number,
        default: 1
    },
    verificationEmail: {
        type: String,
        default: 'jojolin@gmail.com'
    }
})

const emit = defineEmits(['close', 'confirm'])

const syncProxy = ref(false)
const recipientEmail = ref('')
const verificationCode = ref('')
const isVerificationCodeSent = ref(false)

const toggleSyncProxy = () => {
    syncProxy.value = !syncProxy.value
}

const isFormValid = computed(() => {
    return (
        recipientEmail.value &&
        verificationCode.value &&
        /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(recipientEmail.value)
    )
})

const sendVerificationCode = () => {
    isVerificationCodeSent.value = true
    // 实现发送验证码的逻辑
    console.log('Sending verification code...')
}

const cancel = () => {
    emit('close')
}
const commit = () => {
    emit('close')
}
</script>