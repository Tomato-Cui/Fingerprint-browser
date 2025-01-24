<script setup lang="ts">
import { cn } from "@/util/lib";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  IconRulerCrossPen,
  IconStarFallMinimalistic,
  IconLoginArrow,
} from "@/assets/icons/environment-creates/simple-create/index";
import { useRouter, useRoute } from "vue-router";
import { onMounted } from "vue";

const router = useRouter();
const route = useRoute();
const tabs = [
  {
    id: 1,
    title: "快速创建",
    value: "quick",
    icon: IconRulerCrossPen,
    href: "/environment/create/environment-simple-create",
  },
  {
    id: 2,
    title: "高级创建",
    value: "advanced",
    icon: IconStarFallMinimalistic,
    href: "/environment/create/environment-advanced-create",
  },
  {
    id: 3,
    title: "批量导入",
    value: "batch",
    icon: IconLoginArrow,
    href: "/environment/create/environment-batch-import",
  },
];
</script>
<template>
  <Card className="w-full h-full flex flex-col p-0 select-none">
    <CardHeader class="p-0 px-4">
      <CardTitle class="mt-1 mb-2 text-xl" v-if="!route.query.env">新建环境</CardTitle>
      <CardTitle class="mt-1 mb-2 text-xl" v-else>编辑环境</CardTitle>
      <ul
      v-if="!route.query.env"
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
    </CardHeader>
    <div class="grow flex flex-col overflow-hidden">
      <router-view />
    </div>
  </Card>
</template>
