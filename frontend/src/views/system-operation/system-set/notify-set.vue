<template>
    <layout>
        <div class="p-6 space-y-6">
            <!-- Agent Expiration Reminder -->
            <section class="space-y-4">
                <h2 class="text-lg text-gray-400 font-semibold pl-3 border-l-2 border-blue-400">代理到期提醒</h2>
                <div class="bg-gray-50 rounded-xl p-6 gap-6 flex justify-between w-full">
                    <!-- Message Center Notification -->
                    <div class="flex items-center justify-between bg-white p-4 w-1/2">
                        <div class="space-y-1">
                            <div class="font-[600]">消息中心推送</div>
                            <div class="text-sm text-gray-500">当代理可用天数不足时，会发提醒到消息中心</div>
                        </div>
                        <label class="relative inline-flex cursor-pointer items-center">
                            <input type="checkbox" v-model="noteSwitch.agentExpiryReminder" class="peer sr-only">
                            <div
                                class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                            </div>
                        </label>
                    </div>

                    <!-- Email Notification -->
                    <div class="flex items-center justify-between bg-white p-4 w-1/2">
                        <div class="space-y-1">
                            <div class="flex gap-2 flex-col">
                                <span class="font-[600]">消息中心推送</span>
                                <span class="text-gray-500">邮箱：jojolin@gmail.com</span>
                            </div>
                            <div class="text-sm text-gray-500">当代理可用天数不足时，会发提醒到指定邮箱</div>
                        </div>
                        <label class="relative inline-flex cursor-pointer items-center">
                            <input type="checkbox" v-model="noteSwitch.agentExpiryReminder" class="peer sr-only">
                            <div
                                class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                            </div>
                        </label>
                    </div>
                </div>
            </section>

            <!-- Auto Renewal Reminder -->
            <section class="space-y-4">
                <h2 class="text-lg text-gray-400 font-semibold pl-3 border-l-2 border-blue-400">代理自动续费提醒</h2>
                <div class="bg-gray-50 rounded-xl p-6 gap-6 flex justify-between w-full">
                    <!-- Message Center Notification -->
                    <div class="flex items-center justify-between bg-white p-4 w-1/2">
                        <div class="space-y-1">
                            <div class="font-[600]">消息中心推送</div>
                            <div class="text-sm text-gray-500">当代理可用天数不足时，会发提醒到消息中心</div>
                        </div>
                        <label class="relative inline-flex cursor-pointer items-center">
                            <input type="checkbox" v-model="noteSwitch.agentExpiryReminder" class="peer sr-only">
                            <div
                                class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                            </div>
                        </label>
                    </div>

                    <!-- Email Notification -->
                    <div class="flex items-center justify-between bg-white p-4 w-1/2">
                        <div class="space-y-1">
                            <div class="flex items-center gap-2">
                                <span class="font-[600]">消息中心推送</span>
                                <span class="text-gray-500">邮箱：jojolin@gmail.com</span>
                            </div>
                            <div class="text-sm text-gray-500">当代理可用天数不足时，会发提醒到指定邮箱</div>
                        </div>
                        <label class="relative inline-flex cursor-pointer items-center">
                            <input type="checkbox" v-model="noteSwitch.agentExpiryReminder" class="peer sr-only">
                            <div
                                class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                            </div>
                        </label>
                    </div>
                </div>
            </section>

            <!-- Insufficient Balance Reminder -->
            <section class="space-y-4">
                <h2 class="text-lg text-gray-400 font-semibold pl-3 border-l-2 border-blue-400">余额不足提醒</h2>
                <div class="bg-gray-50 rounded-xl p-6 gap-6 flex justify-between w-full">
                    <!-- Message Center Notification -->
                    <div class="flex items-center justify-between bg-white p-4 w-1/2">
                        <div class="space-y-1">
                            <div class="font-[600]">消息中心推送</div>
                            <div class="text-sm text-gray-500">当余额不足以续费时，会发提醒到消息中心</div>
                        </div>
                        <label class="relative inline-flex cursor-pointer items-center">
                            <input type="checkbox" v-model="noteSwitch.agentExpiryReminder" class="peer sr-only">
                            <div
                                class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                            </div>
                        </label>
                    </div>

                    <!-- Email Notification -->
                    <div class="flex items-center justify-between bg-white p-4 w-1/2">
                        <div class="space-y-1">
                            <div class="flex items-center gap-2">
                                <span class="font-[600]">消息中心推送</span>
                                <span class="text-gray-500">邮箱：jojolin@gmail.com</span>
                            </div>
                            <div class="text-sm text-gray-500">当余额不足以续费时，会发提醒到指定邮箱</div>
                        </div>
                        <label class="relative inline-flex cursor-pointer items-center">
                            <input type="checkbox" v-model="noteSwitch.agentExpiryReminder" class="peer sr-only">
                            <div
                                class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                            </div>
                        </label>
                    </div>
                </div>
            </section>
        </div>
    </layout>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import layout from './layout.vue'

const noteSwitch = ref({
    agentExpiryReminder: true
})
const settings = ref({
    expiration: {
        messageCenter: true,
        email: true
    },
    autoRenewal: {
        messageCenter: true,
        email: true
    },
    insufficientBalance: {
        messageCenter: true,
        email: true
    }
})

// Watch for changes and save to backend
watch(settings, (newSettings) => {
    console.log('Settings updated:', newSettings)
    // Add API call here to save settings
}, { deep: true })
</script>