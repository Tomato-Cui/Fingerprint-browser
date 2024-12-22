<template>
  <div
    v-if="props.open"
    class="flex fixed inset-0 z-50 justify-center items-center"
  >
    <!-- Overlay -->
    <div class="absolute inset-0 bg-black bg-opacity-20" @click="close"></div>

    <!-- Modal -->
    <div class="relative bg-white rounded-lg w-[480px] shadow-lg">
      <!-- Header -->
      <div class="flex justify-between items-center px-6 py-4">
        <h3 class="text-lg font-medium">修改代理</h3>
        <button @click="close" class="text-gray-400 hover:text-gray-500">
          ×
        </button>
      </div>

      <!-- Content -->
      <div class="px-6 py-4 space-y-6">
        <div class="flex items-center">
          <label class="w-24 text-gray-600">环境序号</label>
          <span class="px-2 py-1 text-blue-800 bg-blue-100 rounded">{{
            envNumber
          }}</span>
        </div>

        <div class="flex items-center">
          <label class="w-24 text-gray-600">账号平台</label>
          <div class="flex space-x-4">
            <label class="inline-flex items-center">
              <input
                type="radio"
                v-model="platform"
                value="custom"
                class="text-blue-500 form-radio"
              />
              <span class="ml-2">自定义</span>
            </label>
            <label class="inline-flex items-center">
              <input
                type="radio"
                v-model="platform"
                value="added"
                class="text-blue-500 form-radio"
              />
              <span class="ml-2">已添加代理</span>
            </label>
          </div>
        </div>

        <!-- <div class="flex items-center">
          <label class="w-24 text-gray-600">代理类型</label>
          <div class="relative flex-1">
            <select
              v-model="proxyType"
              class="px-3 py-2 w-full rounded-md border border-gray-300 appearance-none focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="no-proxy">NO Proxy (本地直连)</option>
              <option value="http">HTTP Proxy</option>
              <option value="socks">SOCKS Proxy</option>
            </select>
            <div class="flex absolute top-0 right-0 bottom-0 items-center pr-2">
              <svg
                class="w-5 h-5 text-blue-500"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
              >
                <path d="M12 21a9 9 0 100-18 9 9 0 000 18z" />
                <path d="M12 7v4l3.5 3.5" />
              </svg>
            </div>
          </div>
        </div> -->

        <Select v-model="proxyType" class="">
          <div class="flex items-center">
            <div class="w-24">代理类型</div>
            <!-- <SelectTrigger class="w-32"> -->

            <div>
              <SelectTrigger class="w-80 ml-38">
                <SelectValue
                  placeholder=""
                  class="p-2 w-full rounded-lg outline-none"
                />
              </SelectTrigger>
            </div>
          </div>

          <SelectContent>
            <SelectGroup>
              <SelectItem value="no-proxy">NO Proxy (本地直连) </SelectItem>
              <SelectItem value="http"> HTTP Proxy </SelectItem>
              <SelectItem value="socks"> SOCKS Proxy </SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>

        <!-- <div class="flex items-center">
          <label class="w-24 text-gray-600">IP查询通道</label>
          <div class="flex-1">
            <select
              v-model="ipChannel"
              class="px-3 py-2 w-full rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="ip2location">IP2Location</option>
              <option value="other">其他通道</option>
            </select>
          </div>
        </div> -->

        <Select v-model="ipChannel" class="">
          <!-- 修改宽度 -->
          <!-- <SelectTrigger class="w-32"> -->

          <div class="flex items-center">
            <div class="w-24">IP查询通道</div>

            <div>
              <SelectTrigger class="w-80 ml-38">
                <SelectValue
                  placeholder=")"
                  class="p-2 w-full rounded-lg outline-none"
                />
              </SelectTrigger>
            </div>
          </div>
          <SelectContent>
            <SelectGroup>
              <SelectItem value="ip2location"> IP2Location </SelectItem>
              <SelectItem value="other"> 其他通道 </SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>

      <!-- Footer -->
      <div class="flex justify-center px-6 py-4 space-x-4">
        <button
          @click="confirm"
          class="px-6 py-2 text-white bg-blue-500 rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        >
          确定
        </button>
        <button
          @click="close"
          class="px-6 py-2 text-gray-600 bg-gray-100 rounded hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2"
        >
          取消
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";

const props = defineProps({
  open: false,
  title: "",
  envNumber: {
    type: [String, Number],
    default: "7",
  },
});

const emit = defineEmits(["close", "confirm"]);

const platform = ref("custom");
const proxyType = ref("no-proxy");
const ipChannel = ref("ip2location");

const close = () => {
  emit("close");
};

const confirm = () => {
  emit("confirm", {
    platform: platform.value,
    proxyType: proxyType.value,
    ipChannel: ipChannel.value,
  });
  close();
};
</script>
