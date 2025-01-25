<script setup lang="ts">
import { cn } from "@/util/lib";
import { Search } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import Input from "@/components/input.vue";
import {
  IconRecover,
  IconReTrash,
} from "@/assets/icons/environment-trash/index.ts";
import SelectFilter from "./components/select-filter.vue";
import { onMounted, reactive, ref, watch } from "vue";
import {
  environment_trash_query,
  environment_trash_delete_batch,
  environment_trash_recovers,
} from "@/commands/environment-trash";
import Table from "./components/table-main.vue";
import { toast } from "vue-sonner";

const cols = [
  { id: 1, title: "序号" },
  { id: 2, title: "环境名称" },
  { id: 3, title: "账号信息" },
  { id: 4, title: "代理信息" },
  { id: 5, title: "备注" },
  { id: 6, title: "标签" },
  { id: 7, title: "分组" },
  { id: 8, title: "删除信息" },
];
const actionBtns = ref<any[]>([
  {
    id: 1,
    title: "物理删除",
    icon: IconReTrash,
    disabled: true,
    handle: () => {
      let warning = (e: any) => toast.warning("删除失败 " + e);

      environment_trash_delete_batch(selected.value)
        .then((res) => {
          if (res.code == 1) {
            toast.success("删除成功");
          }
          if (res.code != 1) warning(res.message);
        })
        .catch(warning);
      loadData();
    },
  },
  {
    id: 2,
    title: "恢复",
    icon: IconRecover,
    disabled: true,
    handle: () => {
      let warning = (e: any) => toast.warning("恢复失败 " + e);

      environment_trash_recovers(selected.value)
        .then((res) => {
          if (res.code == 1) {
            toast.success("恢复成功");
          }
          if (res.code != 1) warning(res.message);
        })
        .catch(warning);
      loadData();
    },
  },
]);

const data = ref<any[]>([]);
const dataCache = ref<any[]>([]);
const selected = ref<any[]>([]);
const pagination = reactive({
  total: 100,
  pageNum: 1,
  pageSize: 17,
});
const selectFilterRef = reactive<any>({
  proxy: false,
  account: "",
  tag: "",
  group: "",
});

const loadData = () => {
  environment_trash_query(pagination.pageNum, pagination.pageSize).then(
    (res) => {
      pagination.total = res.data.total;
      data.value = res.data.data.map((item: any) => ({
        uuid: item.uuid,
        name: item.name || "--",
        proxy: item.proxy_host ? item.proxy_host + ":" + item.proxy_port : "--",
        account: item.accounts.platform || "--",
        description: item.description || "--",
        tag: item.tag_name || "--",
        group: item.group_name || "--",
        message: {
          from: item.delete_from_user_nickname,
          deleted_at: item.deleted_at,
        },
      }));
      dataCache.value = JSON.parse(JSON.stringify(data.value));
    }
  );
};

const actionBtnActiveHandle = (v: boolean) => {
  actionBtns.value = actionBtns.value.map((item) => ({ ...item, disabled: v }));
};
const updatePageSizeHandle = (pageS: number) => {
  pagination.pageSize = pageS;
};
const updatePageHandle = (pageS: number) => {
  pagination.pageNum = pageS;
};

const selectHandles = {
  selectHandle: (v: boolean, item: string) => {
    if (!v) {
      selected.value = selected.value.filter(
        (selectedItem) => selectedItem !== item
      );
    } else {
      selected.value = [...selected.value, item];
    }
  },
  selectAllHandle: (items: any[]) => {
    selected.value = items;
  },
};

const searchHandle = (sub: string) => {
  let currents = dataCache.value.filter((item) =>
    item.name.toLowerCase().includes(sub.toLowerCase())
  );
  if (currents.length == 0) {
    data.value = [];
  } else if (sub.length == 0) {
    data.value = JSON.parse(JSON.stringify(dataCache.value));
  } else {
    data.value = currents;
  }
};

onMounted(loadData);
watch(selected, () => {
  actionBtnActiveHandle(selected.value.length == 0);
});

watch(
  () => [pagination.pageNum, pagination.pageSize],
  () => loadData()
);

watch(selectFilterRef, (_) => {
  data.value = dataCache.value
    .filter((item) =>
      selectFilterRef.proxy ? item.proxy != "--" : item.proxy == "--"
    )
    .filter((item) =>
      selectFilterRef.account ? selectFilterRef.account == item.account : true
    )
    .filter((item) =>
      selectFilterRef.tag ? selectFilterRef.tag == item.tag : true
    )
    .filter((item) =>
      selectFilterRef.group ? selectFilterRef.group == item.group : true
    );
});
const resetSelectFilterHandle = () => (data.value = dataCache.value);
</script>

<template>
  <div class="flex overflow-hidden flex-col h-full">
    <div class="flex justify-between px-4">
      <div class="flex gap-x-2 items-center mt-1 mb-2 justify-betwee">
        <h3 class="flex items-end text-xl font-semibold tracking-tight">
          回收站
        </h3>
        <p class="text-[#8F9BB3] text-xs flex items-center h-full">
          可恢复最近7天内删除的环境
        </p>
      </div>
      <div class="flex gap-4 justify-between items-center mb-2 select-none">
        <div class="flex flex-1 gap-2 items-center pt-1">
          <div class="relative flex-1 max-w-md">
            <Search
              class="absolute left-3 top-[50%] transform -translate-y-1/2 text-gray-400 h-4 w-4 z-10"
            />
            <Input
              class="pl-8 h-8 text-xs text-gray-500"
              placeholder="根据环境名称搜索"
              @input="(v:any) => searchHandle(v.target.value)"
            />
          </div>
          <SelectFilter
            :data="dataCache"
            :selects="selectFilterRef"
            @update:on-proxy-select="(v) => (selectFilterRef.proxy = v)"
            @update:on-account-select="(v) => (selectFilterRef.account = v)"
            @update:on-tag-select="(v) => (selectFilterRef.tag = v)"
            @update:on-group-select="(v) => (selectFilterRef.group = v)"
            @update:reset="resetSelectFilterHandle"
          />
          <Button
            variant="outline"
            :key="item.id"
            :disabled="item.disabled"
            @click="item.handle"
            v-for="item in actionBtns"
            :class="
              cn(
                'h-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]',
                item.title == '恢复'
                  ? 'bg-[#5050FA] hover:bg-[#5558E3] hover:text-white text-white'
                  : ''
              )
            "
          >
            <component :is="item.icon" />
            {{ item.title }}</Button
          >
        </div>
      </div>
    </div>

    <Table
      :cols="cols"
      :data="data"
      :selected="selected"
      :pagination="pagination"
      @select="selectHandles.selectHandle"
      @select-all="selectHandles.selectAllHandle"
      @update:page="updatePageHandle"
      @update:page-size="updatePageSizeHandle"
    />
  </div>
</template>
