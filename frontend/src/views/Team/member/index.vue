<template>
  <div class="p-4 h-main bg-[#f5f6fa]">
    <div class="bg-white p-6 flex flex-col rounded-lg h-full">
      <!-- Header -->
      <div class="flex justify-between items-center mb-6">
        <div class="flex gap-4">
          <button
            class="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700 transition-colors"
            @click="() => { addMemModel = true, memberObj = null }">
            <UserPlusIcon class="w-5 h-5" />
            添加成员
          </button>
          <!-- <button @click="invMember = true"
            class="border border-blue-600 text-blue-600 px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-50">
            <PlusIcon class="w-5 h-5" />
            邀请成员
          </button> -->
        </div>
        <div class="flex items-center gap-4">
          <div class="relative">
            <!-- 触发器 -->
            <!-- <div
              class="appearance-none bg-[#f5f6fa] border border-gray-300 rounded-lg px-4 py-2 w-[200px] cursor-pointer hover:ring-2 hover:ring-blue-500"
              @click="toggleDropdown">
              {{ selectedGroup || "请选择" }}
              <ChevronDownIcon class="w-5 h-5 absolute right-2 top-2.5 text-gray-500" />
            </div> -->
            <!-- 下拉菜单 -->
            <More>
              <MoreTrigger>
                <div
                  class="appearance-none bg-[#f5f6fa] border border-gray-300 rounded-lg px-4 py-2 w-[200px] cursor-pointer hover:ring-2 hover:ring-blue-500 flex justify-between"
                  @click="toggleDropdown">
                  {{ selectedGroup || "请选择" }}
                  <ChevronDownIcon class="w-5 h-5 absolute right-2 top-2.5 text-gray-500" />
                </div>
              </MoreTrigger>
              <MoreContent>
                <MoreItem class="cursor-pointer w-[200px]" v-for="group in groups" :key="group.groupName"
                  @click="selectGroup(group.groupName)">
                  <Settings2Icon class="w-4 h-4" />
                  {{ group.groupName }}
                </MoreItem>
              </MoreContent>
            </More>
            <!-- <ul v-if="isOpen"
              class="absolute bg-white border border-gray-300 rounded-lg w-[200px] mt-1 shadow-lg z-20 p-5">
              <li v-for="group in groups" :key="group.groupName" @click="selectGroup(group.groupName)"
                class="px-4 py-2 hover:bg-[#e8edff] hover:text-blue-600 rounded-md cursor-pointer">
                {{ group.groupName }}
              </li>
            </ul> -->
          </div>
          <div class="relative">
            <input type="text" placeholder="请输入姓名" v-model="searchName"
              class="pl-10 pr-4 bg-[#f5f6fa] py-2 border border-gray-300 rounded-lg outline-none hover:ring-2 hover:ring-blue-500" />
            <SearchIcon class="w-5 h-5 absolute left-3 top-2.5 text-gray-400" />
          </div>
        </div>
      </div>

      <!-- Table -->
      <div class="flex-1 overflow-auto">
        <table class="w-full border-none flex-1">
          <thead class="bg-gray-50 sticky top-0 z-10">
            <tr>
              <th class="px-6 py-3 text-left text-sm font-medium text-gray-500 w-[100px]">
                序号
              </th>
              <th class="px-6 py-3 text-left text-sm font-medium text-gray-500">
                姓名
              </th>
              <th class="px-6 py-3 text-left text-sm font-medium text-gray-500">
                分组
              </th>
              <th class="px-6 py-3 text-left text-sm font-medium text-gray-500">
                备注
              </th>
              <th class="px-6 py-3 text-left text-sm font-medium text-gray-500">
                邮箱
              </th>
              <!-- <th class="px-6 py-3 text-left text-sm font-medium text-gray-500">
                授权环境
              </th> -->
              <th class="px-6 py-3 text-left text-sm font-medium text-gray-500">
                操作
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(member, index) in filterMember" :key="member.id" :class="{
              'border-t border-gray-100': true,
            }">
              <td class="px-6 py-4 text-sm text-gray-900">{{ index + 1 }}</td>
              <td class="px-6 py-4 text-sm text-gray-900">{{ member.nickname }}</td>
              <td class="px-6 py-4 text-sm text-gray-900"> {{ member.group_name }} </td>
              <td class="px-6 py-4 text-sm text-gray-900"> {{ member.description || '\\' }} </td>
              <td class="px-6 py-4 text-sm text-gray-900"> {{ member.email }} </td>
              <!-- <td class="px-6 py-4 text-sm text-gray-900">{{ member.environment }}</td> -->
              <td class="px-6 py-4 text-sm flex items-center">
                <!-- <button @click="openDetail(member)" class="text-blue-600 hover:text-blue-800 mr-4">
                  详情
                </button> -->
                <button @click="editMember(member)" class="text-blue-600 hover:text-blue-800 mr-4">
                  编辑
                </button>
                <More>
                  <MoreTrigger>
                    <button class="text-gray-400 hover:text-gray-600 flex items-center">
                      <MoreVerticalIcon class="w-6 h-6" />
                    </button>
                  </MoreTrigger>
                  <MoreContent>
                    <MoreItem class="cursor-pointer" @click="forbidMethod(member)">
                      <Settings2Icon class="w-4 h-4" />
                      禁用
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="delMember(member)">
                      <SquarePenIcon class="w-4 h-4" />
                      删除用户
                    </MoreItem>
                  </MoreContent>
                </More>
              </td>
            </tr>
          </tbody>
          <!-- <table v-if="members.length === 0" class="w-[1000px] h-[300px]"> -->

          <!-- </table> -->
        </table>
      </div>
      <!-- <div class="flex items-center justify-center w-full h-full" v-if="members?.length === 0">
        数据为空，没有成员
      </div> -->
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
    </div>

    <!-- Detail Slide Panel -->
    <Transition enter-active-class="transform transition ease-in-out duration-300" enter-from-class="translate-x-full"
      enter-to-class="translate-x-0" leave-active-class="transform transition ease-in-out duration-300"
      leave-from-class="translate-x-0" leave-to-class="translate-x-full" class="z-20">
      <div v-if="selectedGroupMember"
        class="fixed inset-y-0 right-0 w-[600px] bg-white shadow-xl overflow-y-auto flex flex-col">
        <div class="p-6 flex-1 flex flex-col">
          <div class="flex justify-between items-center mb-6">
            <h2 class="text-xl font-medium">成员详情</h2>
            <button @click="selectedGroupMember = false">
              <XIcon class="w-6 h-6 text-gray-400 hover:text-gray-600" />
            </button>
          </div>

          <div class="flex items-center gap-4 mb-8">
            <div
              class="w-12 h-12 bg-gradient-to-br from-blue-400 to-blue-600 rounded-lg flex items-center justify-center">
              <FingerprintIcon class="w-8 h-8 text-white" />
            </div>
            <div>
              <h3 class="font-medium">指纹浏览器</h3>
              <span class="text-sm text-gray-500">指纹浏览器</span>
            </div>
          </div>

          <div class="space-y-6 flex flex-col flex-1">
            <div class="grid grid-cols-2 gap-6">
              <div class="h-[100px]">
                <label class="block text-sm font-medium text-gray-700 mb-1">备注名</label>
                <div class="text-sm">
                  {{ selectedMember.nickname || "\\" }}
                </div>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">姓名</label>
                <div class="text-sm">{{ selectedMember.nickname }}</div>
              </div>
              <div class="h-[100px]">
                <label class="block text-sm font-medium text-gray-700 mb-1">分组</label>
                <div class="text-sm">{{ selectedMember.group_name }}</div>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">邮箱</label>
                <div class="text-sm">{{ selectedMember.email }}</div>
              </div>
            </div>

            <div class="w-[130px] rounded-md shadow-sm p-[5px] bg-[#f5f6f7]">
              <div class="px-4 py-2 text-blue-600 font-medium bg-white">
                已授权环境
              </div>
            </div>
            <div class="bg-gray-50 rounded-lg p-6 !mt-0 flex-1 flex flex-col justify-between">
              <div class="flex flex-col flex-1">
                <!-- Table Header -->
                <div class="grid grid-cols-2 border-b border-gray-200 pb-3">
                  <div class="text-gray-600 font-medium pl-[10px]">环境名称</div>
                  <div class="text-gray-600 font-medium">账号</div>
                </div>

                <!-- Table Content -->
                <ul class="bg-white h-full mt-[20px]">
                  <li class="grid grid-cols-2 py-4 border-b border-gray-100">
                    <div class="text-gray-700 pl-[10px]">新建环境1</div>
                    <div class="text-gray-500">未绑定</div>
                  </li>
                </ul>

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
            </div>
          </div>
        </div>
      </div>
    </Transition>
    <!-- 添加成员 -->
    <addMember v-model:addMemModel="addMemModel" v-model:memberObj="memberObj" />
    <!-- 邀请成员 -->
    <InvMember v-model:invMember="invMember" />
    <!-- 禁用成员 -->
    <AlertModel :title="'禁用成员'" :open="forbidMem" @close="forbidMem = false" @cancel="forbidMem = false"
      @submit="subForbid">
      确定禁用成员 "{{ memberObj.name }}" 吗
    </AlertModel>
    <!-- 删除成员 -->
    <AlertModel :title="'删除成员'" :open="delM" @close="delM = false" @cancel="delM = false" @submit="subDel">
      确定删除成员 "{{ memberObj.name }}" 吗
    </AlertModel>
  </div>
</template>

<script setup>
import InvMember from "./InvMember.vue"; //邀请成员
import addMember from "./addMember.vue"; //添加成员
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
import { ref, computed, onMounted, onBeforeUnmount, reactive, watch } from "vue";
import {
  UserPlusIcon,
  PlusIcon,
  ChevronDownIcon,
  SearchIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  XIcon,
  FingerprintIcon,
  MoreVerticalIcon
} from "lucide-vue-next";
import { useRoute } from "vue-router";
import { AlertModel } from "@/components/alert-model";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { user_receive_query } from "@/commands/user-team-temp";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { query_team_all_user, team_query, query_team_group_all_user, query_current_team_info, remove_current_user, blocked } from "@/commands/team"
import { team_group_query_all } from "@/commands/team-group";
import { toast } from "vue-sonner";

const route = useRoute()
const selectedMember = ref(null);
const selectedGroupMember = ref();
const searchName = ref("");
const isOpen = ref(false);
const selectedGroup = ref("");
const addMemModel = ref(false); //添加成员
const invMember = ref(false)  //邀请成员
const forbidMem = ref(false) //禁用成员
const delM = ref(false)  //删除成员
const pagination = reactive({
  pageIndex: 0,
  pageSize: 16,
  total: 0,
});

const toggleDropdown = () => {
  //打开选项
  // isOpen.value = !isOpen.value;
  selectFlag.value = 1; //开启下拉框标志

};
const selectGroup = (groupName) => {
  //选择选项
  selectedGroup.value = groupName;
  if (groupName === '全部')
    selectedGroup.value = ''
};
const memberObj = ref()
const forbidMethod = (member) => {  //禁用
  forbidMem.value = true
  memberObj.value = member
}
const delMember = (member) => {  //删除
  memberObj.value = member
  delM.value = true
}
const subForbid = () => {  //确定禁用
  forbidMem.value = false
  console.log("memberObj:", memberObj.value);

  blocked(memberObj.value.user_uuid, memberObj.value.team_id).then(res => {
    if (res.data) {
      toast.success('拉黑成功')
      getList()
    } else {
      toast.error("失败失败失败！！！")
    }
  })
}
const subDel = () => { //确认删除
  delM.value = false

  console.log("member:", memberObj.value);

  remove_current_user(memberObj.value.user_uuid, memberObj.value.team_id).then(res => {
    if (res.data) {
      toast.success('删除成功')
      getList()
    } else {
      toast.error("失败失败失败！！！")
    }
  })
}
const members = ref([
  {
    id: 1,
    name: "刘芳",
    group: "管理者",
    groupId: 1,
    contact: "18762736836",
    environment: "/",
    environments: [{ name: "新建环境1", account: "未绑定" }], note: "\\"
  },
  {
    id: 2,
    name: "李国强",
    group: "编者",
    groupId: 2,
    contact: "1526738273@qq.com",
    environment: "/",
    environments: [{ name: "新建环境1", account: "未绑定" }], note: "\\"
  },
]);
const memberTotal = ref(members.value.length)
const groups = ref([
  { id: 0, groupName: "全部" },
  { id: 1, groupName: "管理组" },
  { id: 2, groupName: "权限组" },
  { id: 3, groupName: "编辑组" },
  { id: 4, groupName: "默认组" },
]);
//过滤成员
const filterMember = computed(() => {

  return members.value?.filter((member) => {
    // return true
    const selectGroupQuery = selectedGroup.value === "" || selectedGroup.value === member.group_name
    const selectVal = member.nickname?.includes(searchName.value)
    return selectGroupQuery && selectVal
  });
});

const openDetail = (member) => {
  selectedMember.value = member;
  selectedGroupMember.value = true;
};
//编辑
const editMember = (member) => {
  memberObj.value = member
  addMemModel.value = true
}
watch(() => addMemModel.value, (val) => {  //监听使数据更新后刷新页面
  if(!val){
    getList()
  }
})

const getList = () => {
  //查询用户团队
  query_current_team_info().then(res => {
    if (route.query.groupId !== undefined) {
      //查询分组下成员
      query_team_group_all_user(res.data.id, +route.query.groupId, pagination.pageIndex, pagination.pageSize).then(res => {
        members.value = res.data.data
        pagination.total = res.data.total
      })
    } else {
      //查询该团队下所有成员
      query_team_all_user(res.data.id, pagination.pageIndex, pagination.pageSize).then(res2 => {
        console.log("aaa:", res2.data.data);

        members.value = res2.data.data
        pagination.total = res.data.total
        console.log("?????:", pagination);
        
      })
    }
    //查询分组
  })
}

onMounted(async () => {
  getList()
})
</script>
