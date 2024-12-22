<template>
  <div class="max-w-4xl mx-auto p-6 space-y-8">
    <section>
      <h2 class="text-lg font-medium mb-4">账户信息</h2>
      <div class="bg-gray-50 rounded-lg p-6">
        <div class="flex items-start space-x-4 mb-6">
          <img src="" alt="" class="w-12 h-12 rounded-full" />
          <div>
            <div class="flex items-center space-x-2">
              <span class="font-medium">198581523</span>
              <CopyIcon class="w-4 h-4 text-blue-600 cursor-pointer" />
            </div>
            <div class="text-sm text-gray-500 mt-1">
              <div>用户ID：user_h197twq</div>
              <div>团队ID：team_h11kaom</div>
              <div>团队名称：198581523111</div>
            </div>
          </div>
        </div>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <span class="text-gray-600">邮箱</span>
            <span>yingchao04@qq.com</span>
          </div>

          <div class="flex items-center justify-between">
            <span class="text-gray-600">手机</span>
            <div class="flex items-center space-x-4">
              <span>+86 19*******11</span>
              <button class="text-blue-600 text-sm">修改</button>
              <button class="text-blue-600 text-sm">解绑</button>
            </div>
          </div>

          <div class="flex items-center justify-between">
            <span class="text-gray-600">密码</span>
            <div class="flex items-center space-x-4">
              <span>********</span>
              <button class="text-blue-600 text-sm">修改</button>
            </div>
          </div>

          <div class="flex items-center justify-between">
            <span class="text-gray-600">实名认证</span>
            <div class="flex items-center space-x-2">
              <CheckCircle2Icon class="w-4 h-4 text-green-500" />
              <span>认证通过</span>
              <button class="text-blue-600 text-sm">查看</button>
            </div>
          </div>

          <div class="flex items-center justify-between">
            <span class="text-gray-600">身份验证器</span>
            <div class="flex items-center space-x-2">
              <XCircleIcon class="w-4 h-4 text-gray-400" />
              <span class="text-gray-500">未绑定</span>
              <button class="text-blue-600 border border-blue-600 rounded-md px-3 py-1 text-sm">
                点击绑定
              </button>
            </div>
          </div>
        </div>
      </div>
    </section>

    <section>
      <h2 class="text-lg font-medium mb-4">账户关联</h2>
      <div class="bg-gray-50 rounded-lg p-6">
        <p class="text-gray-500 text-sm mb-4">绑定后可通过第三方应用快速登录</p>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <img src="" alt="Google" class="w-6 h-6" />
              <span>yingchaow29@gmail.com</span>
            </div>
            <button class="text-blue-600 text-sm">解绑</button>
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <img src="" alt="Facebook" class="w-6 h-6" />
              <span>Facebook账号</span>
            </div>
            <button class="text-blue-600 text-sm">绑定</button>
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <img src="" alt="VK" class="w-6 h-6" />
              <span>VK账号</span>
            </div>
            <button class="text-blue-600 text-sm">绑定</button>
          </div>
        </div>
      </div>
    </section>

    <section>
      <h2 class="text-lg font-medium mb-4">登录活动</h2>
      <div class="bg-gray-50 rounded-lg p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <MonitorIcon class="w-5 h-5 text-gray-600" />
              <div>
                <div class="font-medium">DESKTOP-P70KINV</div>
                <div class="text-sm text-gray-500">115.206.218.93 | CN</div>
              </div>
            </div>
            <span class="text-green-500 text-sm">当前</span>
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <LaptopIcon class="w-5 h-5 text-gray-600" />
              <div>
                <div class="font-medium">LAPTOP-0URQE367</div>
                <div class="text-sm text-gray-500">60.177.108.68 | CN</div>
              </div>
            </div>
            <div class="flex items-center space-x-4">
              <span class="text-sm text-gray-500">2024-12-05 20:59</span>
              <button class="text-blue-600 text-sm">退出登录</button>
            </div>
          </div>
        </div>

        <div class="mt-4 text-right">
          <button class="text-blue-600 text-sm flex items-center space-x-1 ml-auto">
            <LogOutIcon class="w-4 h-4" />
            <span>退出全部</span>
          </button>
        </div>
      </div>
    </section>

    <div class="min-h-screen flex flex-col bg-gray-50">
      <div class="flex-1 max-w-2xl mx-auto w-full p-6">
        <h2 class="text-lg font-medium mb-6">其他</h2>

        <div class="space-y-6">
          <div class="space-y-2">
            <label class="block text-gray-600">语言</label>
            <div class="relative">
              <select v-model="language"
                class="w-full appearance-none bg-white border border-gray-200 rounded-lg px-4 py-2.5 pr-10">
                <option value="zh-CN">简体中文</option>
                <option value="en">English</option>
              </select>
              <ChevronDownIcon
                class="w-5 h-5 text-gray-400 absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none" />
            </div>
          </div>

          <div class="space-y-2">
            <label class="block text-gray-600">时区</label>
            <div class="relative">
              <select v-model="timezone"
                class="w-full appearance-none bg-white border border-gray-200 rounded-lg px-4 py-2.5 pr-10">
                <option value="Asia/Shanghai">GMT+08:00 Asia/Shanghai</option>
                <option value="UTC">GMT+00:00 UTC</option>
              </select>
              <ChevronDownIcon
                class="w-5 h-5 text-gray-400 absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none" />
            </div>
          </div>

          <div class="flex items-center justify-between">
            <label class="text-gray-600">通知</label>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="notifications" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer 
                      peer-checked:after:translate-x-full peer-checked:after:border-white 
                      after:content-[''] after:absolute after:top-[2px] after:left-[2px] 
                      after:bg-white after:border-gray-300 after:border after:rounded-full 
                      after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
              <span class="ml-3 text-sm text-gray-600">订阅营销通知 (邮件/短信)</span>
            </label>
          </div>
        </div>
      </div>
    </div>
    <div class="sticky bottom-0 border-t bg-white py-4 px-6">
      <div class="max-w-2xl mx-auto">
        <button @click="saveSettings"
          class="w-32 bg-blue-600 text-white rounded-lg py-2 px-4 hover:bg-blue-700 transition-colors">
          确定
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import {
  CopyIcon,
  CheckCircle2Icon,
  XCircleIcon,
  MonitorIcon,
  LaptopIcon,
  LogOutIcon
} from 'lucide-vue-next'
import { ref } from 'vue'
import { ChevronDownIcon } from 'lucide-vue-next'

const language = ref('zh-CN')
const timezone = ref('Asia/Shanghai')
const notifications = ref(false)

const saveSettings = () => {
  // Handle save settings
  console.log({
    language: language.value,
    timezone: timezone.value,
    notifications: notifications.value
  })
}
</script>