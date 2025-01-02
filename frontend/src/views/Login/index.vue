<template>
  <div class="font-[sans-serif] bg-gray-900 md:h-screen">
    <div class="grid md:grid-cols-2 items-center gap-8 h-full">
      <div class="max-md:order-1 p-4">
        <img src="https://readymadeui.com/signin-image.webp"
          class="lg:max-w-[80%] w-full h-full object-contain block mx-auto" alt="login-image" />
      </div>

      <div class="flex items-center md:p-8 p-6 bg-white md:rounded-tl-[55px] md:rounded-bl-[55px] h-full">
        <!-- 登录 -->
        <form class="max-w-lg w-full mx-auto" v-show="isLoginOrRegister === 'login'">
          <div class="mb-12">
            <h3 class="text-gray-800 text-4xl font-extrabold">登录</h3>
            <p class="text-gray-800 text-sm mt-4 ">没有账号？ <a href="javascript:void(0);"
                @click="() => {isLoginOrRegister = 'register', showPassword = false}"
                class="text-blue-600 font-semibold hover:underline ml-1 whitespace-nowrap">注册</a></p>
          </div>

          <div>
            <label class="text-gray-800 text-xs block mb-2">账号</label>
            <div class="relative flex items-center">
              <input type="text" v-model="loginForm.username" required
                class="w-full text-sm border-b border-gray-300 focus:border-gray-800 px-2 py-3 outline-none"
                placeholder="请输入账号" />
              <svg xmlns="http://www.w3.org/2000/svg" fill="#bbb" stroke="#bbb"
                class="w-[18px] h-[18px] absolute right-2" viewBox="0 0 682.667 682.667">
                <defs>
                  <clipPath id="a" clipPathUnits="userSpaceOnUse">
                    <path d="M0 512h512V0H0Z" data-original="#000000"></path>
                  </clipPath>
                </defs>
                <g clip-path="url(#a)" transform="matrix(1.33 0 0 -1.33 0 682.667)">
                  <path fill="none" stroke-miterlimit="10" stroke-width="40"
                    d="M452 444H60c-22.091 0-40-17.909-40-40v-39.446l212.127-157.782c14.17-10.54 33.576-10.54 47.746 0L492 364.554V404c0 22.091-17.909 40-40 40Z"
                    data-original="#000000"></path>
                  <path
                    d="M472 274.9V107.999c0-11.027-8.972-20-20-20H60c-11.028 0-20 8.973-20 20V274.9L0 304.652V107.999c0-33.084 26.916-60 60-60h392c33.084 0 60 26.916 60 60v196.653Z"
                    data-original="#000000"></path>
                </g>
              </svg>
            </div>
          </div>

          <div class="mt-8">
            <label class="text-gray-800 text-xs block mb-2">密码</label>
            <div class="relative flex items-center">
              <input v-model="loginForm.password" :type="showPassword ? 'text' : 'password'" required
                class="w-full text-sm border-b border-gray-300 focus:border-gray-800 px-2 py-3 outline-none"
                placeholder="请输入密码" />
              <svg xmlns="http://www.w3.org/2000/svg" fill="#bbb" stroke="#bbb"
                class="w-[18px] h-[18px] absolute right-2 cursor-pointer" viewBox="0 0 128 128"
                @click="showPassword = !showPassword">
                <path
                  d="M64 104C22.127 104 1.367 67.496.504 65.943a4 4 0 0 1 0-3.887C1.367 60.504 22.127 24 64 24s62.633 36.504 63.496 38.057a4 4 0 0 1 0 3.887C126.633 67.496 105.873 104 64 104zM8.707 63.994C13.465 71.205 32.146 96 64 96c31.955 0 50.553-24.775 55.293-31.994C114.535 56.795 95.854 32 64 32 32.045 32 13.447 56.775 8.707 63.994zM64 88c-13.234 0-24-10.766-24-24s10.766-24 24-24 24 10.766 24 24-10.766 24-24 24zm0-40c-8.822 0-16 7.178-16 16s7.178 16 16 16 16-7.178 16-16-7.178-16-16-16z"
                  data-original="#000000"></path>
              </svg>
            </div>
          </div>


          <div class="flex flex-wrap items-center justify-between gap-4 mt-6">
            <div class="flex items-center">
              <input id="remember-me" name="remember-me" type="checkbox"
                class="h-4 w-4 shrink-0 text-blue-600 focus:ring-blue-500 border-gray-300 rounded" />
              <label for="remember-me" class="text-gray-800 ml-3 block text-sm">
                记住我
              </label>
            </div>
            <!-- <div>
              <a href="jajvascript:void(0);" class="text-blue-600 font-semibold text-sm hover:underline">
                忘记密码？
              </a>
            </div> -->
          </div>

          <div class="mt-12">
            <button type="button" :disabled="loading" @click="handleLogin"
              class="w-full py-3 px-6 text-sm font-semibold tracking-wider rounded-full text-white bg-gray-800 hover:bg-[#222] focus:outline-none">
              {{ loading ? "登录中..." : "登录" }}
            </button>
          </div>

          <!-- <div class="my-6 flex items-center gap-4">
            <hr class="w-full border-gray-300" />
            <p class="text-sm text-gray-800 text-center">or</p>
            <hr class="w-full border-gray-300" />
          </div>

          <button type="button"
            class="w-full flex items-center justify-center gap-4 py-3 px-6 text-sm font-semibold tracking-wider text-gray-800 border border-gray-300 rounded-full bg-gray-50 hover:bg-gray-100 focus:outline-none">
            <svg xmlns="http://www.w3.org/2000/svg" width="20px" class="inline" viewBox="0 0 512 512">
              <path fill="#fbbd00"
                d="M120 256c0-25.367 6.989-49.13 19.131-69.477v-86.308H52.823C18.568 144.703 0 198.922 0 256s18.568 111.297 52.823 155.785h86.308v-86.308C126.989 305.13 120 281.367 120 256z"
                data-original="#fbbd00" />
              <path fill="#0f9d58"
                d="m256 392-60 60 60 60c57.079 0 111.297-18.568 155.785-52.823v-86.216h-86.216C305.044 385.147 281.181 392 256 392z"
                data-original="#0f9d58" />
              <path fill="#31aa52"
                d="m139.131 325.477-86.308 86.308a260.085 260.085 0 0 0 22.158 25.235C123.333 485.371 187.62 512 256 512V392c-49.624 0-93.117-26.72-116.869-66.523z"
                data-original="#31aa52" />
              <path fill="#3c79e6"
                d="M512 256a258.24 258.24 0 0 0-4.192-46.377l-2.251-12.299H256v120h121.452a135.385 135.385 0 0 1-51.884 55.638l86.216 86.216a260.085 260.085 0 0 0 25.235-22.158C485.371 388.667 512 324.38 512 256z"
                data-original="#3c79e6" />
              <path fill="#cf2d48"
                d="m352.167 159.833 10.606 10.606 84.853-84.852-10.606-10.606C388.668 26.629 324.381 0 256 0l-60 60 60 60c36.326 0 70.479 14.146 96.167 39.833z"
                data-original="#cf2d48" />
              <path fill="#eb4132"
                d="M256 120V0C187.62 0 123.333 26.629 74.98 74.98a259.849 259.849 0 0 0-22.158 25.235l86.308 86.308C162.883 146.72 206.376 120 256 120z"
                data-original="#eb4132" />
            </svg>
            继续使用谷歌
          </button> -->
        </form>
        <!-- 注册 -->
        <form class="max-w-lg w-full mx-auto" v-show="isLoginOrRegister === 'register'">
          <div class="mb-6">
            <h3 class="text-gray-800 text-2xl font-bold">创建一个账号</h3>
          </div>

          <div class="space-y-6">
            <div>
              <label class="text-gray-800 text-sm mb-2 block">用户名</label>
              <div class="relative flex items-center">
                <input name="name" type="text" required v-model="registerForm.username"
                  class="w-full text-sm border-b border-gray-300 focus:border-gray-800 px-2 py-3 outline-none"
                  placeholder="请输入您的名字" />
                <svg xmlns="http://www.w3.org/2000/svg" fill="#bbb" stroke="#bbb" class="w-4 h-4 absolute right-4"
                  viewBox="0 0 24 24">
                  <circle cx="10" cy="7" r="6" data-original="#000000"></circle>
                  <path
                    d="M14 15H6a5 5 0 0 0-5 5 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 5 5 0 0 0-5-5zm8-4h-2.59l.3-.29a1 1 0 0 0-1.42-1.42l-2 2a1 1 0 0 0 0 1.42l2 2a1 1 0 0 0 1.42 0 1 1 0 0 0 0-1.42l-.3-.29H22a1 1 0 0 0 0-2z"
                    data-original="#000000"></path>
                </svg>
              </div>
            </div>

            <div>
              <label class="text-gray-800 text-sm mb-2 block">邮箱</label>
              <div class="relative flex items-center">
                <input name="email" type="email" required v-model="registerForm.email"
                  class="w-full text-sm border-b border-gray-300 focus:border-gray-800 px-2 py-3 outline-none"
                  placeholder="请输入您的邮箱" />
                <svg xmlns="http://www.w3.org/2000/svg" fill="#bbb" stroke="#bbb" class="w-4 h-4 absolute right-4"
                  viewBox="0 0 682.667 682.667">
                  <defs>
                    <clipPath id="a" clipPathUnits="userSpaceOnUse">
                      <path d="M0 512h512V0H0Z" data-original="#000000"></path>
                    </clipPath>
                  </defs>
                  <g clip-path="url(#a)" transform="matrix(1.33 0 0 -1.33 0 682.667)">
                    <path fill="none" stroke-miterlimit="10" stroke-width="40"
                      d="M452 444H60c-22.091 0-40-17.909-40-40v-39.446l212.127-157.782c14.17-10.54 33.576-10.54 47.746 0L492 364.554V404c0 22.091-17.909 40-40 40Z"
                      data-original="#000000"></path>
                    <path
                      d="M472 274.9V107.999c0-11.027-8.972-20-20-20H60c-11.028 0-20 8.973-20 20V274.9L0 304.652V107.999c0-33.084 26.916-60 60-60h392c33.084 0 60 26.916 60 60v196.653Z"
                      data-original="#000000"></path>
                  </g>
                </svg>
              </div>
            </div>

            <div class="mt-8">
            <label class="text-gray-800 text-xs block mb-2">密码</label>
            <div class="relative flex items-center">
              <input v-model="registerForm.password" :type="showPassword ? 'text' : 'password'" required
                class="w-full text-sm border-b border-gray-300 focus:border-gray-800 px-2 py-3 outline-none"
                placeholder="请输入密码" />
              <svg xmlns="http://www.w3.org/2000/svg" fill="#bbb" stroke="#bbb"
                class="w-[18px] h-[18px] absolute right-2 cursor-pointer" viewBox="0 0 128 128"
                @click="showPassword = !showPassword">
                <path
                  d="M64 104C22.127 104 1.367 67.496.504 65.943a4 4 0 0 1 0-3.887C1.367 60.504 22.127 24 64 24s62.633 36.504 63.496 38.057a4 4 0 0 1 0 3.887C126.633 67.496 105.873 104 64 104zM8.707 63.994C13.465 71.205 32.146 96 64 96c31.955 0 50.553-24.775 55.293-31.994C114.535 56.795 95.854 32 64 32 32.045 32 13.447 56.775 8.707 63.994zM64 88c-13.234 0-24-10.766-24-24s10.766-24 24-24 24 10.766 24 24-10.766 24-24 24zm0-40c-8.822 0-16 7.178-16 16s7.178 16 16 16 16-7.178 16-16-7.178-16-16-16z"
                  data-original="#000000"></path>
              </svg>
            </div>
          </div>

            <div>
              <label class="text-gray-800 text-sm mb-2 block">验证码</label>
              <div class="relative flex items-center">
                <input type="text" required v-model="registerForm.code"
                  class="w-full text-sm border-b border-gray-300 focus:border-gray-800 px-2 py-3 outline-none"
                  placeholder="请输入验证码" />
                <svg xmlns="http://www.w3.org/2000/svg" fill="#bbb" stroke="#bbb"
                  class="w-4 h-4 absolute right-4 cursor-pointer" viewBox="0 0 128 128">
                  <path
                    d="M64 104C22.127 104 1.367 67.496.504 65.943a4 4 0 0 1 0-3.887C1.367 60.504 22.127 24 64 24s62.633 36.504 63.496 38.057a4 4 0 0 1 0 3.887C126.633 67.496 105.873 104 64 104zM8.707 63.994C13.465 71.205 32.146 96 64 96c31.955 0 50.553-24.775 55.293-31.994C114.535 56.795 95.854 32 64 32 32.045 32 13.447 56.775 8.707 63.994zM64 88c-13.234 0-24-10.766-24-24s10.766-24 24-24 24 10.766 24 24-10.766 24-24 24zm0-40c-8.822 0-16 7.178-16 16s7.178 16 16 16 16-7.178 16-16-7.178-16-16-16z"
                    data-original="#000000"></path>
                </svg>
                <PrimaryButton @click="registerSendCode" class="whitespace-nowrap ml-2 bg-slate-500">
                  发送验证码
                </PrimaryButton>
              </div>
            </div>

            <!-- <div class="flex items-center">
              <input id="remember-me" name="remember-me" type="checkbox"
                class="h-4 w-4 shrink-0 text-blue-600 focus:ring-blue-500 border-gray-300 rounded" />
              <label for="remember-me" class="ml-3 block text-sm text-gray-800">
                我接受 <a href="javascript:void(0);" class="text-blue-600 font-semibold hover:underline ml-1">《隐私条约》</a>
              </label>
            </div> -->
          </div>

          <div class="!mt-12">
            <button type="button" @click="handleRegister" :disabled="loading"
              class="w-full py-3 px-4 tracking-wider text-sm rounded-full text-white bg-gray-700 hover:bg-gray-800 focus:outline-none">
              {{ loading ? "注册中..." : "注册" }}
            </button>

          </div>
          <p class="text-gray-800 text-sm mt-6 text-center">已经存在账号? <a href="javascript:void(0);"
              @click="() => {isLoginOrRegister = 'login', showPassword = false}" class="text-blue-600 font-semibold hover:underline ml-1">这里登录</a></p>
        </form>
      </div>
    </div>
  </div>
</template>
<script setup>
import { useRouter } from "vue-router";
import Input from "@/components/input.vue";
import { ActivityIcon, MagnetIcon, ChartAreaIcon } from "lucide-vue-next";
import { ref, computed } from "vue";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import {
  PrimaryButton as Button,
  PrimaryButton,
} from "@/components/button/index";
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import {
  LucideGithub,
  LucideMessageCircle,
  LucideSmartphone,
} from "lucide-vue-next";
import { login, register, register_send } from "@/commands/user";
import { toast } from "vue-sonner";
import { storeToRefs } from "pinia";
import { useUserStore } from "@/stores/user";
import TooltipButton from "@/components/tooltip-button.vue";

const activeTab = ref("login");
const loading = ref(false);
const isCodeSending = ref(false);
const countdown = ref(60);
const router = useRouter();
const userSotre = useUserStore();
const isLoginOrRegister = ref("login");  //当前是登录还是注册
const showPassword = ref(false);  //是否显示密码

const loginForm = ref({
  username: "",
  password: "",
  rememberMe: false,
});

const registerForm = ref({
  username: "",
  password: "",
  email: "",
  code: "",
});

const handleLogin = () => {
  loading.value = true;

  login(loginForm.value.username, loginForm.value.password)
    .then((res) => {
      if (res.code == 1) {
        loading.value = false;
        toast.success(res.message);
        router.push("/environment");
        userSotre.login({
          account: loginForm.value.username,
        });
      } else {
        loading.value = false;
        toast.warning(res.message);
      }
    })
    .catch((err) => {
      loading.value = false;
      toast.warning("登录失败");
    });
};

const handleRegister = () => {
  loading.value = true;

  register(
    registerForm.value.email,
    registerForm.value.code,
    registerForm.value.username,
    registerForm.value.password
  )
    .then((res) => {
      loading.value = false;
      res.code ? toast.success(res.message) : toast.warning(res.message);
    })
    .catch((err) => {
      loading.value = false;
      toast.success("注册失败，请前往重新注册");
    });
};

const registerSendCode = () => {
  let email = registerForm.value.email;

  if (email) {
    register_send(email).then((res) => {
      res.code ? toast.success(res.message) : toast.warning(res.message);
    });
  }
};

const forgotPassword = () => {
  console.log("Forgot password");
};
</script>

<style scoped>
  input[type="password"]::-ms-reveal,
  input[type="password"]::-webkit-credentials-auto-fill-button {
    display: none;
  }
</style>
