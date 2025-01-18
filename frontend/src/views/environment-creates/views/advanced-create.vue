<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
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
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "@/components/ui/number-field";
import { useEnvironmentCreatesFromStore } from "@/stores/form/environment-creates";
import ImportProxyModel from "../components/import-proxy-model.vue";
import { useRouter } from "vue-router";
import Layout from "../layout.vue";
import Radio, { type RadioInterface } from "../components/radio.vue";
import { Iconrefresh } from "@/assets/icons";
import { Textarea } from "@/components/ui/textarea";
import { generateRandomUserAgent } from "@/util/lib";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import {
  templaeFormtData as data,
  useEnvironmentAdvancedFormStore,
} from "@/stores/form/environment-advanced";
import { environment_group_query } from "@/commands/environment-group";
import { environment_tag_query } from "@/commands/environment-tag";

const environmentCreatesFrom = useEnvironmentCreatesFromStore();
const forms = useEnvironmentAdvancedFormStore().forms;

const importProxyOpen = ref(false);
const proxyModelCloseHandle = () => (importProxyOpen.value = false);

const router = useRouter();

const proxy_method = ref<RadioInterface>({ value: "新代理" });

onMounted(() => {
  environment_group_query(1, 100).then((res) => {
    if (res.code == 1) {
      let tempGroups = [
        ...data.groups,
        ...res.data.data.map((item: any) => ({
          id: item.id,
          value: item.name,
        })),
      ];
      data.groups = Array.from(
        new Map(tempGroups.map((item) => [item.id, item])).values()
      );
    }
  });
  environment_tag_query(1, 100).then((res) => {
    if (res.code == 1) {
      let tempTags = [
        ...data.tags,
        ...res.data.data.map((item: any) => ({
          id: item.id,
          value: item.name,
        })),
      ];

      data.groups = Array.from(
        new Map(tempTags.map((item) => [item.id, item])).values()
      );
    }
  });
});
</script>
<template>
  <Layout>
    <div class="overflow-hidden grow">
      <div class="overflow-y-auto h-full">
        <div class="flex justify-between items-center px-2 py-3 w-full">
          <div class="p-2">
            <h3 class="mb-1 font-medium">窗口数量</h3>
            <p class="text-sm text-gray-500">最多一次最多可批量创建1000个</p>
          </div>
          <div class="flex flex-col gap-2 items-center rounded-md">
            <NumberField
              v-bind="environmentCreatesFrom.forms.environmentNumbersProps"
            >
              <NumberFieldContent>
                <NumberFieldDecrement class="text-gray-500" />
                <NumberFieldInput
                  class="focus-visible:ring-blue-400 hover:border-blue-500"
                />
                <NumberFieldIncrement class="text-gray-500" />
              </NumberFieldContent>
            </NumberField>
          </div>
        </div>
        <Accordion
          type="single"
          class="px-4 w-full"
          collapsible
          default-value="basic-setting"
        >
          <AccordionItem class="border-0" value="basic-setting">
            <AccordionTrigger
              class="p-3 mb-2 text-sm bg-gray-50 rounded-md hover:no-underline"
              >基础设置</AccordionTrigger
            >
            <AccordionContent class="px-10 py-1 space-y-4">
              <div class="flex items-center">
                <p class="w-32 text-left">名称</p>
                <div>
                  <div class="relative w-full text-sm">
                    <Input
                      v-model="environmentCreatesFrom.forms.environmentName"
                      v-bind="environmentCreatesFrom.forms.environmentNameProps"
                      type="text"
                      placeholder="请输入浏览器名称"
                      class="w-full"
                      maxlength="10"
                    />
                    <span class="absolute top-2 right-3 text-xs text-gray-400"
                      >{{
                        environmentCreatesFrom.forms.environmentName?.length ||
                        0
                      }}/10</span
                    >
                    <span class="text-xs text-red-400">{{
                      environmentCreatesFrom.errors.name
                    }}</span>
                  </div>
                </div>
              </div>
              <div class="flex items-center">
                <p class="w-32 text-left">浏览器</p>
                <Radio
                  :check="forms.browser"
                  :data="data.browserTypes"
                  @update:check="(v) => (forms.browser = v)"
                />
              </div>

              <div class="flex items-center space-y-2">
                <p class="w-32 text-left">操作系统</p>
                <Radio
                  :check="forms.os"
                  :data="data.osTypes"
                  @update:check="(v) => (forms.os = v)"
                />
              </div>

              <div class="flex">
                <div class="flex justify-start items-start">
                  <p class="w-32 text-sm text-left">UA</p>
                </div>
                <div class="relative w-full">
                  <Textarea class="w-full" :placeholder="forms.ua" disabled />
                  <div
                    class="flex absolute top-2 right-2 gap-2"
                    @click="() => (forms.ua = generateRandomUserAgent())"
                  >
                    <Iconrefresh class="w-4 h-4 cursor-pointer fill-gray-800" />
                  </div>
                </div>
              </div>

              <div className="flex flex-col">
                <div class="flex">
                  <p class="w-32 text-sm text-left">端对端加密</p>
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

              <div className="flex items-center">
                <p class="w-32 text-sm text-left">环境分组</p>
                <div class="w-1/2">
                  <Select
                    class="flex"
                    v-model="forms.group"
                    :default-value="forms.group"
                  >
                    <SelectTrigger
                      className="w-full rounded-md flex p-2 border-[1.4px] border-[#C5CEE0] items-center justify-between"
                    >
                      <SelectValue
                        placeholder="选择或新建分组"
                        class="text-sm"
                      />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem
                        v-for="item in data.groups"
                        :value="item.id"
                        >{{ item.value }}</SelectItem
                      >
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-sm text-left">环境标签</p>
                <div class="w-1/2">
                  <Select
                    class="flex"
                    v-model="forms.tag"
                    :default-value="forms.tag"
                  >
                    <SelectTrigger
                      className="w-full rounded-md flex p-2 border-[1.4px] border-[#C5CEE0] items-center justify-between"
                    >
                      <SelectValue
                        placeholder="选择或新建标签"
                        class="text-sm"
                      />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem v-for="item in data.tags" :value="item.id">{{
                        item.value
                      }}</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div class="flex items-center">
                <p class="w-32 text-left">备注</p>
                <div class="relative pt-1 w-2/3 text-sm">
                  <Input
                    v-model="forms.description"
                    type="text"
                    :placeholder="forms.description"
                    class="w-full text-gray-600 px-[0.45rem]"
                    maxlength="28"
                  />
                  <span class="absolute top-2 right-3 text-xs text-gray-400"
                    >{{ forms.description.length || 0 }}/28</span
                  >
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <Accordion type="single" class="px-4 w-full" collapsible>
          <AccordionItem class="border-0" value="proxy-setting">
            <AccordionTrigger
              class="p-3 mb-2 text-sm bg-gray-50 rounded-md hover:no-underline"
              >代理信息</AccordionTrigger
            >
            <AccordionContent class="px-10 py-1 space-y-4">
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">代理方式</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.proxy_methods"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">代理类型</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="No Proxy (本地直连)"
                      class="p-2 w-full rounded-lg outline-none"
                    />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectItem value="no"> No Proxy (本地直连) </SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </div>
              <div class="flex">
                <div class="flex justify-start items-start">
                  <p class="w-32 text-sm text-left">IP查询渠道</p>
                </div>
                <div class="space-y-2 w-1/2">
                  <Select>
                    <SelectTrigger class="w-2/3" disabled>
                      <SelectValue
                        placeholder="IP2Location"
                        class="p-2 w-full rounded-lg outline-none"
                      />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectItem value="IP2Location">
                          IP2Location
                        </SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                  <button
                    class="text-sm border whitespace-nowrap rounded-md px-2 py-2 flex justify-center items-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
                  >
                    网络检测
                  </button>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <!-- Account Platform -->
        <Accordion type="single" class="px-4 w-full" collapsible>
          <AccordionItem value="account-setting" class="border-0">
            <AccordionTrigger
              class="p-3 mb-2 text-sm bg-gray-50 rounded-md hover:no-underline"
              >账号平台
            </AccordionTrigger>
            <AccordionContent class="px-10 py-1 space-y-4">
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">账号平台</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="无"
                      class="p-2 w-full rounded-lg outline-none"
                    />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectItem value="null"> 无 </SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </div>

              <div class="flex items-center space-y-2">
                <p class="w-32 text-sm text-left">标签页</p>
                <Textarea
                  rows="4"
                  placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                  class="px-3 py-2 w-1/2 placeholder-gray-400 rounded-md border"
                ></Textarea>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <!-- Cookie -->
        <Accordion type="single" class="px-4 w-full" collapsible>
          <AccordionItem value="cookie-setting" class="border-0">
            <AccordionTrigger
              class="p-3 mb-2 text-sm bg-gray-50 rounded-md hover:no-underline"
              >Cookie
            </AccordionTrigger>
            <AccordionContent class="px-10 py-1 space-y-4">
              <div className="flex">
                <div class="w-32">
                  <p class="text-sm text-left">Cookie</p>
                </div>
                <div class="grow">
                  <div class="space-y-2 w-full">
                    <button
                      class="w-24 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
                    >
                      导入Cookie
                    </button>
                    <Textarea
                      rows="4"
                      placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                      class="px-3 py-2 w-1/2 placeholder-gray-400 rounded-md border"
                    ></Textarea>
                  </div>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <Accordion type="single" class="px-4 w-full" collapsible>
          <AccordionItem value="startup-page-setting" class="border-0">
            <AccordionTrigger
              class="p-3 mb-2 text-sm bg-gray-50 rounded-md hover:no-underline"
              >浏览器启动页
            </AccordionTrigger>
            <AccordionContent class="px-10 py-1 space-y-4">
              <div className="flex">
                <div class="w-32">
                  <p class="text-sm text-left">启动后</p>
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
                  <p class="text-sm text-left">其他设置</p>
                </div>
                <div class="flex flex-col gap-y-1 grow">
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

        <Accordion type="single" class="px-4 w-full" collapsible>
          <AccordionItem value="webrtc-setting" class="border-0">
            <AccordionTrigger
              class="p-3 mb-2 text-sm bg-gray-50 rounded-md hover:no-underline"
              >高级配置
            </AccordionTrigger>
            <AccordionContent class="px-10 py-1 space-y-4">
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
                <p class="w-32 text-sm text-left">时区</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.timezones"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-sm text-left">WebRTC</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webRtcs"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-sm text-left">地理位置</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.geolocations"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-sm text-left">语言</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.languages"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-sm text-left">分辨率</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.resolution"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">字体</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.font"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">Canvas</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.canvas"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">WebGL图像</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webGLI"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">WebRTC</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webGLM"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">WebGPU</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.webGPU"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">AudioContext</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.zz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">媒体设备</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.zz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">ClientRects</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.zz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">SpeechVoices</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.yz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-sm text-left">硬件并发数</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="4"
                      class="p-2 w-full rounded-lg outline-none"
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
                <p class="w-32 text-sm text-left">设备内存</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="4"
                      class="p-2 w-full rounded-lg outline-none"
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
                  <p class="text-sm text-left">设备名称</p>
                </div>
                <div class="space-y-2 grow">
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
                        class="p-2 w-full rounded-lg outline-none"
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
                  <p class="text-sm text-left">MAC地址</p>
                </div>
                <div class="space-y-2 grow">
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
                        class="p-2 w-full rounded-lg outline-none"
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
                <p class="w-32 text-sm text-left">Do Not Track</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.yz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">蓝牙</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.yz"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-sm text-left">电池</p>
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
                  <p class="text-sm text-left">端口扫描保护</p>
                </div>
                <div class="grow">
                  <div class="space-y-2 w-full">
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
                      class="px-3 py-2 w-1/2 placeholder-gray-400 rounded-md border"
                    ></Textarea>
                  </div>
                </div>
              </div>

              <div className="flex">
                <div class="w-32">
                  <p class="text-sm text-left">启动参数</p>
                </div>
                <Textarea
                  rows="4"
                  placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                  class="px-3 py-2 w-1/2 placeholder-gray-400 rounded-md border"
                ></Textarea>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
    </div>
    <div class="flex gap-x-4 justify-end items-center p-3 border-t">
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
    <ImportProxyModel :open="importProxyOpen" @close="proxyModelCloseHandle" />
  </Layout>
</template>
