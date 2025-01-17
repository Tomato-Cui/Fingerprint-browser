<script setup lang="ts">
import { ref } from "vue";
import { RefreshCw, GripVertical } from "lucide-vue-next";
import Model from "@/components/model/model.vue";
import { PrimaryButton, CancelButton } from "@/components/button";

import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

interface Field {
  id: string;
  label: string;
  enabled: boolean;
}

const fields = ref<Field[]>([
  { id: "proxy_number", label: "代理序号", enabled: true },
  { id: "proxy_name", label: "代理名称", enabled: true },
  { id: "proxy_affiliation", label: "代理归属", enabled: true },
  { id: "operation", label: "操作", enabled: true },
  { id: "proxy_status", label: "代理状态", enabled: true },
  { id: "ip_query", label: "IP查询渠道", enabled: true },
  { id: "group", label: "分组", enabled: false },
  { id: "proxy_type", label: "代理类型", enabled: false },
  { id: "proxy_host", label: "代理主机", enabled: false },
  { id: "browser_env", label: "浏览器环境", enabled: false },
  { id: "cloud_phone_env", label: "云手机环境", enabled: false },
  { id: "creation_info", label: "创建信息", enabled: true },
]);

const emit = defineEmits(["update:setFiled"]);

const onConfirm = () => {
  emit("update:setFiled", false);
};

const props = defineProps({
  setFiled: {
    type: Boolean,
    default: false,
  },
});
</script>

<template>
  <Model
    class="flex fixed inset-0 justify-center items-center bg-black/20"
    :open="props.setFiled"
  >
  <div class="flex flex-auto gap-2 justify-end px-2 py-2 ju">

            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <TooltipButton title="筛选" class="p-2.5 rounded border border-gray-200 hover:bg-gray-100">
                  <LogsIcon class="w-5 h-5 text-gray-600" />
                </TooltipButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
           
                  <DropdownMenuCheckboxItem
                   
             
                 
                  </DropdownMenuCheckboxItem>
            
              </DropdownMenuContent>
            </DropdownMenu>
    </div>


    <div class="bg-white w-[480px] rounded-2xl shadow-xl">

      <div class="flex justify-between items-center p-4 border-b">
        <h2 class="text-lg font-medium">自定义表格字段</h2>
        <button class="p-2 rounded-full transition-colors hover:bg-gray-100">
          <RefreshCw class="w-5 h-5 text-gray-600" />
        </button>
      </div>

     
      <div class="p-4 max-h-[70vh] overflow-y-auto">
        <div class="space-y-3">
          <div
            v-for="field in fields"
            :key="field.id"
            class="flex justify-between items-center p-3 bg-gray-50 rounded-lg"
          >
            <div class="flex gap-3 items-center">
              <GripVertical class="w-4 h-4 text-gray-400" />
              <span class="text-gray-700">{{ field.label }}</span>
            </div>
            <label class="inline-flex relative items-center cursor-pointer">
              <input
                type="checkbox"
                v-model="field.enabled"
                class="sr-only peer"
              />
              <div
                class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"
              ></div>
            </label>
          </div>
        </div>
      </div>

  
      <div class="flex gap-3 justify-end p-4 border-t">
    

          <CancelButton
            class="px-4 border border-gray-300"
           @click="() => emit('update:setFiled', false)"
          >
            取消
          </CancelButton>
          
          <PrimaryButton class="px-4 !bg-[#5050FA]"       
            @click="onConfirm">
            确定
          </PrimaryButton>
      </div>
    </div>
  </Model>
</template>
