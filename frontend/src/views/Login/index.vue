<template>
  <div class="font-[sans-serif] bg-gray-900 md:h-screen">
    <div class="grid gap-8 items-center h-full md:grid-cols-2">
      <div class="p-4 max-md:order-1">
        <img
          src="https://readymadeui.com/signin-image.webp"
          class="lg:max-w-[80%] w-full h-full object-contain block mx-auto"
          alt="login-image"
        />
      </div>

      <div class="flex items-center md:p-8 p-6 bg-white md:rounded-tl-[55px] md:rounded-bl-[55px] h-full">
        <Transition name="fade" mode="out-in">
          <!-- 登录 -->
          <form
            v-if="isLoginOrRegister === 'login'"
            key="login"
            class="mx-auto w-full max-w-lg"
            @keydown.enter="handleLogin"
          >
            <div class="mb-12">
              <h3 class="text-4xl font-extrabold text-gray-800">登录</h3>
              <p class="mt-4 text-sm text-gray-800">
                没有账号？
                <a
                  href="javascript:void(0);"
                  @click="switchForm('register')"
                  class="ml-1 font-semibold text-blue-600 whitespace-nowrap hover:underline"
                  >注册</a
                >
              </p>
            </div>

            <div>
              <label class="block mb-2 text-xs text-gray-800">账号</label>
              <div class="flex relative items-center">
                <input
                  type="text"
                  v-model="loginForm.username"
                  required
                  class="px-2 py-3 w-full text-sm border-b border-gray-300 outline-none focus:border-gray-800"
                  placeholder="请输入账号"
                />
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="#bbb"
                  stroke="#bbb"
                  class="w-[18px] h-[18px] absolute right-2"
                  viewBox="0 0 682.667 682.667"
                >
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
              <label class="block mb-2 text-xs text-gray-800">密码</label>
              <div class="flex relative items-center">
                <input
                  v-model="loginForm.password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  class="px-2 py-3 w-full text-sm border-b border-gray-300 outline-none focus:border-gray-800"
                  placeholder="请输入密码"
                />
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="#bbb"
                  stroke="#bbb"
                  class="w-[18px] h-[18px] absolute right-2 cursor-pointer"
                  viewBox="0 0 128 128"
                  @click="showPassword = !showPassword"
                >
                  <path
                    d="M64 104C22.127 104 1.367 67.496.504 65.943a4 4 0 0 1 0-3.887C1.367 60.504 22.127 24 64 24s62.633 36.504 63.496 38.057a4 4 0 0 1 0 3.887C126.633 67.496 105.873 104 64 104zM8.707 63.994C13.465 71.205 32.146 96 64 96c31.955 0 50.553-24.775 55.293-31.994C114.535 56.795 95.854 32 64 32 32.045 32 13.447 56.775 8.707 63.994zM64 88c-13.234 0-24-10.766-24-24s10.766-24 24-24 24 10.766 24 24-10.766 24-24 24zm0-40c-8.822 0-16 7.178-16 16s7.178 16 16 16 16-7.178 16-16-7.178-16-16-16z"
                    data-original="#000000"></path>
                </svg>
              </div>
            </div>

            <div class="flex flex-wrap gap-4 justify-between items-center mt-6">
              <div class="flex items-center">
                <input
                  id="remember-me"
                  name="remember-me"
                  type="checkbox"
                  class="w-4 h-4 text-blue-600 rounded border-gray-300 shrink-0 focus:ring-blue-500"
                />
                <label
                  for="remember-me"
                  class="block ml-3 text-sm text-gray-800"
                >
                  记住我
                </label>
              </div>

              <div class="text-sm">
                  <a href="javascript:void(0);" @click="switchForm('reset')" class="text-blue-600 hover:underline font-semibold">
                    忘记密码?
                  </a>
                </div>
            </div>

            <div class="mt-12">
              <button type="button" :disabled="loading" @click="handleLogin"
                class="w-full py-3 px-6 text-sm font-semibold tracking-wider rounded-full text-white bg-gray-800 hover:bg-[#222] focus:outline-none">
                {{ loading ? "登录中..." : "登录" }}
              </button>
            </div>
          </form>

          <!-- 注册 -->
          <form
            v-else
            key="register"
            class="mx-auto w-full max-w-lg"
            @keydown.enter="handleRegister"
          >
            <div class="mb-6">
              <h3 class="text-2xl font-bold text-gray-800">创建一个账号</h3>
            </div>

            <div class="space-y-6">
              <div>
                <label class="block mb-2 text-sm text-gray-800">用户名</label>
                <div class="flex relative items-center">
                  <input
                    name="name"
                    type="text"
                    required
                    v-model="registerForm.username"
                    @keydown.enter.prevent
                    class="px-2 py-3 w-full text-sm border-b border-gray-300 outline-none focus:border-gray-800"
                    placeholder="请输入您的名字"
                  />
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="#bbb"
                    stroke="#bbb"
                    class="absolute right-4 w-4 h-4"
                    viewBox="0 0 24 24"
                  >
                    <circle
                      cx="10"
                      cy="7"
                      r="6"
                      data-original="#000000"
                    ></circle>
                    <path
                      d="M14 15H6a5 5 0 0 0-5 5 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 5 5 0 0 0-5-5zm8-4h-2.59l.3-.29a1 1 0 0 0-1.42-1.42l-2 2a1 1 0 0 0 0 1.42l2 2a1 1 0 0 0 1.42 0 1 1 0 0 0 0-1.42l-.3-.29H22a1 1 0 0 0 0-2z"
                      data-original="#000000"></path>
                  </svg>
                </div>
              </div> -->

              <div>
                <label class="block mb-2 text-sm text-gray-800">邮箱</label>
                <div class="flex relative items-center">
                  <input
                    name="email"
                    type="email"
                    required
                    v-model="registerForm.email"
                    @keydown.enter.prevent
                    class="px-2 py-3 w-full text-sm border-b border-gray-300 outline-none focus:border-gray-800"
                    placeholder="请输入您的邮箱"
                  />
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="#bbb"
                    stroke="#bbb"
                    class="absolute right-4 w-4 h-4"
                    viewBox="0 0 682.667 682.667"
                  >
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
                <label class="block mb-2 text-xs text-gray-800">密码</label>
                <div class="flex relative items-center">
                  <input
                    v-model="registerForm.password"
                    :type="showPassword ? 'text' : 'password'"
                    required
                    @keydown.enter.prevent
                    class="px-2 py-3 w-full text-sm border-b border-gray-300 outline-none focus:border-gray-800"
                    placeholder="请输入密码"
                  />
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="#bbb"
                    stroke="#bbb"
                    class="w-[18px] h-[18px] absolute right-2 cursor-pointer"
                    viewBox="0 0 128 128"
                    @click="showPassword = !showPassword"
                  >
                    <path
                      d="M64 104C22.127 104 1.367 67.496.504 65.943a4 4 0 0 1 0-3.887C1.367 60.504 22.127 24 64 24s62.633 36.504 63.496 38.057a4 4 0 0 1 0 3.887C126.633 67.496 105.873 104 64 104zM8.707 63.994C13.465 71.205 32.146 96 64 96c31.955 0 50.553-24.775 55.293-31.994C114.535 56.795 95.854 32 64 32 32.045 32 13.447 56.775 8.707 63.994zM64 88c-13.234 0-24-10.766-24-24s10.766-24 24-24 24 10.766 24 24-10.766 24-24 24zm0-40c-8.822 0-16 7.178-16 16s7.178 16 16 16 16-7.178 16-16-7.178-16-16-16z"
                      data-original="#000000"></path>
                  </svg>
                </div>
              </div>

              <div>
                <label class="block mb-2 text-sm text-gray-800">验证码</label>
                <div class="flex relative items-center">
                  <input
                    type="text"
                    required
                    v-model="registerForm.code"
                    @keydown.enter.prevent
                    class="px-2 py-3 w-full text-sm border-b border-gray-300 outline-none focus:border-gray-800"
                    placeholder="请输入验证码"
                  />
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="#bbb"
                    stroke="#bbb"
                    class="w-[18px] h-[18px] absolute right-2 cursor-pointer"
                    viewBox="0 0 128 128"
                    @click="showPassword = !showPassword"
                  >
                    <path
                      d="M64 104C22.127 104 1.367 67.496.504 65.943a4 4 0 0 1 0-3.887C1.367 60.504 22.127 24 64 24s62.633 36.504 63.496 38.057a4 4 0 0 1 0 3.887C126.633 67.496 105.873 104 64 104zM8.707 63.994C13.465 71.205 32.146 96 64 96c31.955 0 50.553-24.775 55.293-31.994C114.535 56.795 95.854 32 64 32 32.045 32 13.447 56.775 8.707 63.994zM64 88c-13.234 0-24-10.766-24-24s10.766-24 24-24 24 10.766 24 24-10.766 24-24 24zm0-40c-8.822 0-16 7.178-16 16s7.178 16 16 16 16-7.178 16-16-7.178-16-16-16z"
                      data-original="#000000"></path>
                  </svg>
                  <PrimaryButton @click="timeVal !== 0 ? null : registerSendCode()"
                    :disabled="timeVal !== 0 ? true : false"
                    class="whitespace-nowrap ml-2 bg-slate-500 w-[212px] hover:bg-slate-600"
                    :class="{ 'hover:bg-slate-500': timeVal !== 0 }">
                    {{ timeVal !== 0 ? timeVal + '秒后再次发送' : '发送验证码' }}
                  </PrimaryButton>
                  <!-- <PrimaryButton @click="registerSendCode">11</PrimaryButton> -->
                </div>
              </div>
            </div>

            <div class="!mt-12">
              <button
                type="button"
                @click="handleRegister"
                :disabled="loading"
                class="px-4 py-3 w-full text-sm tracking-wider text-white bg-gray-700 rounded-full hover:bg-gray-800 focus:outline-none"
              >
                {{ loading ? "注册中..." : "注册" }}
              </button>
            </div>
            <p class="mt-6 text-sm text-center text-gray-800">
              已经存在账号?
              <a
                href="javascript:void(0);"
                @click="switchForm('login')"
                class="ml-1 font-semibold text-blue-600 hover:underline"
                >这里登录</a
              >
            </p>
          </form>
        </Transition>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { PrimaryButton } from "@/components/button/index";
import { login, register, register_send } from "@/commands/user";
import { toast } from "vue-sonner";
import { useUserStore } from "@/stores/user";
import { useRouter } from "vue-router";

const isLoginOrRegisterOrReset = ref("login");
const showPassword = ref(false);
const resetShowPassword1 = ref(false)
const resetShowPassword2 = ref(false)
const loading = ref(false);
const router = useRouter();
const userStore = useUserStore();
const sendBtnFlag = ref(false);
const timeVal = ref(0); //倒计时时间

const loginForm = ref({
  username: "",
  email: "",
  password: "",
  rememberMe: false,
});

const registerForm = ref({
  username: "",
  password: "",
  email: "",
  code: "",
});

const resetForm = ref({
  email: "",
  code: "",
  password1: "",
  password2: "",
})

const cleanForm = () => {
  loginForm.value.username = ""
  loginForm.value.email = ""
  loginForm.value.password = ""
  loginForm.value.rememberMe = false

  registerForm.value.username = ""
  registerForm.value.password = ""
  registerForm.value.email = ""
  registerForm.value.code = ""

  resetForm.value.email = ""
  resetForm.value.code = ""
  resetForm.value.password1 = ""
  resetForm.value.password2 = ""
}

const switchForm = (form) => {
  isLoginOrRegisterOrReset.value = form;
  showPassword.value = false;
  resetShowPassword1.value = false;
  resetShowPassword2.value = false;
  cleanForm()
};

const handleLogin = () => {
  loading.value = true;
  login(loginForm.value.email, loginForm.value.password)
    .then((res) => {
      loading.value = false;
      if (res.code == 1 && token) {
        if (token) {
          toast.success(res.message);
          router.push("/environment/0");
          userStore.login({
            account: loginForm.value.username,
          });
        }
        wsStore.connect(token).then(() => {});
      } else {
        toast.warning(res.message);
      }
    })
    .catch(() => {
      router.push("/environment/0");
      loading.value = false;
      toast.warning("登录失败");
    });
    cleanForm();
};

const handleRegister = () => {
  loading.value = true;
  register(
    registerForm.value.email,
    registerForm.value.code,
    // registerForm.value.username,
    "",
    registerForm.value.password
  )
    .then((res) => {
      loading.value = false;
      if(res.code){
        toast.success(res.message + '，请登录')
        isLoginOrRegisterOrReset.value = 'login'
      }else{
        toast.warning(res.message)
      }
    })
    .catch(() => {
      loading.value = false;
      toast.warning("注册失败，请重新注册");
    });
    cleanForm();
};

const handleReset = () => {
  if(resetForm.value.password1 != resetForm.value.password2){
    toast.warning("新密码与确认密码不一致")
    return;
  }

  loading.value = true
  setTimeout(() => {
    loading.value = false
  }, 1000)

  cleanForm();
}

//倒计时函数
const countDown = (time) => {

  timeVal.value = time;  // 更新时间
  if (time === 0) {
    console.log('倒计时结束');
    return;
  }

  setTimeout(() => {
    countDown(time - 1);  // 递归调用自己，倒计时减少 1   
  }, 1000);
};


const registerSendCode = () => {
  if (registerForm.value.email) {
    register_send(registerForm.value.email).then((res) => {
      res.code ? toast.success(res.message) : toast.warning(res.message);
      if (res.code === 1) {
        //倒计时
        timeVal.value = 60;
        countDown(timeVal.value);
      }
    });
  }
};
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

input[type="password"]::-ms-reveal,
input[type="password"]::-webkit-credentials-auto-fill-button {
  display: none;
}
</style>