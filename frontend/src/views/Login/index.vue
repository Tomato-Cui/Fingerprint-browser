<template>
  <div class="w-full max-w-md mx-auto pt-20 space-y-6">
    <Tabs default-value="login" class="w-full">
      <TabsList class="grid w-full grid-cols-2">
        <TabsTrigger value="login">登录</TabsTrigger>
        <TabsTrigger value="register">注册</TabsTrigger>
      </TabsList>
      <TabsContent value="login">
        <Card>
          <CardHeader>
            <CardTitle>账号登录</CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <div class="space-y-2">
              <Label for="login-username">账号</Label>
              <Input
                id="login-username"
                class="w-full"
                v-model="loginForm.username"
                placeholder="请输入账号"
              />
            </div>
            <div class="space-y-2">
              <Label for="login-password">密码</Label>
              <Input
                id="login-password"
                @keyup.enter="handleLogin"
                class="w-full"
                v-model="loginForm.password"
                type="password"
                placeholder="请输入密码"
              />
            </div>
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-2">
                <Checkbox id="remember-me" v-model="loginForm.rememberMe" />
                <label
                  for="remember-me"
                  class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                  >记住我</label
                >
              </div>
              <Button variant="link" @click="forgotPassword">忘记密码？</Button>
            </div>
            <Button class="w-full" :disabled="loading" @click="handleLogin">
              {{ loading ? "登录中..." : "登录" }}
            </Button>
          </CardContent>
          <CardFooter>
            <div class="w-full space-y-4">
              <div class="relative">
                <div class="absolute inset-0 flex items-center">
                  <span class="w-full border-t" />
                </div>
                <div class="relative flex justify-center text-xs uppercase">
                  <span class="bg-background px-2 text-muted-foreground"
                    >其他登录方式</span
                  >
                </div>
              </div>
              <div class="flex justify-center space-x-4">
                <Button variant="outline" class="bg-gray-400" size="icon">
                  <LucideGithub class="h-4 w-4" />
                </Button>
                <Button variant="outline" class="bg-gray-400" size="icon">
                  <LucideMessageCircle class="h-4 w-4" />
                </Button>
                <Button variant="outline" class="bg-gray-400" size="icon">
                  <LucideSmartphone class="h-4 w-4" />
                </Button>
              </div>
            </div>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="register">
        <Card>
          <CardHeader>
            <CardTitle>新用户注册</CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <div class="space-y-2">
              <Label for="register-username">账号</Label>
              <Input
                id="register-username"
                class="w-full"
                v-model="registerForm.username"
                placeholder="请输入账号"
              />
            </div>
            <div class="space-y-2">
              <Label for="register-password">密码</Label>
              <Input
                id="register-password"
                class="w-full"
                v-model="registerForm.password"
                type="password"
                placeholder="请输入密码"
              />
            </div>
            <div class="space-y-2">
              <Label for="register-email">邮箱</Label>
              <Input
                id="register-email"
                class="w-full"
                v-model="registerForm.email"
                placeholder="请输入邮箱"
              />
            </div>
            <div class="flex items-center space-x-2">
              <Checkbox id="agree-terms" v-model="registerForm.agreeTerms" />
              <label
                for="agree-terms"
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
              >
                我已阅读并同意
                <a variant="link" class="underline">服务条款</a>
                和
                <a variant="link" class="underline">隐私政策</a>
              </label>
            </div>
            <Button
              class="w-full cursor-pointer"
              :disabled="loading"
              @click="handleRegister"
            >
              {{ loading ? "注册中..." : "注册" }}
            </Button>
          </CardContent>
        </Card>
      </TabsContent>
    </Tabs>
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
import { PrimaryButton as Button } from "@/components/button/index";
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import {
  LucideGithub,
  LucideMessageCircle,
  LucideSmartphone,
} from "lucide-vue-next";
import { login, register } from "@/commands/user";
import { toast } from "vue-sonner";

import { storeToRefs } from "pinia";
import { useUserStore } from "@/stores/user";

const activeTab = ref("login");
const loading = ref(false);
const isCodeSending = ref(false);
const countdown = ref(60);
const router = useRouter();
const userSotre = useUserStore();

const loginForm = ref({
  username: "",
  password: "",
  rememberMe: false,
});

const registerForm = ref({
  username: "",
  password: "",
  email: "",
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
    registerForm.value.username,
    registerForm.value.password
  )
    .then((res) => {
      loading.value = false;
      toast.success("注册成功，请前往登录页面");
    })
    .catch((err) => {
      loading.value = false;
      toast.success("注册失败，请前往重新注册");
    });
};

const forgotPassword = () => {
  console.log("Forgot password");
};
</script>

<style scoped></style>
