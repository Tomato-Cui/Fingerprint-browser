<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Circle } from "@/assets/icons/environment-group-manage";
import { SearchIcon } from "@/assets/icons/environment-bookmark-image";

import BuyProxy from "./manage-proxy-view/buy-proxy.vue";
import { environment_group_query } from "@/commands/environment-group";
import { AddCheck, AddProxy, Buy } from "@/assets/icons/proxy-manage-image";

interface Proxymanage {
  id: number;
  name: string;
  attribution: string;
  state: string;
  channel: string;
  group: string;
  browser_environment: string;
  phone_environment: string;
  create_message: string;
  selected: boolean;
}

const setField = ref(false);

const createProxymanage = ref(false);
const loadData = () => {
  environment_group_query(1, 1000).then((res) => {
    let { data: data_, total } = res.data;
    totalItems.value = total;
    Proxymanage.value = data_;
  });
};

onMounted(() => loadData());

const Proxymanage = ref<Proxymanage[]>([]);

const searchQuery = ref("");

const totalItems = ref(0);
</script>

<template>
  <div class="flex flex-col p-2 h-full">
    <!-- Header -->
    <div class="flex justify-between items-center mt-1">
      <h4 class="text-2xl font-semibold leading-8">代理管理</h4>

      <div class="flex gap-2 items-center">
        <button
          class="text-white hover:bg-[#4338CA] border font-[500] border-gray-300 text-sm rounded-md px-2 py-1.5 flex items-center outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <Circle class="w-[17.5px] h-[18.3px] text-gray-400" />
        </button>

        <div class="relative">
          <span class="absolute left-2 top-1/2 transform -translate-y-1/2">
            <SearchIcon class="w-[14px] h-[14px] text-left text-gray-400" />
          </span>
          <input
            type="search"
            v-model="searchQuery"
            placeholder="搜索"
            class="pl-8 w-54 border-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          />
        </div>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <AddCheck
            class="w-[14px] h-[14px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            批量检测代理
          </span>
        </button>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <AddProxy
            class="w-[14px] h-[14px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            添加代理
          </span>
        </button>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 bg-[#4F46E5] text-white outline-gray-50 hover:outline-gray-100"
          @click="setField = true"
        >
          <Buy class="mr-2 w-[11.67px] h-[11.67px]" />
          购买代理
        </button>
      </div>
    </div>
  </div>
  <BuyProxy v-model:createProxymanage="createProxymanage" />
  <SetField v-model:setField="setField" />
</template>
