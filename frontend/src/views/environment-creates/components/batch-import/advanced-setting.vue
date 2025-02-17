<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import Radio from "../radio.vue";
import { Iconrefresh } from "@/assets/icons";
import { Textarea } from "@/components/ui/textarea";
import {
  templaeFormtData as data,
  useEnvironmentAdvancedFormStore,
} from "@/stores/form/environment-advanced";
import { ref } from "vue";

const { forms, defaultFingerprintHandle } = useEnvironmentAdvancedFormStore();
const hardwareConcurrency = ref<string>("1");
const deviceMemory = ref<string>("1");
</script>

<template>
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
              @click="
                () => {
                  defaultFingerprintHandle();
                  hardwareConcurrency = '1';
                  deviceMemory = '1';
                }
              "
              class="text-sm border whitespace-nowrap rounded-md px-2 py-2 flex justify-center items-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
            >
              <Iconrefresh class="size-4 fill-white" />
              恢复默认设置
            </button>
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">时区</p>
          <div>
            <Radio
              :check="forms.fingerprint.timezone"
              :data="data.fingerprint.timezones"
              @update:check="
                (v) => (forms.fingerprint.timezone = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">WebRTC</p>
          <div>
            <Radio
              :check="forms.fingerprint.webRTC"
              :data="data.fingerprint.webRtc"
              @update:check="
                (v) => (forms.fingerprint.webRTC = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">地理位置</p>
          <div>
            <Radio
              :check="forms.fingerprint.geolocation"
              :data="data.fingerprint.geolocations"
              @update:check="
                (v) => (forms.fingerprint.geolocation = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">语言</p>
          <div>
            <Radio
              :check="forms.fingerprint.language"
              :data="data.fingerprint.languages"
              @update:check="
                (v) => (forms.fingerprint.language = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">分辨率</p>
          <div>
            <Radio
              :check="forms.fingerprint.resolution"
              :data="data.fingerprint.resolutions"
              @update:check="
                (v) => (forms.fingerprint.resolution = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">字体</p>
          <div>
            <Radio
              :check="forms.fingerprint.fonts"
              :data="data.fingerprint.fonts"
              @update:check="
                (v) => (forms.fingerprint.fonts = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">Canvas</p>
          <div>
            <Radio
              :check="forms.fingerprint.canvas"
              :data="data.fingerprint.canvas"
              @update:check="
                (v) => (forms.fingerprint.canvas = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">WebGL图像</p>
          <div>
            <Radio
              :check="forms.fingerprint.webGL"
              :data="data.fingerprint.webGL"
              @update:check="
                (v) => (forms.fingerprint.webGL = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">WebGPU</p>
          <div>
            <Radio
              :check="forms.fingerprint.webGPU"
              :data="data.fingerprint.webGPU"
              @update:check="
                (v) => (forms.fingerprint.webGPU = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">AudioContext</p>
          <div>
            <Radio
              :check="forms.fingerprint.audioContext"
              :data="data.fingerprint.audioContext"
              @update:check="
                (v) => (forms.fingerprint.audioContext = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">媒体设备</p>
          <div>
            <Radio
              :check="forms.fingerprint.mediaDevices"
              :data="data.fingerprint.mediaDevices"
              @update:check="
                (v) => (forms.fingerprint.mediaDevices = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">ClientRects</p>
          <div>
            <Radio
              :check="forms.fingerprint.clientRects"
              :data="data.fingerprint.clientRects"
              @update:check="
                (v) => (forms.fingerprint.clientRects = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">SpeechVoices</p>
          <div>
            <Radio
              :check="forms.fingerprint.speechVoices"
              :data="data.fingerprint.speechVoices"
              @update:check="
                (v) => (forms.fingerprint.speechVoices = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">硬件并发数</p>
          <Select v-model="hardwareConcurrency" :default-value="'1'">
            <SelectTrigger class="w-1/3">
              <SelectValue
                placeholder="1"
                class="p-2 w-full rounded-lg outline-none"
              />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem
                  :key="item.id"
                  :value="item.value as any"
                  v-for="item in data.fingerprint.hardwareConcurrency"
                >
                  {{ item.label }}
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">设备内存</p>
          <Select v-model="deviceMemory" :default-value="deviceMemory">
            <SelectTrigger class="w-1/3">
              <SelectValue
                placeholder="4"
                class="p-2 w-full rounded-lg outline-none"
              />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem
                  :key="item.id"
                  :value="item.value as any"
                  v-for="item in data.fingerprint.deviceMemory"
                >
                  {{ item.label }}
                </SelectItem>
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
                :check="forms.fingerprint.deviceName"
                :data="data.fingerprint.deviceNames"
                @update:check="
                  (v) => (forms.fingerprint.deviceName = { value: v.value })
                "
              />
            </div>
          </div>
        </div>

        <div className="flex">
          <div class="w-32">
            <p class="text-sm text-left">MAC地址</p>
          </div>
          <div class="space-y-2 grow">
            <div class="w-[7.6rem]">
              <Radio
                :check="forms.fingerprint.macAddress"
                :data="data.fingerprint.macAddresses"
                @update:check="
                  (v) => (forms.fingerprint.macAddress = { value: v.value })
                "
              />
            </div>
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">Do Not Track</p>
          <div>
            <Radio
              :check="forms.fingerprint.doNotTrack"
              :data="data.fingerprint.doNotTrack"
              @update:check="
                (v) => (forms.fingerprint.doNotTrack = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">蓝牙</p>
          <div>
            <Radio
              :check="forms.fingerprint.bluetooth"
              :data="data.fingerprint.bluetooth"
              @update:check="
                (v) => (forms.fingerprint.bluetooth = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-sm text-left">电池</p>
          <div>
            <Radio
              :check="forms.fingerprint.battery"
              :data="data.fingerprint.battery"
              @update:check="
                (v) => (forms.fingerprint.battery = { value: v.value })
              "
            />
          </div>
        </div>

        <div className="flex">
          <div class="w-32">
            <p class="text-sm text-left">端口扫描保护</p>
          </div>
          <div class="grow">
            <div class="space-y-2 w-full">
              <div class="w-[6.8rem]">
                <Radio
                  :check="forms.fingerprint.portScanning"
                  :data="data.fingerprint.portScanning"
                  @update:check="
                    (v) => (forms.fingerprint.portScanning = { value: v.value })
                  "
                />
              </div>
            </div>
          </div>
        </div>

        <div className="flex">
          <div class="w-32">
            <p class="text-sm text-left">启动参数</p>
          </div>
          <Textarea
            rows="4"
            placeholder="输入内容可以配置浏览器执行时的启动参数"
            class="px-3 py-2 w-1/2 placeholder-gray-400 rounded-md border"
            v-model="forms.fingerprint.startupParams.value"
          ></Textarea>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
