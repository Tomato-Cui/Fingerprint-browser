<template>
  <div class="p-4 bg-[#f5f6fa] h-main flex">
    <div class="flex flex-col flex-1 justify-between p-6 h-full bg-white rounded-xl">
      <!-- Header -->
      <div lass="bg-white rounded-lg p-4 m-[20px] h-[50px]">
        <div class="flex justify-between items-center mb-[20px]">
          <div class="relative flex-1 flex items-center gap-4 min-w-[500px] max-w-[1000px]">
            <!-- 搜索框 -->
            <div
              class="relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none hover:ring-1 hover:ring-blue-500">
              <input v-model="selectVal" type="text" :placeholder="'请输入' +
                (selectType === 1
                  ? '名称'
                  : selectType === 2
                    ? '备注'
                    : '手机号或邮箱')
                " class="w-full pl-10 pr-4 py-2 rounded-lg border-gray-200 bg-[#f9f9f9] outline-none" />
              <SearchIcon class="absolute top-2.5 left-3 w-5 h-5 text-gray-400" />
              <More>
                <MoreTrigger>
                  <button class="p-2 ml-4 rounded-lg hover:bg-gray-100">
                    <SlidersHorizontalIcon class="w-5 h-5 text-gray-500" />
                  </button>
                </MoreTrigger>
                <MoreContent>
                  <MoreItem class="cursor-pointer" @click="selectT(1)">
                    <Settings2Icon class="w-4 h-4" />名称
                  </MoreItem>
                  <MoreItem class="cursor-pointer" @click="selectT(2)">
                    <SquarePenIcon class="w-4 h-4" />备注
                  </MoreItem>
                  <MoreItem class="cursor-pointer" @click="selectT(3)">
                    <Trash2Icon class="w-4 h-4" />手机号/邮箱
                  </MoreItem>
                </MoreContent>
              </More>
            </div>

            <div class="relative">
              <More>
                <MoreTrigger>
                  <div
                    class="appearance-none bg-[#f5f6fa] border border-gray-300 rounded-lg px-4 py-2 w-[200px] cursor-pointer hover:ring-1 hover:ring-blue-500 flex items-center justify-between"
                    @click="toggleDropdown">
                    {{ selectedLabel || "请选择" }}
                    <ChevronDownIcon class="w-5 h-5 absolute right-2 top-2.5 text-gray-500" />
                  </div>
                </MoreTrigger>
                <MoreContent>
                  <MoreItem class="cursor-pointer w-[200px]" v-for="status in status" :key="status.label"
                    @click="selectStatus(status.label)">
                    <Settings2Icon class="w-4 h-4" />
                    {{ status.label }}
                  </MoreItem>
                </MoreContent>
              </More>
            </div>
          </div>
          <!-- <div class="flex gap-4 items-center ml-4"></div> -->
        </div>
      </div>

      <!-- Table -->
      <div class="flex overflow-auto flex-col flex-1 justify-between bg-white rounded-lg">
        <!-- Table Header -->
        <div class="grid grid-cols-6 px-6 py-3 bg-gray-50 text-sm text-gray-500">
          <div>姓名</div>
          <!-- <div>分组</div> -->
          <div>备注</div>
          <div>邮箱</div>
          <!-- <div>邀请人</div> -->
          <div>申请时间</div>
          <div>状态</div>
          <div>操作</div>
        </div>

        <!-- Table Body -->
        <div class="divide-y flex-1 flex flex-col overflow-auto" v-if="users?.length !== 0">
          <div v-for="user in filterUsers" :key="user.id"
            class="grid grid-cols-6 px-6 py-4 items-center hover:bg-gray-50 hover:bg-custom-light-blue"
            :class="{ 'border-t border-gray-100': true }">

            <div class="text-gray-900 text-ellipsis overflow-hidden whitespace-nowrap">{{ user.nickname }}</div>
            <!-- <div class="text-gray-600 text-ellipsis overflow-hidden whitespace-nowrap">{{ user.group_name }}</div> -->
            <div class="text-gray-600 text-ellipsis overflow-hidden whitespace-nowrap">{{ user.description }}</div>
            <div class="text-gray-600 text-ellipsis overflow-hidden whitespace-nowrap">{{ user.email }}</div>
            <!-- <div class="text-gray-600 text-ellipsis overflow-hidden whitespace-nowrap">{{ user.owner_name }}</div> -->
            <div class="text-gray-600 text-ellipsis overflow-hidden whitespace-nowrap">{{ user.created_at }}</div>
            <div>
              <span class="px-4 py-2 text-sm rounded-sm" :class="{
                'bg-emerald-50 text-emerald-500': user.allow_2 === 1,
                'bg-yellow-50 text-yellow-500': user.allow_2 === 0,
                'bg-red-50 text-red-500': user.allow_2 === -1,
              }">
                {{ getStatusText(user.allow_2) }}
              </span>
            </div>
            <div class="flex gap-2 text-sm">
              <template v-if="user.allow_2 === 0">
                <button class="text-blue-500 hover:underline" @click="agreeApplyM(user)">同意</button>
                <span class="text-gray-300">|</span>
                <button class="text-blue-500 hover:underline" @click="refApply(user)">拒绝</button>
              </template>
              <template v-else>
                {{ getActionText(user.allow_2) }}
              </template>
            </div>
          </div>
        </div>
        <div class="flex-1" v-else>
          <div
            class="flex justify-center items-center w-full h-[80px] hover:bg-gray-50 translate ease-in-out delay-100 duration-100">
            数据为空，没有成员
          </div>
        </div>
      </div>


      <!-- Pagination -->
      <div class="flex items-center justify-end space-x-2 py-1">
        <div class="flex-1 text-sm text-muted-foreground">
          共{{ pagination.total }}条
        </div>
        <div class="space-x-2">
          <Pagination :total="pagination.total" :itemsPerPage="pagination.pageSize" :default-page="1">
            <PaginationList v-slot="{ items }" class="flex items-center gap-1">
              <PaginationFirst @click="() => paginationClickHandle(0)" />
              <PaginationPrev @click="() => paginationClickHandle(pagination.pageIndex - 1)" />

              <template v-for="(item, index) in items">
                <PaginationListItem v-if="item.type === 'page'" :key="index" :value="item.value" as-child>
                  <Button class="w-10 h-10 p-0" @click="() => paginationClickHandle(index)" :variant="item.value === pagination.pageIndex + 1 ? 'default' : 'outline'
                    ">
                    {{ item.value }}
                  </Button>
                </PaginationListItem>
                <PaginationEllipsis v-else :key="item.type" :index="index" />
              </template>

              <PaginationNext @click="() => paginationClickHandle(pagination.pageIndex + 1)" />
              <PaginationLast @click="() =>
                paginationClickHandle(
                  Math.ceil(pagination.total / pagination.pageSize) - 1
                )
                " />
            </PaginationList>
          </Pagination>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch, reactive } from "vue";
import {
  SearchIcon,
  ChevronDownIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  SlidersHorizontalIcon,
} from "lucide-vue-next";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { Settings2Icon, SquarePenIcon, Trash2Icon } from "lucide-vue-next";
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
import { team_receive_query, team_allow, reject } from '@/commands/user-team-temp'
import { query_current_team_info } from "@/commands/team"
import { toast } from "vue-sonner";

//搜索条件
const selectType = ref(1);
const selectVal = ref(""); //搜索值
//选择搜索条件
const selectT = (type) => {
  selectType.value = type;
};
onMounted(() => { });
const pagination = reactive({
  pageIndex: 1,
  pageSize: 16,
  total: 0,
});
const paginationClickHandle = (index) => {
  loadData(Number(route.params.id), index, pagination.pageSize);
  pagination.pageIndex = index;
};
// 审核筛选区--------------------------------
const isOpen = ref(false); //审核筛选
const selectFlag = ref(0);
const selectedLabel = ref("待审核");
const status = ref([
  { id: 1, label: "待审核" },
  { id: 2, label: "已同意" },
  { id: 3, label: "已拒绝" },
]);

const toggleDropdown = () => {
  //打开选项
  // isOpen.value = !isOpen.value;
  selectFlag.value = 1; //开启下拉框标志
};
const seStuVal = ref(0)
const selectStatus = (label) => {
  //选择选项
  selectedLabel.value = label;
  seStuVal.value = label === '待审核' ? 0 : label === '已同意' ? 1 : -1  //当前数据审核状态
};


const users = ref([]);
const filterUsers = computed(() => {
  //过滤数组

  const selectQuery = ref(true);   //查询搜索值
  return users.value.filter((user) => {
    if (selectType.value === 1) {
      selectQuery.value = user.nickname?.includes(selectVal.value);
    } else if (selectType.value === 2) {
      selectQuery.value = user.description?.includes(selectVal.value);
    } else {
      selectQuery.value = user.email?.includes(selectVal.value);
    }

    const selectStatus = user.allow_2 === seStuVal.value;  //查询状态    
    return selectQuery.value && selectStatus;
  });
});
const userTotal = ref(filterUsers.value.length)   //数据条数
watch(() => filterUsers.value, () => {
  userTotal.value = filterUsers.value.length
})

const getStatusText = (status) => {
  const stateMenu = ['已拒绝', '待审核', '已同意']

  return stateMenu[status + 1];
};

const getActionText = (status) => {
  const stateMenu = ['已同意', '已拒绝']
  return stateMenu[status];
};

//同意申请
const agreeApplyM = (user) => {
  user.status = 1
  team_allow(user.id, user.user_uuid, user.team_id).then(res => {
    toast.message(res.message)
    getList()
  })
}
//拒绝申请
const refApply = (user) => {
  user.status = -1
  reject(user.id).then(res => {
    toast.message(res.message)
    getList()
  })
}

const getList = () => {
  query_current_team_info().then(res => {
    team_receive_query(res.data.id, pagination.pageIndex, pagination.pageSize).then(res => {
      // users.value = res.data.data
      users.value = res.data.data.map(item => {
        // 判断delete_at是否为空，若不为空，则将allow_2设置为-1
        if (item.deleted_at) {
          item.allow_2 = -1;
        }
        return item;
      })
      pagination.total = res.data.total
    })
  })
}

onMounted(() => {
  getList()
})
</script>
