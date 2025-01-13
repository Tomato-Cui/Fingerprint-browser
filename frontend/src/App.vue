<template>
  <template v-if="route.path === '/'">
    <Login />
  </template>
  <template v-else>
    <TooltipProvider>
      <Layout>
        <router-view />
      </Layout>
    </TooltipProvider>
  </template>
  <Toaster :rich-colors="true" position="top-center" />
</template>

<script setup>
import Layout from "@/components/layouts/index.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useRoute } from "vue-router";
import { Toaster } from "@/components/ui/sonner";
import { isLogin } from "@/commands/user";
import { onMounted, ref, watch } from "vue";
import Login from "@/views/Login/index.vue";

const route = useRoute();
</script>
