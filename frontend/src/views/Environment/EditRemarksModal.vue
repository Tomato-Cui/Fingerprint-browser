<template>
  <div
    v-if="props.open"
    class="flex fixed inset-0 z-50 justify-center items-center"
  >
    <!-- Overlay -->
    <div class="absolute inset-0 bg-black bg-opacity-20" @click="close"></div>

    <!-- Modal -->
    <div class="relative bg-white rounded-lg w-[480px] shadow-lg">
      <!-- Header -->
      <div class="flex justify-between items-center px-6 py-4">
        <h3 class="text-lg font-medium">修改备注</h3>
        <button @click="close" class="text-gray-400 hover:text-gray-500">
          ×
        </button>
      </div>

      <!-- Content -->
      <div class="px-6 py-4 space-y-4">
        <div class="flex items-center">
          <label class="w-20 text-gray-600">环境序号</label>
          <span class="px-2 py-1 text-blue-800 bg-blue-100 rounded">{{
            envNumber
          }}</span>
        </div>

        <div class="flex items-start">
          <label for="remarks" class="mt-2 w-20 text-gray-600">备注</label>
          <textarea
            id="remarks"
            v-model="remarks"
            rows="4"
            placeholder="请输入备注"
            class="flex-1 px-3 py-2 rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          ></textarea>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex justify-center px-6 py-4 space-x-4">
        <button
          @click="confirm"
          class="px-6 py-2 text-white bg-blue-500 rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        >
          确定
        </button>
        <button
          @click="close"
          class="px-6 py-2 text-gray-600 bg-gray-100 rounded hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2"
        >
          取消
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

const props = defineProps({
  open: false,
  title: "",
  envNumber: {
    type: [String, Number],
    default: "7",
  },
  initialRemarks: {
    type: String,
    default: "",
  },
});

const emit = defineEmits(["close", "confirm"]);

const remarks = ref(props.initialRemarks);

const close = () => {
  emit("close");
};

const confirm = () => {
  emit("confirm", remarks.value);
  close();
};
</script>
