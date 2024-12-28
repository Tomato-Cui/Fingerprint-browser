<script setup lang="ts">
import { XIcon } from "lucide-vue-next";

const props = defineProps({
  itemData: Object,
  open: false,
  title: "",
});

const emit = defineEmits(["close"]);
</script>

<template>
  <Transition
    class="z-20"
    enter-active-class="transition duration-300 ease-in-out transform"
    enter-from-class="translate-x-full"
    enter-to-class="translate-x-0"
    leave-active-class="transition duration-300 ease-in-out transform"
    leave-from-class="translate-x-0"
    leave-to-class="translate-x-full"
  >
    <div
      v-if="props.open"
      class="fixed inset-y-0 right-0 w-[500px] bg-white shadow-xl overflow-y-auto flex flex-col"
    >
      <div class="flex flex-col flex-1 p-6">
        <div
          class="flex justify-between items-center pb-2 mb-6 border-b border-gray-300"
        >
          <h2 class="text-xl font-medium">账号详情</h2>
          <button @click="() => emit('close')">
            <XIcon class="w-6 h-6 text-gray-400 hover:text-gray-600" />
          </button>
        </div>

        <div class="w-full max-w-lg bg-white rounded-lg">
          <!-- Modal Content -->
          <div class="">
            <div class="space-y-8">
              <!-- Account Info -->
              <div class="flex gap-3 items-center">
                <img
                  alt=""
                  class="w-10 h-10"
                  src="../../../src/assets/images/guge.svg"
                />
                <div>
                  <h3 class="text-lg font-bold">GM1145</h3>
                  <p class="text-gray-500">(HeomicazTendi...)</p>
                </div>
              </div>

              <!-- Platform -->
              <div class="space-y-1">
                <h3 class="font-bold text-gray-500">所属平台：</h3>
                <p>{{ props.itemData.platform }}</p>
              </div>

              <!-- Environment -->
              <div class="space-y-1">
                <h3 class="font-bold text-gray-500">所属环境：</h3>
                <div class="inline-block px-3 py-1 bg-gray-100 rounded">
                  <span>1145</span>
                  <span class="ml-2 text-gray-400">--</span>
                </div>
              </div>

              <!-- Authorization Members -->
              <div class="space-y-3">
                <h3 class="font-bold text-gray-500">授权成员：</h3>
                <div class="flex flex-wrap gap-2">
                  <div v-for="i in 6" :key="i" class="flex space-x-1">
                    <span
                      class="px-2 py-1 text-blue-700 bg-blue-100 rounded"
                      style="padding: 10px"
                      >员工
                      <span
                        class="px-2 py-1 text-green-700 bg-green-100 rounded"
                        >员工</span
                      >
                    </span>
                  </div>
                </div>
              </div>

              <!-- Add Time -->
              <div class="space-y-1">
                <h3 class="font-bold text-gray-500">添加时间：</h3>
                <p>2023-07-09 09:43:15</p>
              </div>

              <!-- Added By -->
              <div class="space-y-1">
                <h3 class="font-bold text-gray-500">添加人：</h3>
                <p>本人</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped></style>
