<script setup lang="ts">
import { cn } from "@/util/lib";
import { Search, UserCogIcon, UserXIcon } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import Input from "@/components/input.vue";
import {
  IconAddUser,
  IconRefresh,
  IconActiveUser,
  IconBlockUser,
} from "@/assets/icons/team/index.ts";
import SelectFilter from "./components/select-filter.vue";
import { onMounted, reactive, ref, watch } from "vue";
import {
  blocked,
  query_team_all_user,
  un_blocked,
  remove_current_user,
} from "@/commands/team";
import Table from "./components/table-main.vue";
import TooltipButton from "@/components/tooltip-button.vue";
import { useProfileStore } from "@/stores/profile";
import InviteMembers from "./components/invite-members.vue";
import SettingModel from "./components/setting-model.vue";
import { toast } from "vue-sonner";
import MemberSettingModel from "./components/member-setting-model.vue";

const cols = [
  { id: 1, title: "登陆邮箱" },
  { id: 3, title: "昵称" },
  { id: 4, title: "角色" },
  { id: 4, title: "已授权环境" },
  { id: 6, title: "状态" },
  { id: 2, title: "操作" },
];

const store = useProfileStore();
const data = ref<any[]>([]);
const dataCache = ref<any[]>([]);
const selected = ref<any[]>([]);
const inviteMemberModelOpen = ref(false);
const teamettingModelOpen = ref(false);
const memberSettingModel = ref({
  open: false,
  info: {},
});
const pagination = reactive({
  total: 100,
  pageNum: 1,
  pageSize: 17,
});

const checks = ref([
  {
    id: "status",
    title: "全部状态",
    check: false,
  },
  {
    id: "enable",
    title: "启用",
    check: false,
  },
  {
    id: "disable",
    title: "禁用",
    check: false,
  },
]);

const loadData = () => {
  let teamInfo = store.getCurrentTeam();

  query_team_all_user(
    teamInfo!.id,
    pagination.pageNum,
    pagination.pageSize
  ).then((res) => {
    pagination.total = res.data.total;
    data.value = res.data.data;
    dataCache.value = JSON.parse(JSON.stringify(data.value));
  });
};

const updatePageSizeHandle = (pageS: number) => {
  pagination.pageSize = pageS;
};
const updatePageHandle = (pageS: number) => {
  pagination.pageNum = pageS;
};
const checkHandle = (k: string, v: boolean) => {
  checks.value = checks.value.map((item) => {
    if (item.id == k) {
      item.check = v;
    }
    return item;
  }) as any;
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
    item.email.toLowerCase().includes(sub.toLowerCase())
  );
  if (currents.length == 0) {
    data.value = [];
  } else if (sub.length == 0) {
    data.value = JSON.parse(JSON.stringify(dataCache.value));
  } else {
    data.value = currents;
  }
};

const batchHandles = {
  blockeds: () => {
    Promise.all([
      ...[...selected.value].map((item) => {
        [blocked(item, store.getCurrentTeam()!.id)];
      }),
    ]).then((_res) => {
      toast.success("禁用成功");
      loadData();
    });
  },
  unblockeds: () => {
    Promise.all([
      ...[...selected.value].map((item) => {
        [un_blocked(item, store.getCurrentTeam()!.id)];
      }),
    ]).then((_res) => {
      toast.success("解除禁用成功");
      loadData();
    });
  },
  removes: () => {
    Promise.all([
      ...[...selected.value].map((item) => {
        [remove_current_user(item, store.getCurrentTeam()!.id)];
      }),
    ]).then((_res) => {
      toast.success("删除成功");
      loadData();
    });
  },
};

onMounted(() => loadData());
watch(() => [pagination.pageNum, pagination.pageSize], loadData);
watch(checks, () => {
  let option1 = checks.value[0].check,
    option2 = checks.value[1].check,
    option3 = checks.value[2].check;
  if (option1) {
    data.value = dataCache.value;
  } else if (option2 && option3) {
    data.value = dataCache.value;
  } else if (option2) {
    data.value = dataCache.value.filter((item) => !item.blocked);
  } else if (option3) {
    data.value = dataCache.value.filter((item) => item.blocked);
  } else {
    data.value = dataCache.value;
  }
});
</script>

<template>
  <div class="flex flex-col h-full overflow-hidden">
    <div class="flex justify-between px-4">
      <div class="flex items-center justify-betwee gap-x-2 mt-1 mb-2">
        <h3 class="font-semibold tracking-tight text-xl flex items-end">
          团队
        </h3>
      </div>
      <div class="flex items-center justify-between mb-2 gap-4 select-none">
        <div class="flex items-center gap-2 flex-1 pt-1">
          <TooltipButton
            title="刷新"
            class="border rounded-md h-8 w-8 flex items-center justify-center"
          >
            <IconRefresh class="cursor-pointer" @click="() => loadData()" />
          </TooltipButton>

          <div class="relative flex-1 max-w-md">
            <Search
              class="absolute left-3 top-[50%] transform -translate-y-1/2 text-gray-400 h-4 w-4 z-10"
            />
            <Input
              class="pl-8 h-8 text-xs text-gray-500"
              placeholder="根据用户邮箱搜索"
              @input="(v:any) => searchHandle(v.target.value)"
            />
          </div>
          <Button
            variant="outline"
            @click="() => (teamettingModelOpen = true)"
            class="h-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]"
          >
            <UserCogIcon />
            团队设置</Button
          >
          <SelectFilter
            :data="dataCache"
            :checks="checks"
            @update:on-check="checkHandle"
          />
          <Button
            variant="outline"
            @click="() => (inviteMemberModelOpen = true)"
            :class="
              cn(
                'h-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]',
                'bg-[#5050FA] hover:bg-[#5558E3] hover:text-white text-white'
              )
            "
          >
            <IconAddUser />
            邀请成员</Button
          >
        </div>
      </div>
    </div>
    <div class="px-4 space-x-2 pb-2" v-if="selected.length > 0">
      <Button
        class="px-2 py-1 bg-[#5050FA] text-white hover:bg-[#5d5dff]"
        @click="batchHandles.unblockeds"
        ><IconActiveUser />启用成员</Button
      >
      <Button
        class="px-2 py-1 bg-[#EDEDFF80] text-slate-700 hover:bg-[#dfdffb80]"
        @click="batchHandles.blockeds"
        ><IconBlockUser />禁用成员</Button
      >
      <Button
        class="px-2 py-1 bg-[#FFE5E7] text-[#ED003F] hover:bg-[#ffdfe2]"
        @click="batchHandles.removes"
        ><UserXIcon /> 删除成员</Button
      >
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
      @action="
        (v:any) =>
          (memberSettingModel = {
            open: true,
            info:{...v},
          })
        "
    />
    <InviteMembers
      :open="inviteMemberModelOpen"
      @close="inviteMemberModelOpen = false"
    />
    <SettingModel
      :open="teamettingModelOpen"
      @close="teamettingModelOpen = false"
    />
    <MemberSettingModel
      :open="memberSettingModel.open"
      :current_user_info="memberSettingModel.info"
      @close="memberSettingModel.open = false"
      @reload="loadData"
    />
  </div>
</template>
