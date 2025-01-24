<script setup lang="ts">
import { cn } from "@/util/lib";

import {
  IconRulerCrossPen,
  IconStarFallMinimalistic,
} from "@/assets/icons/environment-creates/simple-create/index";
import { useRouter, useRoute } from "vue-router";

import { Circle, Filter } from "@/assets/icons/environment-group-manage";
import {
  SearchIcon,
} from "@/assets/icons/environment-bookmark-image";

import {
  AddCheck,
  AddProxy,
  Buy,
} from "@/assets/icons/proxy-manage-image";

import {
  FileText,
  PenModify,
  Plate,
  Round,
} from "@/assets/icons/proxy-manage-image";

const router = useRouter();

const route = useRoute();
const tabs = [
  {
    id: 1,
    title: "自有代理",
    value: "quick",
    icon: IconRulerCrossPen,
    href: "/private-proxy",
  },
  {
    id: 2,
    title: "平台代理",
    value: "advanced",
    icon: IconStarFallMinimalistic,
    href: "/platform-proxy",
  },
];
</script>

<template>
  <div class="h-full border border-red-500">
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
            placeholder="搜索"
            class="pl-8 w-54 border-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          />
        </div>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <Filter
            class="w-[10.21px] h-[10.21px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            筛选
          </span>
        </button>

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
        >
          <Buy class="mr-2 w-[11.67px] h-[11.67px]" />
          购买代理
        </button>
      </div>
    </div>

    <div class="h-full">
      <div class="flex flex-col h-full">
        <ul
          class="flex justify-start px-0 pb-0 w-full text-sm bg-white rounded-none border-b"
        >
          <li
            :key="item.id"
            v-for="item in tabs"
            @click="router.push(item!.href)"
            :class="
              cn(
                'border-transparent border-b-[3px] rounded-none flex items-center pb-1 px-6 cursor-pointer',
                route.path == item.href ? 'text-[#5050FA] border-[#5050FA]' : ''
              )
            "
          >
            <div class="flex gap-x-3 justify-center items-center">
              <component
                :is="item.icon"
                :class="
                  cn(
                    'size-4 fill-[#8F9BB3]',
                    route.path == item.href ? 'fill-[#5050FA]' : ''
                  )
                "
              />
              {{ item.title }}
            </div>
          </li>
        </ul>

        <div class="flex flex-row gap-x-2 m-2 w-full">
          <button
            class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          >
            <FileText
              class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
            />
            <span class="font-sans font-semibold text-center text-black">
              设置分组
            </span>
          </button>

          <button
            class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          >
            <PenModify
              class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
            />
            <span class="font-sans font-semibold text-center text-black">
              修改IP查询渠道
            </span>
          </button>

          <button
            class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          >
            <Plate
              class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
            />
            <span class="font-sans font-semibold text-center text-black">
              修改代理
            </span>
          </button>

          <button
            class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          >
            <Round
              class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
            />
            <span class="font-sans font-semibold text-center text-black">
              删除代理
            </span>
          </button>
        </div>

        <div class="flex flex-col h-full">
          <slot name="manage-proxy-content"></slot>
        </div>
      </div>
    </div>
  </div>
</template>
