<template>
  <Teleport to="body" class="z-20">
    <div
      v-if="props.addMemModel"
      class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-30"
    >
      <div class="flex fixed inset-0 justify-center items-center bg-black/20">
        <div class="bg-white rounded-lg w-full min-w-[500px] max-w-[800px]">
          <!-- Modal Header -->
          <div class="flex justify-between items-center p-4 border-b">
            <h2 class="text-lg font-medium">
              {{ props.memberObj !== null ? "修改成员" : "添加成员" }}
            </h2>
            <button
              @click="closeModal"
              class="text-gray-400 hover:text-gray-600"
            >
              <XIcon class="w-5 h-5" />
            </button>
          </div>

          <!-- Modal Content -->
          <div class="p-6">
            <form @submit.prevent="handleSubmit" class="space-y-6">
              <!-- Login Account -->
              <div class="flex items-start" v-if="!props.memberObj">
                <label class="flex justify-end pt-2 pr-3 w-24">
                  <span class="text-red-500">*</span> 账号:
                </label>
                <div class="flex flex-col flex-1">
                  <input
                    v-model="formData.account"
                    type="text"
                    placeholder="请输入用户邮箱"
                    class="flex-1 px-3 py-2 rounded border focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                    @input="validateEmail"
                  />
                  <p v-if="emailError" style="color: red">{{ emailError }}</p>
                </div>
              </div>

              <!-- Remark Name -->
              <div class="flex items-start">
                <label class="flex justify-end pt-2 pr-3 w-24">
                  <span class="text-red-500">*</span> 备注:
                </label>
                <input
                  v-model="formData.remarkName"
                  type="text"
                  placeholder="请输入备注名"
                  class="flex-1 px-3 py-2 rounded border focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>

              <!-- Group Name -->
              <div class="flex items-start" v-if="props.memberObj">
                <label class="flex justify-end pt-2 pr-3 w-24">
                  <span class="text-red-500">*</span> 分组名称:
                </label>
                <div class="flex-1">
                  <div class="flex gap-2">
                    <More>
                      <MoreTrigger class="flex-1">
                        <button
                          @click="searchGroup"
                          class="flex justify-between items-center px-4 py-2 w-full bg-white rounded-md border border-gray-300 hover:ring-2 hover:ring-blue-500 focus:outline-none"
                          :class="{ 'text-gray-400': formData.groupId === 0 }"
                        >
                          {{ teamGroupName }}
                          <ChevronDownIcon class="inline-block ml-1 w-4 h-4" />
                        </button>
                      </MoreTrigger>
                      <MoreContent class="w-[240px]">
                        <MoreItem
                          v-for="group in teamGroup"
                          class="w-full cursor-pointer"
                          @click="formData.groupId = group.id"
                        >
                          <Settings2Icon class="w-4 h-4" />{{ group.name }}
                        </MoreItem>
                      </MoreContent>
                    </More>
                  </div>
                </div>
              </div>
            </form>
          </div>

          <!-- Modal Footer -->
          <div class="flex justify-end p-4 space-x-3 border-t">
            <button
              @click="closeModal"
              class="px-6 py-2 rounded border hover:bg-gray-50"
            >
              取消
            </button>
            <button
              type="submit"
              @click="handleSubmit"
              class="px-6 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
            >
              确定
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>

  <!-- class: 这里class的宽度就是弹出框的宽度.请自行设置 -->
  <Model
    class="min-w-[868px]"
    :title="'授权环境'"
    :open="alertEnvironment"
    @close="handleClose"
  >
    <div class="flex flex-col px-2 pt-6 space-x-4">
      <!-- 在这里书写弹出框主题内容代码 -->
      <div class="max-w-3xl mx-[50px] p-6">
        <!-- <div class="mb-6">
          <h1 class="mb-2 text-lg font-medium">请选择需授权的环境</h1>
        </div> -->

        <div
          class="flex relative rounded-md hover:outline-none hover:ring-2 hover:ring-blue-500"
        >
          <div class="inline-block relative">
            <More>
              <MoreTrigger>
                <button
                  @click=""
                  class="w-[130px] px-4 py-2 bg-white border border-r-0 border-gray-300 rounded-l-md hover:bg-gray-50 focus:outline-none"
                >
                  {{
                    searchForm.selectType === 0
                      ? "环境名称"
                      : searchForm.selectType === 1
                      ? "IP地址"
                      : "备注"
                  }}
                  <ChevronDownIcon class="inline-block ml-1 w-4 h-4" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectType = 0"
                >
                  <Settings2Icon class="w-4 h-4" />环境名称
                </MoreItem>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectType = 1"
                >
                  <SquarePenIcon class="w-4 h-4" />IP地址
                </MoreItem>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectType = 2"
                >
                  <Trash2Icon class="w-4 h-4" />备注
                </MoreItem>
              </MoreContent>
            </More>
          </div>
          <input
            type="text"
            :placeholder="
              '请输入' +
              (searchForm.selectType === 0
                ? '环境名称'
                : searchForm.selectType === 1
                ? 'IP地址'
                : '备注')
            "
            class="flex-1 px-4 py-2 rounded-r-md border border-gray-300 focus:outline-none"
          />
        </div>

        <div class="flex gap-4 justify-between my-6">
          <div class="relative w-1/3">
            <More>
              <MoreTrigger class="w-full">
                <button
                  @click=""
                  class="flex justify-between items-center px-4 py-2 w-full bg-white rounded-md border border-gray-300 hover:ring-2 hover:ring-blue-500 focus:outline-none"
                >
                  {{
                    searchForm.selectIsAc === 0
                      ? "所有环境"
                      : searchForm.selectIsAc === 1
                      ? "已授权"
                      : "未授权"
                  }}
                  <ChevronDownIcon class="inline-block ml-1 w-4 h-4" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectIsAc = 0"
                >
                  <Settings2Icon class="w-4 h-4" />所有环境
                </MoreItem>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectIsAc = 1"
                >
                  <SquarePenIcon class="w-4 h-4" />已授权
                </MoreItem>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectIsAc = 2"
                >
                  <Trash2Icon class="w-4 h-4" />未授权
                </MoreItem>
              </MoreContent>
            </More>

            <!-- <ChevronDownIcon class="absolute right-2 top-1/2 w-5 h-5 text-gray-400 -translate-y-1/2" /> -->
          </div>

          <div class="relative w-1/3">
            <More>
              <MoreTrigger class="w-full">
                <button
                  @click=""
                  class="flex justify-between items-center px-4 py-2 w-full bg-white rounded-md border border-gray-300 hover:ring-2 hover:ring-blue-500 focus:outline-none"
                >
                  {{ optionName }}
                  <ChevronDownIcon class="inline-block ml-1 w-4 h-4" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectOption = 0"
                >
                  <Settings2Icon class="w-4 h-4" />所有操作类型
                </MoreItem>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectOption = 1"
                >
                  <SquarePenIcon class="w-4 h-4" />Windows
                </MoreItem>
                <MoreItem
                  class="cursor-pointer"
                  @click="searchForm.selectOption = 2"
                >
                  <Trash2Icon class="w-4 h-4" />Android
                </MoreItem>
              </MoreContent>
            </More>
            <!-- <ChevronDownIcon class="absolute right-2 top-1/2 w-5 h-5 text-gray-400 -translate-y-1/2" /> -->
          </div>

          <div class="relative w-1/3">
            <More>
              <MoreTrigger class="w-full">
                <button
                  @click=""
                  class="flex justify-between items-center px-4 py-2 w-full bg-white rounded-md border border-gray-300 hover:ring-2 hover:ring-blue-500 focus:outline-none"
                >
                  {{ envGroupName }}
                  <ChevronDownIcon class="inline-block ml-1 w-4 h-4" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem
                  v-for="group in envGroup"
                  class="cursor-pointer"
                  @click="searchForm.selectGroup = group.id"
                >
                  <Settings2Icon class="w-4 h-4" />{{ group.groupName }}
                </MoreItem>
              </MoreContent>
            </More>
            <!-- <ChevronDownIcon class="absolute right-2 top-1/2 w-5 h-5 text-gray-400 -translate-y-1/2" /> -->
          </div>
        </div>

        <!-- Environment List -->
        <div class="rounded-md border">
          <!-- Select All Header -->
          <div class="flex justify-between p-4 bg-gray-50 border-b">
            <label class="flex items-center">
              <input
                type="checkbox"
                :checked="isAllSelected"
                :indeterminate="isIndeterminate"
                @change="toggleSelectAll"
                class="w-4 h-4 text-blue-600 rounded border-gray-300 focus:ring-blue-500"
              />
              <span class="ml-2">全选</span>
            </label>
            <p class="text-sm text-gray-500">
              已选择 {{ selectedCount }} 个环境
            </p>
          </div>

          <!-- Environment Items -->
          <div class="divide-y">
            <div
              v-for="env in environments"
              :key="env.id"
              class="flex items-center p-4 hover:bg-gray-50"
            >
              <label class="flex flex-1 items-center">
                <input
                  type="checkbox"
                  v-model="selectedIds"
                  :value="env.id"
                  class="w-4 h-4 text-blue-600 rounded border-gray-300 focus:ring-blue-500"
                />
                <span class="flex items-center ml-2">
                  <GridIcon class="mr-2 w-4 h-4 text-gray-400" />
                  <span>{{ env.name }}</span>
                </span>
              </label>
              <span class="mx-4 text-gray-400">--</span>
              <span class="px-2 py-1 text-sm text-white bg-green-500 rounded"
                >动</span
              >
            </div>
          </div>
        </div>
      </div>

      <div class="mx-[50px] px-6 pl-[3.5rem]">
        <div class="flex justify-start py-8 gap-x-4 pb-[3.5rem]">
          <PrimaryButton class="px-8" @click="subSelectEnv">确定</PrimaryButton>
          <CancelButton class="px-8" @click="handleClose">取消</CancelButton>
        </div>
      </div>
    </div>
  </Model>
</template>

<script setup>
import {
  ref,
  watch,
  defineEmits,
  defineProps,
  computed,
  onMounted,
  reactive,
} from "vue";
import {
  XIcon,
  PlusIcon,
  SearchIcon,
  ChevronDownIcon,
  GridIcon,
  CircleX,
} from "lucide-vue-next";
import { useRouter } from "vue-router";
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { team_group_query_all } from "@/commands/team-group";
import {
  team_query,
  query_current_team_info,
  team_modify_team_user_info,
} from "@/commands/team";
import { team_send } from "@/commands/user-team-temp";
import { toast } from "vue-sonner";

const emailError = ref(""); // 存储错误信息
const router = useRouter();
const props = defineProps({
  addMemModel: Boolean,
  memberObj: Object, //编辑成员信息
});
const memberObj = ref(null);
watch(
  () => props.memberObj,
  (val) => {
    memberObj.value = val; //赋值被编辑用户信息

    if (val) {
      searchGroup(); //获取分组信息以得到分组id
      setTimeout(() => {
        //延时使分组数据成功获取
        formData.value.groupId = teamGroup.value?.find(
          (item) => item.name === val.group_name
        )?.id; //赋值分组ID
      }, 100);
      formData.value.remarkName = val.description;
      teamGroupName.value = val.group_name;
    }
  }
);
const emit = defineEmits(["update:addMemModel"]);
const formData = ref({
  account: "",
  name: "自动获取的姓名",
  remarkName: "",
  groupName: "",
  groupId: 0,
  environments: [],
  envNameList: [], //所选环境名称
});
const clearFormData = () => {
  formData.value.account = "";
  formData.value.name = "自动获取的姓名";
  formData.value.remarkName = "";
  formData.value.groupName = "";
  formData.value.environments = [];
  formData.value.envNameList = [];
};
const teamGroupName = ref("请选择分组");
watch(
  () => formData.value.groupId,
  (val) => {
    teamGroupName.value = teamGroup.value.find(
      (group) => group.id === val
    )?.name;
  }
);

const groups = ref([]);

watch(
  () => formData.value.account,
  async (newValue) => {
    if (newValue) {
      // Simulate API call to fetch user name
      await new Promise((resolve) => setTimeout(resolve, 500));
    } else {
      formData.value.name = "";
    }
  }
);

const handleAddGroup = () => {
  router.push("/team/group");
};

const handleSelectEnvironment = () => {
  alertEnvironment.value = true;
  selectedIds.value = formData.value.environments;
};

//ss代码之-----刷新选中的环境id
const refreshList = () => {
  formData.value.environments = [];
  environments.value.forEach((env) => {
    if (formData.value.envNameList.includes(env.name)) {
      formData.value.environments.push(env.id);
    }
  });
};
const handleSubmit = () => {
  //提交添加成员
  //重新赋值环境id数组，防止中途删除的数据
  refreshList();
  console.log("Form submitted:", formData.value);
  if (props.memberObj) {
    //修改
    // console.log("props.member:", props.memberObj);
    // console.log("form::::::::::", formData.value);

    team_modify_team_user_info(
      props.memberObj.team_id,
      formData.value.groupId,
      props.memberObj.user_uuid,
      formData.value.remarkName
    ).then((res) => {
      toast.message(res.message);
    });
  } else {
    //发送邀请
    query_current_team_info().then((res) => {
      team_send(
        res.data.id,
        formData.value.account,
        formData.value.remarkName
      ).then((res) => {
        if (res.message.includes("发送失败")) {
          toast.warning(res.message);
        } else {
          toast.success("邀请发送成功");
        }
      });
    });
  }
  emit("update:addMemModel", false);
  // closeModal();
};

const closeModal = () => {
  //取消
  emit("update:addMemModel", false);
  clearFormData();
  selectedIds.value = [];
};

// 校验邮箱格式的函数
const validateEmail = () => {
  const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
  if (!formData.value.account) {
    emailError.value = "邮箱不能为空";
  } else if (!emailRegex.test(formData.value.account)) {
    emailError.value = "邮箱格式不正确";
  } else {
    emailError.value = ""; // 清除错误信息
  }
};

//选择环境---------------------------------
const alertEnvironment = ref(false);
const searchForm = reactive({
  searchQuery: "", //输入查询条件
  selectType: 0, //查询类型  0:环节名称  1:IP地址  2：备注
  selectIsAc: 0, //0：所有环境  1：已授权  2：未授权
  selectOption: 0, //操作类型---0：所有  1：Windows 2：Android ...
  selectGroup: 0, // 分组
});
const envGroupName = ref("所有");
watch(
  () => searchForm.selectGroup,
  () => {
    //监听选中环境分组名称
    const group = envGroup.value.find(
      (group) => group.id === searchForm.selectGroup
    );
    envGroupName.value = group.groupName;
  }
);
const selectedIds = ref([]);
const handleClose = () => {
  alertEnvironment.value = false;
  selectedIds.value = [];
};
const subSelectEnv = () => {
  // 关闭弹窗
  alertEnvironment.value = false;

  // 打印选择的 ID
  console.log("选择的环境 ID 列表：", selectedIds.value);

  // 赋值 ID 列表
  formData.value.environments = [...selectedIds.value]; // 转为数组形式

  // 清空环境名称列表，避免重复累积
  formData.value.envNameList = [];

  // 遍历 environments，匹配 selectedIds，赋值环境名称列表
  environments.value.forEach((env) => {
    if (selectedIds.value.includes(env.id)) {
      // 使用 includes 检查是否存在
      console.log("匹配的环境：", env);
      formData.value.envNameList.push(env.name);
    }
  });

  // 清空选择的 ID
  selectedIds.value = [];
};

// Generate sample environments
const environments = ref([
  { id: 1, name: "新建环境1", groupId: 1, optionType: 1 },
  { id: 2, name: "新建环境2", groupId: 1, optionType: 1 },
  { id: 3, name: "新建环境3", groupId: 1, optionType: 1 },
  { id: 4, name: "新建环境4", groupId: 1, optionType: 1 },
  { id: 5, name: "新建环境5", groupId: 1, optionType: 1 },
  { id: 6, name: "新建环境6", groupId: 1, optionType: 1 },
]);
//环境的分组 （用于查询按分组查询环境）
const envGroup = ref([
  { id: 1, groupName: "分组a" },
  { id: 2, groupName: "分组b" },
  { id: 3, groupName: "分组c" },
  { id: 4, groupName: "分组d" },
]);
//团队的分组
const teamGroup = ref([]);
const optionName = ref("所有操作类型");
watch(
  () => searchForm.selectOption,
  (val) => {
    optionName.value =
      val === 0 ? "所有操作系统" : val === 1 ? "Windows" : "Android";
  }
);
// Computed properties for checkbox states
const isAllSelected = computed(() => {
  return (
    environments.value.length > 0 &&
    selectedIds.value.length === environments.value.length
  );
});

const isIndeterminate = computed(() => {
  return selectedIds.value.length > 0 && !isAllSelected.value;
});

// Toggle all checkboxes
const toggleSelectAll = () => {
  if (isAllSelected.value) {
    selectedIds.value = [];
  } else {
    selectedIds.value = environments.value.map((env) => env.id);
  }
};

const selectAllCheckbox = ref(null);

const selectedCount = computed(
  () => environments.value.filter((env) => env.selected).length
);

function updateSelectAll() {
  const checkedCount = selectedCount.value;
  isIndeterminate.value =
    checkedCount > 0 && checkedCount < environments.value.length;
  if (selectAllCheckbox.value) {
    selectAllCheckbox.value.indeterminate = isIndeterminate.value;
  }
}

onMounted(() => {
  updateSelectAll();
});
const searchGroup = () => {
  team_query(1, 10).then((res) => {
    // 查询团队下所有分组
    team_group_query_all(res.data.data[0].id).then((res) => {
      teamGroup.value = res.data;
    });
  });
};

//环境列表
const sequences = ref(Array.from({ length: 28 }, (_, i) => i + 1));
const isExpanded = ref(false);

const displayedItems = computed(() => {
  return sequences.value.slice(0, 5);
});

const removeSequence = (name, index) => {
  formData.value.envNameList = formData.value.envNameList.filter(
    (_, i) => i !== index
  );
  refreshList();
};
</script>

<style scoped>
select {
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.5rem center;
  background-repeat: no-repeat;
  background-size: 1.5em 1.5em;
  padding-right: 2.5rem;
}

/* Custom scrollbar styling */
.overflow-y-auto {
  scrollbar-width: thin;
  scrollbar-color: #cbd5e0 #edf2f7;
}

.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #edf2f7;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: #cbd5e0;
  border-radius: 3px;
}
</style>
