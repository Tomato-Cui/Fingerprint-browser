<script setup lang="ts">
import { IconLogo } from "@/assets/icons/sidebar/index";
import {
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
} from "@/components/ui/sidebar";
</script>

<template>
  <SidebarHeader>
    <SidebarMenu>
      <SidebarMenuButton
        size="lg"
        class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
      >
        <div
          class="flex aspect-square items-center justify-center rounded-lg text-sidebar-primary-foreground"
        >
          <component
            :is="IconLogo"
            class="size-9 transition-all ease-in-out delay-100"
          />
        </div>
        <div
          class="flex-1 text-left font-alimama-shuheiti font-bold text-[22px] whitespace-nowrap group-data-[state=collapsed]:hidden"
        >
          幻境浏览器
        </div>
      </SidebarMenuButton>
    </SidebarMenu>
  </SidebarHeader>
</template>
