<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineEmits, defineProps, reactive } from 'vue'
import { UserPlus2Icon, UsersIcon } from 'lucide-vue-next'
import { IconCreateTeam, IconJoinTeam } from "@/assets/icons";

const props = defineProps({
    createAndJoin: Boolean
})
const emit = defineEmits(['update:createAndJoin'])

const joinTeamDialog = ref(false) //加入团队
const createTeamDialog = ref(false) //创建团队

const joinForm = reactive({
    teamCode: "",
    teamLink: "",
    description: ""
})
const createForm = reactive({
    teamName: "",
    remark: ""
})
//确认加入团队
const subJoinTeam = () => {
    joinTeamDialog.value = false
}
</script>

<template>
    <!-- class: 这里class的宽度就是弹出框的宽度.请自行设置 -->
    <Model class="min-w-[700px]" :title="'加入创建团队'" :open="props.createAndJoin"
        @close="() => emit('update:createAndJoin', false)">
        <div class="space-x-4 pt-6 pb-6 flex flex-col">
            <!-- 在这里书写弹出框主题内容代码 -->

            <div class=" p-4 space-y-4">
                <!-- Join Team Card -->
                <div class="border rounded-lg p-6 flex items-center justify-between">
                    <div>
                        <h2 class="text-xl font-medium text-blue-600">加入新团队</h2>
                        <p class="text-gray-500 mt-1 text-sm">
                            如果你的团队已经存在，你可以通过团队码或者二维码加入
                        </p>
                    </div>
                    <button @click="joinTeamDialog = true"
                        class="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors flex items-center space-x-2">
                        <IconJoinTeam class="w-4 h-4" />
                        <span>前去加入</span>
                    </button>
                </div>

                <!-- Create Team Card -->
                <div class="border rounded-lg p-6 flex items-center justify-between">
                    <div>
                        <div class="flex items-center space-x-2">
                            <span class="text-xl font-medium text-emerald-500">创建</span>
                            <span class="text-xl font-medium text-gray-900">新团队</span>
                        </div>
                        <p class="text-gray-500 mt-1 text-sm">
                            创建团队后，你将成为团队BOSS,可添加环境、成员等
                        </p>
                    </div>
                    <button @click="createTeamDialog = true"
                        class="border border-gray-200 text-gray-700 px-6 py-2 rounded-lg hover:bg-gray-50 transition-colors flex items-center space-x-2">
                        <IconCreateTeam class="w-4 h-4" />
                        <span>立即创建</span>
                    </button>
                </div>
            </div>
        </div>
    </Model>

    <!-- 创建团队 -->
    <Model class="" :title="'创建团队'" :open="createTeamDialog" @close="() => createTeamDialog = false">
        <div class="space-x-4 p-4 flex flex-col">
            <!-- 在这里书写弹出框主题内容代码 -->

            <!-- 主体 -->
            <div class="h-[100px] flex flex-col justify-between mb-6">
                <div class="flex justify-center items-center gap-4">
                    <label class="w-[80px] flex justify-end">团队名称</label>
                    <input v-model="createForm.teamName" type="text" placeholder="请输入团队名称"
                        class="border border-gray-400 rounded-sm h-[40px] px-5" />
                </div>
                <div class="flex justify-center items-center gap-4">
                    <label class="w-[80px] flex justify-end">备注</label>
                    <input v-model="createForm.remark" type="text" placeholder="请输入备注"
                        class="border border-gray-400 rounded-sm h-[40px] px-5" />
                </div>
            </div>

            <div class="flex justify-center border-t">
                <div class="flex justify-start py-8 gap-x-4">
                    <PrimaryButton class="px-8" @click="() => createTeamDialog = false">确定
                    </PrimaryButton>
                    <CancelButton class="px-8" @click="() => createTeamDialog = false">取消
                    </CancelButton>
                </div>
            </div>
        </div>
    </Model>
    <!-- 加入团队 -->
    <Model class="" :title="'加入团队'" :open="joinTeamDialog" @close="() => joinTeamDialog = false">
        <div class="space-x-4 pt-6 flex flex-col">
            <!-- 在这里书写弹出框主题内容代码 -->
            <div class="h-[150px] flex flex-col justify-between mb-6">
                <div class="flex justify-center items-center gap-4">
                    <label class="w-[80px] flex justify-end">团队码</label>
                    <input v-model="joinForm.teamCode" type="text" placeholder="请输入团队码"
                        class="border border-gray-400 rounded-sm h-[40px] px-5" />
                </div>
                <div class="flex justify-center items-center gap-4">
                    <label class="w-[80px] flex justify-end">团队链接</label>
                    <input v-model="joinForm.teamLink" type="text" placeholder="请输入团队链接"
                        class="border border-gray-400 rounded-sm h-[40px] px-5" />
                </div>
                <div class="flex justify-center items-center gap-4">
                    <label class="w-[80px] flex justify-end">备注</label>
                    <input v-model="joinForm.description" type="text" placeholder="请输入团队链接"
                        class="border border-gray-400 rounded-sm h-[40px] px-5" />
                </div>
            </div>

            <div class="flex justify-center border-t">
                <div class="flex justify-start py-8 gap-x-4">
                    <PrimaryButton class="px-8" @click="subJoinTeam">确定
                    </PrimaryButton>
                    <CancelButton class="px-8" @click="() => joinTeamDialog = false">取消
                    </CancelButton>
                </div>
            </div>
        </div>
    </Model>
</template>