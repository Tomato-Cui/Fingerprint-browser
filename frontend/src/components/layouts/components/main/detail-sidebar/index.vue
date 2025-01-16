<script setup lang="ts">
import { cn } from "@/util/lib";
import Sidebar from "./Sidebar.vue";
import SidebarProvider from "./SidebarProvider.vue";
import SidebarTrigger from "./SidebarTrigger.vue";
import SidebarInset from "./SidebarInset.vue";
import Breadcrumb from "../breadcrumb.vue";
import { SidebarHeader, SidebarContent } from "@/components/ui/sidebar/index";
import { useEnvironmentAdvancedFormStore } from "@/stores/form/environment-advanced";
import { ref } from "vue";

const open = ref<boolean>(true);
const forms = useEnvironmentAdvancedFormStore().forms;
</script>

<template>
  <SidebarProvider class="h-full rounded-xl overflow-hidden">
    <Sidebar side="right" collapsible="icon" class="h-full rounded-br-xl">
      <SidebarHeader
        class="p-0 h-14 flex items-center justify-center border-none"
      >
        <div
          :class="
            cn(
              'flex items-center group-data-[state=collapsed]:justify-center justify-between w-full text-xs border-b pb-2',
              open
                ? ''
                : 'shadow rounded-md border p-0 w-10 h-10 rotate-180 -ml-4'
            )
          "
        >
          <div class="flex items-center justify-center">
            <SidebarTrigger @click="open = !open" class="hover:bg-white" />
            <p class="group-data-[state=collapsed]:hidden font-bold">
              环境描述
            </p>
          </div>
          <button
            class="group-data-[state=collapsed]:hidden mr-2 text-xs border rounded-md px-2 py-1 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
          >
            刷新指纹
          </button>
        </div>
      </SidebarHeader>
      <SidebarContent>
        {{ forms }}
      </SidebarContent>
    </Sidebar>
    <SidebarInset class="grow">
      <Breadcrumb />
      <div
        :class="
          cn(
            open ? 'w-[calc(100%_-_var(--sidebar-width))]' : 'w-full',
            'overflow-hidden rounded-lg grow transition-all ease-in-out delay-400'
          )
        "
      >
        <slot></slot>
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
