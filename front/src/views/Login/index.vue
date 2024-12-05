<template>
  <div class="login-container">
    <div class="login-box">
      <div class="login-left">
        <div class="welcome-text">
          <h2>欢迎使用</h2>
          <p>专业的浏览器指纹管理系统</p>
        </div>
        <div class="feature-list">
          <div class="feature-item">
            <el-icon>
              <Monitor />
            </el-icon>
            <span>多浏览器管理</span>
          </div>
          <div class="feature-item">
            <el-icon>
              <Lock />
            </el-icon>
            <span>安全防护</span>
          </div>
          <div class="feature-item">
            <el-icon>
              <DataAnalysis />
            </el-icon>
            <span>数据分析</span>
          </div>
        </div>
      </div>

      <div class="login-right">
        <el-tabs v-model="activeTab" class="login-tabs">
          <el-tab-pane label="登录" name="login">
            <el-form :model="loginForm" :rules="loginRules" ref="loginFormRef" class="login-form">
              <h3 class="form-title">账号登录</h3>

              <el-form-item prop="username">
                <el-input v-model="loginForm.username" placeholder="请输入账号" prefix-icon="el-icon-user"
                  class="custom-input" />
              </el-form-item>

              <el-form-item prop="password">
                <el-input v-model="loginForm.password" type="password" placeholder="请输入密码" prefix-icon="el-icon-lock"
                  show-password class="custom-input" />
              </el-form-item>

              <div class="remember-forgot">
                <el-checkbox v-model="loginForm.rememberMe">记住我</el-checkbox>
                <el-button type="text" @click="forgotPassword">忘记密码？</el-button>
              </div>

              <el-button type="primary" class="submit-btn" :loading="loading" @click="handleLogin">
                登录
              </el-button>

              <div class="other-login">
                <div class="divider">
                  <span>其他登录方式</span>
                </div>
                <div class="social-login">
                  <el-button circle>
                    <el-icon>
                      <Platform />
                    </el-icon>
                  </el-button>
                  <el-button circle>
                    <el-icon>
                      <ChatDotRound />
                    </el-icon>
                  </el-button>
                  <el-button circle>
                    <el-icon>
                      <Iphone />
                    </el-icon>
                  </el-button>
                </div>
              </div>
            </el-form>
          </el-tab-pane>

          <el-tab-pane label="注册" name="register">
            <el-form :model="registerForm" :rules="registerRules" ref="registerFormRef" class="login-form">
              <h3 class="form-title">新用户注册</h3>

              <el-form-item prop="username">
                <el-input v-model="registerForm.username" placeholder="请输入账号" prefix-icon="el-icon-user"
                  class="custom-input" />
              </el-form-item>

              <el-form-item prop="password">
                <el-input v-model="registerForm.password" type="password" placeholder="请输入密码" prefix-icon="el-icon-lock"
                  show-password class="custom-input" />
              </el-form-item>

              <el-form-item prop="confirmPassword">
                <el-input v-model="registerForm.confirmPassword" type="password" placeholder="请确认密码"
                  prefix-icon="el-icon-lock" show-password class="custom-input" />
              </el-form-item>

              <el-form-item prop="email">
                <el-input v-model="registerForm.email" placeholder="请输入邮箱" prefix-icon="el-icon-mail"
                  class="custom-input" />
              </el-form-item>

              <el-form-item prop="verificationCode">
                <div class="verification-code-input">
                  <el-input v-model="registerForm.verificationCode" placeholder="请输入验证码" prefix-icon="el-icon-key"
                    class="custom-input" />
                  <el-button type="primary" :disabled="isCodeSending" @click="sendVerificationCode">
                    {{ codeButtonText }}
                  </el-button>
                </div>
              </el-form-item>

              <el-form-item class="agreement">
                <el-checkbox v-model="registerForm.agreeTerms">
                  我已阅读并同意
                  <el-button type="text">服务条款</el-button>
                  和
                  <el-button type="text">隐私政策</el-button>
                </el-checkbox>
              </el-form-item>

              <el-button type="primary" class="submit-btn" :loading="loading" :disabled="!registerForm.agreeTerms"
                @click="handleRegister">
                注册
              </el-button>
            </el-form>
          </el-tab-pane>
        </el-tabs>
      </div>
    </div>
  </div>
</template>
<script setup>
import { ref } from "vue";
import {
  ElMessage,
  ElButton,
  ElInput,
  ElCheckbox,
  ElForm,
  ElFormItem,
  ElTabs,
  ElTabPane,
  ElIcon,
} from "element-plus";
import { useRouter } from "vue-router";
import {
  Monitor,
  Lock,
  DataAnalysis,
  Platform,
  ChatDotRound,
  Iphone,
} from "@element-plus/icons-vue"; // 导入你需要的图标

import { login, register, sendCode } from "@/api/loginService";
import { login as loginCommand } from "@/command/auth";

// 登录操作
const handleLogin = async () => {
  loading.value = true;
  try {
    // 调用登录服务
    const response = await login(loginForm.value);
    let token = response.data.token;
    // 登录成功后，检查返回的数据中是否包含 token
    if (token) {
      await loginCommand(token);

      // 保存 token 到 localStorage
      localStorage.setItem("token", token);
      ElMessage.success("登录成功");

      // 跳转到首页
      router.push("/home");
    } else {
      // 如果没有 token，则提示用户登录失败
      throw new Error("登录失败，未返回 token");
    }
  } catch (error) {
    // 统一的错误处理
    if (error.response) {
      // 处理 HTTP 请求的错误信息（比如 4xx、5xx 错误）
      ElMessage.error(
        error.response?.data?.message || "登录失败，请检查网络或账户信息"
      );
    } else if (error.request) {
      // 请求已发出，但没有收到响应（可能是网络问题）
      ElMessage.error("网络异常，请检查您的网络连接");
    } else {
      // 其他错误（如 token 错误等）
      ElMessage.error(error.message || "登录失败");
    }
  } finally {
    // 无论成功或失败，都需要停止加载状态
    loading.value = false;
  }
};

// 忘记密码
const forgotPassword = () => {
  ElMessage.info("忘记密码");
};

// 登录和注册表单数据
const loginForm = ref({
  username: "",
  password: "",
  rememberMe: false,
});

// 注册表单数据
const registerForm = ref({
  username: "",
  password: "",
  confirmPassword: "",
  email: "",
  verificationCode: "",
  agreeTerms: false,
});

// 表单验证规则
const registerRules = {
  username: [{ required: true, message: "请输入账号", trigger: "blur" }],
  password: [{ required: true, message: "请输入密码", trigger: "blur" }],
  confirmPassword: [
    { required: true, message: "请确认密码", trigger: "blur" },
    {
      validator(rule, value, callback) {
        if (value !== registerForm.value.password) {
          callback(new Error("密码与确认密码不一致"));
        } else {
          callback();
        }
      },
      trigger: "blur",
    },
  ],
  email: [{ required: true, message: "请输入邮箱", trigger: "blur" }],
};

const activeTab = ref("login");
const loading = ref(false);
const isCodeSending = ref(false);
const codeButtonText = ref("获取验证码");
const router = useRouter();

// 注册操作
const handleRegister = async () => {
  loading.value = true;

  // 创建符合新 JSON 格式的请求体
  const registerData = {
    account: {
      id: null,
      type: 1, // 假设类型为 1，表示普通用户
      email: registerForm.value.email,
      username: registerForm.value.username,
      password: registerForm.value.password,
    },
    captcha: registerForm.value.verificationCode,
  };

  try {
    // 调用注册API
    console.log(registerData);
    await register(registerData);
    ElMessage.success("注册成功");

    router.push("/login");
  } catch (error) {
    ElMessage.error(error.message);
  } finally {
    loading.value = false;
  }
};

// 发送验证码
const sendVerificationCode = async () => {
  if (!registerForm.value.email) {
    ElMessage.warning("请输入邮箱地址！");
    return;
  }
  isCodeSending.value = true;
  codeButtonText.value = "发送中...";

  try {
    // 调用发送验证码的API
    await sendCode(registerForm.value.email);
    ElMessage.success("验证码已发送，请查收");
  } catch (error) {
    ElMessage.error("验证码发送失败");
  } finally {
    isCodeSending.value = false;
    codeButtonText.value = "重新获取";
  }
};
</script>

<style scoped>
/* 这里可以添加你自定义的样式 */
</style>

<style scoped>
.login-container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

/* 去掉浏览器默认的间距和填充 */

.login-box {
  width: 900px;
  height: 600px;
  background: white;
  border-radius: 20px;
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
  display: flex;
  overflow: hidden;
}

.login-left {
  width: 40%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 40px;
  color: white;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.welcome-text {
  margin-bottom: 40px;
}

.welcome-text h2 {
  font-size: 32px;
  margin-bottom: 10px;
}

.welcome-text p {
  font-size: 16px;
  opacity: 0.8;
}

.feature-list {
  margin-top: 40px;
}

.feature-item {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

.feature-item .el-icon {
  margin-right: 10px;
  font-size: 24px;
}

.login-right {
  width: 60%;
  /* padding: 40px; */
}

.login-tabs {
  height: 100%;
}

.form-title {
  text-align: center;
  margin-bottom: 30px;
  color: #333;
  font-size: 24px;
}

.login-form {
  max-width: 380px;
  margin: 0 auto;
  /* height: auto; */
}

.custom-input :deep(.el-input__wrapper) {
  box-shadow: 0 0 0 1px #dcdfe6 inset;
  border-radius: 8px;
  padding: 12px;
}

.custom-input :deep(.el-input__wrapper:hover) {
  box-shadow: 0 0 0 1px #409eff inset;
}

.remember-forgot {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.submit-btn {
  width: 100%;
  padding: 12px;
  font-size: 16px;
  border-radius: 8px;
}

.verification-code-input {
  display: flex;
  gap: 10px;
}

.verification-code-input .el-button {
  width: 120px;
}

.other-login {
  margin-top: 30px;
}

.divider {
  display: flex;
  align-items: center;
  margin: 20px 0;
}

.divider::before,
.divider::after {
  content: "";
  flex: 1;
  height: 1px;
  background: #dcdfe6;
}

.divider span {
  padding: 0 10px;
  color: #909399;
  font-size: 14px;
}

.social-login {
  display: flex;
  justify-content: center;
  gap: 20px;
}

.agreement {
  margin: 20px 0;
}

:deep(.el-tabs__nav) {
  width: 100%;
  display: flex;
}

:deep(.el-tabs__item) {
  flex: 1;
  text-align: center;
}
</style>
