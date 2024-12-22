<template>
  <div v-if="props.open" class="overflow-y-auto fixed inset-0 z-50">
    <!-- Backdrop -->
    <div class="fixed inset-0 bg-black bg-opacity-25 transition-opacity"></div>

    <!-- Modal -->
    <div class="flex justify-center items-center p-4 min-h-screen">
      <div
        class="overflow-hidden relative w-full max-w-md bg-white rounded-lg shadow-xl transition-all transform"
        style="min-width: 500px"
      >
        <!-- Header -->
        <div class="flex justify-between items-center px-4 py-3 border-b">
          <h3 class="text-lg font-medium">导入</h3>
          <button @click="closeModal" class="p-1 rounded-md hover:bg-gray-100">
            <XIcon class="w-5 h-5 text-gray-500" />
          </button>
        </div>

        <!-- Content -->
        <div class="px-4 py-5">
          <!-- Info Message -->
          <div
            class="flex items-start p-3 mb-4 text-sm text-blue-600 bg-blue-50 rounded-md"
          >
            <InfoIcon class="flex-shrink-0 mr-2 w-5 h-5" />
            <p>
              仅支持后缀名为xlsx格式，文件大小限10MB以内，每次导入条数不超过2500条
            </p>
          </div>

          <!-- Template Selection -->
          <div class="mb-4">
            <p class="mb-2 text-sm text-gray-700">模板类型</p>
            <div class="flex gap-2">
              <button
                class="px-3 py-1.5 text-sm rounded-md"
                :class="
                  selectedTemplate === 'hubstudio'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-100 text-gray-700'
                "
                @click="selectedTemplate = 'hubstudio'"
              >
                Hubstudio模板
              </button>
              <button
                class="px-3 py-1.5 text-sm rounded-md"
                :class="
                  selectedTemplate === 'other'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-100 text-gray-700'
                "
                @click="selectedTemplate = 'other'"
              >
                其他模板
              </button>
            </div>
          </div>

          <!-- File Upload Area -->
          <div
            class="flex flex-col justify-center items-center p-6 mb-4 rounded-lg border-2 border-gray-300 border-dashed hover:border-blue-500"
            @dragover.prevent
            @drop.prevent="handleFileDrop"
            @click="triggerFileInput"
          >
            <FolderIcon class="mb-3 w-10 h-10 text-gray-400" />
            <p class="mb-2 text-sm text-gray-600">将文件拖到此处，或点击上传</p>
            <p class="text-xs text-gray-400">
              仅支持xlsx格式，文件大小限10MB以内
            </p>
            <input
              type="file"
              ref="fileInput"
              class="hidden"
              accept=".xlsx"
              @change="handleFileSelect"
            />
          </div>

          <!-- Advanced Settings -->
          <div class="mb-4">
            <button
              class="flex items-center text-sm text-blue-600"
              @click="toggleAdvanced"
            >
              高级设置
              <ChevronDownIcon
                class="ml-1 w-4 h-4 transition-transform"
                :class="{ 'rotate-180': showAdvanced }"
              />
            </button>
          </div>
        </div>

        <!-- Footer -->
        <div class="flex gap-2 justify-end px-4 py-3 bg-gray-50 border-t">
          <button
            @click="closeModal"
            class="px-4 py-2 text-sm text-gray-700 rounded-md hover:bg-gray-100"
          >
            取消
          </button>
          <button
            @click="closeModal"
            class="px-4 py-2 text-sm text-white bg-blue-600 rounded-md hover:bg-blue-700"
          >
            确定
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { XIcon, InfoIcon, FolderIcon, ChevronDownIcon } from "lucide-vue-next";

const props = defineProps({
  open: false,
  title: "",
});

const emit = defineEmits(["close", "confirm"]);

const selectedTemplate = ref("hubstudio");
const showAdvanced = ref(false);
const fileInput = ref(null);

const closeModal = () => {
  emit("close");
};

const handleConfirm = () => {
  emit("confirm");
};

const toggleAdvanced = () => {
  showAdvanced.value = !showAdvanced.value;
};

const triggerFileInput = () => {
  fileInput.value.click();
};

const handleFileSelect = (event) => {
  const file = event.target.files[0];
  if (file) {
    // Handle file upload logic here
    console.log("Selected file:", file);
  }
};

const handleFileDrop = (event) => {
  const file = event.dataTransfer.files[0];
  if (file) {
    // Handle file upload logic here
    console.log("Dropped file:", file);
  }
};
</script>
