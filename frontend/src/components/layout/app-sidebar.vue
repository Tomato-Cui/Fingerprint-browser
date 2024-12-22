<script setup>
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarHeader,
} from "../ui/sidebar/index";
import { IconLogo, IconEnvironmentAppend, IconEnvironmentAppend1, } from "@/assets/icons/index";
import { menuTop, menuBottom } from './menu'
import MenuItem from './menu-item.vue'
import { useRouter } from "vue-router";
const router = useRouter();
</script>

<template>
  <Sidebar class="h-screen bg-sidebar" collapsible="none">
    <SidebarHeader class="">
      <div class="flex gap-x-5 justify-center items-center flex-wrap h-topbar">
        <IconLogo class="w-[calc(34/270*100%)] min-w-10" />
        <p class="text-[24px]">指纹浏览器</p>
      </div>
      <div class="flex flex-row items-center p-0 h-12 text-white rounded-xl min-h-14 bg-primary mx-2"
        @click="() => router.push('/environment-action/create')">
        <SidebarContent class="text-[16px] flex flex-row flex-auto justify-center items-center cursor-pointer">
          <IconEnvironmentAppend1 class="w-6" />
          新建环境
        </SidebarContent>
        <SidebarContent class="border-l-2 text-[16px] flex justify-center items-center h-full cursor-pointer">
          <IconEnvironmentAppend class="w-1/2" />
        </SidebarContent>
      </div>
    </SidebarHeader>
    <SidebarContent class="relative px-2 hidden-scrollbar mx-2">
      <SidebarGroup class="p-0 pb-8 mt-4">
        <SidebarGroupContent>
          <SidebarMenu class="flex flex-col gap-y-2">
            <MenuItem :menus="menuTop" />
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
      <SidebarGroup class="p-0 pt-8 border-t border-white">
        <SidebarGroupContent>
          <SidebarMenu>
            <MenuItem :menus="menuBottom" />
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
</template>

<style scoped></style>
