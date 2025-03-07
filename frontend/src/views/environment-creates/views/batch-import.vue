<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import BasicSetting from "../components/batch-import/basic-setting.vue";
import AdvancedSetting from "../components/batch-import/advanced-setting.vue";
import { batchCreateHandle } from "../index";
import { convertToCSV, downloadCSV } from "@/util/lib";
import EncryptModel from "../components/encrypt-model.vue";
import Loading from "@/components/loading/index.vue";
import { toast } from "vue-sonner";

const router = useRouter();

const file = ref<any>();
const readData = ref<any>();
const readerHandle = (data: any) => {
  readData.value = data;
};

const createing = ref(false);
const encrypts = ref<any[]>([]);

const handleSubmit = async () => {
  if (readData.value && readData.value.length > 0) {
    const loadingTimeout = setTimeout(() => {
      createing.value = true;
    }, 1000);
    readData.value.map(async (item: any) => {
      let name = item.name;

      try {
        let res = await batchCreateHandle(name);
        if (res.code == 1) {
          toast.success("创建成功", { position: "top-right" });
          encrypts.value = res.data
            .filter((item: any) => item.encrypt)
            .map((item: any) => ({ name: item.name, encrypt: item.encrypt }));
        }
      } catch (err: any) {
        toast.warning(err);
      }
    });

    clearTimeout(loadingTimeout);
    createing.value = false;
  }
};
</script>
<template>
    <div class="grow overflow-hidden">
      <div class="overflow-y-auto h-full my-3">
        <BasicSetting :file="file" @reader="readerHandle" />
        <AdvancedSetting />
      </div>
    </div>
    <div class="border-t flex gap-x-4 justify-end items-center p-3">
      <button
        @click="router.go(-1)"
        class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 active:scale-[.98]"
      >
        取消
      </button>
      <button
        @click="handleSubmit"
        class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
      >
        确认
      </button>
    </div>

    <Loading :open="createing">
      <LoadingIcon />
      <span class="text-xs whitespace-nowrap">创建浏览器环境中, 稍等.</span>
    </Loading>
    <EncryptModel
      :open="encrypts.length > 0"
      title="密码本"
      @close="encrypts = []"
    >
      <div
        class="grid grid-cols-9 mx-8 py-6 rounded-md max-h-[24rem] overflow-auto"
      >
        <div class="col-span-2 border-b text-sm text-right py-1">环境名称</div>
        <div class="col-span-1 border-b text-sm text-right py-1"></div>
        <div class="col-span-6 border-b text-sm text-left py-1">密码</div>

        <template v-for="item in encrypts" :key="item.name">
          <div
            class="col-span-2 text-xs flex items-center h-8 justify-end border-b"
          >
            {{ item.name }}
          </div>
          <div class="col-span-1 text-sm text-right border-b"></div>
          <div
            class="col-span-6 truncate overflow-hidden text-xs flex items-center h-8 justify-start border-b"
          >
            {{ item.encrypt }}
          </div>
        </template>
      </div>

      <div class="border-t py-4 flex justify-end px-4">
        <button
          @click="downloadCSV(convertToCSV(encrypts))"
          class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
        >
          下载
        </button>
      </div>
    </EncryptModel>
</template>
