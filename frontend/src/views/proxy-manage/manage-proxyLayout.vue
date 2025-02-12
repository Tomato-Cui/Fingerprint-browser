<script setup lang="ts">
import { cn } from "@/util/lib";
import {
  IconRulerCrossPen,
  IconStarFallMinimalistic,
} from "@/assets/icons/environment-creates/simple-create/index";
import { useRouter, useRoute } from "vue-router";

import SelectFilter from "./select-filter.vue";

import { onMounted, reactive, ref, watch } from "vue";
import { Circle } from "@/assets/icons/environment-group-manage";
import { SearchIcon } from "@/assets/icons/environment-bookmark-image";

import { AddProxy, AddCheck, Buy } from "@/assets/icons/proxy-manage-image";
import {
  environment_trash_delete_batch,
  environment_trash_query,
  environment_trash_recovers,
} from "@/commands/environment-trash";
import { IconRecover, IconReTrash } from "@/assets/icons/environment-trash";
import { toast } from "vue-sonner";

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

// const cols = [
//   { id: 1, title: "序号" },
//   { id: 2, title: "环境名称" },
//   { id: 3, title: "账号信息" },
//   { id: 4, title: "代理信息" },
//   { id: 5, title: "备注" },
//   { id: 6, title: "标签" },
//   { id: 7, title: "分组" },
//   { id: 8, title: "删除信息" },
// ];
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
        tag: item.tag_name || "abc",
        group: item.group_name || "--",
        message: {
          from: item.delete_from_user_nickname,
          deleted_at: item.deleted_at,
        },
      }));
      data.value[0].tag = "1";
      dataCache.value = JSON.parse(JSON.stringify(data.value));
    }
  );
};

const actionBtnActiveHandle = (v: boolean) => {
  actionBtns.value = actionBtns.value.map((item) => ({ ...item, disabled: v }));
};
// const updatePageSizeHandle = (pageS: number) => {
//   pagination.pageSize = pageS;
// };
// const updatePageHandle = (pageS: number) => {
//   pagination.pageNum = pageS;
// };

// const selectHandles = {
//   selectHandle: (v: boolean, item: string) => {
//     if (!v) {
//       selected.value = selected.value.filter(
//         (selectedItem) => selectedItem !== item
//       );
//     } else {
//       selected.value = [...selected.value, item];
//     }
//   },
//   selectAllHandle: (items: any[]) => {
//     selected.value = items;
//   },
// };

// const searchHandle = (sub: string) => {
//   let currents = dataCache.value.filter((item) =>
//     item.name.toLowerCase().includes(sub.toLowerCase())
//   );
//   if (currents.length == 0) {
//     data.value = [];
//   } else if (sub.length == 0) {
//     data.value = JSON.parse(JSON.stringify(dataCache.value));
//   } else {
//     data.value = currents;
//   }
// };

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
  <div class="flex flex-col h-full">
    <div class="flex flex-row">
      <div class="flex gap-x-2 justify-between items-center px-4 mt-1 mb-2">
        <h3 class="flex items-end text-xl font-semibold tracking-tight">
          代理管理
        </h3>
      </div>

      <div class="flex flex-row gap-2 items-center mr-2 ml-auto">
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

        <!-- <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <Filter
            class="w-[10.21px] h-[10.21px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            筛选
          </span>
        </button> -->

        <SelectFilter
          :data="dataCache"
          :selects="selectFilterRef"
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          @update:on-proxy-select="(v: boolean) => (selectFilterRef.proxy = v)"
          @update:on-account-select="(v: string) => (selectFilterRef.account = v)"
          @update:on-tag-select="(v: string) => (selectFilterRef.tag = v)"
          @update:on-group-select="(v: string) => (selectFilterRef.group = v)"
          @update:reset="resetSelectFilterHandle"
        />

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <AddCheck
            class="w-[14px] h-[14px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans text-center text-black"> 批量检测代理 </span>
        </button>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          @click="router.push('/single-new-proxy')"
        >
          <AddProxy
            class="w-[14px] h-[14px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans text-center text-black"> 添加代理 </span>
        </button>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 bg-[#4F46E5] text-white outline-gray-50 hover:outline-gray-100"
        >
          <Buy class="mr-2 w-[11.67px] h-[11.67px]" />
          购买代理
        </button>
      </div>
    </div>

    <div class="flex flex-col">
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
    </div>

    <div class="flex overflow-auto flex-col grow">
      <slot name="manage-proxy-content"></slot>
    </div>
  </div>
</template>
