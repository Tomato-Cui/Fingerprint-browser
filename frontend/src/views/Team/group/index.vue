<template>
  <div class="p-4 bg-[#f5f6fa] h-main">
    <div class="bg-white p-6 rounded-xl flex flex-col h-full">
      <!-- Header -->
      <div class="flex justify-between items-center mb-6">
        <div class="flex items-center gap-3">
          <button @click="() => { showAddModal = true; groupObj = null }"
            class="flex items-center gap-2 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors bg-primary">
            <FolderPlusIcon class="w-5" />
            创建分组
          </button>

          <!-- <TooltipButton title="删除" @click="() => console.log('abc')">
            <IconTeamGroupRefresh class="w-[25px] h-[25px]" />
          </TooltipButton> -->
          <Tooltip class="relative">
            <TooltipTrigger>
              <div
                class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
                :class="{ 'cursor-not-allowed opacity-50': selectIds.size === 0 }"
                @click="selectIds.size !== 0 ? authPri() : null">
                <KeyRound class="w-[25px] h-[25px] " />
              </div>
            </TooltipTrigger>
            <TooltipContent
              class="absolute mt-[50px] w-[55px] -translate-x-1/2 bg-gray-800 text-white text-sm px-3 py-1 rounded shadow-lg">
              <p>授权</p>
            </TooltipContent>
          </Tooltip>
        </div>

        <div class="relative">
          <input type="text" v-model="searchQuery" placeholder="请输入分组名称"
            class="pl-10 pr-4 py-2 w-64 rounded-lg border border-gray-200 hover:ring-2 hover:ring-blue-500 bg-[#f9f9f9] focus:ring-2 focus:ring-blue-500 focus:outline-none" />

          <SearchIcon class="w-5 h-5 text-gray-400 absolute left-3 top-2.5" />
        </div>

      </div>

      <!-- <div class="flex flex-col justify-between flex-1"> -->
      <!-- Table -->
      <!-- <div class="bg-white rounded-lg shadow"> -->
      <!-- Table Header -->
      <div class="grid grid-cols-6 px-6 py-3 bg-gray-50 text-base text-gray-500 h-[60px] sticky top-0 z-10">
        <div class="flex items-center justify-center">
          <input type="checkbox" class="rounded border-gray-300" v-model="selectAll" @change="toggleSelectAll"
            :indeterminate="isIndeterminate" :checked="isChecked">
        </div>
        <div class="">分组</div>
        <div class="">成员数量</div>
        <div class="">备注</div>
        <div class="">操作</div>
        <div class="">更多</div>
      </div>

      <!-- Table Body -->
      <div class="divide-y overflow-auto flex-1">
        <div v-for="(group, index) in filteredGroups" :key="group.id"
          class="grid grid-cols-6 px-6 py-4 items-center text-sm hover:bg-custom-light-blue" :class="{
            'border-t border-gray-100': true,
            'bg-blue-50 hover:bg-blue-100': group.selected,
            'hover:bg-blue-100': !group.selected,
          }">
          <div class="p-4 text-center">
            <div class="flex justify-center">
              <input type="checkbox" class="rounded border-gray-300" v-model="group.selected"
                :disabled="group.is_delete === 0" @change="selectGroup(group)" />
            </div>
          </div>
          <div class="text-gray-900 ">
            {{ group.name }}
          </div>
          <div class="text-gray-600 ">
            {{ group.member_count || 0 }}
          </div>
          <div class="text-gray-600 ">
            {{ group.description || "\\" }}
          </div>
          <div class="flex gap-2">
            <button
              class="flex items-center gap-1 text-blue-500 border border-blue-500 px-3 py-1 rounded-lg hover:bg-blue-50 transition-colors"
              @click="addMem(group)">
              <PlusIcon class="w-4 h-4" />
              添加成员
            </button>
          </div>
          <div class="flex items-center">
            <button class="text-blue-500 hover:underline" @click="toAllMem(group.id)">查看成员</button>
            <More>
              <MoreTrigger>
                <button class="text-gray-400 hover:text-gray-600 ml-5 flex items-center" @click="toggleTooltip">
                  <MoreVerticalIcon class="w-6 h-6" style="line-height: 10px;" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem class="cursor-pointer flex items-center" @click="editGroup(group)">
                  <Settings2 class="w-4 h-4" />编辑分组
                </MoreItem>
                <MoreItem class="cursor-pointer flex items-center" v-if="group.is_delete !== 0"
                  @click="delGroup(group)">
                  <Settings2 class="w-4 h-4" />删除分组
                </MoreItem>
              </MoreContent>
            </More>
          </div>
        </div>
      </div>
      <!-- </div> -->

      <!-- Pagination -->
      <div class="flex justify-between items-center mt-4 px-2">
        <div class="text-sm text-gray-500">共 {{ groupTotal }} 项数据</div>
        <div class="flex items-center gap-2">
          <!-- Previous button -->
          <button @click="handlePageChange(currentPage - 1)" :disabled="currentPage === 1"
            class="w-[100px] px-4 py-2 border rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:hover:bg-white">
            ←
          </button>

          <!-- Page numbers -->
          <template v-for="number in displayedPages" :key="number">
            <button v-if="number !== '...'" @click="handlePageChange(number)" :class="[
              'w-10 h-10 rounded-md border flex items-center justify-center',
              currentPage === number
                ? 'bg-blue-500 text-white border-blue-500'
                : 'hover:bg-gray-50',
            ]">
              {{ number }}
            </button>
            <span v-else class="w-10 h-10 flex items-center justify-center">
              ...
            </span>
          </template>

          <!-- Next button -->
          <button @click="handlePageChange(currentPage + 1)" :disabled="currentPage === totalPages"
            class="w-[100px] px-4 py-2 border rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:hover:bg-white">
            →
          </button>
        </div>
      </div>
      <!-- </div> -->
    </div>
  </div>

  <!-- 添加分组 -->
  <addGroup v-model:showAddModal="showAddModal" v-model:groupObj="groupObj" />
  <!-- 授权 -->
  <auth v-model:authModel="authModel" />
  <!-- 添加成员 -->
  <addMember v-model:addMemModel="addMemModel" v-model:groupObj="groupObj" />
  <!-- 删除分组 -->
  <Teleport to="body" class="z-30">
    <div class="fixed inset-0 bg-black/20 flex items-center justify-center z-30" v-if="clickDel">
      <div class="bg-white rounded-lg w-[400px]">
        <!-- Modal Header -->
        <div class="flex justify-between items-center p-4">
          <h2 class="text-lg font-medium">提示</h2>
          <button @click="clickDel = false" class="text-gray-400 hover:text-gray-600">
            <XIcon class="w-5 h-5" />
          </button>
        </div>
        <p class="px-[40px]">确认删除分组“{{ groupObj.name }}”吗？</p>

        <!-- Modal Footer -->
        <div class="flex justify-center space-x-3 p-4">
          <button @click="handleSubmitDel" class="px-8 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
            确定
          </button>
          <button @click="clickDel = false" class="px-8 py-2 border rounded-md hover:bg-gray-50">
            取消
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import addGroup from "./addGroup.vue";
import auth from "./auth.vue";
import addMember from "./addMember.vue";
import { ref, computed, watch, onMounted } from "vue";
import { More, MoreContent, MoreItem, MoreTrigger } from '@/components/more'
import {
  SearchIcon,
  FolderPlusIcon,
  PlusIcon,
  MoreVerticalIcon,
  KeyRound,
  ChevronLeftIcon,
  Settings2,
  ChevronRightIcon,
} from "lucide-vue-next";
import { IconTeamGroupRefresh } from "@/assets/icons";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useRouter } from "vue-router";
import { buttonVariants } from "@/components/ui/button";
import { team_query } from "@/commands/team";

const router = useRouter()
const searchQuery = ref("");
const showAddModal = ref(false); //添加分组
const authModel = ref(false); //授权
const addMemModel = ref(false); //添加成员
const clickDel = ref(false); //点击删除
const selectAll = ref(false) //全选
const groups = ref([   //分组列表    is_delete----是否可删除
  { id: 1, name: "管理者", memberCount: 12, note: "\\", is_delete: 1, },
  { id: 2, name: "编辑者", memberCount: 23, note: "跨境电商", is_delete: 1, },
  { id: 3, name: "查看者", memberCount: 12, note: "\\", is_delete: 1, },
  { id: 4, name: "超级管理员", memberCount: 23, note: "广告传媒", is_delete: 1, },
  { id: 5, name: "普通分组", memberCount: 22, note: "测试", is_delete: 0, },
  { id: 6, name: "普通分2", memberCount: 22, note: "测试", is_delete: 0, },
]);
const groupTotal = ref(groups.value.length)
const isTooltipVisible = ref(false); // 控制 Tooltip 的可见性

const toggleTooltip = () => {
  isTooltipVisible.value = !isTooltipVisible.value; // 切换 Tooltip 显示状态
  console.log("isTool----:", isTooltipVisible.value);
};
const filteredGroups = computed(() => {
  return groups.value.filter((group) =>
    group.name?.includes(searchQuery.value)  //分组名称筛选
  );
});
const props = defineProps({ //分页
  totalPages: {  //总页数
    type: Number,  //类型
    required: true,  //必填
    default: 20,  //默认值
  },
  initialPage: {  //初始页
    type: Number,  //类型
    default: 1,  //默认值
  },
  maxPageNumbers: {  //最大显示页数
    type: Number,  //类型
    default: 5,  //默认值
  },
});

watch(() => showAddModal.value, async (newVal) => {
});

const emit = defineEmits(["page-change"]);  //分页

const currentPage = ref(props.initialPage);  //当前页
const totalPages = ref(props.totalPages); // 本地存储总页数

const displayedPages = computed(() => {  //显示页数
  const pages = []; // 用于存放要显示的页码
  const maxPageNumbers = props.maxPageNumbers;  //最大显示页数

  pages.push(1);  //第一页

  if (currentPage.value <= Math.ceil(maxPageNumbers / 2)) { //当前页小于最大显示页数的一半
    for (let i = 2; i <= Math.min(maxPageNumbers, totalPages.value); i++) { //显示2到最大显示页数
      pages.push(i); //添加页数
    }
    if (totalPages.value > maxPageNumbers) { //总页数大于最大显示页数
      pages.push("...");  //添加...
      pages.push(totalPages.value);  //添加总页数
    }
  } else if (currentPage.value >= totalPages.value - Math.floor(maxPageNumbers / 2)) { //当前页大于等于总页数减去最大显示页数的一半
    pages.push("...");//添加...
    for (let i = totalPages.value - maxPageNumbers + 1; i <= totalPages.value; i++) {  //显示总页数减去最大显示页数加1到总页数
      pages.push(i);  //添加页数
    }
  } else { //当前页在中间
    pages.push("..."); //添加...
    for (let i = currentPage.value - Math.floor(maxPageNumbers / 2); i <= currentPage.value + Math.floor(maxPageNumbers / 2); i++) {  //显示当前页减去最大显示页数的一半到当前页加上最大显示页数的一半
      pages.push(i);  //添加页数
    }
    pages.push("...");  //添加...
    pages.push(totalPages.value);  //添加总页数
  }

  return pages;
});

const handlePageChange = async (page) => {
  if (page < 1 || page > totalPages.value || page === currentPage.value) {
    return;
  }
  currentPage.value = page;
  emit("page-change", page);

  selectAll.value = false  //全选置空
};

//添加成员
const addMem = (group) => {
  groupObj.value = group
  // console.log(groupObj.value.name + "-----" + group.name);
  addMemModel.value = true;
};

const groupObj = ref(null)
//编辑分组
const editGroup = (group) => {
  showAddModal.value = true  //打开添加分组弹出框
  groupObj.value = group
};
const delGroup = (group) => {  //删除分组
  groupObj.value = group
  clickDel.value = true;
};
const handleConfirm = () => {
  //新建分组
  console.log("新建分组");
  showAddModal.value = false;
};
//关闭删除提示
const handleSubmitDel = () => {
  // groups.value.filter((group) => group.id === delId.value)
  clickDel.value = false
  groups.value = groups.value.filter((group) => group.id !== groupObj.value.id)  //模拟删除
  filteredGroups.value = filteredGroups.value.filter((group) => group.id !== groupObj.value.id)
}
const toAllMem = (groupId) => {
  router.push({ path: "/team/member", query: { groupId: groupId } })
}

const selectIds = ref(new Set()); // 选中分组的id集合
//单选
const selectGroup = (group) => {
  // 检查group.id是否存在于Set中
  const isIn = selectIds.value.has(group.id);

  if (isIn) {
    // 如果存在，删除
    selectIds.value.delete(group.id);
  } else {
    // 如果不存在，添加
    selectIds.value.add(group.id);
  }
  // console.log([...selectIds.value]); // 打印当前选中的id数组
  // console.log("----:", selectIds.value.size);

  if (selectIds.value.size > 0 && selectIds.value.size < groups.value.length - 4) {  //全选时修改半选中状态
    isIndeterminate.value = true
  } else {
    isIndeterminate.value = false
    console.log(selectIds.value.size === groups.value.length - 4);

    if (selectIds.value.size === groups.value.length - 4) {  //全选时修改选中状态
      isChecked.value = true
    } else {
      isChecked.value = false
    }
  }
};

//全选
const toggleSelectAll = () => {
  groups.value.forEach(group => {
    if (group.is_delete !== 0) {
      group.selected = selectAll.value
      selectIds.value.add(group.id)
    }
  })
  if (!selectAll.value) {
    selectIds.value.clear()
    isIndeterminate.value = false
    isChecked.value = false
  } else {
    isIndeterminate.value = false
    isChecked.value = true
  }

}
// 计算属性：是否为半选状态  selectIds.value.length > 0 && selectIds.value.length < groups.value.length - 4
const isIndeterminate = ref(false);  //半选中状态
const isChecked = ref(false)  //选中状态
//授权
const authPri = () => {
  authModel.value = true;
  // console.log("-=-=-:", selectIds.value);

}

onMounted(async () => {
  team_query(currentPage, 10).then(res => {
    console.log("res----:", res);
    groups.value = res
    console.log("groups----:", groups.value);
    
  })
})
</script>

<style scoped>
.relative {
  background: rgba(249, 249, 249, 1), rgba(255, 255, 255, 1);
}
</style>
