<script setup lang="ts">
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();

function isNumber(str: any) {
  str = str.trim();
  return !isNaN(parseFloat(str)) && isFinite(str);
}
</script>
<template>
  <div class="flex items-center gap-2 px-4 mt-2">
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem
          @click="router.push('/environment/0')"
          class="text-xs cursor-pointer"
        >
          Home
        </BreadcrumbItem>

        <template
          v-for="item in route.path
            .split('/')
            .filter((item) => item.length > 0)
            .filter((item) => !isNumber(item))"
        >
          <BreadcrumbSeparator> / </BreadcrumbSeparator>
          <BreadcrumbItem class="text-xs cursor-pointer">
            {{ item }}
          </BreadcrumbItem>
        </template>
      </BreadcrumbList>
    </Breadcrumb>
  </div>
</template>
