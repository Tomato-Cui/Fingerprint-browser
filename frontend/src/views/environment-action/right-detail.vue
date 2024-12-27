<script setup></script>
<template>
  <div
    class="hidden xl:flex flex-col bg-gray-50 xl:w-1/3 2xl:w-1/4 rounded-md border shadow-sm p-4 h-[calc(100%-1%)] text-xs"
  >
    <div class="flex items-center justify-between mb-6">
      <h3 class="text-base font-medium">概要</h3>
      <button class="p-1 hover:bg-gray-100 rounded">
        <Menu class="w-5 h-5 text-gray-400" />
      </button>
    </div>

    <div class="space-y-4 flex-auto overflow-hidden max-h-[calc(100%-15%)]">
      <div class="flex justify-between">
        <span class="text-gray-600">浏览器</span>
        <span>SunBrowser[智能匹配]</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">User-Agent</span>
        <span class="text-right text-xs text-gray-600"
          >Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML,
          like Gecko) Chrome/131.0.6778.70 Safari/537.36</span
        >
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">WebRTC</span>
        <span>禁用</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">时区</span>
        <span>基于 IP</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">地理位置</span>
        <span>[询问]基于IP</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">语言</span>
        <span>基于 IP</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">界面语言</span>
        <span>基于语言</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">分辨率</span>
        <span>基于 User-Agent</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">字体</span>
        <span>默认</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">字体列表</span>
        <span>默认</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">字体指纹</span>
        <span>默认</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">WebRTC</span>
        <span>默认</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">Canvas</span>
        <span>默认</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">WebGL图像</span>
        <span>默认</span>
      </div>
      <div class="flex justify-between">
        <span class="text-gray-600">硬件并发数</span>
        <span>默认</span>
      </div>
      <div class="mt-6 text-sm">
        <span class="text-gray-600">可前往</span>
        <a href="#" class="text-blue-600 hover:underline">偏好设置</a>
        <span class="text-gray-600">自定义指纹默认值。</span>
      </div>
    </div>

    <button
      class="w-full mt-6 px-4 py-2 text-blue-500 rounded-md border-2 border-blue-600 hover:border-blue-600 flex items-center justify-center gap-2"
    >
      <ShuffleIcon class="w-4 h-4" />
      一键随机生成
    </button>
  </div>
</template>
