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
    title: "快速创建",
    value: "quick",
    icon: IconRulerCrossPen,
    href: "/environment-simple-create",
  },
  {
    id: 2,
    title: "高级创建",
    value: "advanced",
    icon: IconStarFallMinimalistic,
    href: "/environment-advanced-create",
  },
  {
    id: 3,
    title: "批量导入",
    value: "batch",
    icon: IconLoginArrow,
    href: "/environment-batch-import",
  },
];
</script>
<template>
  <Card className="w-full h-full flex flex-col p-0">
    <CardHeader class="p-0 px-4">
      <CardTitle class="mt-1 mb-2 text-xl">新建环境</CardTitle>
      <ul
        class="bg-white border-b rounded-none w-full flex justify-start pb-0 px-0 text-sm"
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
          <div class="flex items-center justify-center gap-x-3">
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
    </CardHeader>
    <slot />
  </Card>
</template>
