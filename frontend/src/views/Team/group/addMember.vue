<template>
  <Teleport to="body" class="z-20">
    <div
      v-if="props.addMemModel"
      class="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50"
    >
      <div class="fixed inset-0 bg-black/20 flex items-center justify-center">
        <div class="bg-white rounded-lg w-[800px] min-w-xl">
          <!-- Modal Header -->
          <div class="flex justify-between items-center p-4 border-b">
            <h2 class="text-lg font-medium">添加成员</h2>
            <button
              @click="closeModal"
              class="text-gray-400 hover:text-gray-600"
            >
              <XIcon class="w-5 h-5" />
            </button>
          </div>

          <!-- Modal Content -->
           <!-- {{ props.groupObj }} -->
          <div class="p-6 px-[150px]  h-[350px]">
            <form @submit.prevent="handleSubmit" class="space-y-6 h-full flex flex-col justify-between">
              <!-- Login Account -->
              <div class="flex items-center">
                <label class="mb-1 pr-3 w-[110px] flex justify-end">
                  <span class="text-red-500">*</span> 登陆账号:
                </label>
                <input
                  v-model="formData.account"
                  type="text"
                  placeholder="请输入手机号/邮箱"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>

              <!-- Name -->
              <div class="flex items-center">
                <label class="mb-1 pr-3 w-[110px] flex justify-end">姓名:</label>
                <input
                  v-model="formData.name"
                  type="text"
                  placeholder="输入手机号/邮箱后自动获取"
                  class="w-full px-3 py-2 border rounded-md bg-gray-50 focus:outline-none"
                  readonly
                />
              </div>

              <!-- Remark Name -->
              <div class="flex items-center">
                <label class="mb-1 pr-3 w-[110px] flex justify-end">
                  <span class="text-red-500">*</span> 备注名:
                </label>
                <input
                  v-model="formData.remarkName"
                  type="text"
                  placeholder="请输入备注名"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>

              <!-- Group Name -->
              <div class="flex items-center">
                <label class="mb-1 pr-3 w-[110px] flex justify-end">分组名:</label>
                <input
                  v-model="formData.groupName"
                  type="text"
                  class="w-full px-3 py-2 rounded-md focus:outline-none"
                  readonly
                />
              </div>
            </form>
          </div>

          <!-- Modal Footer -->
          <div class="flex justify-center space-x-3 p-4 border-t">
            <button
              @click="handleSubmit"
              class="px-8 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              确定
            </button>
            <button
              @click="closeModal"
              class="px-8 py-2 border rounded-md hover:bg-gray-50"
            >
              取消
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { ref, watch, defineEmits, defineProps, onMounted } from "vue";
import { XIcon } from "lucide-vue-next";
import { team_allow, team_send } from "@/commands/user-team-temp";
import { query_current_team_info } from '@/commands/team'

const props = defineProps({
  addMemModel: {
    type: Boolean,
    required: true,
  },
  groupObj: Object
});
console.log("props::::::::::", props);

onMounted(() => {
  console.log("----------------:", props.groupObj);
  
  if(props.groupObj){
    formData.value.groupName = props.groupObj.name
  }
})
const emit = defineEmits(["update:addMemModel"]);
const formData = ref({
  account: "",
  name: "",
  remarkName: "",
  groupName: "编辑者",
});
watch(() => props.groupObj, (val) => {
  formData.value.groupName = props.groupObj.name
})

// Watch for account changes to simulate auto-fetching name
watch(
  () => formData.value.account,
  async (newValue) => {
    if (newValue) {
      // Simulate API call to fetch user name
      await new Promise((resolve) => setTimeout(resolve, 500));
      formData.value.name = "自动获取的姓名";
    } else {
      formData.value.name = "";
    }
  }
);

const handleSubmit = () => {
  // Handle form submission
  // console.log("Form submitted:", formData.value);
  emit("update:addMemModel", false);
  query_current_team_info().then(res => {
    team_send(res.data.id, formData.value.account, formData.value.remarkName).then(res => {})
  })
  closeModal();
};

const closeModal = () => {
  // Emit close event to parent
  emit("update:addMemModel", false);
};
</script>
