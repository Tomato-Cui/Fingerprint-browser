<script setup lang="ts">
import {
  SidebarGroup,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import {
  IconAddCircle,
  IconGlobal,
  IconImport,
} from "@/assets/icons/sidebar/index";
import { useRouter } from "vue-router";
const router = useRouter();
const toEnvrionmentCreatePage = () =>
  router.push("/environment/create/environment-simple-create");

const toEnvrionmentImportPage = () =>
  router.push("/environment/create/environment-batch-import");
</script>

<template>
  <SidebarGroup>
    <SidebarMenu class="space-y-2 border-b pb-4">
      <SidebarMenuItem>
        <SidebarMenuButton
          tooltip="新建环境"
          class="p-0 pl-4 h-12 flex bg-gradient-to-l from-[#7D41FF] to-[#5E51FF] rounded-sm group-data-[state=collapsed]:justify-center overflow-hidden relative"
          @click="toEnvrionmentCreatePage"
        >
          <component :is="IconAddCircle" class="size-6" />
          <p
            class="flex-1 pl-1 flex items-center text-white font-bold text-[16px] whitespace-nowrap group-data-[state=collapsed]:hidden"
          >
            新建环境
          </p>
          <component
            :is="IconGlobal"
            class="size-14 absolute -top-4 -right-4"
          />
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton
          tooltip="批量导入/创建"
          @click="toEnvrionmentImportPage"
          class="p-0 pl-4 h-12 flex rounded-sm relative group-data-[state=collapsed]:justify-center gap-x-4 bg-white dark:text-black dark:hover:bg-white"
        >
          <component :is="IconImport" class="size-5" />
          <p
            class="flex-1 flex items-center text-[16px] whitespace-nowrap group-data-[state=collapsed]:hidden"
          >
            批量导入/创建
          </p>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarGroup>
</template>
