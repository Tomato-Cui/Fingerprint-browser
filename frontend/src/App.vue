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
  <Updator />
</template>

<script setup>
import Layout from "@/components/layouts/index.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useRoute, useRouter } from "vue-router";
import { Toaster } from "@/components/ui/sonner";
import { isLogin } from "@/commands/user";
import { onMounted, ref, watch } from "vue";
import Login from "@/views/Login/index.vue";
import { set_token } from "@/commands/user";
import Updator from "./components/updator.vue";

const route = useRoute();
const router = useRouter();

onMounted(() => {
  if (route.path === "/") {
    const token = localStorage.getItem("token");
    if (token) {
      set_token(token);
      router.push("/environment/0");
    }
  }
});
</script>
