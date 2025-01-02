<template>
  <div class="p-4 bg-gray-50 h-main">
    <div class="gap-6 bg-white h-full px-[40px] py-[30px] rounded-xl flex">
      <!-- Left Column -->
      <div class="space-y-3 w-2/3 h-4/5">
        <div class="h-[170px] mb-[20px]">
          <h2 class="text-lg font-medium pb-[10px] mb-[15px] border-b">
            端口设置
          </h2>
          <div class="space-y-4">
            <input
              type="text"
              v-model="endpoint"
              class="p-2 w-full rounded-md border border-gray-200"
              placeholder="127.0.0.1"
            />
            <input
              type="text"
              v-model="port"
              class="p-2 w-full rounded-md border border-gray-200"
              placeholder=":50000"
            />
          </div>
        </div>

        <div class="">
          <h2 class="text-lg font-medium mb-[20px] pb-[10px] border-b">
            接口文档
          </h2>
          <div class="flex mb-[70px]">
            <button
              class="flex items-center px-4 py-2 text-white bg-blue-600 rounded-l-md"
            >
              <FileIcon class="mr-2 w-5 h-5" />
              文档链接
            </button>
            <input
              type="text"
              v-model="docUrl"
              class="flex-1 p-2 rounded-r-md border border-gray-200"
              placeholder="https://www.iconfont.cn/search/index?"
            />
          </div>
        </div>

        <div class="space-y-3 h-[300px]">
          <div class="text-gray-300 mb-[30px]">
            <span class="mb-2 font-medium text-gray-600">功能概述：</span
            >用户可以通过 API
            的方式来读写账号配置信息、创建、打开和管理环境，还可以配合
            Selenium、Puppeteer 和 Playwright
            等自动化框架实现浏览器操作的自动化...
          </div>

          <div class="text-gray-300 mb-[30px]">
            <span class="mb-2 font-medium text-gray-600">接口限制：</span
            >API请求频率限制：100次/分钟
          </div>

          <div class="text-gray-300 mb-[30px]">
            <span class="mb-2 font-medium text-gray-600">使用说明：</span>更多
            API 功能和使用方法，请参考 API 文档
          </div>
        </div>
      </div>

      <!-- Right Column -->
      <div
        class="flex flex-col justify-between p-6 w-1/3 h-full bg-gray-700 rounded-lg"
      >
        <div class="">
          <h2 class="mb-6 text-lg font-medium text-white">API密钥</h2>
          <div class="flex flex-col items-end p-4 mb-4 bg-gray-800 rounded-md">
            <div class="mb-2 text-white break-all">
              {{ maskedApiKey }}
            </div>
            <button
              class="text-blue-500 w-[134px] h-[32px] opacity-100 rounded-[4px] bg-white border border-[#0054fe] flex items-center justify-center"
            >
              <RefreshCcw class="w-[20px] h-[20px] mr-[10px]" />
              重置API密钥
            </button>
          </div>
          <p class="mb-4 text-sm text-white">*请确保您的API密钥安全保存</p>
        </div>

        <button
          @click="copyApiKey"
          class="py-2 w-full text-white bg-blue-600 rounded-md transition-colors hover:bg-blue-700"
        >
          复制密钥
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";
import { FileIcon, RefreshCwIcon, RefreshCcw } from "lucide-vue-next";

const endpoint = ref("127.0.0.1");
const port = ref(":50000");
const docUrl = ref("https://www.iconfont.cn/search/index?");
const apiKey = ref("b303666e7e5da02c9c5a9007b86b7a2b");

const maskedApiKey = computed(() => {
  return apiKey.value;
});

const copyApiKey = async () => {
  try {
    await navigator.clipboard.writeText(apiKey.value);
    alert("API密钥已复制到剪贴板");
  } catch (err) {
    console.error("复制失败:", err);
  }
};

const resetApiKey = () => {
  // In a real application, this would make an API call to reset the key
  apiKey.value =
    Math.random().toString(36).substring(2) +
    Math.random().toString(36).substring(2);
};
</script>
