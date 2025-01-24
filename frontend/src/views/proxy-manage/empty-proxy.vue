<script setup lang="ts">
import { ClockIcon, ShoppingCartIcon } from "lucide-vue-next";
// import { IconApple } from "@/assets/icons/environment-bookmark-image";
import { AddCard } from "@/assets/icons/proxy-manage-image";
import { useRouter } from "vue-router";

const router = useRouter();
const addProxy = () => {
  router.push({
    name: "single-new-proxy",
  });
};

const buyProxy = () => {
  console.log("Buying proxy...");
  // Implement proxy purchase logic
};
</script>

<template>
  <div class="flex flex-col h-full bg-white rounded-md">
    <!-- Header -->
    <header class="p-6">
      <h4 class="text-2xl font-semibold leading-8">代理管理</h4>
    </header>

    <!-- Main Content -->
    <div
      class="flex flex-col flex-1 justify-center items-center px-4 text-center"
    >
      <!-- Illustration -->
      <div class="">
        <AddCard class="w-[240px] h-[209px]" />
      </div>

      <!-- Text Content -->
      <h2 class="mb-4 text-2xl font-semibold">
        我们提供了丰富、安全、稳定的代理服务
      </h2>
      <p class="mb-8 max-w-lg text-gray-500">
        通过代理，可以让浏览器拥有专用网络，防关联效果更佳
      </p>

      <!-- Action Buttons -->
      <div class="flex gap-4">
        <button
          @click="addProxy"
          class="inline-flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white rounded-md border border-gray-300 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          <ClockIcon class="mr-2 w-5 h-5 text-gray-400" />
          添加代理
        </button>
        <button
          @click="buyProxy"
          class="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-md border border-transparent hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          <ShoppingCartIcon class="mr-2 w-5 h-5" />

          购买代理
        </button>
      </div>
    </div>
  </div>
</template>
