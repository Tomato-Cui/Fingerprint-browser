<template>
    <Model class="min-w-[600px]" title="同步器" :open="props.open">
        <div class="fixed inset-0 bg-black bg-opacity-25 flex items-center justify-center p-4">
            <div class="bg-white w-full max-w-2xl rounded-2xl overflow-hidden">
                <!-- Header -->
                <div class="flex items-center justify-between px-6 py-4">
                    <h2 class="text-lg font-medium">同步器</h2>
                    <button class="text-gray-400 hover:text-gray-600">
                        <XIcon class="w-5 h-5" />
                    </button>
                </div>

                <!-- Content -->
                <div class="px-6 py-8">
                    <h1 class="text-2xl font-medium text-center mb-4">此功能需要授权</h1>
                    <p class="text-gray-500 text-center mb-8">
                        要激活浏览器窗口并让操作同步，需要给以下系统权限~
                    </p>

                    <!-- Permission Cards -->
                    <div class="grid md:grid-cols-2 gap-6">
                        <!-- Automation Card -->
                        <div class="bg-gray-50 rounded-xl p-6 flex flex-col items-center">
                            <div class="w-16 h-16 mb-4 text-gray-700">
                                <Automation class="w-full h-full" />
                            </div>
                            <h3 class="text-lg font-medium mb-6">自动化</h3>
                            <button
                                class="w-full bg-blue-600 hover:bg-blue-700 text-white rounded-lg py-3 px-4 flex items-center justify-center gap-2">
                                <VerifiedCheckIcon class="w-5 h-5" />
                                去授权
                            </button>
                        </div>

                        <!-- Auxiliary Functions Card -->
                        <div class="bg-gray-50 rounded-xl p-6 flex flex-col items-center">
                            <div class="w-16 h-16 mb-4 text-gray-700">
                                <VectorIcon class="w-full h-full" />
                            </div>
                            <h3 class="text-lg font-medium mb-6">辅助功能</h3>
                            <button
                                class="w-full bg-blue-600 hover:bg-blue-700 text-white rounded-lg py-3 px-4 flex items-center justify-center gap-2">
                                <VerifiedCheckIcon class="w-5 h-5" />
                                去授权
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </Model>

</template>

<script setup>
import { Model } from "@/components/model/index";
import { ref, defineProps, defineEmits } from 'vue'
import { VerifiedCheckIcon, Automation, VectorIcon } from "@/assets/icons/synchronizer";
import {
    XIcon,
    RefreshCwIcon,
    WrenchIcon,
    CheckCircleIcon
} from 'lucide-vue-next'

const props = defineProps({
    open: false,
})
const emit = defineEmits(['close'])
</script>