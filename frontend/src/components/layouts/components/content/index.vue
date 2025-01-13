<script setup lang="ts">
import Top from "./top.vue";
import Bottom from "./bottom.vue";
import { SidebarContent } from "@/components/ui/sidebar";
</script>

<template>
  <SidebarContent class="overflow-x-hidden">
    <Top />
    <Bottom />
  </SidebarContent>
</template>
