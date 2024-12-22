<script setup>
import RightMenu from './right-menu.vue'
</script>

<template>
    <div class="w-full bg-sidebar h-topbar">
        <div class="flex items-center justify-between px-6 h-full space-x-14">
            <div class="hidden space-x-6 rounded-full h-11 px-1 bg-[#FFFFFF66] md:flex">
                <button
                    class="px-3 my-1 rounded-full whitespace-nowrap bg-[#FFFFFFFF] text-primary cursor-not-allowed">环境管理</button>
                <button class="px-3 my-1 rounded-full whitespace-nowrap cursor-not-allowed ">费用中心</button>
                <button class="px-3 my-1 rounded-full whitespace-nowrap cursor-not-allowed ">推广奖励</button>
                <button class="px-3 my-1 rounded-full whitespace-nowrap cursor-not-allowed ">资源广场</button>
            </div>
            <RightMenu />
        </div>
    </div>
</template>

<style scoped></style>