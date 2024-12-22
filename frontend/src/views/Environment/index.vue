<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev,
} from "@/components/ui/pagination";
import { ref, onMounted, reactive } from "vue";
import TooltipButton from "@/components/tooltip-button.vue";
import { LogsIcon } from "lucide-vue-next";
import SearchInput from "./search-input.vue";
import GroupSelect from "./group-select.vue";
import SortBtn from "./sort-btn.vue";
import { environment_query } from "@/commands/environment";
// import DataTable, { type Payment } from "./data-table.vue";
import DataTable, { type Payment } from "./data-table.vue";

const data = ref<Array<Payment>>([]);
const pagination = reactive({
  pageIndex: 0,
  pageSize: 16,
  total: 0,
});

const loadData = (index: number, size: number) => {
  environment_query(index, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
    console.log(data.value);
    
  });
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));
const paginationClickHandle = (index: number) => {
  loadData(index, pagination.pageSize);
  pagination.pageIndex = index;
};
</script>

<template>
  <div class="flex flex-col p-3 bg-gray-50 h-main">
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow px-2"
    >
      <div class="flex w-full">
        <div class="flex gap-2 items-center py-2 w-3/4">
          <GroupSelect />
          <SearchInput />
        </div>
        <div class="flex gap-2 py-2 ju justify-end flex-auto px-2">
          <TooltipButton
            title="筛选"
            class="p-2.5 hover:bg-gray-100 rounded border-gray-200 border"
          >
            <LogsIcon class="h-5 w-5 text-gray-600" />
          </TooltipButton>
          <SortBtn />
        </div>
      </div>
      <div class="flex flex-col h-full">
        <div class="rounded-md flex-auto h-0 overflow-auto">
          <DataTable :data="data" :pagination="pagination" />
        </div>

        <div class="flex items-center justify-end space-x-2 py-1">
          <div class="flex-1 text-sm text-muted-foreground">
            共{{ pagination.total }}条.
          </div>
          <div class="space-x-2">
            <Pagination
              :total="pagination.total"
              :itemsPerPage="pagination.pageSize"
              :default-page="1"
            >
              <PaginationList
                v-slot="{ items }"
                class="flex items-center gap-1"
              >
                <PaginationFirst />
                <PaginationPrev />

                <template v-for="(item, index) in items">
                  <PaginationListItem
                    v-if="item.type === 'page'"
                    :key="index"
                    :value="item.value"
                    as-child
                  >
                    <Button
                      class="w-10 h-10 p-0"
                      @click="() => paginationClickHandle(index)"
                      :variant="
                        item.value === pagination.pageIndex + 1
                          ? 'default'
                          : 'outline'
                      "
                    >
                      {{ item.value }}
                    </Button>
                  </PaginationListItem>
                  <PaginationEllipsis v-else :key="item.type" :index="index" />
                </template>

                <PaginationNext />
                <PaginationLast />
              </PaginationList>
            </Pagination>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
