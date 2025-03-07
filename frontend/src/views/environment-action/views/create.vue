<script setup>
import {IconChrome, IconFirefox, IconLinuxOs, IconMacOs, IconWindowOs,} from "@/assets/icons/index";
import {Accordion, AccordionContent, AccordionItem, AccordionTrigger,} from "@/components/ui/accordion";
import {Switch} from "@/components/ui/switch";
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,} from "@/components/select";
import Input from "@/components/input.vue";
import {CopyIcon} from "lucide-vue-next";
import {Tabs, TabsContent, TabsList, TabsTrigger} from "@/components/ui/tabs";
import Alignment from "../components/alignment.vue";
import {useEnvironmentCreateFromStore} from "@/stores/form/environment-create";
import {ref} from "vue";


const environmentCreateFrom = useEnvironmentCreateFromStore();
// 选择操作系统的方法
const selectOs = (os) => {
  environmentCreateFrom.forms.os = os; // 更新选中的操作系统
};

</script>
<template>
  <Accordion type="single" class="w-full px-6" collapsible default-value="basic-setting">
    <AccordionItem class="border-0" value="basic-setting">
      <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">基础设置</AccordionTrigger>
      <AccordionContent class="px-4 space-y-4">
        <div class="space-y-2 flex items-center">
          <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
            名称
          </p>
          <div class="relative text-sm w-full">
            <Input
              v-model="environmentCreateFrom.forms.environmentName"
              v-bind="environmentCreateFrom.forms.environmentNameProps"
              type="text"
              placeholder="请输入环境名称"
              class="w-full"
              maxlength="10"
            />
            <span class="absolute right-3 top-2 text-xs text-gray-400"
              >{{ environmentCreateFrom.forms.environmentName?.length || 0 }}/10</span
            >
            <span class="text-red-400">{{
              environmentCreateFrom.errors.name
            }}</span>
          </div>
        </div>
        <div class="space-y-2 flex items-center">
          <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
            浏览器
          </p>
          <div class="w-full">
            <div class="w-1/2 flex gap-x-4">

              <Select v-model="environmentCreateFrom.forms.browser" v-bind="environmentCreateFrom.forms.browserProps">
                <SelectTrigger>
                  <IconChrome class="w-6 h-6" />
                  <SelectValue placeholder="Chrome" class="w-full pl-4 rounded-lg outline-none" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="chrome"> Chrome </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
              <Select v-model="environmentCreateFrom.forms.browser" v-bind="environmentCreateFrom.forms.browserProps">
                <SelectTrigger>
                  <IconFirefox class="w-6 h-6" />
                  <SelectValue placeholder="FireFox" class="w-full pl-4 rounded-lg outline-none" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="firefox"> FireFox </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>

              <span class="text-red-400">{{
                  environmentCreateFrom.errors.browser
                }}</span>
            </div>
          </div>
        </div>

        <div class="space-y-2 flex items-center">
          <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
            操作系统
          </p>

          <div class="w-full">
            <div class="flex gap-4">
              <!-- Windows 操作系统按钮 -->
              <button
                  class="flex items-center gap-2 px-4 py-2 border rounded-md"
                  :class="{ 'bg-blue-50 border-blue-200 text-blue-600': environmentCreateFrom.forms.os === 'Windows' }"
                  @click="selectOs('Windows')"
                  v-bind="environmentCreateFrom.forms.osProps"
              >
                <IconWindowOs class="w-5 h-5" />
                Windows
              </button>

              <!-- MacOS 操作系统按钮 -->
              <button
                  class="flex items-center gap-2 px-4 py-2 border rounded-md"
                  :class="{ 'bg-blue-50 border-blue-200 text-blue-600': environmentCreateFrom.forms.os === 'MacOs' }"
                  @click="selectOs('MacOs')"
                  v-bind="environmentCreateFrom.forms.osProps"
              >
                <IconMacOs class="w-5 h-5" />
                MacOs
              </button>

              <!-- Linux 操作系统按钮 -->
              <button
                  class="flex items-center gap-2 px-4 py-2 border rounded-md"
                  :class="{ 'bg-blue-50 border-blue-200 text-blue-600': environmentCreateFrom.forms.os === 'Linux' }"
                  @click="selectOs('Linux')"
                  v-bind="environmentCreateFrom.forms.osProps"
              >
                <IconLinuxOs class="w-5 h-5" />
                Linux
              </button>
            </div>
            <span class="text-red-400">{{
                environmentCreateFrom.errors.os
              }}</span>
          </div>

        </div>

        <div class="space-y-2 flex items-center">
          <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
            User-Agent
          </p>
          <div class="w-full">
            <div class="flex gap-4">
              <Select>
                <SelectTrigger class="w-1/4">
                  <SelectValue placeholder="全部" class="w-full p-2 rounded-lg outline-none" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="1"> 版本1 </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
              <div class="flex-auto relative">
                <Input type="text" placeholder="Mozilla/5.0(Windows NT 10.0;Win64; x64)..."
                       v-model="environmentCreateFrom.forms.ua"
                       v-bind="environmentCreateFrom.forms.uaProps"
                       class="w-full text-sm text-gray-500" />
                <div class="absolute right-2 top-2 flex gap-2">
                  <button class="p-1 hover:bg-gray-100 rounded">
                    <CopyIcon class="w-4 h-4 text-gray-400" />
                  </button>
                  <button class="p-1 hover:bg-gray-100 rounded">
                    <Link class="w-4 h-4 text-blue-600" />
                  </button>
                </div>
              </div>
              <span class="text-red-400">{{
                  environmentCreateFrom.errors.ua
                }}</span>
            </div>
          </div>
        </div>


        <!-- Notes -->
        <div class="space-y-2 flex items-center">
          <div class="w-full">
            <div class="flex relative">
              <p
                class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8"
              >
                备注
              </p>
              <Input
                v-model="environmentCreateFrom.forms.environmentDescription"
                v-bind="environmentCreateFrom.forms.environmentDescriptionProps"
                placeholder="请输入备注"
                rows="3"
                class="w-full px-3 py-2 border rounded-md"
              ></Input>
              <span class="absolute right-3 top-2 text-gray-400 text-xs"
                >0/1500</span
              >
            </div>
            <span class="text-red-400 pl-[7.85rem]">{{
              environmentCreateFrom.errors.description
            }}</span>
          </div>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>

  <Accordion type="single" class="w-full px-6 mb-2" collapsible>
    <AccordionItem class="border-0" value="proxy-setting">
      <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50">代理信息</AccordionTrigger>
      <AccordionContent class="px-4 pt-2">
        <Tabs default-value="custom" class="">
          <TabsList class="flex w-72 ml-[8.25rem]">
            <TabsTrigger value="custom"> 自定义代理 </TabsTrigger>
            <TabsTrigger value="appended"> 已添加的代理 </TabsTrigger>
            <TabsTrigger value="extract"> API提取 </TabsTrigger>
          </TabsList>
          <TabsContent value="custom" class="space-y-2">
            <div class="flex items-center">
              <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
                代理类型
              </p>
              <div class="w-full flex gap-x-4">
                <Select
                    v-model="environmentCreateFrom.forms.kind"
                    v-bind="environmentCreateFrom.forms.kindProps"
                >
                  <SelectTrigger class="w-1/3">
                    <SelectValue placeholder="No Proxy (本地直连)" class="w-full p-2 rounded-lg outline-none" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectItem value="kind1"> No Proxy (本地直连) </SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
                <button class="px-4 py-2 border rounded-md hover:bg-gray-50 whitespace-nowrap">
                  检查网络
                </button>
                <span class="text-red-400 pl-[7.85rem]">{{
                    environmentCreateFrom.errors.kind
                  }}</span>
              </div>
            </div>
            <div class="flex items-center">
              <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
                IP查询渠道
              </p>
              <div class="w-full">
                <div class="flex gap-4">
                  <Select>
                    <SelectTrigger class="w-1/3">
                      <SelectValue placeholder="IP2Location" class="w-full p-2 rounded-lg outline-none" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectItem value="IP2Location">
                          IP2Location
                        </SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>
          </TabsContent>
          <TabsContent value="appended"> </TabsContent>
          <TabsContent value="extract"> </TabsContent>
        </Tabs>
      </AccordionContent>
    </AccordionItem>
  </Accordion>

  <!-- Account Platform -->
  <Accordion type="single" class="w-full px-6" collapsible>
    <AccordionItem value="account-setting" class="border-0">
      <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">账号平台
      </AccordionTrigger>
      <AccordionContent class="px-4 space-y-4">
        <div class="space-y-2 flex items-center">
          <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
            账号平台
          </p>
          <div class="w-full flex gap-x-4">
            <Select>
              <SelectTrigger class="w-full">
                <SelectValue placeholder="无" class="w-full p-2 rounded-lg outline-none" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="null"> 无 </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </div>
        <div class="space-y-2 flex items-center">
          <p class="w-36 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
            标签页
          </p>
          <div class="w-full flex gap-x-4">
            <textarea rows="4" placeholder="输入网址 (每行一个网址)&#10;www.google.com&#10;www.facebook.com"
                      class="w-full px-3 py-2 border rounded-md placeholder-gray-400"></textarea>
          </div>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>

  <Accordion type="single" class="w-full px-6" collapsible>
    <AccordionItem value="webrtc-setting" class="border-0">
      <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">WebRTC
      </AccordionTrigger>
      <AccordionContent class="px-4 space-y-4">
        <!-- WebRTC -->
        <Alignment title="WebRTC">
          <button class="px-4 py-1.5 rounded-md text-sm" :class="webRTC === '转发' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'
            ">
            转发
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm" :class="webRTC === '替换' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'
            ">
            替换
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm" :class="webRTC === '真实' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'
            ">
            真实
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm" :class="webRTC === '禁用' ? 'bg-blue-50 text-blue-600' : 'text-gray-600'
            ">
            禁用
          </button>
        </Alignment>

        <!-- Time Zone -->
        <Alignment title="时区">
          <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">
            基于 IP
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            真实
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            自定义
          </button>
        </Alignment>

        <!-- Geolocation -->
        <Alignment title="地理位置">
          <label class="flex items-center gap-2">
            <Input type="radio" checked class="text-blue-600" />
            <span class="text-sm">每次询问</span>
          </label>
          <label class="flex items-center gap-2">
            <Input type="radio" class="text-blue-600" />
            <span class="text-sm">始终允许</span>
          </label>
        </Alignment>

        <!-- Language -->
        <Alignment title="语言">
          <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">
            基于 IP
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            自定义
          </button>
        </Alignment>

        <!-- Interface Language -->
        <Alignment title="界面语言">
          <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">
            基于语言
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            真实
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            自定义
          </button>
        </Alignment>

        <!-- Resolution -->
        <Alignment title="分辨率">
          <div class="flex gap-2">
            <button class="px-4 py-1.5 rounded-md text-sm text-gray-600 bg-blue-50">
              随机
            </button>
            <button class="px-4 py-1.5 rounded-md text-sm">预定义</button>
            <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
              自定义
            </button>
          </div>
          <div class="relative">
            <Select>
              <SelectTrigger class="w-full">
                <SelectValue placeholder="基于 User-Agent" class="w-full p-2 rounded-lg outline-none" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="user-agent"> 基于 User-Agent </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </Alignment>

        <!-- Font -->
        <Alignment title="界面语言">
          <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">
            默认
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            自定义
          </button>
        </Alignment>

        <!-- Hardware Noise -->
        <Alignment title="硬件噪音">
          <div class="space-y-4">
            <label class="block text-sm font-medium text-gray-700"></label>
            <div class="flex items-center gap-6">
              <label class="flex items-center gap-2">
                <Switch />
                <span class="text-sm">Canvas</span>
              </label>
              <label class="flex items-center gap-2">
                <Switch />
                <span class="text-sm">AudioContext</span>
              </label>
            </div>
            <div class="flex items-center gap-6">
              <label class="flex items-center gap-2">
                <Switch />
                <span class="text-sm">WebGL图像</span>
              </label>
              <label class="flex items-center gap-2">
                <Switch />
                <span class="text-sm">SpeechVoices</span>
              </label>
            </div>

            <div class="flex items-center gap-6">
              <label class="flex items-center gap-2">
                <Switch />
                <span class="text-sm">ClientRects</span>
              </label>
              <label class="flex items-center gap-2">
                <Switch />
                <span class="text-sm">媒体设备[Auto]</span>
                <button class="text-blue-600 text-sm hover:underline">
                  编辑
                </button>
              </label>
            </div>
          </div>
        </Alignment>

        <!-- WebGL Data -->
        <div class="space-y-2">
          <div class="flex">
            <p class="w-32 flex justify-end items-center text-sm font-medium text-gray-700 pr-8">
              操作系统
            </p>
            <div class="flex gap-2">
              <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">
                默认
              </button>
              <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
                自定义
              </button>
            </div>
          </div>
          <div class="w-full ml-32 space-y-2 pt-4">
            <label class="block text-sm font-medium text-gray-700">厂商</label>
            <div class="relative">
              <Select>
                <SelectTrigger class="w-full">
                  <SelectValue placeholder="ANGLE (Intel, Intel(R) UHD Graphics 610 Dir..."
                               class="p-2 rounded-lg outline-none" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="user-agent">
                      Google Inc. (Intel)
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </div>

            <label class="block text-sm font-medium text-gray-700">渲染器</label>
            <div class="relative">
              <Select>
                <SelectTrigger class="w-full">
                  <SelectValue placeholder="ANGLE (Intel, Intel(R) UHD Graphics 610 Dir..."
                               class="p-2 rounded-lg outline-none" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="user-agent">
                      ANGLE (Intel, Intel(R) UHD Graphics 610 Dir...
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </div>
          </div>
        </div>

        <!-- WebGPU -->
        <Alignment title="WebGPU">
          <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">
            基于 WebGL
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            真实
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            禁用
          </button>
        </Alignment>
      </AccordionContent>
    </AccordionItem>
  </Accordion>

  <!-- Advanced Settings -->
  <Accordion type="single" class="w-full px-6" collapsible>
    <AccordionItem value="software-setting" class="border-0">
      <AccordionTrigger class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2">应用</AccordionTrigger>
      <AccordionContent class="px-4 space-y-4">
        <Alignment title="应用">
          <button class="px-4 py-1.5 rounded-md text-sm bg-blue-50 text-blue-600">
            跟随团队
          </button>
          <button class="px-4 py-1.5 rounded-md text-sm text-gray-600">
            取消跟随
          </button>
        </Alignment>
        <p class="text-sm text-gray-500 ml-[8.25rem] mt-1">
          [应用中心-团队应用] 中已开启的应用将安装到浏览器。
        </p>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
