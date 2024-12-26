<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineEmits, defineProps, reactive, watch, onMounted, onUnmounted } from 'vue'
import { UserPlus2Icon, UsersIcon } from 'lucide-vue-next'
import { IconCreateTeam, IconJoinTeam } from "@/assets/icons";
import { toast } from "vue-sonner";
import Input from "@/components/ui/input/Input.vue";

const props = defineProps({
    editAccountDialog: Boolean,
    groupUuid: Number
})
const emit = defineEmits(['update:editAccountDialog'])

const editAccount = reactive({
    environmentNumber: 7,
    selectedPlatform: "火狐"
})
//清空表单
const clearForm = () => {
    // editAccount.environmentNumber = 7
    editAccount.selectedPlatform = "火狐"
}
//确认
const subMit = () => {
    emit('update:editAccountDialog', false)
    console.log("submit:", editAccount);
}
watch(() => props.editAccountDialog, (val) => {
    clearForm()
})

// const environmentNumber = ref(7)  // 环境序号
// const selectedPlatform = ref('火狐')  // 选中的平台
const isOpen = ref(false)
const platforms = ref(['火狐', '谷歌', '微软边缘'])

const toggleDropdown = () => {
    isOpen.value = !isOpen.value
}

const selectPlatform = (platform) => {
    editAccount.selectedPlatform = platform
    isOpen.value = false
}

// Close dropdown when clicking outside
const handleClickOutside = (event) => {
    if (!event.target.closest('.relative')) {
        isOpen.value = false
    }
}

// Add and remove event listener
onMounted(() => {
    document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
})
</script>

<template>
    <!-- 创建团队 -->
    <Model class="min-w-[600px]" :title="'修改账号'" :open="props.editAccountDialog"
        @close="() => emit('update:editAccountDialog', false)">
        <div class="space-x-4 p-4 flex flex-col">
            <!-- 在这里书写弹出框主题内容代码 -->

            <!-- 主体 -->
            <div class="px-6 pb-8">
                <div class="space-y-4">
                    <!-- Environment Number Field -->
                    <div class="flex items-center">
                        <label class="text-gray-600 w-[100px]">环境序号</label>
                        <div class="bg-blue-50 px-4 py-2 rounded-md">
                            <span class="text-blue-600">{{ props.groupUuid }}</span>
                        </div>
                    </div>

                    <!-- Account Platform Dropdown -->
                    <div class="flex items-center">
                        <label class="text-gray-600 w-[100px]">账号平台</label>
                        <div class="relative flex-1">
                            <button @click="toggleDropdown"
                                class="w-full px-4 py-2 text-left border rounded-md flex items-center justify-between hover:border-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                                <div class="flex items-center">
                                    <div class="flex space-x-1 mr-2">
                                        <div v-for="i in 3" :key="i" class="w-1.5 h-1.5 bg-blue-500 rounded-full"></div>
                                    </div>
                                    <span>{{ editAccount.selectedPlatform }}</span>
                                </div>
                                <svg class="w-5 h-5 text-gray-400" :class="{ 'transform rotate-180': isOpen }"
                                    fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M19 9l-7 7-7-7" />
                                </svg>
                            </button>

                            <!-- Dropdown Menu -->
                            <div v-if="isOpen" class="absolute w-full mt-1 bg-white border rounded-md shadow-lg z-10">
                                <div v-for="platform in platforms" :key="platform" @click="selectPlatform(platform)"
                                    class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                    {{ platform }}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="flex items-center">
                        <label class="text-gray-600 w-[100px]">用户名</label>
                        <div class="px-4 py-2 rounded-md flex-1 border focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                            <input class="w-full outline-none" placeholder="请输入用户名"/>
                        </div>
                    </div>

                    <div class="flex items-center">
                        <label class="text-gray-600 w-[100px]">密码</label>
                        <div class="px-4 py-2 rounded-md flex-1 border focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                            <input class="w-full outline-none" placeholder="请输入密码" type="password"/>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex justify-center border-t">
                <div class="flex justify-start py-8 gap-x-4">
                    <PrimaryButton class="px-8" @click="subMit">确定
                    </PrimaryButton>
                    <CancelButton class="px-8" @click="() => emit('update:editAccountDialog', false)">取消
                    </CancelButton>
                </div>
            </div>
        </div>
    </Model>
</template>