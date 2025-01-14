<script setup lang="ts">
import { cn } from "@/util/lib";
import {
  SidebarGroup,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useRouter } from "vue-router";

import { menus } from "../../menu";
const router = useRouter();
</script>

<template>
  <SidebarGroup>
    <SidebarMenu>
      <SidebarMenuItem v-for="item in menus" :key="item.title" as-child>
        <SidebarMenuButton
          :tooltip="item.title"
          @click="router.push(item.url)"
          :class="
            cn(
              'p-0 h-12 rounded-sm group-data-[state=collapsed]:!p-0',
              item.isActive ? 'bg-[#D8DCF3] text-[#5050FA]' : ''
            )
          "
        >
          <div
            :class="
              cn(
                'flex gap-x-4 h-2/3 items-center justify-start',
                item.isActive
                  ? 'border-l-[3px] border-[#6A6AFF]'
                  : 'border-l-[3px] border-transparent'
              )
            "
          >
            <div
              :class="
                cn(
                  'flex group-data-[state=collapsed]:justify-center pl-4 gap-x-4 items-center'
                )
              "
            >
              <component :is="item.icon" class="size-5" />
              <p
                class="'flex-1 flex items-center text-[15px] whitespace-nowrap group-data-[state=collapsed]:hidden'"
              >
                {{ item.title }}
              </p>
            </div>
          </div>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarGroup>
</template>
