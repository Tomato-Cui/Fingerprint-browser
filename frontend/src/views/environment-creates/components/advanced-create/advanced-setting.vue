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
          <p class="w-32 text-left text-sm">时区</p>
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
          <p class="w-32 text-left text-sm">WebRTC</p>
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
          <p class="w-32 text-left text-sm">地理位置</p>
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
          <p class="w-32 text-left text-sm">语言</p>
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
          <p class="w-32 text-left text-sm">分辨率</p>
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
          <p class="w-32 text-left text-sm">字体</p>
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
          <p class="w-32 text-left text-sm">Canvas</p>
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
          <p class="w-32 text-left text-sm">WebGL图像</p>
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
          <p class="w-32 text-left text-sm">WebGPU</p>
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
          <p class="w-32 text-left text-sm">AudioContext</p>
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
          <p class="w-32 text-left text-sm">媒体设备</p>
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
          <p class="w-32 text-left text-sm">ClientRects</p>
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
          <p class="w-32 text-left text-sm">SpeechVoices</p>
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
          <p class="w-32 text-left text-sm">硬件并发数</p>
          <Select v-model="hardwareConcurrency" :default-value="'1'">
            <SelectTrigger class="w-1/3">
              <SelectValue
                placeholder="1"
                class="w-full p-2 rounded-lg outline-none"
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
          <p class="w-32 text-left text-sm">设备内存</p>
          <Select v-model="deviceMemory" :default-value="deviceMemory">
            <SelectTrigger class="w-1/3">
              <SelectValue
                placeholder="4"
                class="w-full p-2 rounded-lg outline-none"
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
            <p class="text-left text-sm">设备名称</p>
          </div>
          <div class="grow space-y-2">
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
            <p class="text-left text-sm">MAC地址</p>
          </div>
          <div class="grow space-y-2">
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
          <p class="w-32 text-left text-sm">Do Not Track</p>
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
          <p class="w-32 text-left text-sm">蓝牙</p>
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
          <p class="w-32 text-left text-sm">电池</p>
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
            <p class="text-left text-sm">端口扫描保护</p>
          </div>
          <div class="grow">
            <div class="w-full space-y-2">
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
            <p class="text-left text-sm">启动参数</p>
          </div>
          <Textarea
            rows="4"
            placeholder="输入内容可以配置浏览器执行时的启动参数"
            class="w-1/2 px-3 py-2 border rounded-md placeholder-gray-400"
            v-model="forms.fingerprint.startupParams.value"
          ></Textarea>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
