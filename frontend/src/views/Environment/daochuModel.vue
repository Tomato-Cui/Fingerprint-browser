<template>
  <div v-if="props.open" class="fixed inset-0 z-50 overflow-y-auto">
    <!-- Backdrop -->
    <div class="fixed inset-0 bg-black bg-opacity-25 transition-opacity"></div>

    <!-- Modal -->
    <div class="flex min-h-screen items-center justify-center p-4">
      <div class="relative w-full max-w-lg transform overflow-hidden rounded-lg bg-white shadow-xl transition-all">
        <!-- Header -->
        <div class="flex items-center justify-between border-b px-4 py-3">
          <h3 class="text-lg font-medium">导出</h3>
          <button
              @click="closeModal"
              class="rounded-md p-1 hover:bg-gray-100"
          >
            <XIcon class="h-5 w-5 text-gray-500" />
          </button>
        </div>

        <!-- Content -->
        <div class="px-4 py-5">
          <!-- Info Message -->
          <div class="mb-4 flex items-start rounded-md bg-blue-50 p-3 text-sm text-blue-600">
            <InfoIcon class="mr-2 h-5 w-5 flex-shrink-0" />
            <p>注意：选(文本文件)可以导出完整的Cookie信息。</p>
          </div>

          <!-- Environment Selection -->
          <div class="mb-4">
            <div class="flex gap-2">
              <button
                  v-for="env in ['环境1', '环境2']"
                  :key="env"
                  class="rounded-full px-3 py-1 text-sm"
                  :class="selectedEnv === env ? 'bg-blue-100 text-blue-600' : 'bg-gray-100 text-gray-600'"
                  @click="selectedEnv = env"
              >
                {{ env }}
                <XCircleIcon v-if="env === '环境2'" class="ml-1 inline-block h-4 w-4" />
              </button>
            </div>
          </div>

          <!-- File Type Selection -->
          <div class="mb-4">
            <p class="mb-2 text-sm text-gray-700">文件类型</p>
            <div class="flex gap-4">
              <label class="flex items-center">
                <input
                    type="radio"
                    v-model="fileType"
                    value="xlsx"
                    class="text-blue-600 focus:ring-blue-500"
                />
                <span class="ml-2 text-sm">Excel文件(*.xlsx)</span>
              </label>
              <label class="flex items-center">
                <input
                    type="radio"
                    v-model="fileType"
                    value="txt"
                    class="text-blue-600 focus:ring-blue-500"
                />
                <span class="ml-2 text-sm">文本文件(*.txt)</span>
              </label>
            </div>
          </div>

          <!-- Export Content -->
          <div class="rounded-lg border border-gray-200 p-4">
            <div class="grid grid-cols-2 gap-4">
              <!-- Left Column -->
              <div class="space-y-3">
                <label class="flex items-center justify-between">
                  <div class="flex items-center">
                    <input
                        type="checkbox"
                        v-model="exportOptions.browserInfo"
                        class="text-blue-600 focus:ring-blue-500"
                    />
                    <span class="ml-2 text-sm">浏览器信息</span>
                  </div>
                  <ChevronRightIcon class="h-4 w-4 text-gray-400" />
                </label>
                <label class="flex items-center justify-between">
                  <div class="flex items-center">
                    <input
                        type="checkbox"
                        v-model="exportOptions.accountInfo"
                        class="text-blue-600 focus:ring-blue-500"
                    />
                    <span class="ml-2 text-sm">账号信息</span>
                  </div>
                  <ChevronRightIcon class="h-4 w-4 text-gray-400" />
                </label>
                <label class="flex items-center justify-between">
                  <div class="flex items-center">
                    <input
                        type="checkbox"
                        v-model="exportOptions.proxyConfig"
                        class="text-blue-600 focus:ring-blue-500"
                    />
                    <span class="ml-2 text-sm">代理配置</span>
                  </div>
                  <ChevronRightIcon class="h-4 w-4 text-gray-400" />
                </label>
                <label class="flex items-center justify-between">
                  <div class="flex items-center">
                    <input
                        type="checkbox"
                        v-model="exportOptions.fingerprint"
                        class="text-blue-600 focus:ring-blue-500"
                    />
                    <span class="ml-2 text-sm">指纹信息</span>
                  </div>
                  <ChevronRightIcon class="h-4 w-4 text-gray-400" />
                </label>
              </div>

              <!-- Right Column -->
              <div class="space-y-3">
                <label class="flex items-center">
                  <input
                      type="checkbox"
                      v-model="exportOptions.envOrder"
                      class="text-blue-600 focus:ring-blue-500"
                  />
                  <span class="ml-2 text-sm">环境序号</span>
                </label>
                <label class="flex items-center">
                  <input
                      type="checkbox"
                      v-model="exportOptions.envId"
                      class="text-blue-600 focus:ring-blue-500"
                  />
                  <span class="ml-2 text-sm">环境ID</span>
                </label>
                <label class="flex items-center">
                  <input
                      type="checkbox"
                      v-model="exportOptions.groupName"
                      class="text-blue-600 focus:ring-blue-500"
                  />
                  <span class="ml-2 text-sm">分组名称</span>
                </label>
                <label class="flex items-center">
                  <input
                      type="checkbox"
                      v-model="exportOptions.name"
                      class="text-blue-600 focus:ring-blue-500"
                  />
                  <span class="ml-2 text-sm">名称</span>
                </label>
                <label class="flex items-center">
                  <input
                      type="checkbox"
                      v-model="exportOptions.remarks"
                      class="text-blue-600 focus:ring-blue-500"
                  />
                  <span class="ml-2 text-sm">备注</span>
                </label>
                <label class="flex items-center">
                  <input
                      type="checkbox"
                      v-model="exportOptions.openUrl"
                      class="text-blue-600 focus:ring-blue-500"
                  />
                  <span class="ml-2 text-sm">打开URL</span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="flex justify-end gap-2 border-t bg-gray-50 px-4 py-3">
          <button
              @click="closeModal"
              class="rounded-md px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
          >
            取消
          </button>
          <button
              @click="closeModal"
              class="rounded-md bg-blue-600 px-4 py-2 text-sm text-white hover:bg-blue-700"
          >
            确定
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { XIcon, InfoIcon, ChevronRightIcon, XCircleIcon } from 'lucide-vue-next'

const props = defineProps({
  open: false,
  title: '',
})

const emit = defineEmits(['close', 'confirm'])

const selectedEnv = ref('环境1')
const fileType = ref('xlsx')
const exportOptions = reactive({
  browserInfo: true,
  accountInfo: false,
  proxyConfig: false,
  fingerprint: false,
  envOrder: true,
  envId: true,
  groupName: false,
  name: true,
  remarks: false,
  openUrl: false
})

const closeModal = () => {
  emit('close')
}

const handleConfirm = () => {
  emit('confirm', {
    environment: selectedEnv.value,
    fileType: fileType.value,
    options: { ...exportOptions }
  })
}
</script>