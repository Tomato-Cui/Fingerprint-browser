<script setup lang="ts">
import Header from "./header.vue";
import { SidebarInset } from "@/components/ui/sidebar";
</script>

<template>
  <SidebarInset class="bg-sidebar h-screen overflow-hidden">
    <main
      class="bg-white mx-2 my-4 rounded-xl shadow-lg flex flex-col h-[calc(100%_-_32px)]"
    >
      <Header />
      <div class="flex-grow gap-4 px-4 pt-0 my-4 overflow-auto">
        <slot></slot>
      </div>
    </main>
  </SidebarInset>
</template>
