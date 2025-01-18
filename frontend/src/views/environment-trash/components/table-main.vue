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
  "update:pageSize": [value: number];
  "update:page": [value: number];
}>();

const selectAlled = ref<boolean>(false);

watch(selectAlled, (_) => {
  if (selectAlled.value) {
    emits(
      "selectAll",
      props.data.map((item) => item.uuid)
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
          <TableHead class="text-left" v-for="item in cols" :key="item.id">
            {{ item.title }}
          </TableHead>
        </TableRow>
      </TableHeader>

      <TableBody>
        <EmptyTableMain :data="data" :cols="cols" />
        <TableRow v-for="(item, index) in data">
          <TableCell
            ><Checkbox
              class="data-[state=checked]:bg-[#5050FA] border-gray-200"
              :checked="selected.includes(item.uuid)"
              @update:checked="(v:boolean) => emits('select',v, item.uuid)"
          /></TableCell>
          <TableCell>{{ index + 1 }}</TableCell>

          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.name }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.account }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.proxy }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.description }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.tag }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs w-24">
              {{ item.group }}
            </p>
          </TableCell>
          <TableCell>
            <p class="overflow-hidden truncate text-xs">
              删除用户:
              <span>{{ item.message.from }}</span>
            </p>
            <p class="flex items-center text-xs">
              删除时间:
              <span class="inline-block w-28 truncate">{{
                new Date(item.message.deleted_at).toLocaleString()
              }}</span>
            </p>
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
