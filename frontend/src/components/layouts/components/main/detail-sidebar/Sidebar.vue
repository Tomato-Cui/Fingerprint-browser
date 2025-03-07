<script setup lang="ts">
import type { SidebarProps } from "@/components/ui/sidebar";
import { Sheet, SheetContent } from "@/components/ui/sheet";
import { cn } from "@/util/lib";
import {
  useSidebar,
  SIDEBAR_WIDTH_MOBILE,
} from "@/components/ui/sidebar/utils";

defineOptions({
  inheritAttrs: false,
});

const props = withDefaults(defineProps<SidebarProps>(), {
  side: "left",
  variant: "sidebar",
  collapsible: "offcanvas",
});

const { isMobile, state, openMobile, setOpenMobile } = useSidebar();
</script>

<template>
  <div
    v-if="collapsible === 'none'"
    :class="
      cn(
        'flex h-full w-[--sidebar-width] flex-col text-sidebar-foreground',
        props.class
      )
    "
    v-bind="$attrs"
  >
    <slot />
  </div>

  <Sheet
    v-else-if="isMobile"
    :open="openMobile"
    v-bind="$attrs"
    @update:open="setOpenMobile"
  >
    <SheetContent
      data-sidebar="sidebar"
      data-mobile="true"
      :side="side"
      class="w-[--sidebar-width] bg-sidebar p-0 text-sidebar-foreground [&>button]:hidden"
      :style="{
        '--sidebar-width': SIDEBAR_WIDTH_MOBILE,
      }"
    >
      <div class="flex h-full w-full flex-col">
        <slot />
      </div>
    </SheetContent>
  </Sheet>

  <div
    v-else
    class="group peer hidden md:block"
    :data-state="state"
    :data-collapsible="state === 'collapsed' ? collapsible : ''"
    :data-variant="variant"
    :data-side="side"
  >
    <!-- This is what handles the sidebar gap on desktop  -->
    <div
      :class="
        cn(
          'duration-200 absolute inset-y-0 z-10 hidden w-[--sidebar-width] transition-[left,right,width] ease-linear md:flex',
          'group-data-[state=collapsed]:h-14 group-data-[state=collapsed]:border-none',
          side === 'left'
            ? 'left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]'
            : 'right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]',
          // Adjust the padding for floating and inset variants.
          variant === 'floating' || variant === 'inset'
            ? 'p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)_+_theme(spacing.4)_+2px)]'
            : 'group-data-[collapsible=icon]:w-[--sidebar-width-icon] group-data-[side=right]:border-l',
          props.class
        )
      "
      v-bind="$attrs"
    >
      <div
        data-sidebar="sidebar"
        class="flex h-full w-full flex-col bg-white group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:border-sidebar-border group-data-[variant=floating]:shadow rounded-br-xl"
      >
        <slot />
      </div>
    </div>
  </div>
</template>
