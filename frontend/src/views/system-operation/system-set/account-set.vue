<template>
    <layout>
        <div class="max-w-2xl p-6">
            <form @submit.prevent="handleSubmit" class="space-y-6">
                <!-- User Nickname -->
                <div class="flex items-center gap-[100px]">
                    <label class="w-24 text-gray-600">用户昵称</label>
                    <div class="flex-1 flex items-center gap-2">
                        <input type="text" v-model="form.nickname" placeholder="默认昵称"
                            class="w-full px-4 py-2 rounded-lg border border-gray-200 focus:outline-none focus:ring-2 focus:ring-[#5B5BFA] focus:border-transparent" />
                        <button type="button" class="text-[#5B5BFA]">
                            <EditIcon class="w-5 h-5" />
                        </button>
                    </div>
                </div>

                <!-- Team Name -->
                <div class="flex items-center gap-[100px]">
                    <label class="w-24 text-gray-600">团队名称</label>
                    <div class="flex-1 flex items-center gap-2">
                        <input type="text" v-model="form.teamName" placeholder="默认团队名称"
                            class="w-full px-4 py-2 rounded-lg border border-gray-200 focus:outline-none focus:ring-2 focus:ring-[#5B5BFA] focus:border-transparent" />
                        <button type="button" class="text-[#5B5BFA]">
                            <EditIcon class="w-5 h-5" />
                        </button>
                    </div>
                </div>

                <!-- Role -->
                <div class="flex items-center gap-[100px]">
                    <label class="w-24 text-gray-600">角色</label>
                    <input type="text" value="超级管理员" disabled
                        class="flex-1 px-4 py-2 rounded-lg bg-gray-50 border border-gray-200 text-gray-500" />
                </div>

                <!-- Email -->
                <div class="flex items-center gap-[100px]">
                    <label class="w-24 text-gray-600">邮箱</label>
                    <div class="flex-1 flex items-center gap-2">
                        <input type="email" v-model="form.email" disabled
                            class="flex-1 px-4 py-2 rounded-lg bg-gray-50 border border-gray-200" />
                        <button type="button"
                            class="flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                            <RefreshIcon class="w-4 h-4" />
                            变更登录邮箱
                        </button>
                    </div>
                </div>

                <!-- Password -->
                <div class="flex items-center gap-[100px]">
                    <label class="w-24 text-gray-600">密码</label>
                    <div class="flex-1 flex items-center gap-2">
                        <input type="password" value="********" disabled
                            class="flex-1 px-4 py-2 rounded-lg bg-gray-50 border border-gray-200" />
                        <button type="button"
                            class="flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                            <RefreshIcon class="w-4 h-4" />
                            变更登录密码
                        </button>
                    </div>
                </div>

                <!-- Authenticator -->
                <div class="flex items-center gap-[100px]">
                    <label class="w-24 text-gray-600">身份验证器</label>
                    <div class="flex-1 flex items-center gap-2">
                        <input type="text" v-model="form.authenticator" disabled
                            class="flex-1 px-4 py-2 rounded-lg bg-gray-50 border border-gray-200" />
                        <button type="button"
                            class="flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                            <QRCodeIcon class="w-4 h-4" />
                            绑定
                        </button>
                    </div>
                </div>

                <!-- Third Party Login -->
                <div class="flex items-start gap-[100px]">
                    <label class="w-24 text-gray-600 pt-2">第三方登录</label>
                    <div class="flex-1 space-y-4">
                        <!-- Chrome -->
                        <div class="flex items-center gap-4">
                            <!-- <div class="flex items-center gap-2 flex-1">
                                <img src="https://hebbkx1anhila5yf.public.blob.vercel-storage.com/image-3qWRbMq3QxZBe5UA0TxyEa1cX1RQR6.png"
                                    alt="Chrome" class="w-6 h-6" />
                                <span>chrome</span>
                            </div> -->
                            <!-- <BrowsersChromeIcon class="size-[50px]"/> -->
                             <img :src="ChromeP" alt="" srcset="">
                            <button type="button"
                                class="flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <QRCodeIcon class="w-4 h-4" />
                                绑定
                            </button>
                        </div>

                        <!-- Firefox -->
                        <div class="flex items-center gap-4">
                            <div class="flex items-center gap-2">
                                <!-- <img src="https://hebbkx1anhila5yf.public.blob.vercel-storage.com/image-3qWRbMq3QxZBe5UA0TxyEa1cX1RQR6.png"
                                    alt="Firefox" class="w-6 h-6" />
                                <span>Firefox Browser</span> -->
                                <!-- <BrowsersFoxIcon class="size-[150px]"/> -->
                                 <img :src="FoxP" alt="" srcset="">
                            </div>
                            <button type="button"
                                class="flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
                                <QRCodeIcon class="w-4 h-4" />
                                绑定
                            </button>
                        </div>
                    </div>
                </div>
            </form>
        </div>
    </layout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import layout from './layout.vue';
import { EditIcon, RefreshIcon, QRCodeIcon } from '@/assets/icons/system-operation/index';
import ChromeP from '@/assets/icons/system-operation/Chome.png'
import FoxP from '@/assets/icons/system-operation/Fox.png'

const form = ref({
    nickname: '默认昵称',
    teamName: '默认团队名称',
    email: 'jojolin@gmail.com',
    authenticator: 'jojolin@gmail.com'
})

const handleSubmit = () => {
    // Handle form submission
    console.log('Form submitted:', form.value)
}
</script>