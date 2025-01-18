<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import Input from "@/components/input.vue";
import Radio from "../radio.vue";
import { Iconrefresh } from "@/assets/icons";
import { Textarea } from "@/components/ui/textarea";
import {
  templaeFormtData as data,
  useEnvironmentAdvancedFormStore,
} from "@/stores/form/environment-advanced";
import { Switch } from "@/components/ui/switch";
import { FolderIcon, FolderPlusIcon } from "lucide-vue-next";
import { ref } from "vue";
import * as XLSX from "xlsx";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";

const props = defineProps<{
  file: any;
}>();
const emits = defineEmits(["reader"]);

const { forms, randomUA } = useEnvironmentAdvancedFormStore();

const isDragging = ref(false);
const isUploaded = ref(false);
const uploadedFileName = ref("");
const fileInput = ref<any>(null);

// 触发文件选择
const triggerFileInput = () => {
  fileInput.value.click();
};

// 处理文件选择
const handleFileSelect = async (e: any) => {
  const file = e.target.files[0];
  if (file) {
    if (
      file.type ===
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" &&
      file.size <= 5 * 1024 * 1024
    ) {
      emits("reader", await readExcelFile(file));
      isUploaded.value = true;
      uploadedFileName.value = file.name;
    } else {
      alert("文件类型不正确或大小超过5MB");
    }
  }
};

// 读取 Excel 文件
const readExcelFile = (file: File) => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e: any) => {
      const data = new Uint8Array(e.target.result);
      const workbook = XLSX.read(data, { type: "array" });
      const firstSheetName = workbook.SheetNames[0]; // 获取第一个工作表名
      const worksheet = workbook.Sheets[firstSheetName]; // 获取第一个工作表
      const jsonData = XLSX.utils.sheet_to_json(worksheet); // 将工作表转换为 JSON
      resolve(jsonData);
    };
    reader.onerror = (error) => {
      reject(error);
    };
    reader.readAsArrayBuffer(file);
  });
};

// 拖拽相关方法
const handleDragOver = (e: any) => {
  e.preventDefault();
  isDragging.value = true;
};

const handleDragLeave = (e: any) => {
  e.preventDefault();
  isDragging.value = false;
};

const handleDrop = async (e: any) => {
  e.preventDefault();
  isDragging.value = false;
  const files = e.dataTransfer.files;
  if (files.length > 0) {
    const file = files[0];
    if (
      file.type ===
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" &&
      file.size <= 5 * 1024 * 1024
    ) {
      // let _data = await readExcelFile(file); // 读取 Excel 文件
      isUploaded.value = true;
      uploadedFileName.value = file.name;
    } else {
      alert("文件类型不正确或大小超过5MB");
    }
  }
};
</script>

<template>
  <Accordion
    type="single"
    class="w-full px-4"
    collapsible
    default-value="basic-setting"
  >
    <AccordionItem class="border-0" value="basic-setting">
      <AccordionTrigger
        class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
        >基础设置</AccordionTrigger
      >
      <AccordionContent class="px-10 space-y-4 py-4">
        <div class="flex items-center">
          <p class="w-32 text-left">名称</p>
          <div class="relative text-sm w-1/2 flex flex-col">
            <Input
              v-model="forms.name"
              type="text"
              :placeholder="data.name.placeholder"
              maxlength="20"
            />
          </div>
        </div>
        <div class="flex items-center">
          <p class="w-32 text-left">浏览器</p>
          <Radio
            :check="forms.browser"
            :data="data.browserTypes"
            @update:check="(v:any) => (forms.browser = {value: v.value})"
          />
        </div>

        <div class="space-y-2 flex items-center">
          <p class="w-32 text-left">操作系统</p>
          <Radio
            :check="forms.os"
            :data="data.osTypes"
            @update:check="(v:any) => (forms.os = {value: v.value})"
          />
        </div>

        <div class="flex">
          <div class="flex items-start justify-start">
            <p class="w-32 text-left text-sm">UA</p>
          </div>
          <div class="w-full relative">
            <Textarea class="w-full" :placeholder="forms.ua" disabled />
            <div class="absolute right-2 top-2 flex gap-2" @click="randomUA">
              <Iconrefresh class="w-4 h-4 fill-gray-800 cursor-pointer" />
            </div>
          </div>
        </div>

        <div className="flex flex-col">
          <div class="flex">
            <p class="w-32 text-left text-sm">端对端加密</p>
            <Radio
              :check="forms.encryPtSwitch"
              :data="data.encrypts"
              @update:check="(v:any) => (forms.encryPtSwitch = v)"
            />
          </div>
          <div className="text-muted-foreground pl-32 pt-2 text-xs">
            开启后，只有通过密钥授权的账号和设备可以解密和访问此环境
          </div>
        </div>

        <div class="flex">
          <p class="w-32 text-left text-sm">批量导入</p>

          <div class="grow">
            <div
              class="border-2 border-dashed border-gray-200 rounded-lg p-8 text-center h-60"
              @dragover.prevent="handleDragOver"
              @dragleave.prevent="handleDragLeave"
              @drop.prevent="handleDrop"
              :class="{ 'bg-blue-50 border-blue-200': isDragging }"
            >
              <!-- 上传成功后的显示区域 -->
              <div
                v-if="isUploaded"
                class="mb-4 flex flex-col items-center justify-center"
              >
                <CheckCircleIcon class="w-12 h-12 mx-auto text-green-500" />
                <div>
                  <p class="text-green-600">文件上传成功！</p>
                  <p class="text-sm text-gray-500">
                    文件名：{{ uploadedFileName }}
                  </p>
                </div>
              </div>

              <!-- 上传前的显示区域 -->
              <div v-else class="w-full">
                <div class="mb-4">
                  <FolderIcon class="w-12 h-12 mx-auto text-gray-300" />
                </div>

                <p class="text-gray-600 mb-2">请拖拽到此处或点击上传</p>
                <p class="text-sm text-gray-500">
                  请将编辑好的文件在此处上传，仅支持.xlsx类型文件，最大不超过5MB。
                </p>
              </div>

              <!-- 文件输入 -->
              <input
                type="file"
                ref="fileInput"
                accept=".xlsx"
                class="hidden"
                v-bind="props.file"
                @change="handleFileSelect"
              />

              <!-- 上传按钮 -->
              <div class="flex justify-center">
                <button
                  class="mt-4 px-6 py-2 text-sm border border-gray-200 rounded-md hover:bg-gray-50 flex justify-center items-center"
                  @click="triggerFileInput"
                >
                  <FolderPlusIcon class="w-6 h-6 mx-auto text-gray-300 mr-2" />
                  上传文件
                </button>
              </div>
            </div>

            <div class="flex justify-between mt-2">
              <button
                class="text-sm border whitespace-nowrap rounded-md px-2 py-2 flex justify-center items-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
              >
                下载模板
              </button>
              <p class="text-[#5050FA] underline hover:cursor-not-allowed">
                从其他浏览器导入
              </p>
            </div>
          </div>
        </div>

        <div className="flex">
          <div class="w-32">
            <p class="text-left text-sm">启动后</p>
          </div>
          <div class="grow">
            <RadioGroup
              :defaultValue="forms.setting.startedBrowserDefaultUrl"
              v-model="forms.setting.startedBrowserDefaultUrl"
            >
              <div className="flex items-center space-x-2">
                <RadioGroupItem
                  value="0"
                  id="option-one"
                  class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]"
                />
                <Label htmlFor="option-one">继续浏览上次打开的网页</Label>
              </div>
              <div className="flex items-center space-x-2">
                <RadioGroupItem
                  value="1"
                  id="option-two"
                  class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]"
                />
                <Label htmlFor="option-two">打开指定网页</Label>
              </div>
            </RadioGroup>
          </div>
        </div>
        <div className="flex">
          <div class="w-32">
            <p class="text-left text-sm">其他设置</p>
          </div>
          <div class="grow flex flex-col gap-y-1">
            <div class="flex gap-x-2">
              <Switch
                :checked="forms.setting.startedBrowserAccountUrlIsOpen"
                @update:checked="
                  forms.setting.startedBrowserAccountUrlIsOpen =
                    !forms.setting.startedBrowserAccountUrlIsOpen
                "
                class="data-[state=checked]:bg-[#5050FA]"
              />
              <p>同时打开「账号」已配置的平台网页</p>
            </div>
            <div class="flex gap-x-2">
              <Switch
                class="data-[state=checked]:bg-[#5050FA]"
                :checked="forms.setting.startedBrowserStayCheckHome"
                @update:checked="
                  forms.setting.startedBrowserStayCheckHome =
                    !forms.setting.startedBrowserStayCheckHome
                "
              />
              <p>打开后默认停留在检测页</p>
            </div>
          </div>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
