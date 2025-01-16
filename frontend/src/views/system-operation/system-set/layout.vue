<script setup lang="ts">
import { ref } from "vue";
import { cn } from "@/util/lib";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { useRouter, useRoute } from "vue-router";
import { SearchIcon } from "@/assets/icons/environment";
import { WrapperIcon, LogoutBlackIcon } from "@/assets/icons/system-operation/index"

const router = useRouter();
const route = useRoute();
const searchQuery = ref('')
const tabs = [
    {
        id: 1,
        title: "账号设置",
        value: "account",
        href: "/accountSet",
    },
    {
        id: 2,
        title: "通知设置",
        value: "notification",
        href: "/notifySet",
    },
    {
        id: 3,
        title: "全局设置",
        value: "globe",
        href: "",
    },
    {
        id: 4,
        title: "本地设置",
        value: "local",
        href: "",
    },
];
</script>
<template>
    <Card className="w-full h-full flex flex-col p-0">
        <CardHeader class="p-0 px-4">
            <CardTitle class="mt-1 mb-2">
                <div class="flex justify-between">
                    <span class="text-xl">系统设置</span>
                    <div class="space-x-3 flex items-center">
                        <div class="flex items-center border rounded-lg pl-2">
                            <SearchIcon class="size-5" />
                            <input type="search" placeholder="搜索" v-model="searchQuery"
                                class="rounded-md px-3 py-1.5 text-sm font-[500] outline-none w-[260px] h-[35px]" />
                        </div>
                        <button class="text-[16px] font-[500] border p-2 rounded-lg flex min-w-fit items-center gap-2 hover:bg-gray-100"><LogoutBlackIcon class="size-5"/> 退出登录</button>
                        <button class="text-[16px] font-[500] border p-2 rounded-lg flex min-w-fit items-center gap-2 border-red-400 bg-red-50 hover:bg-red-100"><WrapperIcon class="size-5"/> 注销</button>
                    </div>
                </div>
            </CardTitle>
            <ul class="bg-white border-b rounded-none w-full flex justify-start pb-0 px-0 text-sm">
                <li :key="item.id" v-for="item in tabs" @click="router.push(item!.href)" :class="cn(
                    'border-transparent border-b-[3px] rounded-none flex items-center pb-1 px-6 cursor-pointer',
                    route.path == item.href ? 'text-[#5050FA] border-[#5050FA]' : ''
                )
                    ">
                    <div class="flex items-center justify-center gap-x-3">
                        {{ item.title }}
                    </div>
                </li>
            </ul>
        </CardHeader>
        <div class="grow flex flex-col overflow-hidden">
            <slot />
        </div>
    </Card>
</template>
