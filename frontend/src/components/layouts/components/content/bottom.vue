<script setup lang="ts">
import { cn } from "@/util/lib";
import {
  SidebarGroup,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useRouter, useRoute } from "vue-router";

import { menus } from "../../menu";
const router = useRouter();
const route = useRoute();
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
              'p-0 h-12 rounded-sm group-data-[state=collapsed]:!p-0  dark:fill-white dark:text-[#C5CEE0] transition-all duration-200 ease-in-out delay-50',
              route.path === item.url
                ? 'bg-[#D8DCF3]  text-[#5050FA] dark:text-[#7171FF] dark:bg-[#3366FF33]'
                : ' '
            )
          "
        >
          <div
            :class="
              cn(
                'flex gap-x-4 h-2/3 items-center justify-start',
                route.path === item.url
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
              <component
                :is="item.icon"
                :stroke="route.path === item.url ? '#5050FA' : ''"
                :class="
                  cn(
                    'size-5  fill-black',
                    route.path === item.url
                      ? 'fill-[#5050FA]'
                      : 'dark:fill-white'
                  )
                "
              />
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
