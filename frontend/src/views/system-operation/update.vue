<template>
  <Transition name="modal-fade">
    <div
      v-if="props.open"
      ref="targetDiv"
      class="bg-white rounded-xl w-[322px] max-w-md mx-4 border shadow-xl z-[999]"
    >
      <!-- <div class="border-b">
        <div class="flex w-1/2 text-sm px-2">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="currentTab = tab.id"
            class="flex-1 py-1 pt-3 text-center relative"
            :class="currentTab === tab.id ? 'text-[#5B5BFA]' : 'text-gray-500'"
          >
            {{ tab.name }}
            <div
              class="absolute bottom-0 left-0 w-full h-0.5 bg-[#5B5BFA] transition-transform duration-300"
              :class="currentTab === tab.id ? 'scale-x-100' : 'scale-x-0'"
            ></div>
          </button>
        </div>
      </div> -->

      <!-- Content -->
      <div class="p-1 overflow-hidden">
        <TransitionGroup name="tab-fade" mode="out-in">
          <div v-if="currentTab === 'client'" key="client">
            <div
              v-if="updateStore.state.version"
              class="bg-slate-50 rounded-md p-2 flex gap-x-2"
            >
              <div>
                <h3>版本: {{ updateStore.state.version }}</h3>
                <p class="text-xs text-gray-500 mt-2">
                  更新说明: {{ updateStore.state.note }}
                </p>
              </div>
              <div>
                <div
                  class="border border-gray-600 rounded-full inline-block p-1 cursor-pointer"
                  @click="updateStore.updateDialogOpen"
                >
                  <DownloadIcon class="text-gray-600 text-sm w-4 h-4" />
                </div>
              </div>
            </div>
            <div
              v-else
              class="h-32 flex flex-col items-center justify-center gap-y-4"
            >
              <span><FolderOpenIcon class="w-10 h-10 text-gray-600" /></span>
              <span class="text-sm text-gray-500"
                >当前版本已是最新的版本。</span
              >
            </div>
          </div>

          <div v-else key="core" class="bg-gray-50 rounded-lg p-6">
            <!-- Core Update Content -->
            <div class="flex items-center gap-2 mb-4">
              <span class="text-lg">内核更新</span>
              <span class="text-gray-400 ml-auto">发布时间: 2025.01.13</span>
            </div>
            <ul class="space-y-2 mb-6">
              <li class="flex items-start gap-2">
                <span>1.</span>
                <span>更新内核版本至最新版</span>
              </li>
            </ul>
          </div>
        </TransitionGroup>
      </div>

      <!-- <div class="p-4 flex gap-4 justify-end">
        <button
          @click="$emit('skip')"
          class="px-6 py-2 border rounded-lg hover:bg-gray-50 transition-colors"
        >
          跳过本版本
        </button>
        <button
          @click="$emit('update')"
          class="px-6 py-2 bg-[#5B5BFA] text-white rounded-lg hover:bg-[#4A4AE5] transition-colors"
        >
          立即更新
        </button>
      </div> -->
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, defineEmits, defineProps, onMounted, onUnmounted } from "vue";
import { useUpdateStore } from "@/stores/update";
import { FolderOpenIcon, DownloadIcon } from "lucide-vue-next";

const updateStore = useUpdateStore();

const props = defineProps<{
  open: boolean;
}>();
const currentTab = ref("client");

const emit = defineEmits(["close", "skip", "update"]);

const targetDiv = ref<any>(null);

const handleClick = (event: MouseEvent) => {
  // 判断是否点击到了指定的 div
  if (targetDiv.value && !targetDiv.value.contains(event.target)) {
    if (props.open === true) emit("close"); //未点击到则关闭弹窗
  }
};

// 在组件挂载时添加事件监听
onMounted(() => {
  document.addEventListener("click", handleClick);
});

// 在组件卸载时移除事件监听
onUnmounted(() => {
  document.removeEventListener("click", handleClick);
});
</script>

<style scoped>
/* Modal Fade Animation */
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

/* Tab Content Transition */
.tab-fade-enter-active,
.tab-fade-leave-active {
  transition: all 0.3s ease;
}

.tab-fade-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.tab-fade-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

.tab-fade-leave-active {
  position: absolute;
}
</style>
