<template>
  <Teleport to="body" class="z-20">
    <div
      v-if="props.authModel"
      class="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50"
    >
      <div class="fixed inset-0 bg-black/20 flex items-center justify-center">
        <div class="bg-white rounded-lg w-[800px] min-w-xl">
          <!-- Modal Header -->
          <div class="flex justify-between items-center p-4 border-b">
            <h2 class="text-lg font-medium">授权</h2>
            <button
              @click="closeModal"
              class="text-gray-400 hover:text-gray-600"
            >
              <XIcon class="w-5 h-5" />
            </button>
          </div>

          <!-- Modal Content -->
          <div class="px-[100px] py-[30px]">
            <!-- Group Section -->
            <div class="mb-6 flex items-center gap-5">
              <div class="text-gray-600 mb-2 w-[60px]">分组</div>
              <div class="bg-blue-50 px-4 py-2 rounded">跨境电商</div>
            </div>
  
            <!-- Permissions Section -->
            <div class="">
              <!-- <div class="text-sm text-gray-500 mb-4 flex items-center gap-5">
                <div class="text-gray-600 mb-2 w-[60px]">权限管理</div>
                <div class="x-4 py-2 rounded">
                  请为用户组分配权限功能，同个用户组的用户权限是相同一致
                </div>
              </div> -->

              <div class="flex gap-5">
                <div class="w-[60px]">权限</div>
                <div class="flex border rounded flex-1">
                  <!-- Main Permissions -->
                  <div class="w-1/2 border-r h-[336px] overflow-auto">
                    <div
                      v-for="perm in permissions"
                      :key="perm.id"
                      class="flex items-center p-3 hover:bg-gray-50 cursor-pointer"
                      :class="{ 'bg-blue-50': selectedParent === perm.id }"
                      @click="selectParent(perm.id)"
                    >
                      <input
                        type="checkbox"
                        :checked="isParentChecked(perm)"
                        :indeterminate="isParentIndeterminate(perm)"
                        @change="toggleParent(perm)"
                        class="mr-2 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      />
                      <span class="text-gray-700">{{ perm.name }}</span>
                      <ChevronRightIcon
                        v-if="perm.children?.length"
                        class="w-4 h-4 ml-auto text-gray-400"
                      />
                    </div>
                  </div>

                  <!-- Sub Permissions -->
                  <div
                    class="w-1/2 h-[336px] overflow-auto"
                    v-if="selectedParent !== null"
                  >
                    <div
                      v-for="child in currentChildren"
                      :key="child.id"
                      class="flex items-center p-3 hover:bg-gray-50"
                    >
                      <input
                        type="checkbox"
                        v-model="selectedPermissions"
                        :value="child.id"
                        @change="updateParentState"
                        class="mr-2 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      />
                      <span class="text-gray-700">{{ child.name }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Modal Footer -->
          <div class="flex justify-end space-x-3 p-4 border-t">
            <button
              @click="closeModal"
              class="px-6 py-2 border rounded-md hover:bg-gray-50"
            >
              取消
            </button>
            <button
              @click="handleSubmit"
              class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              确定
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { ref, computed, defineEmits, defineProps, watch } from "vue";
import { XIcon, ChevronRightIcon } from "lucide-vue-next";
const props = defineProps({
  authModel: {
    type: Boolean,
    required: true,
  },
});
const emit = defineEmits(["update:authModel"]);
watch(
  () => props.authModel,
  (val) => {
    emit("update:authModel", props.authModel);
  }
);

// Mock permissions data
const permissions = ref([
  {
    id: 1,
    name: "团队管理",
    children: [],
  },
  {
    id: 2,
    name: "环境管理",
    children: [],
  },
  {
    id: 3,
    name: "分组管理",
    children: [
      { id: 31, name: "分组管理页面" },
      { id: 32, name: "新建分组" },
      { id: 33, name: "删除分组" },
      { id: 34, name: "修改分组" },
    ],
  },
  {
    id: 4,
    name: "应用中心",
    children: [],
  },
  {
    id: 5,
    name: "回收站",
    children: [],
  },
  {
    id: 6,
    name: "RPA",
    children: [],
  },
  {
    id: 7,
    name: "代理管理",
    children: [],
  },
  {
    id: 8,
    name: "12管理",
    children: [],
  },
]);

const selectedParent = ref(null);
const selectedPermissions = ref([]);

const currentChildren = computed(() => {
  if (selectedParent.value === null) return [];
  const parent = permissions.value.find((p) => p.id === selectedParent.value);
  return parent?.children || [];
});

const selectParent = (id) => {
  selectedParent.value = id;
};

const isParentChecked = (parent) => {
  if (!parent.children?.length)
    return selectedPermissions.value.includes(parent.id);
  return parent.children.every((child) =>
    selectedPermissions.value.includes(child.id)
  );
};

const isParentIndeterminate = (parent) => {
  if (!parent.children?.length) return false;
  const childrenSelected = parent.children.some((child) =>
    selectedPermissions.value.includes(child.id)
  );
  return childrenSelected && !isParentChecked(parent);
};

const toggleParent = (parent) => {
  if (!parent.children?.length) {
    const index = selectedPermissions.value.indexOf(parent.id);
    if (index === -1) {
      selectedPermissions.value.push(parent.id);
    } else {
      selectedPermissions.value.splice(index, 1);
    }
    return;
  }

  const childIds = parent.children.map((child) => child.id);
  if (isParentChecked(parent)) {
    selectedPermissions.value = selectedPermissions.value.filter(
      (id) => !childIds.includes(id)
    );
  } else {
    selectedPermissions.value = [
      ...new Set([...selectedPermissions.value, ...childIds]),
    ];
  }
};

const updateParentState = () => {
  // This will trigger recomputation of checked and indeterminate states
};

const closeModal = () => {
  // Emit close event
  emit("update:authModel", false);
};

const handleSubmit = () => {
  console.log("Selected permissions:", selectedPermissions.value);
  emit("update:authModel", false);
  closeModal();
};
</script>

<style scoped>
input[type="checkbox"]:indeterminate {
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3e%3cpath fill='none' stroke='%23fff' stroke-linecap='round' stroke-linejoin='round' stroke-width='3' d='M6 10h8'/%3e%3c/svg%3e");
  background-color: #2563eb;
  border-color: #2563eb;
}
</style>
