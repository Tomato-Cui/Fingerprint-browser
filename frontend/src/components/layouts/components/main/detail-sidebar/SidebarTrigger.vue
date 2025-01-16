<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { Button } from "@/components/ui/button";
import { cn } from "@/util/lib";
import { useSidebar } from "@/components/ui/sidebar/utils";
import { IconGroup1 } from "@/assets/icons/sidebar/index";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();

const { toggleSidebar } = useSidebar();
</script>

<template>
  <Button
    data-sidebar="trigger"
    variant="ghost"
    size="icon"
    :class="cn('h-7 w-7', props.class)"
    @click="toggleSidebar"
  >
    <IconGroup1 class="size-4" />
    <span class="sr-only">Toggle Sidebar</span>
  </Button>
</template>
