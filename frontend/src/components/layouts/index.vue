<script setup lang="ts">
import { Sidebar, SidebarProvider } from "@/components/ui/sidebar";
import Logo from "./components/logo.vue";
import Content from "./components/content/index.vue";
import Footer from "./components/footer.vue";
import Main from "./components/main/index.vue";
import Mounted from "./components/on-mounted.vue";
import { onMounted, ref } from "vue";

const loading = ref(true);
const loadingHandle = () => (loading.value = false);

onMounted(() => {
  setTimeout(() => {
    loading.value = false;
  }, 1000);
});
</script>

<template>
  <SidebarProvider>
    <Sidebar collapsible="icon">
      <Logo />
      <Content />
      <Footer />
    </Sidebar>
    <Main>
      <Mounted @loading="loadingHandle" v-if="loading" />
      <slot v-else />
    </Main>
  </SidebarProvider>
</template>
