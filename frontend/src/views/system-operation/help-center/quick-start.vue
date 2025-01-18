<script setup lang="ts">
import { ref, computed } from 'vue';
import layout from './layout.vue';
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";

const quickStartItems = [
    {
        title: '入门常见问题',
        content: 'Q:Morelogin是什么产品？A：Morelogin是一款专业且安全的多账号管理工具，包括指纹浏览器和云手机，可为每个账号提供独立安全的环境，并提供高效的功能为多账号运营提升效率。我们主要有以下几点功能：- 批量账号管理- 自动设置浏览器独立指纹- 打开多个浏览器环境彼此独立- 支持同步器和API自动化，提高效率- 团队协作功能，支持多成员协作使用- 支持主流代理网络Q：什么的电脑配置才能使用Morelogin？A：RAM： 建议最低4GB；2GB以上的可用磁盘空间；有效的GPU；支持Windows 10 64位、MacOS12.6.1及以上。Q:在正式购买前是否可以试用Morelogin？A：可以，您可以注册个人免费版使用，或者可以找客服领取套餐。Q：Morelogin只能在电脑端使用吗？A：是的，Morelogin是PC端软件，需要在PC端安装使用，不支持在移动端安装使用Morelogin。但是，您可以在Morelogin上创建移动端的浏览器环境或者云手机使用。Q:回收站可以清空吗？删除浏览器环境占用套餐环境数上限吗？A：【回收站】仅显示最近7天删除的浏览器环境，删除7天后的浏览器环境系统会自动清除，也可以自行选择并彻底删除。需要注意彻底删除的环境或删除超过7天的环境将无任何方法恢复。 回收站（删除）的浏览器环境不占用套餐浏览器环境数上限'
    },
    {
        title: '创建账号',
        content: '创建账号的步骤说明...'
    },
    {
        title: '开始使用幻境浏览器',
        content: '浏览器使用指南...'
    },
    {
        title: '设置中心',
        content: '设置中心的详细说明...'
    },
    {
        title: '快速参与推广返现',
        content: '推广返现活动说明...'
    }
]
const expandedItems = ref(Array(quickStartItems.length).fill(false))



const toggleItem = (index: number) => {
    expandedItems.value[index] = !expandedItems.value[index]
}
</script>

<template>
    <layout>
        <Accordion v-for="(item, index) in quickStartItems" type="single" class="w-full px-4" collapsible>
            <AccordionItem class="border-0" value="proxy-setting">
                <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">{{ item.title }}
                </AccordionTrigger>
                <AccordionContent class="px-10 space-y-4 py-1">
                    {{ item.content }}
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    </layout>
</template>