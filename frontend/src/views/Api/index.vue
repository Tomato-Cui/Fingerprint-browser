<script setup>
import { ref, computed } from "vue";
import Input from "@/components/input.vue";
import { FileIcon, RefreshCwIcon, RefreshCcw } from "lucide-vue-next";
import Switch from "@/components/switch.vue";
import { DefaultButton } from "@/components/button";
import { RefreshCcwIcon, CopyIcon } from "lucide-vue-next";
import TooltipButton from "@/components/tooltip-button.vue";

const endpoint = ref("127.0.0.1");
const port = ref(":5678");
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
  apiKey.value =
    Math.random().toString(36).substring(2) +
    Math.random().toString(36).substring(2);
};
</script>

<template>
  <div class="h-main bg-gray-50 p-4 mx-auto w-[800px]">
    <div>
      <div class="flex justify-end border-b py-2 gap-x-4 items-center">
        <Switch />
      </div>
      <div class="my-4">
        <p class="py-1 border-l-4 border-gray-700 px-4">API密钥</p>
        <div class="justify-end border-b my-2 py-1 border rounded-sm">
          <div class="flex gap-x-2 mx-2">
            <TooltipButton
              title="刷新"
              class="bg-gray-700 text-white rounded-md h-12 w-12 flex items-center justify-center"
            >
              <RefreshCcwIcon class="w-4 h-4" />
            </TooltipButton>
            <div
              class="bg-gray-800 text-white text-sm rounded-sm h-12 px-4 justify-center flex flex-col my-2 flex-auto relative overflow-hidden"
            >
              3d50a489984362c71713b9fd1cf79ef0
              <span
                class="absolute top-0 right-0 rounded-bl-md bg-gray-600 p-0.5"
                ><CopyIcon class="w-3.5 h-3.5 cursor-pointer text-gray-100"
              /></span>
            </div>
          </div>
          <p class="text-xs text-right text-gray-500 mx-2">
            不要向任何人透露你的密钥
          </p>

          <div class="border-t mt-4 px-2 py-4 h-[4.5rem] flex gap-x-2">
            <span class="whitespace-nowrap flex items-center">设置</span>
            <div class="flex w-full">
              <input
                type="text"
                class="border flex-auto outline-none rounded-tl-md px-4"
                placeholder="127.0.0.1"
              />
              <input
                type="text"
                class="border-r border-t border-b w-1/4 outline-none rounded-br-md px-4"
                placeholder="5678"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
    <div>
      <div class="mt-8 grid grid-cols-2 gap-2 gap-y-8 text-sm">
        <div class="bg-gray-200 p-2 rounded-sm">
          <p class="py-1 border-l-4 border-gray-500 px-4">使用须知</p>
          <p class="py-4">
            用户可以通过 API
            的方式来读写账号配置信息、创建、打开和管理环境，还可以配合
            Selenium、Puppeteer 和 Playwright 等自动化框架实现浏览器操作的自动化
          </p>
        </div>
        <div class="bg-gray-200 p-2 rounded-sm">
          <p class="py-1 border-l-4 border-gray-500 px-4">API 接口限制</p>
          <p class="py-4">API 请求频率限制：100 次 / 分钟</p>
        </div>
      </div>
      <div class="bg-gray-200 p-2 rounded-sm text-sm mt-2">
        <p class="py-4">
          更多 API 功能和使用方法, 请参考
          <span class="underline cursor-pointer">API文档</span>
        </p>
      </div>
    </div>
  </div>
</template>
