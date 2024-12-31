<script setup>
import { IconChevronDown, IconTransform, IconMessage, IconUpdate } from '@/assets/icons/index'
import { Eraser as EraserIcon, Download as DownloadIcon, User2 as User2Icon, UserPlus as UserPlusIcon, Users as UsersIcon, Tag as TagIcon, LogOut as LogOutIcon, LanguagesIcon, BellIcon, HistoryIcon, ChevronUpIcon, ChevronDownIcon } from 'lucide-vue-next'
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from '@/components/ui/popover'
import { PrimaryButton, CancelButton } from '@/components/button'
import InvMember from '@/views/Team/member/InvMember.vue'
import changeTeam from './com/change-team.vue'
import { ref } from "vue"
import { AlertModel } from "@/components/alert-model";
import CreateAndJoin from './com/create-and-join.vue'
import { useRouter } from 'vue-router'
import { logout } from '@/commands/user'
import { useUserStore } from '@/stores/user'
import NotificationList from './com/notification-list.vue'

const router = useRouter()
const invMember = ref(false)
const switchTeam = ref(false)
const quitTeamDialog = ref(false)
const createAndJoin = ref(false)
const profileDown = ref(false)

const userSotre = useUserStore();
const userInfo = userSotre.getUserInfo();

const subQuit = () => {
    quitTeamDialog.value = false
    logout().then(res => {
        if (res.data) {
            router.push('/');
            userSotre.logout();
        }
    })
}
</script>

<template>
    <div class="flex items-center space-x-6">
        <Popover>
            <PopoverTrigger as-child>
                <button
                    class="flex justify-center items-center h-11 w-11 rounded-md hover:bg-sidebar border-[#FFFFFFFF] border-[1px] transform ease-in-out duration-500">
                    <LanguagesIcon class="w-ful h-full text-slate-500" />
                </button>
            </PopoverTrigger>
            <PopoverContent class="min-w-48 w-48 p-2 text-sm" align="end">
                <div class="flex flex-col">
                    <div class="hover:bg-sidebar hover:text-primary px-2 py-1 rounded-md cursor-pointer">简体中文</div>
                    <div class="hover:bg-sidebar hover:text-primary px-2 py-1 rounded-md cursor-pointer">English</div>
                </div>
            </PopoverContent>
        </Popover>

        <Popover>
            <PopoverTrigger as-child>
                <button
                    class="flex justify-center items-center h-11 w-11 rounded-md hover:bg-sidebar border-[#FFFFFFFF] border-[1px] transform ease-in-out duration-500">
                    <BellIcon class="w-ful h-full text-slate-500" />
                </button>
            </PopoverTrigger>
            <PopoverContent class="w-80 h-[calc(100vh_/_2)] p-0 rounded-lg overflow-hidden" align="end">
                <div class="py-4 flex flex-col h-full">
                    <!-- Tab Navigation -->
                    <div class="flex items-center justify-between px-4 border-b text-sm">
                        <div class="flex gap-6 divide-x">
                            <button class="relative text-blue-600 font-medium border-blue-600 border-b-[2px] py-2 w-12">
                                全部
                                <span class="absolute -top-1 -right-2 w-2 h-2 bg-red-500 rounded-full"></span>
                            </button>
                            <button class="text-gray-500 pl-6">
                                未读
                            </button>
                        </div>
                        <button class="flex items-center gap-1 text-gray-500 ml-4 border px-2 py-2 rounded-md text-xs">
                            <EraserIcon class="w-4 h-4" />
                            全部已读
                        </button>
                    </div>

                    <!-- Notification List -->
                    <div class="divide-y flex-auto h-full">
                        <div class="flex gap-3 p-4 h-full">
                            <NotificationList />
                        </div>
                    </div>

                    <!-- No More Messages -->
                    <div class="py-4 text-center text-gray-400 text-sm">
                        没有更多了
                    </div>
                </div>
            </PopoverContent>
        </Popover>
        <div class="bg-gray-200 w-[1px]">&nbsp;</div>
        <Popover>
            <PopoverTrigger as-child>
                <button
                    class="flex justify-center items-center h-11 w-11 rounded-md hover:bg-sidebar border-[#FFFFFFFF] border-[1px] transform ease-in-out duration-500">
                    <HistoryIcon class="w-ful h-full text-slate-500" />
                </button>
            </PopoverTrigger>
            <PopoverContent class="w-96 p-3" align="end">
                <div class="">
                    <!-- Header -->
                    <div class="flex justify-between items-center mb-4">
                        <h1 class="text-lg font-medium">更新中心</h1>
                        <button class="flex items-center gap-2 text-blue-600 hover:text-blue-700 text-sm">
                            <DownloadIcon class="w-4 h-4" />
                            全部下载
                        </button>
                    </div>

                    <!-- Update Card -->
                    <div class="bg-gray-50 rounded-lg p-2">
                        <div class="flex items-center gap-2 mb-4">
                            <h2 class="text-sm font-medium">更新中心</h2>
                            <span class="px-1 text-xs font-medium text-white bg-red-500">NEW</span>
                        </div>

                        <div class="space-y-4 text-xs">
                            <!-- Version Info -->
                            <div class="text-gray-700">
                                <p class="mb-2">v2.7.4.9 更新说明:</p>
                                <ul class="list-decimal list-inside space-y-1 pl-1">
                                    <li>新增 Proxy302auto 代理类型</li>
                                    <li>[窗口同步]支持"文本换行"</li>
                                </ul>
                            </div>

                            <!-- Warning Message -->
                            <div class="text-red-500 text-sm">
                                注意：更新会关闭当前设备已打开的浏览器，请注意保存！！！
                            </div>

                            <!-- Action Buttons -->
                            <div class="flex justify-end gap-3 mt-6">
                                <CancelButton class="text-gray-500 hover:text-gray-600">
                                    跳过
                                </CancelButton>
                                <PrimaryButton>
                                    立即下载
                                </PrimaryButton>
                            </div>
                        </div>
                    </div>
                </div>
            </PopoverContent>
        </Popover>

        <Popover>
            <PopoverTrigger as-child>
                <div class="flex items-center space-x-4 h-11 w-36 px-2 bg-sidebar rounded-md cursor-pointer"
                    @click="profileDown = !profileDown">
                    <img class="w-10 h-10 rounded-full" :src="userInfo.avatar" alt="User Avatar">
                    <div class="flex flex-auto items-center gap-x-1 overflow-hidden">
                        <p class="flex-auto text-gray-500 text-sm text-ellipsis overflow-hidden whitespace-nowrap">{{
                            userInfo.account }}</p>
                        <p class="w-4 h-4">
                            <ChevronUpIcon v-if="profileDown" class="w-4 h-4 text-gray-500 " />
                            <ChevronDownIcon v-else class="w-4 h-4 text-gray-500 " />
                        </p>
                    </div>
                </div>
            </PopoverTrigger>
            <PopoverContent class=" w-48 rounded-xl p-2" align="end">
                <div class="flex flex-col gap-y-2">
                    <div @click="router.push('/personal/account')"
                        class="hover:bg-sidebar hover:text-primary px-2 py-1 rounded-md cursor-pointer flex items-center gap-x-2 text-sm hidden">
                        <User2Icon class="w-4 h-4" />
                        用户中心
                    </div>
                    <!-- <div @click="invMember = true"
                        class="hover:bg-sidebar hover:text-primary px-2 py-1 rounded-md cursor-pointer flex items-center gap-x-2 text-sm">
                        <UserPlusIcon class="w-4 h-4" />
                        邀请成员
                    </div> -->
                    <div @click="switchTeam = true"
                        class="hover:bg-sidebar hover:text-primary px-2 py-1 rounded-md cursor-pointer flex items-center gap-x-2 text-sm">
                        <UsersIcon class="w-4 h-4" />
                        切换团队
                    </div>
                    <div @click="createAndJoin = true"
                        class="hover:bg-sidebar hover:text-primary px-2 py-1 rounded-md cursor-pointer flex items-center gap-x-2 text-sm">
                        <TagIcon class="w-4 h-4" />
                        加入团队/邀请成员
                    </div>
                    <div @click="quitTeamDialog = true"
                        class="hover:bg-sidebar hover:text-primary px-2 py-1 rounded-md cursor-pointer flex items-center gap-x-2 text-sm">
                        <LogOutIcon class="w-4 h-4" />
                        退出登录
                    </div>
                </div>
            </PopoverContent>
        </Popover>
    </div>

    <!-- 邀请成员 -->
    <InvMember v-model:invMember="invMember" />
    <!-- 切换团队 -->
    <changeTeam v-model:switchTeam="switchTeam" />
    <!-- 删除消息弹出框 -->
    <AlertModel :title="'退出登录'" :open="quitTeamDialog" @close="() => (quitTeamDialog = false)"
        @cancel="() => (quitTeamDialog = false)" @submit="subQuit">
        确定退出登录吗？
    </AlertModel>
    <!-- 加入创建团队 -->
    <CreateAndJoin v-model:createAndJoin="createAndJoin" />
</template>
