<script setup lang="ts">
import { Checkbox } from "@/components/ui/checkbox";
import Pagination from "./pagination.vue";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import EmptyTableMain from "./empty-table-main.vue";
import { ref, watch } from "vue";

let props = defineProps<{
  cols: any[];
  data: any[];
  selected: any[];
  pagination: {
    total: number;
    pageNum: number;
    pageSize: number;
  };
}>();

const emits = defineEmits<{
  select: [boolean, string];
  selectAll: [any[]];
  action: [any];
  "update:pageSize": [value: number];
  "update:page": [value: number];
}>();

const selectAlled = ref<boolean>(false);
watch(selectAlled, (_) => {
  if (selectAlled.value) {
    emits(
      "selectAll",
      props.data.filter((item) => !item.is_leader).map((item) => item.user_uuid)
    );
  } else {
    emits("selectAll", []);
  }
});
</script>
<template>
  <div class="rounded-lg border grow mx-4 mb-6 overflow-y-auto">
    <Table>
      <TableHeader class="sticky top-0">
        <TableRow>
          <TableHead
            ><Checkbox
              class="data-[state=checked]:bg-[#5050FA] border-gray-200"
              v-model:checked="selectAlled"
          /></TableHead>
          <TableHead
            class="text-left"
            v-for="item in props.cols"
            :key="item.id"
          >
            {{ item.title }}
          </TableHead>
        </TableRow>
      </TableHeader>

      <TableBody>
        <EmptyTableMain :data="props.data" :cols="props.cols" />
        <TableRow v-for="item in props.data">
          <TableCell
            ><Checkbox
              :disabled="item.is_leader"
              class="data-[state=checked]:bg-[#5050FA] border-gray-200"
              :checked="selected.includes(item.user_uuid)"
              @update:checked="(v:boolean) => emits('select',v, item.user_uuid)"
          /></TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.email }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.nickname || "--" }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.is_leader ? "所有者" : item.group_name }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">全部环境</p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              <span
                :class="[
                  'w-2 h-2 inline-block rounded-full',
                  item.blocked ? 'bg-red-500' : 'bg-green-500',
                ]"
              ></span>
              {{ !item.blocked ? "正常" : "已封禁" }}
            </p>
          </TableCell>
          <TableCell>
            <button
              :disabled="item.is_leader"
              @click="
                () =>
                  emits('action', {
                    uuid: item.user_uuid,
                    email: item.email,
                    nickname: item.nickname,
                    permission: item.group_name,
                  })
              "
              class="bg-[#EDEDFF] border border-[#5050FA] text-xs p-1 rounded-sm disabled:cursor-not-allowed disabled:bg-white disabled:border-[#a4a4fd]"
            >
              账号设置
            </button>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>

  <Pagination
    :total="pagination.total"
    :page-size="pagination.pageSize"
    @update:page-size="(v:number) => emits('update:pageSize', v)"
    @update:page="(v:number) => emits('update:page', v)"
  />
</template>
