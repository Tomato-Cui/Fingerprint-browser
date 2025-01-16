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
    <div class="grow overflow-hidden">
      <div class="overflow-y-auto h-full">
        <div class="w-full px-2 py-3 flex items-center justify-between">
          <div class="p-2">
            <h3 class="font-medium mb-1">窗口数量</h3>
            <p class="text-sm text-gray-500">最多一次最多可批量创建1000个</p>
          </div>
          <div class="flex flex-col items-center gap-2 rounded-md">
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
                  :check="forms.browser"
                  :data="data.browserTypes"
                  @update:check="(v) => (forms.browser = v)"
                />
              </div>

              <div class="space-y-2 flex items-center">
                <p class="w-32 text-left">操作系统</p>
                <Radio
                  :check="forms.os"
                  :data="data.osTypes"
                  @update:check="(v) => (forms.os = v)"
                />
              </div>

              <div class="flex">
                <div class="flex items-start justify-start">
                  <p class="w-32 text-left text-sm">UA</p>
                </div>
                <div class="w-full relative">
                  <Textarea class="w-full" :placeholder="forms.ua" disabled />
                  <div
                    class="absolute right-2 top-2 flex gap-2"
                    @click="() => (forms.ua = generateRandomUserAgent())"
                  >
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
                    @update:check="(v) => (forms.encryPtSwitch = v)"
                  />
                </div>
                <div className="text-muted-foreground pl-32 pt-2 text-xs">
                  开启后，只有通过密钥授权的账号和设备可以解密和访问此环境
                </div>
              </div>

              <div className="flex items-center">
                <p class="w-32 text-left text-sm">环境分组</p>
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
                <p class="w-32 text-left text-sm">环境标签</p>
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
                <div class="relative text-sm w-2/3 pt-1">
                  <Input
                    v-model="forms.description"
                    type="text"
                    :placeholder="forms.description"
                    class="w-full text-gray-600 px-[0.45rem]"
                    maxlength="28"
                  />
                  <span class="absolute right-3 top-2 text-xs text-gray-400"
                    >{{ forms.description.length || 0 }}/28</span
                  >
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <Accordion type="single" class="w-full px-4" collapsible>
          <AccordionItem class="border-0" value="proxy-setting">
            <AccordionTrigger
              class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
              >代理信息</AccordionTrigger
            >
            <AccordionContent class="px-10 space-y-4 py-1">
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">代理方式</p>
                <div>
                  <Radio
                    :check="proxy_method"
                    :data="data.proxy_methods"
                    @update:check="(v) => (proxy_method.value = v.value)"
                  />
                </div>
              </div>
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">代理类型</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="No Proxy (本地直连)"
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
              <div class="flex">
                <div class="flex items-start justify-start">
                  <p class="w-32 text-left text-sm">IP查询渠道</p>
                </div>
                <div class="w-1/2 space-y-2">
                  <Select>
                    <SelectTrigger class="w-2/3" disabled>
                      <SelectValue
                        placeholder="IP2Location"
                        class="w-full p-2 rounded-lg outline-none"
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
        <Accordion type="single" class="w-full px-4" collapsible>
          <AccordionItem value="account-setting" class="border-0">
            <AccordionTrigger
              class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
              >账号平台
            </AccordionTrigger>
            <AccordionContent class="px-10 space-y-4 py-1">
              <div className="flex items-center">
                <p class="w-32 text-left text-sm">账号平台</p>
                <Select>
                  <SelectTrigger class="w-1/3">
                    <SelectValue
                      placeholder="无"
                      class="w-full p-2 rounded-lg outline-none"
                    />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectItem value="null"> 无 </SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </div>

              <div class="space-y-2 flex items-center">
                <p class="w-32 text-left text-sm">标签页</p>
                <Textarea
                  rows="4"
                  placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                  class="w-1/2 px-3 py-2 border rounded-md placeholder-gray-400"
                ></Textarea>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <!-- Cookie -->
        <Accordion type="single" class="w-full px-4" collapsible>
          <AccordionItem value="cookie-setting" class="border-0">
            <AccordionTrigger
              class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
              >Cookie
            </AccordionTrigger>
            <AccordionContent class="px-10 space-y-4 py-1">
              <div className="flex">
                <div class="w-32">
                  <p class="text-left text-sm">Cookie</p>
                </div>
                <div class="grow">
                  <div class="w-full space-y-2">
                    <button
                      class="w-24 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
                    >
                      导入Cookie
                    </button>
                    <Textarea
                      rows="4"
                      placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                      class="w-1/2 px-3 py-2 border rounded-md placeholder-gray-400"
                    ></Textarea>
                  </div>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>

        <Accordion type="single" class="w-full px-4" collapsible>
          <AccordionItem value="startup-page-setting" class="border-0">
            <AccordionTrigger
              class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
              >浏览器启动页
            </AccordionTrigger>
            <AccordionContent class="px-10 space-y-4 py-1">
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
    <ImportProxyModel :open="importProxyOpen" @close="proxyModelCloseHandle" />
  </Layout>
</template>
