<template>
  <Teleport to="body" class="z-20">
    <div v-if="props.addMemModel" class="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50">
      <div class="fixed inset-0 bg-black/20 flex items-center justify-center">
        <div class="bg-white rounded-lg w-full min-w-[500px] max-w-[800px]">
          <!-- Modal Header -->
          <div class="flex justify-between items-center p-4 border-b">
            <h2 class="text-lg font-medium">{{ props.memberObj !== null ? '修改成员' : '添加成员' }}</h2>
            <button @click="closeModal" class="text-gray-400 hover:text-gray-600">
              <XIcon class="w-5 h-5" />
            </button>
          </div>

          <!-- Modal Content -->
          <div class="p-6">
            <form @submit.prevent="handleSubmit" class="space-y-6">
              <!-- Login Account -->
              <div class="flex items-start">
                <label class="w-24 pt-2 pr-3 flex justify-end">
                  <span class="text-red-500">*</span> 登录账号:
                </label>
                <div class="flex flex-col flex-1">
                  <input v-model="formData.account" type="text" placeholder="请输入手机号/邮箱号"
                    class="flex-1 px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500" required
                    @input="validateEmail" />
                  <p v-if="emailError" style="color: red;">{{ emailError }}</p>
                </div>
              </div>

              <!-- Name -->
              <!-- <div class="flex items-start">
                <label class="w-24 pt-2 pr-3 flex justify-end">姓名:</label>
                <input v-model="formData.name" type="text" placeholder="输入手机号/邮箱号后自动获取"
                  class="flex-1 px-3 py-2 border rounded bg-gray-50 text-gray-500 outline-none" readonly />
              </div> -->

              <!-- Remark Name -->
              <div class="flex items-start">
                <label class="w-24 pt-2 pr-3 flex justify-end">
                  <span class="text-red-500">*</span> 备注:
                </label>
                <input v-model="formData.remarkName" type="text" placeholder="请输入备注名"
                  class="flex-1 px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required />
              </div>

              <!-- Group Name -->
              <!-- <div class="flex items-start">
                <label class="w-24 pt-2 pr-3 flex justify-end">
                  <span class="text-red-500">*</span> 分组名称:
                </label>
                <div class="flex-1">
                  <div class="flex gap-2">
                    <More>
                      <MoreTrigger class="flex-1">
                        <button @click="searchGroup"
                          class="w-full px-4 py-2 bg-white border border-gray-300 rounded-md hover:ring-2 hover:ring-blue-500 focus:outline-none flex justify-between items-center"
                          :class="{ 'text-gray-400': formData.groupId === 0 }">
                          {{ teamGroupName }}
                          <ChevronDownIcon class="inline-block w-4 h-4 ml-1" />
                        </button>
                      </MoreTrigger>
                      <MoreContent class="w-[240px]">
                        <MoreItem v-for="group in teamGroup" class="cursor-pointer w-full"
                          @click="formData.groupId = group.id">
                          <Settings2Icon class="w-4 h-4" />{{ group.name }}
                        </MoreItem>
                      </MoreContent>
                    </More>
                    <button type="button" @click="handleAddGroup"
                      class="text-blue-500 hover:text-blue-600 flex items-center hover:bg-gray-100 px-2">
                      <PlusIcon class="w-4 h-4 mr-1" />
                      添加分组
                    </button>
                  </div>
                </div>
              </div> -->

              <!-- Environment Authorization -->
               <!-- <div v-if="!props.memberObj" class="flex items-start">
                <label class="w-24 pt-2 pr-3 flex justify-end">授权环境:</label>
                <div class="flex-1">
                  <div v-if="formData.environments.length === 0">
                    <button type="button" @click="handleSelectEnvironment"
                      class="px-4 py-2 bg-blue-500 text-white rounded flex items-center hover:bg-blue-600">
                      <PlusIcon class="w-4 h-4 mr-1" />
                      选择环境
                    </button>
                    <p class="mt-2 text-sm text-red-400">
                      请选择需要授权给该员工管理的环境
                    </p>
                  </div>

                  <div v-else class="w-full">
                    <div class="flex items-center justify-between mb-2">
                      <button type="button" @click="handleSelectEnvironment"
                        class="px-4 py-2 bg-blue-500 text-white rounded flex items-center hover:bg-blue-600">
                        <PlusIcon class="w-4 h-4 mr-1" />
                        选择环境
                      </button>
                      <div class="w-[140px] flex justify-between">
                        <span class="text-sm text-gray-600">已选{{ formData.environments.length }}个环境</span>
                        <button v-if="formData.envNameList.length > 4" @click="isExpanded = !isExpanded"
                          class="text-blue-500 text-sm hover:text-blue-600">
                          {{ isExpanded ? '收起' : '展开' }}
                        </button>
                      </div>
                    </div>

                    <div v-if="!isExpanded" class="flex flex-wrap gap-2">
                      <div v-for="(item, index) in formData.envNameList.slice(0, 4)" :key="index"
                        class="px-3 py-1 bg-blue-50 text-blue-600 rounded-md text-sm max-w-[150px]">
                        {{ item }}
                      </div>
                      <div v-if="formData.envNameList.length > 4" class="px-3 py-1 text-gray-500 text-sm">
                        ...
                      </div>
                    </div>

                    <div v-else class="border rounded-md max-h-[120px] overflow-y-auto">
                      <div class="grid grid-cols-4 gap-2 p-2">
                        <div v-for="(item, index) in formData.envNameList" :key="index"
                          class="relative group px-3 py-1 bg-blue-50 text-blue-600 rounded-md text-sm flex items-center justify-between min-w-[150px]">
                          <span>{{ item }}</span>
                          <button @click="removeSequence(item, index)"
                            class="opacity-0 group-hover:opacity-100 transition-opacity">
                            <CircleX class="w-4 h-4 text-gray-400 hover:text-gray-600" />
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div> -->
            </form>
          </div>

          <!-- Modal Footer -->
          <div class="flex justify-end space-x-3 p-4 border-t">
            <button @click="closeModal" class="px-6 py-2 border rounded hover:bg-gray-50">
              取消
            </button>
            <button type="submit" @click="handleSubmit" class="px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
              确定
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>

  <!-- class: 这里class的宽度就是弹出框的宽度.请自行设置 -->
  <Model class="min-w-[868px]" :title="'授权环境'" :open="alertEnvironment" @close="handleClose">
    <div class="space-x-4 px-2 pt-6 flex flex-col">
      <!-- 在这里书写弹出框主题内容代码 -->
      <div class="max-w-3xl mx-[50px] p-6">
        <!-- <div class="mb-6">
          <h1 class="text-lg font-medium mb-2">请选择需授权的环境</h1>
        </div> -->

        <div class="relative flex hover:outline-none hover:ring-2 hover:ring-blue-500 rounded-md">
          <div class="relative inline-block">
            <More>
              <MoreTrigger>
                <button @click=""
                  class="w-[130px] px-4 py-2 bg-white border border-r-0 border-gray-300 rounded-l-md hover:bg-gray-50 focus:outline-none">
                  {{ searchForm.selectType === 0 ? '环境名称' : searchForm.selectType === 1 ? 'IP地址' : '备注' }}
                  <ChevronDownIcon class="inline-block w-4 h-4 ml-1" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem class="cursor-pointer" @click="searchForm.selectType = 0">
                  <Settings2Icon class="w-4 h-4" />环境名称
                </MoreItem>
                <MoreItem class="cursor-pointer" @click="searchForm.selectType = 1">
                  <SquarePenIcon class="w-4 h-4" />IP地址
                </MoreItem>
                <MoreItem class="cursor-pointer" @click="searchForm.selectType = 2">
                  <Trash2Icon class="w-4 h-4" />备注
                </MoreItem>
              </MoreContent>
            </More>

          </div>
          <input type="text"
            :placeholder="'请输入' + (searchForm.selectType === 0 ? '环境名称' : searchForm.selectType === 1 ? 'IP地址' : '备注')"
            class="flex-1 px-4 py-2 border border-gray-300 rounded-r-md focus:outline-none" />
        </div>

        <div class="flex gap-4 my-6 justify-between">
          <div class="relative w-1/3">
            <More>
              <MoreTrigger class="w-full">
                <button @click=""
                  class="w-full px-4 py-2 bg-white border border-gray-300 rounded-md hover:ring-2 hover:ring-blue-500 focus:outline-none flex justify-between items-center">
                  {{ searchForm.selectIsAc === 0 ? '所有环境' : searchForm.selectIsAc === 1 ? '已授权' : '未授权' }}
                  <ChevronDownIcon class="inline-block w-4 h-4 ml-1" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem class="cursor-pointer" @click="searchForm.selectIsAc = 0">
                  <Settings2Icon class="w-4 h-4" />所有环境
                </MoreItem>
                <MoreItem class="cursor-pointer" @click="searchForm.selectIsAc = 1">
                  <SquarePenIcon class="w-4 h-4" />已授权
                </MoreItem>
                <MoreItem class="cursor-pointer" @click="searchForm.selectIsAc = 2">
                  <Trash2Icon class="w-4 h-4" />未授权
                </MoreItem>
              </MoreContent>
            </More>

            <!-- <ChevronDownIcon class="absolute right-2 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" /> -->
          </div>

          <div class="relative w-1/3">
            <More>
              <MoreTrigger class="w-full">
                <button @click=""
                  class="w-full px-4 py-2 bg-white border border-gray-300 rounded-md hover:ring-2 hover:ring-blue-500 focus:outline-none flex justify-between items-center">
                  {{ optionName }}
                  <ChevronDownIcon class="inline-block w-4 h-4 ml-1" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem class="cursor-pointer" @click="searchForm.selectOption = 0">
                  <Settings2Icon class="w-4 h-4" />所有操作类型
                </MoreItem>
                <MoreItem class="cursor-pointer" @click="searchForm.selectOption = 1">
                  <SquarePenIcon class="w-4 h-4" />Windows
                </MoreItem>
                <MoreItem class="cursor-pointer" @click="searchForm.selectOption = 2">
                  <Trash2Icon class="w-4 h-4" />Android
                </MoreItem>
              </MoreContent>
            </More>
            <!-- <ChevronDownIcon class="absolute right-2 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" /> -->
          </div>

          <div class="relative w-1/3">
            <More>
              <MoreTrigger class="w-full">
                <button @click=""
                  class="w-full px-4 py-2 bg-white border border-gray-300 rounded-md hover:ring-2 hover:ring-blue-500 focus:outline-none flex justify-between items-center">
                  {{ envGroupName }}
                  <ChevronDownIcon class="inline-block w-4 h-4 ml-1" />
                </button>
              </MoreTrigger>
              <MoreContent>
                <MoreItem v-for="group in envGroup" class="cursor-pointer" @click="searchForm.selectGroup = group.id">
                  <Settings2Icon class="w-4 h-4" />{{ group.groupName }}
                </MoreItem>
              </MoreContent>
            </More>
            <!-- <ChevronDownIcon class="absolute right-2 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" /> -->
          </div>
        </div>

        <!-- Environment List -->
        <div class="border rounded-md">
          <!-- Select All Header -->
          <div class="p-4 border-b bg-gray-50 flex justify-between">
            <label class="flex items-center">
              <input type="checkbox" :checked="isAllSelected" :indeterminate="isIndeterminate" @change="toggleSelectAll"
                class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
              <span class="ml-2">全选</span>
            </label>
            <p class="text-sm text-gray-500">已选择 {{ selectedCount }} 个环境</p>
          </div>

          <!-- Environment Items -->
          <div class="divide-y">
            <div v-for="env in environments" :key="env.id" class="p-4 flex items-center hover:bg-gray-50">
              <label class="flex items-center flex-1">
                <input type="checkbox" v-model="selectedIds" :value="env.id"
                  class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
                <span class="ml-2 flex items-center">
                  <GridIcon class="w-4 h-4 text-gray-400 mr-2" />
                  <span>{{ env.name }}</span>
                </span>
              </label>
              <span class="text-gray-400 mx-4">--</span>
              <span class="bg-green-500 text-white px-2 py-1 rounded text-sm">动</span>
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
import { ref, watch, defineEmits, defineProps, computed, onMounted, reactive } from "vue";
import { XIcon, PlusIcon, SearchIcon, ChevronDownIcon, GridIcon, CircleX } from "lucide-vue-next";
import { useRouter } from "vue-router";
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { team_group_query_all } from "@/commands/team-group";
import { team_query } from "@/commands/team";

const emailError = ref(""); // 存储错误信息
const router = useRouter()
const props = defineProps({
  addMemModel: Boolean,
  memberObj: Object,  //编辑成员信息
});
const memberObj = ref(null)
watch(() => props.memberObj, (val) => {
  memberObj.value = val    //赋值被编辑用户信息
  if (val) {
    formData.value.name = val.name
    // formData.value.account = val.name
    formData.value.remarkName = val.contact
  }
})
const emit = defineEmits(["update:addMemModel"]);
const formData = ref({
  account: "",
  name: "自动获取的姓名",
  remarkName: "",
  groupName: "",
  groupId: 0,
  environments: [],
  envNameList: [],  //所选环境名称
});
const clearFormData = () => {
  formData.value.account = "";
  formData.value.name = "自动获取的姓名";
  formData.value.remarkName = "";
  formData.value.groupName = "";
  formData.value.environments = [];
  formData.value.envNameList = []
}
const teamGroupName = ref("请选择分组")
watch(() => formData.value.groupId, (val) => {
  teamGroupName.value = teamGroup.value.find((group) => group.id === val).name
})

const groups = ref(["管理员组", "普通用户组", "访客组"]);

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
  router.push("/team/group")
};

const handleSelectEnvironment = () => {
  alertEnvironment.value = true
  selectedIds.value = formData.value.environments
};

//ss代码之-----刷新选中的环境id
const refreshList = () => {
  formData.value.environments = []
  environments.value.forEach((env) => {
    if (formData.value.envNameList.includes(env.name)) {
      formData.value.environments.push(env.id)
    }
  })
}
const handleSubmit = () => {  //提交添加成员
  //重新赋值环境id数组，防止中途删除的数据
  refreshList()
  console.log("Form submitted:", formData.value);
  emit("update:addMemModel", false);
  // closeModal();
};

const closeModal = () => {  //取消
  emit("update:addMemModel", false);
  clearFormData()
  selectedIds.value = []
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
const alertEnvironment = ref(false)
const searchForm = reactive({
  searchQuery: "",  //输入查询条件
  selectType: 0,   //查询类型  0:环节名称  1:IP地址  2：备注
  selectIsAc: 0,  //0：所有环境  1：已授权  2：未授权
  selectOption: 0,  //操作类型---0：所有  1：Windows 2：Android ...
  selectGroup: 0,  // 分组
})
const envGroupName = ref("所有")
watch(() => searchForm.selectGroup, () => {  //监听选中环境分组名称
  const group = envGroup.value.find((group) => group.id === searchForm.selectGroup)
  envGroupName.value = group.groupName
})
const selectedIds = ref([])
const handleClose = () => {
  alertEnvironment.value = false
  selectedIds.value = []
}
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
    if (selectedIds.value.includes(env.id)) { // 使用 includes 检查是否存在
      console.log("匹配的环境：", env);
      formData.value.envNameList.push(env.name);
    }
  });

  // 清空选择的 ID
  selectedIds.value = [];
};

// Generate sample environments
const environments = ref([
  { id: 1, name: '新建环境1', groupId: 1, optionType: 1, },
  { id: 2, name: '新建环境2', groupId: 1, optionType: 1, },
  { id: 3, name: '新建环境3', groupId: 1, optionType: 1, },
  { id: 4, name: '新建环境4', groupId: 1, optionType: 1, },
  { id: 5, name: '新建环境5', groupId: 1, optionType: 1, },
  { id: 6, name: '新建环境6', groupId: 1, optionType: 1, },
])
//环境的分组 （用于查询按分组查询环境）
const envGroup = ref([
  { id: 1, groupName: "分组a" },
  { id: 2, groupName: "分组b" },
  { id: 3, groupName: "分组c" },
  { id: 4, groupName: "分组d" },
])
//团队的分组
const teamGroup = ref([
  { id: 1, groupName: "分组a" },
  { id: 2, groupName: "分组b" },
  { id: 3, groupName: "分组c" },
  { id: 4, groupName: "分组d" },
])
const optionName = ref('所有操作类型')
watch(() => searchForm.selectOption, (val) => {
  optionName.value = val === 0 ? '所有操作系统' : val === 1 ? 'Windows' : 'Android'
})
// Computed properties for checkbox states
const isAllSelected = computed(() => {
  return environments.value.length > 0 && selectedIds.value.length === environments.value.length
})

const isIndeterminate = computed(() => {
  return selectedIds.value.length > 0 && !isAllSelected.value
})

// Toggle all checkboxes
const toggleSelectAll = () => {
  if (isAllSelected.value) {
    selectedIds.value = []
  } else {
    selectedIds.value = environments.value.map(env => env.id)
  }
}

const selectAllCheckbox = ref(null)

const selectedCount = computed(() =>
  environments.value.filter(env => env.selected).length
)

function updateSelectAll() {
  const checkedCount = selectedCount.value
  isIndeterminate.value = checkedCount > 0 && checkedCount < environments.value.length
  if (selectAllCheckbox.value) {
    selectAllCheckbox.value.indeterminate = isIndeterminate.value
  }
}

onMounted(() => {
  updateSelectAll()

})
const searchGroup = () => {
  team_query(1, 10).then(res => {
    // 查询团队下所有分组
    team_group_query_all(res.data.data[0].id).then(res => {
      teamGroup.value = res.data
    })
  })
}

//环境列表
const sequences = ref(Array.from({ length: 28 }, (_, i) => i + 1))
const isExpanded = ref(false)

const displayedItems = computed(() => {
  return sequences.value.slice(0, 5)
})

const removeSequence = (name, index) => {
  formData.value.envNameList = formData.value.envNameList.filter((_, i) => i !== index)
  refreshList()
}
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
  scrollbar-color: #CBD5E0 #EDF2F7;
}

.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #EDF2F7;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: #CBD5E0;
  border-radius: 3px;
}
</style>
