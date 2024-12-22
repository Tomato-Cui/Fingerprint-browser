<template>
  <Teleport to="body" class="z-20">
    <div v-if="props.showAddModal" class="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50">
      <!-- 主体 -->
      <div class="fixed inset-0 bg-black/20 flex items-center justify-center">
        <div class="bg-white rounded-lg w-full max-w-2xl p-6">
          <!-- Header -->
          <div class="flex justify-between items-center mb-6">
            <h2 class="text-xl font-medium">{{ props.groupObj === null ? '新建分组' : '修改分组' }}</h2>
            <button @click="closeModal" class="text-gray-400 hover:text-gray-600">
              <XIcon class="w-5 h-5" />
            </button>
          </div>

          <!-- Form -->
          <div class="space-y-4 mb-6">
            <div class="flex items-center">
              <label class="mb-1 w-[120px] flex justify-end pr-[15px]">
                <span class="text-red-500">*</span> 分组名称:
              </label>
              <div class="flex flex-col w-full">
                <input v-model="formData.name" type="text" placeholder="请输入分组名称" @input="valitadeGroupName"
                  class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
                  :class="{ 'border-red-500': groupNameError }" />
                <p v-if="groupNameError" class="text-red-500 text-sm">{{ groupNameError }}</p>
              </div>
            </div>

            <div class="flex items-center">
              <label class="mb-1 w-[120px] flex justify-end pr-[15px]">备注:</label>
              <input v-model="formData.remarks" type="text" placeholder="请输入备注内容"
                class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none" />
            </div>
          </div>

          <!-- Permissions -->
          <div class="mb-6">
            <div class="flex items-center my-[10px]">
              <h3 class="mb-2 w-[100px] flex justify-end pr-[15px]">权限管理:</h3>
              <p class="text-sm text-gray-500 mb-4">
                请为分组添加权限功能，同个分组的成员权限是相同一致
              </p>
            </div>


            <div class="flex gap-2">
              <!-- First Column -->
              <div class="w-1/3 h-[300px] bg-gray-50 rounded-lg p-2 overflow-auto">
                <div v-for="item in permissions" :key="item.id"
                  class="flex items-center p-2 hover:bg-gray-100 rounded cursor-pointer"
                  :class="{ 'bg-gray-100': selectedLevel1 === item.id }" @click="selectLevel1(item.id)">
                  <input type="checkbox" :checked="isChecked(item)" :indeterminate="isIndeterminate(item)"
                    @change="toggleLevel1(item)" class="mr-2" />
                  <span>{{ item.name }}</span>
                  <span class="ml-auto">›</span>
                </div>
              </div>

              <!-- Second Column -->
              <div class="w-1/3 h-[300px] bg-gray-50 rounded-lg p-2 overflow-auto">
                <div v-for="item in getCurrentLevel2Items()" :key="item.id"
                  class="flex items-center p-2 hover:bg-gray-100 rounded cursor-pointer"
                  :class="{ 'bg-gray-100': selectedLevel2 === item.id }" @click="selectLevel2(item.id)">
                  <input type="checkbox" :checked="isChecked(item)" :indeterminate="isIndeterminate(item)"
                    @change="toggleLevel2(item)" class="mr-2" />
                  {{ item.name }}
                  <span class="ml-auto" v-if="item.children">›</span>
                </div>
              </div>

              <!-- Third Column -->
              <div class="w-1/3 h-[300px] bg-gray-50 rounded-lg p-2 overflow-auto">
                <div v-for="item in getCurrentLevel3Items()" :key="item.id"
                  class="flex items-center p-2 hover:bg-gray-100 rounded">
                  <input type="checkbox" :checked="isChecked(item)" @change="toggleLevel3(item)" class="mr-2" />
                  {{ item.name }}
                  <!-- <span class="ml-auto">›</span> -->
                </div>
              </div>
            </div>
          </div>

          <!-- Footer -->
          <div class="flex justify-end space-x-4">
            <button @click="closeModal" class="px-4 py-2 border rounded-lg hover:bg-gray-50">
              取消
            </button>
            <button @click="submitForm" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">
              确定
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { ref, computed, defineEmits, defineProps, onMounted, watch } from "vue";
import { XIcon } from "lucide-vue-next";

const props = defineProps({
  //接收父组件传递的控制参数
  showAddModal: {
    type: Boolean,
    required: true, //必须传参
  },
  groupObj: Object
});

// 使用 emit 触发父组件更新的事件
const emit = defineEmits(["update:showAddModal"]);
watch(
  () => props.showAddModal,
  (newVel) => {
    emit("update:showAddModal", props.showAddModal);
    // console.log("pppppppppppp:", props.showAddModal);
  }
);
watch(() => props.groupObj, (val) => {
  console.log("groupObj:::", val);
  if (val != null) {
    formData.value.name = props.groupObj.name
    if (props.groupObj.note != '\\')
      formData.value.remarks = props.groupObj.note
  }

})

const formData = ref({
  name: "",
  remarks: "",
});
const clearForm = () => {
  formData.value.name = "";
  formData.value.remarks = "";
  groupNameError.value = ''
  selectedItems.value.clear()
  selectedLevel1.value = null
  selectedLevel2.value = null
}

const selectedLevel1 = ref(null);
const selectedLevel2 = ref(null);
const selectedItems = ref(new Set()); //追踪所选权限的 ID

const permissions = ref([
  //权限列表
  {
    id: 1,
    name: "环境",
    children: [
      {
        id: 11,
        name: "我的环境/应用环境",
        checked: false,
        children: [
          { id: 111, name: "数据查看", checked: false },
          { id: 112, name: "批量导入", checked: false },
          { id: 113, name: "数据查看", checked: false },
          { id: 114, name: "批量导入", checked: false },
          { id: 115, name: "数据查看", checked: false },
          { id: 116, name: "批量导入", checked: false },
          { id: 117, name: "数据查看", checked: false },
          { id: 118, name: "批量导入", checked: false },
          { id: 119, name: "数据查看", checked: false },
          { id: 1110, name: "批量导入", checked: false },
        ],
      },
      {
        id: 12,
        name: "已转移环境",
        checked: false,
        children: null,
      },
    ],
  },
  {
    id: 2,
    name: "云手机",
    children: [
      {
        id: 21,
        name: "已转移环境",
        checked: false,
        children: [
          { id: 211, name: "数据查看", checked: false },
          { id: 212, name: "批量导入", checked: false },
        ],
      },
      {
        id: 22,
        name: "应用管理",
        checked: false,
        children: [
          { id: 221, name: "数据查看", checked: false },
          { id: 222, name: "批量导入", checked: false },
        ],
      },
    ],
  },
  {
    id: 3,
    name: "TK运营平台",
    children: [
      {
        id: 31,
        name: "服务管理-团队扩展",
        checked: false,
        children: [
          { id: 311, name: "数据查看", checked: false },
          { id: 312, name: "批量导入", checked: false },
        ],
      },
      {
        id: 32,
        name: "代理购销",
        checked: false,
        children: [
          { id: 321, name: "数据查看", checked: false },
          { id: 322, name: "批量导入", checked: false },
        ],
      },
    ],
  },
  {
    id: 4,
    name: "资源",
    children: [
      {
        id: 41,
        name: "团队网络",
        checked: false,
        children: [
          { id: 411, name: "数据查看", checked: false },
          { id: 412, name: "批量导入", checked: false },
        ],
      },
      {
        id: 42,
        name: "回收站",
        checked: false,
        children: [
          { id: 421, name: "数据查看", checked: false },
          { id: 422, name: "批量导入", checked: false },
        ],
      },
    ],
  },
]);

const selectLevel1 = (id) => {
  selectedLevel1.value = id;
  selectedLevel2.value = null;
};

const selectLevel2 = (id) => {
  selectedLevel2.value = id;
};

const getCurrentLevel2Items = () => {
  const level1Item = permissions.value.find(
    (item) => item.id === selectedLevel1.value
  );
  return level1Item?.children || [];
};

const getCurrentLevel3Items = () => {
  const level2Items = getCurrentLevel2Items();
  const level2Item = level2Items.find(
    (item) => item.id === selectedLevel2.value
  );
  return level2Item?.children || [];
};

const isChecked = (item) => {
  // console.log(item);
  // console.log(selectedItems.value);
  // console.log(!item.children);

  //判断权限项是否选中
  if (!item.children) {
    return selectedItems.value.has(item.id);
  }

  const allChildren = getAllChildren(item);
  return allChildren.every((childId) => selectedItems.value.has(childId));
};

const isIndeterminate = (item) => {
  if (!item.children) return false;

  const allChildren = getAllChildren(item);
  const someSelected = allChildren.some(
    (
      childId //当前子权限有选中
    ) => selectedItems.value.has(childId)
  );
  const allSelected = allChildren.every(
    (
      childId //当前子权限未被全部选中
    ) => selectedItems.value.has(childId)
  );

  return someSelected && !allSelected;
};

const getAllChildren = (item) => {
  //获取所有当前权限子项的id
  const children = [];
  if (item.children) {
    item.children.forEach((child) => {
      if (child.children) {
        children.push(...getAllChildren(child));
      } else {
        children.push(child.id);
      }
    });
  }
  return children;
};

const toggleLevel1 = (item) => {
  const allChildren = getAllChildren(item);
  const allSelected = allChildren.every((childId) =>
    selectedItems.value.has(childId)
  );

  if (allSelected) {
    allChildren.forEach((childId) => selectedItems.value.delete(childId));
  } else {
    allChildren.forEach((childId) => selectedItems.value.add(childId));
  }
};

const toggleLevel2 = (item) => {
  if (!item.children) {    //二级判断
    if (selectedItems.value.has(item.id)) {
      selectedItems.value.delete(item.id)
    } else {
      selectedItems.value.add(item.id)
    }
    return
  }
  const allChildren = getAllChildren(item);
  const allSelected = allChildren.every((childId) =>
    selectedItems.value.has(childId)
  );

  if (allSelected) {
    allChildren.forEach((childId) => selectedItems.value.delete(childId));
  } else {
    allChildren.forEach((childId) => selectedItems.value.add(childId));
  }
};

const toggleLevel3 = (item) => {
  if (selectedItems.value.has(item.id)) {
    selectedItems.value.delete(item.id);
  } else {
    selectedItems.value.add(item.id);
  }
};

const closeModal = () => {
  // Implement close modal logic
  emit("update:showAddModal", false);
  clearForm()  //清空表单
};

const groupNameError = ref("");    //分组名称错误提示
watch(() => formData.value.name, (newVal) => {
  if(groupNameError.value === "分组名称不能为空" && newVal){
    groupNameError.value = ""
  }
})
const submitForm = async () => {
  // Implement form submission logic
  //校验分组名称
  if (!formData.value.name) {
    groupNameError.value = "分组名称不能为空";
    return;
  }

  console.log({
    ...formData.value,
    permissions: Array.from(selectedItems.value),
  });
  emit("update:showAddModal", false);
  // console.log("value:::", formData.value);

  clearForm()
};
</script>

<style scoped>
/* Add custom styles for indeterminate checkbox if needed */
input[type="checkbox"]:indeterminate {
  background-color: #3b82f6;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 16 16'%3e%3cpath stroke='white' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M4 8h8'/%3e%3c/svg%3e");
}
</style>
