<template>
  <Model :open="props.open" @close="emit('close')" class="min-w-[700px]">
    <!-- 内容区域 -->
    <div class="p-6 space-y-6">
      <!-- 位置选择 -->
      <div class="flex space-x-8">
        <h3 class="text-gray-700">位置</h3>

        <div class="space-y-3">
          <!-- 此电脑选项 -->
          <label class="flex gap-2 items-start cursor-pointer">
            <input
              type="checkbox"
              v-model="clearLocal"
              class="mt-1 text-blue-500 rounded focus:ring-blue-500"
            />
            <div>
              <div class="text-gray-700">此电脑</div>
              <div class="text-sm text-gray-500">
                清除后，浏览器中网站的登录状态等将会全部失效
              </div>
            </div>
          </label>

          <!-- 云端选项 -->
          <label class="flex gap-2 items-start cursor-pointer">
            <input
              type="checkbox"
              v-model="clearCloud"
              class="mt-1 text-blue-500 rounded focus:ring-blue-500"
            />
            <div>
              <div class="text-gray-700">云端</div>
              <div class="text-sm text-gray-500">清除云端缓存数据</div>
            </div>
          </label>
        </div>
      </div>

      <!-- 表格 -->
      <div class="border rounded-lg max-h-[300px] overflow-y-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="sticky top-0 z-10 bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-700">
                环境
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-700">
                所属平台
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-700">
                分组
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-700">
                代理信息
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="item in props.data"
              :key="item.id"
              class="hover:bg-gray-50"
            >
              <td class="px-4 py-4 text-sm text-gray-900">{{ item.name }}</td>
              <td class="px-4 py-4 text-sm text-gray-900">
                {{ item.accounts.platform || "--" }}
              </td>
              <td class="px-4 py-4 text-sm text-gray-900">
                {{ item.group_name || "--" }}
              </td>
              <td class="px-4 py-4 text-sm text-gray-500">
                {{ item.proxy_host || "暂无代理" }}
              </td>
            </tr>
          </tbody>
        </table>

        <!-- 分页 -->
        <!-- <div class="flex justify-between items-center px-4 py-3 bg-white border-t">
                    <div class="text-sm text-gray-700">
                        共 {{ total }} 项
                    </div>
                    <div class="flex gap-2 items-center">
                        <button class="p-1 rounded hover:bg-gray-100" :disabled="currentPage === 1"
                            @click="currentPage--">
                            <ChevronLeftIcon class="w-5 h-5" />
                        </button>
                        <span class="min-w-[2rem] text-center">{{ currentPage }}</span>
                        <button class="p-1 rounded hover:bg-gray-100" :disabled="currentPage === totalPages"
                            @click="currentPage++">
                            <ChevronRightIcon class="w-5 h-5" />
                        </button>
                        <select v-model="pageSize"
                            class="py-1 pr-8 pl-2 ml-2 text-sm rounded-md border focus:outline-none focus:ring-2 focus:ring-blue-500">
                            <option value="10">10条/页</option>
                            <option value="20">20条/页</option>
                            <option value="50">50条/页</option>
                        </select>
                    </div>
                </div> -->
      </div>
    </div>

    <!-- footer -->
    <div
      class="flex justify-end items-center px-4 py-2 space-x-3 w-full border-t"
    >
      <cancelButton @click="cancel">取消</cancelButton>
      <primaryButton @click="commit">确认</primaryButton>
    </div>
  </Model>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Model } from "@/components/model";
import primaryButton from "@/components/button/primary-button.vue";
import cancelButton from "@/components/button/cancel-button.vue";
import { toast } from "vue-sonner";

const emit = defineEmits(["close"]);
const props = defineProps<{
  open: Boolean;
  data: any[];
}>();

const clearLocal = ref(false);
const clearCloud = ref(false);
// const pageSize = ref(10)
// const total = ref(2)

// const tableData = ref([
//     {
//         id: 1,
//         selected: true,
//         name: 'P-1',
//         platform: 'P-1',
//         group: '',
//         proxy: '- -',
//         status: 'running'
//     },
//     {
//         id: 2,
//         selected: false,
//         name: 'P-2',
//         platform: 'P-1',
//         group: '',
//         proxy: '- -',
//         status: 'stopped'
//     }
// ])

// const selectAll = computed({
//     get: () => tableData.value.every(item => item.selected),
//     set: (value) => tableData.value.forEach(item => item.selected = value)
// })

// const totalPages = computed(() => Math.ceil(total.value / pageSize.value))

// const toggleSelectAll = () => {
//     const newValue = !selectAll.value
//     tableData.value.forEach(item => item.selected = newValue)
// }

// const toggleStatus = (item: any) => {
//     item.status = item.status === 'running' ? 'stopped' : 'running'
// }

const cancel = () => {
  emit("close");
};
const commit = () => {
  toast.success("清除缓存成功");
  emit("close");
};
</script>
