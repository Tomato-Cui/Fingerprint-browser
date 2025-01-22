<script setup lang="ts">
import Header from "./header.vue";
import { SidebarInset } from "@/components/ui/sidebar";
import Breadcrumb from "./breadcrumb.vue";
import DetailSidebar from "./detail-sidebar/index.vue";
import { useRoute } from "vue-router";
const route = useRoute();

const detailRoutes = [
  "/environment/create/environment-advanced-create",
  "/environment/create/environment-batch-import",
];
</script>

<template>
  <SidebarInset class="bg-sidebar overflow-hidden h-screen">
    <main
      class="bg-background mx-2 my-4 rounded-xl shadow-lg flex flex-col h-[calc(100%_-_32px)]"
      v-if="detailRoutes.findIndex((item) => item == route.path) != -1"
    >
      <Header />
      <div class="flex-grow gap-4 h-full overflow-hidden relative">
        <DetailSidebar>
          <slot />
        </DetailSidebar>
      </div>
    </main>

    <main
      class="bg-layout-background mx-2 my-4 rounded-xl shadow-lg flex flex-col h-[calc(100%_-_32px)]"
      v-else
    >
      <Header />
      <Breadcrumb />
      <div class="flex-grow gap-4 h-full overflow-auto relative">
        <slot></slot>
      </div>
    </main>
  </SidebarInset>
</template>
