<script setup>
import { Model } from "@/components/model/index";
import { ref, defineEmits, defineProps, watch, onMounted } from "vue";
import { XIcon, CheckIcon, SearchIcon } from "lucide-vue-next";
import { PrimaryButton, CancelButton, LinkButton } from "@/components/button";
import {
  environment_query,
  environment_query_by_extension,
} from "@/commands/environment";
import {
  extension_environmnet_use_extension,
  extension_environmnet_remove_extension,
} from "@/commands/extension";
import Input from "@/components/input.vue";
import { cn } from "@/util/lib";

const props = defineProps({
  open: false,
  title: "",
  extensionUuid: "",
});

const emit = defineEmits(["close"]);
const tabIndex = ref(0);
const data = ref([]);

const loadData = () => {
  environment_query(0, 100).then((res) => {
    let { data: data_, total } = res.data;
    data.value = data_;
  });
};

onMounted(async () => {
  loadData();
});

watch(props, async (newProps) => {
  let extensionUuid = newProps.extensionUuid;
  if (extensionUuid) {
    let res = await environment_query_by_extension(extensionUuid, 1, 100);
    checkeds.value = res.data.data;
  }
});

const checkeds = ref([]);
const search = ref("");

const appendHandle = (item) => {
  checkeds.value.push(item);
};
const removeHandle = async (item) => {
  let extensionUuid = props.extensionUuid;
  
  if (extensionUuid) {
    let res = await await extension_environmnet_remove_extension(
      extensionUuid,
      item.uuid
    );
  }
  checkeds.value = checkeds.value.filter((i) => i.uuid != item.uuid);
};
const checkAllHandle = () => {
  checkeds.value = data.value;
};
const removeAllHandle = () => {
  checkeds.value = [];
};
watch(search, (newV) => {
  if (newV) {
    options.value = options.value.filter((item) => item.includes(newV));
  } else {
    options.value = [];
  }
});
const onSubmitHandle = async () => {
  let uuids = checkeds.value.map((item) => item.uuid);

  let res = await extension_environmnet_use_extension(
    props.extensionUuid,
    uuids
  );
  emit("close");
};
</script>

<template>
  <Model
    class="max-w-[1125px]"
    :title="props.title"
    description="(请选择需要添加该扩展的环境)"
    :open="props.open"
    @close="() => emit('close')"
  >
    <!-- 主体 -->
    <div class="bg-white rounded-lg w-full px-6">
      <div class="space-y-4 mb-2">
        <div class="flex items-center text-sm text-gray-600 gap-x-4">
          <span class="text-red-500">*</span> 扩展名称: <span>Google 翻译</span>
        </div>
      </div>

      <!-- Permissions -->
      <div class="w-full rounded overflow-hidden">
        <div class="flex w-full gap-x-4">
          <div class="flex-1 flex items-center text-sm h-12 gap-x-4">
            <p
              :class="
                cn(
                  'cursor-pointer',
                  tabIndex == 0 && 'border-b-2 border-blue-500 text-blue-500'
                )
              "
              @click="() => (tabIndex = 0)"
            >
              按环境
            </p>
            <p
              :class="
                cn(
                  'cursor-pointer',
                  tabIndex == 1 && 'border-b-2 border-blue-500 text-blue-500'
                )
              "
              @click="() => (tabIndex = 1)"
            >
              按平台
            </p>
          </div>
          <div class="flex-1 flex justify-between items-center text-sm h-12">
            <p class="flex items-center">已选环境({{ checkeds.length }})</p>
            <LinkButton @click="() => removeAllHandle()">清空已选</LinkButton>
          </div>
        </div>
        <div class="flex gap-x-4 h-[28rem]">
          <div
            v-if="tabIndex == 1"
            class="flex-1 border rounded-md flex justify-center items-center text-gray-500 text-sm"
          >
            暂无匹配的数据，请重新搜索
          </div>
          <div
            v-if="tabIndex == 0"
            class="flex-1 border rounded-md flex flex-col p-2"
          >
            <div class="">
              <div class="relative">
                <SearchIcon
                  class="absolute left-2 top-1.5 z-10 text-gray-500"
                />
                <Input
                  class="w-full text-sm text-gray-600 pl-10"
                  v-model="search"
                />
              </div>
              <div class="px-4 flex gap-x-2 text-sm py-2 select-none">
                <input
                  type="checkbox"
                  id="check-all"
                  @change="
                    (v) =>
                      v.target.checked ? checkAllHandle() : removeAllHandle()
                  "
                />
                <label for="check-all">全选</label>
              </div>
            </div>
            <div class="overflow-auto flex-auto flex flex-col gap-y-4 py-1">
              <button
                v-for="item in data"
                @click="
                  () =>
                    checkeds.findIndex((i) => i.uuid == item.uuid) != -1
                      ? removeHandle(item)
                      : appendHandle(item)
                "
                :class="
                  cn(
                    'text-left px-4 hover:text-primary flex justify-between',
                    checkeds.findIndex((i) => i.uuid == item.uuid) != -1
                      ? 'text-primary'
                      : ''
                  )
                "
              >
                {{ item.name }}
                <CheckIcon
                  v-if="checkeds.findIndex((i) => i.uuid == item.uuid) != -1"
                  class="w-3 h-3"
                />
              </button>
            </div>
          </div>
          <div class="border rounded-md flex-1 p-2">
            <div class="overflow-auto flex flex-wrap gap-2">
              <button
                v-for="item in checkeds"
                class="border p-1 px-2 text-sm rounded-md flex justify-center items-center border-blue-500 hover:border-blue-700 hover:text-blue-700 text-blue-600 cursor-default"
              >
                {{ item.name }}
                <XIcon
                  @click="() => removeHandle(item)"
                  class="w-4 h-4 cursor-pointer"
                />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex justify-end space-x-4 py-6">
        <CancelButton @click="() => emit('close')"> 取消 </CancelButton>
        <PrimaryButton @click="onSubmitHandle"> 确定 </PrimaryButton>
      </div>
    </div>
  </Model>
</template>

<style scoped>
/* Add custom styles for indeterminate checkbox if needed */
input[type="checkbox"]:indeterminate {
  background-color: #3b82f6;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 16 16'%3e%3cpath stroke='white' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M4 8h8'/%3e%3c/svg%3e");
}
</style>
