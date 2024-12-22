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
        <h3 class="text-lg font-medium">修改账号</h3>
        <button @click="close" class="text-gray-400 hover:text-gray-500">
          ×
        </button>
      </div>

      <!-- Content -->
      <div class="px-6 py-4 space-y-6">
        <div class="flex items-center">
          <label class="w-24 text-gray-600">环境序号</label>
          <span class="px-2 py-1 text-blue-800 bg-blue-100 rounded">{{
            envNumber
          }}</span>
        </div>

        <div class="flex items-center">
          <label class="w-24 text-gray-600">账号平台</label>
          <select
            v-model="platform"
            class="flex-1 px-3 py-2 bg-white rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="none">无</option>
            <option value="google">Google</option>
            <option value="facebook">Facebook</option>
          </select>
        </div>

        <!-- <div class="flex items-start">
          <label class="mt-2 w-24 text-gray-600">标签页</label>
          <textarea
            v-model="urls"
            rows="3"
            placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
            class="flex-1 px-3 py-2 rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          ></textarea>
        </div> -->

        <!-- <div class="flex items-start">
          <label class="mt-2 w-24 text-gray-600">Cookie</label>
          <div class="flex-1 space-y-2">
            <input
              v-model="cookie"
              type="text"
              placeholder="支持格式: JSON、Netscape、Name=Value"
              class="px-3 py-2 w-full rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
            <div class="flex justify-end">
              <button
                @click="mergeCookie"
                class="flex items-center text-sm text-blue-500 hover:text-blue-600"
              >
                <span class="mr-1">+</span>
                合并Cookie
              </button>
            </div>
          </div>
        </div> -->
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
    default: "11",
  },
});

const emit = defineEmits(["close", "confirm"]);

const platform = ref("none");
const urls = ref("");
const cookie = ref("");

const close = () => {
  emit("close");
};

const confirm = () => {
  emit("confirm", {
    platform: platform.value,
    urls: urls.value.split("\n").filter((url) => url.trim()),
    cookie: cookie.value,
  });
  close();
};

const mergeCookie = () => {
  // Implement cookie merging logic here
  console.log("Merging cookies...");
};
</script>
