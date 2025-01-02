<template>
  <div class="font-[sans-serif] bg-gray-900 md:h-screen">
    <div class="grid md:grid-cols-2 items-center gap-8 h-full">
      <div class="max-md:order-1 p-4">
        <img src="https://readymadeui.com/signin-image.webp"
          class="lg:max-w-[80%] w-full h-full object-contain block mx-auto" alt="login-image" />
      </div>

      <div class="flex items-center md:p-8 p-6 bg-white md:rounded-tl-[55px] md:rounded-bl-[55px] h-full">
        <Transition name="fade" mode="out-in">
          <!-- 登录 -->
          <form v-if="isLoginOrRegister === 'login'" key="login" class="max-w-lg w-full mx-auto">
            <div class="mb-12">
              <h3 class="text-gray-800 text-4xl font-extrabold">登录</h3>
              <p class="text-gray-800 text-sm mt-4">没有账号？ 
                <a href="javascript:void(0);"
                  @click="switchForm('register')"
                  class="text-blue-600 font-semibold hover:underline ml-1 whitespace-nowrap">注册</a>
              </p>
            </div>

            <div>
              <label class="text-gray-800 text-xs block mb-2">账号</label>
              <div class="relative flex items-center">
                <input type="text" v-model="loginForm.username" required
                  class="w-full text-sm border-b border-gray-300 focus:border-gray-800 px-2 py-3 outline-none"
                  placeholder="请输入账号" />
                <svg xmlns="http://www.w3.org/2000/svg" fill="#bbb" stroke="#bbb"
                  class="w-[18px] h-[18px] absolute right-2" viewBox="0 0 682.667 682.667">
                  <!-- SVG path data -->
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
                  <!-- SVG path data -->
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
            </div>

            <div class="mt-12">
              <button type="button" :disabled="loading" @click="handleLogin"
                class="w-full py-3 px-6 text-sm font-semibold tracking-wider rounded-full text-white bg-gray-800 hover:bg-[#222] focus:outline-none">
                {{ loading ? "登录中..." : "登录" }}
              </button>
            </div>
          </form>

          <!-- 注册 -->
          <form v-else key="register" class="max-w-lg w-full mx-auto">
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
                    <!-- SVG path data -->
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
                    <!-- SVG path data -->
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
                    <!-- SVG path data -->
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
                    <!-- SVG path data -->
                  </svg>
                  <PrimaryButton @click="registerSendCode" class="whitespace-nowrap ml-2 bg-slate-500">
                    发送验证码
                  </PrimaryButton>
                </div>
              </div>
            </div>

            <div class="!mt-12">
              <button type="button" @click="handleRegister" :disabled="loading"
                class="w-full py-3 px-4 tracking-wider text-sm rounded-full text-white bg-gray-700 hover:bg-gray-800 focus:outline-none">
                {{ loading ? "注册中..." : "注册" }}
              </button>
            </div>
            <p class="text-gray-800 text-sm mt-6 text-center">已经存在账号? 
              <a href="javascript:void(0);"
                @click="switchForm('login')"
                class="text-blue-600 font-semibold hover:underline ml-1">这里登录</a>
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

const isLoginOrRegister = ref("login");
const showPassword = ref(false);
const loading = ref(false);
const router = useRouter();
const userStore = useUserStore();

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

const switchForm = (form) => {
  isLoginOrRegister.value = form;
  showPassword.value = false;
};

const handleLogin = () => {
  loading.value = true;
  login(loginForm.value.username, loginForm.value.password)
    .then((res) => {
      loading.value = false;
      if (res.code == 1) {
        toast.success(res.message);
        router.push("/environment");
        userStore.login({
          account: loginForm.value.username,
        });
      } else {
        toast.warning(res.message);
      }
    })
    .catch(() => {
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
    .catch(() => {
      loading.value = false;
      toast.warning("注册失败，请重新注册");
    });
};

const registerSendCode = () => {
  if (registerForm.value.email) {
    register_send(registerForm.value.email).then((res) => {
      res.code ? toast.success(res.message) : toast.warning(res.message);
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