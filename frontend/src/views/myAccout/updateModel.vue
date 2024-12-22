<script setup >
import {ChevronDownIcon, EyeIcon, EyeOffIcon, HelpCircleIcon, XIcon} from "lucide-vue-next";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import {reactive, ref} from 'vue'
const showPassword = ref(false)
const props = defineProps({
    open: false,
    title: '',
  });

const form = reactive({
  platform: 'Gmail',
  accountName: 'GM1145',
  email: '',
  password: ''
})

  const emit = defineEmits(['close']);

</script>
<template>
  <div v-if="props.open" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg w-full max-w-md mx-4">
      <!-- Header -->
      <div class="flex justify-between items-center p-4">
        <h2 class="text-lg font-medium">编辑账号</h2>
        <button
            class="text-gray-400 hover:text-gray-600"
            @click="() => emit('close')"
        >
          <XIcon class="h-5 w-5"/>
        </button>
      </div>

      <!-- Form Content -->
      <div class="p-4 space-y-4">
        <!-- Platform -->
        <div>
          <label class="block text-sm mb-1">
            平台
            <span class="text-red-500">*</span>
          </label>
            <Select >
              <SelectTrigger  class="bg-gray-50 p-1">
                <SelectValue
                    placeholder="Gmail"
                    v-model="form.platform"
                    class="w-full p-2 rounded-lg outline-none"
                />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="Gmail">
                    Gmail
                  </SelectItem>
                  <SelectItem value="guge">
                    Guge
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
        </div>

        <!-- Account Name -->
        <div>
          <label class="block text-sm mb-1">账号名称</label>
          <div class="relative">
            <input
                v-model="form.accountName"
                class="w-full border border-gray-200 rounded-md shadow-sm p-2 text-sm bg-gray-50"
                placeholder="GM1145"
                type="text"
            >
            <button class="absolute right-2 top-1/2 transform -translate-y-1/2">
              <HelpCircleIcon class="h-4 w-4 text-gray-400"/>
            </button>
          </div>
        </div>

        <!-- Email -->
        <div>
          <label class="block text-sm mb-1">
            账号
            <span class="text-red-500">*</span>
          </label>
          <input
              v-model="form.email"
              class="w-full border border-gray-200 rounded-md shadow-sm p-2 text-sm"
              placeholder="example@gmail.com"
              type="email"
          >
        </div>

        <!-- Password -->
        <div>
          <label class="block text-sm mb-1">
            账号密码
            <span class="text-red-500">*</span>
          </label>
          <div class="relative">
            <input
                v-model="form.password"
                :type="showPassword ? 'text' : 'password'"
                class="w-full border border-gray-200 rounded-md shadow-sm p-2 text-sm"
                placeholder="••••••••"
            >
            <button
                class="absolute right-2 top-1/2 transform -translate-y-1/2"
                @click="showPassword = !showPassword"
            >
              <EyeIcon v-if="!showPassword" class="h-4 w-4 text-gray-400"/>
              <EyeOffIcon v-else class="h-4 w-4 text-gray-400"/>
            </button>
          </div>
        </div>

        <!-- 2FA Section -->
        <div class="flex items-center justify-between text-sm">
          <div class="flex items-center space-x-1 text-gray-500">
            <span>2FA验证</span>
            <HelpCircleIcon class="h-4 w-4"/>
          </div>
          <button class="text-blue-500 hover:text-blue-600">
            上传二维码
          </button>
        </div>
        <div class="text-xs text-gray-400">
          请输入人工验证码后上传二维码
        </div>

        <!-- Action Buttons -->
        <div class="flex justify-end space-x-3 pt-4">
          <button
              class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800"
              @click="() => emit('close')"
          >
            取消
          </button>
          <button
              class="px-4 py-2 text-sm text-white bg-blue-500 rounded-md hover:bg-blue-600"
              @click="() => emit('close')"
          >
            确定
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
