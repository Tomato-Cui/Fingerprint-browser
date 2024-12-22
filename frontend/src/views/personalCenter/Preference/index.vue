<!-- 偏好设置 -->
<template>
  <div class="max-w-3xl mx-auto p-6 space-y-6">
    <!-- Info Banner -->
    <div class="bg-blue-50 border border-blue-100 rounded-lg p-4 flex items-center space-x-2">
      <InfoIcon class="w-5 h-5 text-blue-500" />
      <span class="text-sm text-blue-700">新建浏览器时，将优先以偏好设置的参数作为预设参数。</span>
    </div>

    <!-- Browser Selection -->
    <div class="grid grid-cols-12 items-center gap-4">
      <label class="col-span-2 text-gray-600">浏览器</label>
      <div class="col-span-10 flex space-x-4">
        <div class="relative w-48">
          <select class="w-full appearance-none bg-white border border-gray-200 rounded-lg pl-10 pr-8 py-2">
            <option>SunBrowser</option>
          </select>
          <div class="absolute left-3 top-2.5">
            <img src="" class="w-5 h-5" alt="SunBrowser" />
          </div>
          <ChevronDownIcon class="absolute right-3 top-3 w-4 h-4 text-gray-400" />
        </div>
        <div class="relative w-48">
          <select class="w-full appearance-none bg-white border border-gray-200 rounded-lg pl-10 pr-8 py-2">
            <option>FlowerBrowser</option>
          </select>
          <div class="absolute left-3 top-2.5">
            <img src="" class="w-5 h-5" alt="FlowerBrowser" />
          </div>
          <ChevronDownIcon class="absolute right-3 top-3 w-4 h-4 text-gray-400" />
        </div>
      </div>
    </div>

    <!-- OS Match -->
    <div class="grid grid-cols-12 items-center gap-4">
      <label class="col-span-2 text-gray-600">操作系统</label>
      <div class="col-span-10">
        <label class="inline-flex items-center space-x-2">
          <div class="relative inline-block w-10 h-6 rounded-full bg-blue-600">
            <input type="checkbox" class="sr-only peer" checked>
            <span class="absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform duration-200 peer-checked:translate-x-4"></span>
          </div>
          <span class="text-sm">基于电脑匹配</span>
        </label>
      </div>
    </div>

    <!-- User Agent -->
    <div class="grid grid-cols-12 items-center gap-4">
      <label class="col-span-2 text-gray-600">User-Agent</label>
      <div class="col-span-10">
        <div class="relative w-full">
          <select class="w-full appearance-none bg-white border border-gray-200 rounded-lg px-4 py-2">
            <option>全部</option>
          </select>
          <ChevronDownIcon class="absolute right-3 top-3 w-4 h-4 text-gray-400" />
        </div>
      </div>
    </div>

    <!-- WebRTC -->
    <div class="grid grid-cols-12 items-start gap-4">
      <label class="col-span-2 text-gray-600">WebRTC</label>
      <div class="col-span-10">
        <div class="inline-flex rounded-lg border border-gray-200 p-1">
          <button class="px-4 py-1 rounded text-sm" :class="webrtc === '转发' ? 'bg-white shadow' : 'text-gray-500'" @click="webrtc = '转发'">转发</button>
          <button class="px-4 py-1 rounded text-sm" :class="webrtc === '替换' ? 'bg-white shadow' : 'text-gray-500'" @click="webrtc = '替换'">替换</button>
          <button class="px-4 py-1 rounded text-sm" :class="webrtc === '真实' ? 'bg-white shadow' : 'text-gray-500'" @click="webrtc = '真实'">真实</button>
          <button class="px-4 py-1 rounded text-sm text-blue-600" :class="webrtc === '禁用' ? 'bg-white shadow' : ''" @click="webrtc = '禁用'">禁用</button>
        </div>
      </div>
    </div>

    <!-- Time Zone -->
    <div class="grid grid-cols-12 items-start gap-4">
      <label class="col-span-2 text-gray-600">时区</label>
      <div class="col-span-10 space-y-4">
        <div class="inline-flex rounded-lg border border-gray-200 p-1">
          <button class="px-4 py-1 rounded text-sm" :class="timezone === '基于IP' ? 'bg-white shadow' : 'text-gray-500'" @click="timezone = '基于IP'">基于 IP</button>
          <button class="px-4 py-1 rounded text-sm" :class="timezone === '真实' ? 'bg-white shadow' : 'text-gray-500'" @click="timezone = '真实'">真实</button>
          <button class="px-4 py-1 rounded text-sm text-blue-600" :class="timezone === '自定义' ? 'bg-white shadow' : ''" @click="timezone = '自定义'">自定义</button>
        </div>
        <div class="relative w-full">
          <select class="w-full appearance-none bg-white border border-gray-200 rounded-lg px-4 py-2">
            <option>GMT-12:00 Etc/GMT+12</option>
          </select>
          <ChevronDownIcon class="absolute right-3 top-3 w-4 h-4 text-gray-400" />
        </div>
      </div>
    </div>

    <!-- Geolocation -->
    <div class="grid grid-cols-12 items-start gap-4">
      <label class="col-span-2 text-gray-600">地理位置</label>
      <div class="col-span-10 space-y-4">
        <div class="inline-flex rounded-lg border border-gray-200 p-1">
          <button class="px-4 py-1 rounded text-sm" :class="location === '基于IP' ? 'bg-white shadow' : 'text-gray-500'" @click="location = '基于IP'">基于 IP</button>
          <button class="px-4 py-1 rounded text-sm text-blue-600" :class="location === '自定义' ? 'bg-white shadow' : ''" @click="location = '自定义'">自定义</button>
          <button class="px-4 py-1 rounded text-sm" :class="location === '禁止' ? 'bg-white shadow' : 'text-gray-500'" @click="location = '禁止'">禁止</button>
        </div>
        
        <div class="space-y-4 bg-gray-50 p-4 rounded-lg">
          <div class="flex items-center space-x-4">
            <label class="inline-flex items-center">
              <input type="radio" name="location-type" class="form-radio text-blue-600" checked>
              <span class="ml-2">每次询问</span>
            </label>
            <label class="inline-flex items-center">
              <input type="radio" name="location-type" class="form-radio text-blue-600">
              <span class="ml-2">始终允许</span>
            </label>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm mb-1">
                <span class="text-red-500">*</span>
                纬度/经度
              </label>
              <input type="number" value="0" class="w-full border border-gray-200 rounded-lg px-3 py-2">
            </div>
            <div>
              <label class="block text-sm mb-1 invisible">占位</label>
              <input type="number" value="0" class="w-full border border-gray-200 rounded-lg px-3 py-2">
            </div>
          </div>

          <div>
            <label class="block text-sm mb-1">精度(米)</label>
            <input type="number" value="1000" class="w-full border border-gray-200 rounded-lg px-3 py-2">
          </div>
        </div>
      </div>
    </div>

    <!-- Language -->
    <div class="grid grid-cols-12 items-start gap-4">
      <label class="col-span-2 text-gray-600">语言</label>
      <div class="col-span-10 space-y-4">
        <div class="inline-flex rounded-lg border border-gray-200 p-1">
          <button class="px-4 py-1 rounded text-sm" :class="lang === '基于IP' ? 'bg-white shadow' : 'text-gray-500'" @click="lang = '基于IP'">基于 IP</button>
          <button class="px-4 py-1 rounded text-sm text-blue-600" :class="lang === '自定义' ? 'bg-white shadow' : ''" @click="lang = '自定义'">自定义</button>
        </div>

        <div class="space-y-2 bg-gray-50 p-4 rounded-lg">
          <div class="flex items-center justify-between bg-white p-3 rounded border border-gray-200">
            <span>英语（美国）</span>
            <MoreVerticalIcon class="w-4 h-4 text-gray-400" />
          </div>
          <div class="flex items-center justify-between bg-white p-3 rounded border border-gray-200">
            <span>英语</span>
            <MoreVerticalIcon class="w-4 h-4 text-gray-400" />
          </div>
          <button class="flex items-center space-x-1 text-blue-600 text-sm mt-2">
            <PlusIcon class="w-4 h-4" />
            <span>添加语言</span>
          </button>
        </div>
      </div>
    </div>
    
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { 
  InfoIcon, 
  ChevronDownIcon,
  MoreVerticalIcon,
  PlusIcon
} from 'lucide-vue-next'

const webrtc = ref('转发')
const timezone = ref('基于IP')
const location = ref('基于IP')
const lang = ref('基于IP')
</script>

<style scoped lang="less">
</style>
