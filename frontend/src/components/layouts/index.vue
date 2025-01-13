<script setup lang="ts">
import { Sidebar, SidebarProvider } from "@/components/ui/sidebar";
import Logo from "./components/logo.vue";
import Content from "./components/content/index.vue";
import Footer from "./components/footer.vue";
import Main from "./components/main/index.vue";
</script>

<template>
  <SidebarProvider>
    <Sidebar collapsible="icon">
      <Logo />
      <Content />
      <Footer />
    </Sidebar>
    <Main>
      <slot />
    </Main>
  </SidebarProvider>
</template>
