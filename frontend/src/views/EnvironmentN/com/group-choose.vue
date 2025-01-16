<script setup lang="ts">
import { onMounted, ref } from "vue";
import { cn } from "@/util/lib";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { useRouter, useRoute } from "vue-router";
import { environment_group_query } from "@/commands/environment-group"

const router = useRouter();
const route = useRoute();
const tabs = ref<any>([]);

const getList = () => {
  environment_group_query(1, 1000).then((res: any) => {
    console.log("res:", res.data);
    let table: any[] = []
    table = res.data.data.map((item: any) => {
      return {
        id: item.id,
        title: item.name,
        value: item.name,
        href: '/environment/' + item.id,
      }
    })
    tabs.value = [{
      id: 1,
      title: "默认分组",
      value: "default",
      href: "/environment/0",
    }, ...table]
  })
}
onMounted(() => {
  getList()
})
</script>

<template>
  <Card className="w-full flex flex-col p-0 flex-1 overflow-auto">
    <CardHeader class="p-0">
      <ul class="bg-white border-b rounded-none w-full flex justify-start pb-0 px-0 text-sm mb-3 space-x-6">
        <li :key="item.id" v-for="item in tabs" @click="router.push(item.href)" :class="cn(
          'border-transparent border-b-[3px] rounded-none flex items-center pb-1 cursor-pointer',
          route.path == item.href ? 'text-[#5050FA] border-[#5050FA]' : ''
        )
          ">
          <div class="flex items-center justify-center gap-x-3">
            {{ item.title }}
          </div>
        </li>
      </ul>
    </CardHeader>
    <slot />
  </Card>
</template>
