<template>
  <div class="p-4 bg-[#f5f6fa] h-main">
    <div class="bg-white p-6 rounded-xl flex flex-col h-full">
      <!-- Header -->
      <div class="bg-white rounded-lg py-4 mb-6">
        <!-- <div class="flex justify-between items-center"> -->

        <div
          class="relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none hover:ring-1 transform ease-in-out delay-150 duration-100 hover:ring-blue-500 ">
          <input v-model="selectVal" type="text"
            :placeholder="'请输入' + (selectType === 1 ? '名称' : selectType === 2 ? '备注' : '邮箱')"
            class="w-full pl-10 pr-4 py-2 rounded-lg border-gray-200 bg-[#f9f9f9] outline-none" />
          <SearchIcon class="w-5 h-5 text-gray-400 absolute left-3 top-2.5" />
          <More>
            <MoreTrigger>
              <button class="ml-4 p-2 hover:bg-gray-100 rounded-lg">
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
                <Trash2Icon class="w-4 h-4" />邮箱
              </MoreItem>
            </MoreContent>
          </More>
        </div>
      </div>
      <!-- </div> -->

      <!-- Table -->

      <!-- Table Header -->
      <div class="grid grid-cols-5 px-6 py-3 bg-gray-50 border-b text-sm text-gray-500">
        <div>姓名</div>
        <div>分组</div>
        <div>备注</div>
        <div>邮箱</div>
        <!-- <div>授权环境</div> -->
        <div>操作</div>
        <!-- <div></div> -->
      </div>

      <!-- <div class="flex flex-col flex-1 justify-between"> -->
      <!-- Table Body -->
      <div class="divide-y overflow-auto flex-1" v-if="users?.length !== 0">
        <div v-for="user in filterUsers" :key="user.id"
          class="grid grid-cols-5 px-6 py-4 items-center hover:bg-gray-50 hover:bg-custom-light-blue"
          :class="{ 'border-t border-gray-100': true }">
          <div class="text-gray-900">{{ user.nickname }}</div>
          <div class="text-gray-600">{{ user.group_name }}</div>
          <div class="text-gray-600">{{ user.description || '\\' }}</div>
          <div class="text-gray-600">{{ user.email }}</div>
          <!-- <div class="text-gray-400">/</div> -->
          <!-- <div class="text-gray-400">/</div>
          <div class="text-gray-400">/</div> -->
          <div class="flex gap-2 text-sm bg-red">
            <button class="text-blue-500 hover:underline" @click="recoverUser(user)">恢复</button>
          </div>
          <!-- <div></div> -->
        </div>
      </div>
      <div v-else class="flex-1">
        <div class="flex justify-center items-center w-full h-[80px] hover:bg-gray-50 translate ease-in-out delay-100 duration-100">
          数据为空，没有成员
        </div>
      </div>

      <!-- Pagination -->
      <div class="flex items-center justify-end space-x-2 py-1">
        <div class="flex-1 text-sm text-muted-foreground">
          共{{ pagination.total }}条.
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
      <!-- </div> -->
    </div>
  </div>
  <!-- </div> -->
  <!-- 启用消息弹出框 -->
  <AlertModel :title="'启用成员'" :open="alertEnable" @close="() => (alertEnable = false)"
    @cancel="() => (alertEnable = false)" @submit="subEnAbleUser">
    确定启用成员 "{{ userObj.nickname }}" 吗？
  </AlertModel>
  <!-- 删除消息弹出框 -->
  <AlertModel :title="'恢复成员'" :open="alertDelUser" @close="() => (alertDelUser = false)"
    @cancel="() => (alertDelUser = false)" @submit="subRecoverUser">
    确定恢复成员 "{{ userObj.nickname }}" 吗？
  </AlertModel>
</template>

<script setup>
import { ref, computed, watch, reactive } from "vue";
import {
  SearchIcon,
  ChevronDownIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  MoreVerticalIcon,
  SlidersHorizontalIcon,
  UserIcon,
} from "lucide-vue-next";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { Settings2Icon, SquarePenIcon, Trash2Icon } from "lucide-vue-next";
import { AlertModel } from "@/components/alert-model";
import { onMounted, defineProps, defineEmits } from "vue";
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
import { team_query, query_team_all_blocked_user } from "@/commands/team";
import { un_blocked } from '@/commands/team'
import { toast } from "vue-sonner";

const pagination = reactive({
  pageIndex: 1,
  pageSize: 16,
  total: 0,
});
const paginationClickHandle = (index) => {
  pagination.pageIndex = index;
};
const users = ref([
  {
    id: 1,
    name: "刘芳",
    group: "管理者",
    contact: "18762736836",
    note: "备注"
  },
  {
    id: 2,
    name: "李国强",
    group: "编辑者",
    contact: "1526738273@qq.com",
    note: "备注"
  }
  ,
  {
    id: 3,
    name: "WeiErLite",
    group: "编辑者",
    contact: "1526738273@qq.com",
    note: "备注"
  }
  ,
  {
    id: 4,
    name: "花姑娘",
    group: "编辑者",
    contact: "1526738273@qq.com",
    note: "备注"
  }
  ,
  {
    id: 5,
    name: "吆西",
    group: "编辑者",
    contact: "1526738273@qq.com",
    note: "备注"
  }
]);
const filterUsers = computed(() => {  //过滤数组
  return users.value.filter((user) => {
    if (selectType.value === 1) {
      return user.nickname?.includes(selectVal.value);
    } else if (selectType.value === 2) {
      return user.description?.includes(selectVal.value);
    } else {
      return user.email?.includes(selectVal.value);
    }
    return true
  })
})
const alertEnable = ref(false)  //启用成员
const alertDelUser = ref(false)  //删除成员
const userObj = ref()
const enAbleUser = (user) => { //启用提示框
  alertEnable.value = true
  userObj.value = user
}
const subEnAbleUser = () => { //确认启用成员
  alertEnable.value = false
  users.value = users.value.filter((user) => user.id !== userObj.value.id)
  filterUsers.value = filterUsers.value.filter((user) => user.id !== userObj.value.id)
}
const recoverUser = (user) => {  //恢复提示框
  alertDelUser.value = true
  userObj.value = user
}
const subRecoverUser = () => {  //确认恢复成员
  alertDelUser.value = false
  console.log("----!!!:", userObj.value);

  un_blocked(userObj.value.user_uuid, userObj.value.team_id).then(res => {
    if (res.data) {
      toast.success('恢复成功')
      getList()
    }
    else
      toast.error('恢复失败')
  })
  users.value = users.value.filter((user) => user.id !== userObj.value.id)
  filterUsers.value = filterUsers.value.filter((user) => user.id !== userObj.value.id)
  // console.log("value----:", filterUsers.value);
}
//搜索条件
const selectType = ref(1)
const selectVal = ref("")  //搜索值
//选择搜索条件
const selectT = (type) => {
  selectType.value = type
}

const total = ref(filterUsers.value.length)  //数据总条数
watch(() => filterUsers.value, (newVal) => {
  total.value = filterUsers.value.length
})

const getList = () => {
  team_query(1, 10).then(res => {
    query_team_all_blocked_user(res.data.data[0].id, pagination.pageIndex, pagination.pageSize).then(res => {
      users.value = res.data.data
      pagination.total = res.data.total
    })
  })
}

onMounted(async () => {
  getList()
});
</script>
