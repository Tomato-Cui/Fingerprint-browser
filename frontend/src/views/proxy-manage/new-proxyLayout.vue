<script setup lang="ts">
import { cn } from "@/util/lib";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  IconRulerCrossPen,
  IconStarFallMinimalistic,
  IconLoginArrow,
} from "@/assets/icons/environment-creates/simple-create/index";
import { useRouter, useRoute } from "vue-router";

const router = useRouter();

const route = useRoute();
const tabs = [
  {
    id: 1,
    title: "单个添加",
    value: "quick",
    icon: IconRulerCrossPen,
    href: "/single-new-proxy",
  },
  {
    id: 2,
    title: "批量添加",
    value: "advanced",
    icon: IconStarFallMinimalistic,
    href: "/batch-new-proxy",
  },
];
</script>
<template>
  <div class="h-full">
    <div class="flex flex-col h-full">
      <h4 class="text-2xl font-semibold leading-8">书签</h4>

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
      <div class="flex-col h-full">
        <slot></slot>
      </div>
    </div>
  </div>
</template>
