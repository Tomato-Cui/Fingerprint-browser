<script setup lang="ts">
import {
  IconLinuxOs,
  IconMacOs,
  IconWindowOs,
  IconFirefox,
} from "@/assets/icons/index";
import {
  IconChrome,
  IconFrame,
  IconFrame1,
  IconFrame2,
  IconFrame3,
  IconFrame4,
} from "@/assets/icons/environment-creates/simple-create/index";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import Input from "@/components/input.vue";
import { Textarea } from "@/components/ui/textarea";
import { CopyIcon, FolderIcon, FolderPlusIcon } from "lucide-vue-next";
import Alignment from "../components/alignment.vue";
import { reactive, ref } from "vue";
import * as XLSX from "xlsx";
import { useEnvironmentCreatesFromStore } from "@/stores/form/environment-creates";
import { useImportCenterFromStore } from "@/stores/form/import-center.js";
import { useRouter } from "vue-router";
import Layout from "../layout.vue";
import Radio, { type RadioInterface } from "../components/radio.vue";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";

const environmentCreatesFrom = useEnvironmentCreatesFromStore();
const importFrom = useImportCenterFromStore() as any;
// 状态管理
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
      importFrom.forms.file = await readExcelFile(file);

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

const router = useRouter();

const data = {
  browserTypes: [
    { value: "chrome", icon: IconChrome },
    { value: "firefox", icon: IconFrame },
  ] as RadioInterface[],
  osTypes: [
    { value: "Windows", icon: IconFrame1 },
    { value: "MacOs", icon: IconFrame2 },
    { value: "Android", icon: IconFrame3 },
    { value: "IOS", icon: IconFrame4 },
  ] as RadioInterface[],
  encrypts: [{ value: "开启" }, { value: "关闭" }] as RadioInterface[],
  groups: [{ id: 0, value: "未知分组" }] as any[],
  proxy_methods: [
    { id: 0, value: "新代理" },
    { id: 1, value: "使用现有代理" },
  ],
  timezones: [
    { id: 0, value: "匹配IP" },
    { id: 1, value: "自定义" },
  ],
  webRtcs: [
    { id: 0, value: "转发" },
    { id: 1, value: "隐私" },
    { id: 2, value: "替换" },
    { id: 3, value: "真实" },
    { id: 4, value: "禁用" },
  ],
  geolocations: [
    { id: 0, value: "询问" },
    { id: 1, value: "禁用" },
  ],
  languages: [
    { id: 0, value: "匹配IP" },
    { id: 1, value: "自定义" },
  ],
  resolution: [
    { id: 0, value: "真实" },
    { id: 1, value: "自定义" },
  ],
  font: [
    { id: 0, value: "真实" },
    { id: 1, value: "自定义" },
  ],
  canvas: [
    { id: 0, value: "噪音" },
    { id: 1, value: "真实" },
  ],
  webGLI: [
    { id: 0, value: "噪音" },
    { id: 1, value: "真实" },
  ],
  webGLM: [
    { id: 0, value: "真实" },
    { id: 1, value: "关闭硬件加速" },
  ],
  webGPU: [
    { id: 0, value: "真实" },
    { id: 1, value: "禁用" },
    { id: 2, value: "基于WebGL匹配" },
  ],
  zz: [
    { id: 0, value: "噪音" },
    { id: 1, value: "真实" },
  ],
  yz: [
    { id: 0, value: "隐私" },
    { id: 1, value: "真实" },
  ],
  kg: [
    { id: 0, value: "开启" },
    { id: 1, value: "关闭" },
  ],
};

const proxy_method = ref<RadioInterface>({ value: "新代理" });
const forms = reactive({
  browserType: ref<RadioInterface>({
    value: "chrome",
  }),
  osType: ref<RadioInterface>({
    value: "Windows",
  }),
  encryPtSwitch: ref<RadioInterface>({
    value: "关闭",
  }),
  numbers: ref<number>(1),
  group: ref<any>(0),
});
</script>
<template>
  <Layout>
    <div class="grow overflow-hidden">
      <div class="overflow-y-auto h-full my-3">
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
            <AccordionContent class="px-10 space-y-4 py-1">
              <div class="flex items-center">
                <p class="w-32 text-left">名称</p>
                <div>
                  <div class="relative text-sm w-full">
                    <Input
                      v-model="environmentCreatesFrom.forms.environmentName"
                      v-bind="environmentCreatesFrom.forms.environmentNameProps"
                      type="text"
                      placeholder="请输入浏览器名称"
                      class="w-full"
                      maxlength="10"
                    />
                    <span class="absolute right-3 top-2 text-xs text-gray-400"
                      >{{
                        environmentCreatesFrom.forms.environmentName?.length ||
                        0
                      }}/10</span
                    >
                    <span class="text-red-400 text-xs">{{
                      environmentCreatesFrom.errors.name
                    }}</span>
                  </div>
                </div>
              </div>
              <div class="flex items-center">
                <p class="w-32 text-left">浏览器</p>
                <Radio
                  :check="forms.browserType"
                  :data="data.browserTypes"
                  @update:check="(v) => (forms.browserType = v)"
                />
              </div>

              <div class="space-y-2 flex items-center">
                <p class="w-32 text-left">操作系统</p>
                <Radio
                  :check="forms.osType"
                  :data="data.osTypes"
                  @update:check="(v) => (forms.osType = v)"
                />
              </div>

              <div class="flex">
                <div class="flex items-start justify-start">
                  <p class="w-32 text-left text-sm">UA</p>
                </div>
                <div class="w-1/2">
                  <div class="flex flex-col gap-4">
                    <Select>
                      <SelectTrigger>
                        <SelectValue
                          placeholder="全部"
                          class="w-full p-2 rounded-lg outline-none"
                        />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectItem value="all"> 全部 </SelectItem>
                        </SelectGroup>
                      </SelectContent>
                    </Select>
                    <div class="w-full relative">
                      <Textarea />
                      <div class="absolute -right-2 top-2 flex gap-2">
                        <button class="p-1 hover:bg-gray-100 rounded">
                          <CopyIcon class="w-4 h-4 text-gray-400" />
                        </button>
                        <button class="p-1 hover:bg-gray-100 rounded">
                          <Link class="w-4 h-4 text-blue-600" />
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div className="flex flex-col">
                <div class="flex">
                  <p class="w-32 text-left text-sm">端对端加密</p>

                  <Radio
                    :check="forms.encryPtSwitch"
                    :data="data.encrypts"
                    @update:check="(v) => (forms.encryPtSwitch = v)"
                  />
                </div>
                <div className="text-muted-foreground pl-32 pt-2 text-xs">
                  开启后，只有通过密钥授权的账号和设备可以解密和访问此环境
                </div>
              </div>

              <div class="flex">
                <p class="w-32 text-left text-sm">批量导入</p>

                <div>
                  <div
                    class="border-2 border-dashed border-gray-200 rounded-lg p-8 text-center"
                    @dragover.prevent="handleDragOver"
                    @dragleave.prevent="handleDragLeave"
                    @drop.prevent="handleDrop"
                    :class="{ 'bg-blue-50 border-blue-200': isDragging }"
                  >
                    <!-- 上传成功后的显示区域 -->
                    <div v-if="isUploaded" class="mb-4">
                      <CheckCircleIcon
                        class="w-12 h-12 mx-auto text-green-500"
                      />
                      <p class="text-green-600 mt-2">文件上传成功！</p>
                      <p class="text-sm text-gray-500">
                        文件名：{{ uploadedFileName }}
                      </p>
                    </div>

                    <!-- 上传前的显示区域 -->
                    <div v-else>
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
                      v-bind="importFrom.forms.fileProps"
                      @change="handleFileSelect"
                    />

                    <!-- 上传按钮 -->
                    <div class="flex justify-center">
                      <button
                        class="mt-4 px-6 py-2 text-sm border border-gray-200 rounded-md hover:bg-gray-50 flex justify-center items-center"
                        @click="triggerFileInput"
                      >
                        <FolderPlusIcon
                          class="w-6 h-6 mx-auto text-gray-300 mr-2"
                        />
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
                    <p class="text-[#5050FA] underline">从其他浏览器导入</p>
                  </div>
                </div>
              </div>

              <div className="flex">
                <div class="w-32">
                  <p class="text-left text-sm">启动后</p>
                </div>
                <div class="grow">
                  <RadioGroup defaultValue="option-one">
                    <div className="flex items-center space-x-2">
                      <RadioGroupItem
                        value="option-one"
                        id="option-one"
                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]"
                      />
                      <Label htmlFor="option-one">继续浏览上次打开的网页</Label>
                    </div>
                    <div className="flex items-center space-x-2">
                      <RadioGroupItem
                        value="option-two"
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
                    <Switch class="data-[state=checked]:bg-[#5050FA]" />
                    <p>同时打开「账号」已配置的平台网页</p>
                  </div>
                  <div class="flex gap-x-2">
                    <Switch class="data-[state=checked]:bg-[#5050FA]" />
                    <p>打开后默认停留在检测页</p>
                  </div>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <Accordion type="single" class="w-full px-4" collapsible>
          <AccordionItem value="webrtc-setting" class="border-0">
            <AccordionTrigger
              class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
              >高级配置
            </AccordionTrigger>
            <AccordionContent class="px-10 space-y-4 py-1">
              <div className="flex">
                <div class="w-32">
                  <button
                    class="text-sm border whitespace-nowrap rounded-md px-2 py-2 flex justify-center items-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
                  >
                    <Iconrefresh class="size-4" />
                    恢复默认设置
                  </button>
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">时区</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.timezones"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">WebRTC</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webRtcs"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">地理位置</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.geolocations"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">语言</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.languages"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">分辨率</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.resolution"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">字体</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.font"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">Canvas</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.canvas"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">WebGL图像</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webGLI"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">WebRTC</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webGLM"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">WebGPU</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webGPU"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">AudioContext</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.zz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">媒体设备</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.zz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">ClientRects</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.zz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">SpeechVoices</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.yz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">硬件并发数</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="4"
                      class="w-full p-2 rounded-lg outline-none"
                    />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectItem value="no"> No Proxy (本地直连) </SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">设备内存</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="4"
                      class="w-full p-2 rounded-lg outline-none"
                    />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectItem value="no"> No Proxy (本地直连) </SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </div>

              <div className="flex">
                <div class="w-32">
                  <p class="text-left text-sm">设备名称</p>
                </div>
                <div class="grow space-y-2">
                  <div class="w-32">
                    <Radio
                      :check="proxy_method"
                      :data="data.font"
                      @update:check="(v) => (proxy_method.value = v.value)"
                    />
                  </div>
                  <Select>
                    <SelectTrigger class="w-1/2">
                      <SelectValue
                        placeholder="选择或输入分组"
                        class="w-full p-2 rounded-lg outline-none"
                      />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectItem value="no">
                          No Proxy (本地直连)
                        </SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                </div>
              </div>
              <div className="flex">
                <div class="w-32">
                  <p class="text-left text-sm">MAC地址</p>
                </div>
                <div class="grow space-y-2">
                  <div class="w-[7.6rem]">
                    <Radio
                      :check="proxy_method"
                      :data="data.font"
                      @update:check="(v) => (proxy_method.value = v.value)"
                    />
                  </div>
                  <Select>
                    <SelectTrigger class="w-1/2">
                      <SelectValue
                        placeholder="选择或输入分组"
                        class="w-full p-2 rounded-lg outline-none"
                      />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectItem value="no">
                          No Proxy (本地直连)
                        </SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">Do Not Track</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.yz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">蓝牙</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.yz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">电池</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.yz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex">
                <div class="w-32">
                  <p class="text-left text-sm">端口扫描保护</p>
                </div>
                <div class="grow">
                  <div class="w-full space-y-2">
                    <div class="w-[7.6rem]">
                      <Radio
                        :check="proxy_method"
                        :data="data.font"
                        @update:check="(v) => (proxy_method.value = v.value)"
                      />
                    </div>
                    <Textarea
                      rows="4"
                      placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                      class="w-1/2 px-3 py-2 border rounded-md placeholder-gray-400"
                    ></Textarea>
                  </div>
                </div>
              </div>

              <div className="flex">
                <div class="w-32">
                  <p class="text-left text-sm">启动参数</p>
                </div>
                <Textarea
                  rows="4"
                  placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                  class="w-1/2 px-3 py-2 border rounded-md placeholder-gray-400"
                ></Textarea>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
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
        @click=""
        class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
      >
        确认
      </button>
    </div>
  </Layout>
</template>
