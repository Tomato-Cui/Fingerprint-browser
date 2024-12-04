<!-- 我的账户 -->
<template>
  <div class="footer">
    <el-button @click="submit">确定</el-button>
  </div>
  <div class="account-main">
    <div class="account-information">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">
        账户信息
      </div>
      <div class="account-i-top">
        <el-avatar :size="50" :src="circleUrl" />
        <div class="account-i-t-main1">
          <div>
            {{ userInfo.username }}<el-icon style="margin-left: 10px" @click="editName"
              ><Edit style="color: #1e4cfc"
            /></el-icon>
          </div>
          <div style="color: #8f92a0">用户ID： {{ userInfo.user_id }}</div>
          <div style="color: #8f92a0">团队名称：{{ userInfo.team_name }}</div>
        </div>
        <div class="account-i-t-main2">
          <div style="color: #8f92a0">团队ID： {{ userInfo.team_id }}</div>
        </div>
      </div>
      <div class="account-i-info">
        <el-row :gutter="30">
          <el-col :span="8" class="acc-i-col1" style="">邮箱</el-col>
          <el-col :span="16" class="acc-i-col2" v-if="true"
            ><el-button @click="bindEmail" v-if="!userInfo.email"
              >绑定邮箱</el-button
            >
            <div v-else>{{ userInfo.email }}</div>
          </el-col>
          <el-col :span="16" class="acc-i-col2" v-else>
            <el-icon><SuccessFilled style="color: #10a893" /></el-icon>已绑定
          </el-col>
        </el-row>
        <el-row :gutter="30">
          <el-col :span="8" class="acc-i-col1">手机</el-col>
          <el-col :span="16" class="acc-i-col2"
            >+86 {{ maskedPhoneNumber(userInfo.phone) }}
            <el-link type="primary" @click="editPhone">修改</el-link></el-col
          >
        </el-row>
        <el-row :gutter="30">
          <el-col :span="8" class="acc-i-col1">密码</el-col>
          <el-col :span="16" class="acc-i-col2"
            >{{ maskedPassword(userInfo.password) }}
            <el-link type="primary" @click="editPassword">修改</el-link></el-col
          >
        </el-row>
        <el-row :gutter="30">
          <el-col :span="8" class="acc-i-col1">实名认证</el-col>
          <el-col :span="16" v-if="!userInfo.is_real_name" class="acc-i-col2"
            ><el-icon><CircleCloseFilled style="color: #cbccd1" /></el-icon
            >未认证
            <el-button style="margin-left: 20px" @click="editRealName"
              >前往认证</el-button
            ></el-col
          >
          <el-col :span="16" v-else class="acc-i-col2"
            ><el-icon><SuccessFilled style="color: #10a893" /></el-icon>认证通过
            <el-link type="primary" @click="viewRealName">查看</el-link></el-col
          >
        </el-row>
        <el-row :gutter="30">
          <el-col :span="8" class="acc-i-col1">身份验证器</el-col>
          <el-col :span="16" v-if="true" class="acc-i-col2"
            ><el-icon><CircleCloseFilled style="color: #cbccd1" /></el-icon
            >未绑定
            <el-button style="margin-left: 20px" @click="editAuthenticator"
              >前往绑定</el-button
            ></el-col
          >
          <el-col :span="16" v-else class="acc-i-col2"
            ><el-icon><SuccessFilled style="color: #10a893" /></el-icon
            >已绑定</el-col
          >
        </el-row>
      </div>
    </div>
    <div class="account-linkage">
      <div
        style="font-size: 18px; font-weight: 800; margin: 15px 0; display: flex"
      >
        账户关联
        <div
          style="
            font-size: 14px;
            color: #8f92a0;
            margin-left: 20px;
            line-height: 200%;
          "
        >
          绑定后可通过第三方应用快速登录
        </div>
      </div>
      <div class="account-l-main">
        <el-row :gutter="30">
          <el-col :span="8">
            <div class="acc-l-m-item">
              <div>
                <el-icon><Star /></el-icon>Google账号
              </div>
              <el-link type="primary" @click="bindAccount">绑定</el-link>
            </div>
          </el-col>
          <el-col :span="8">
            <div class="acc-l-m-item">
              <div>
                <el-icon><Star /></el-icon>Google账号
              </div>
              <el-link type="primary" @click="bindAccount">绑定</el-link>
            </div>
          </el-col>
          <el-col :span="8">
            <div class="acc-l-m-item">
              <div>
                <el-icon><Star /></el-icon>Google账号
              </div>
              <el-link type="primary" @click="bindAccount">绑定</el-link>
            </div>
          </el-col>
        </el-row>
      </div>
    </div>
    <div class="login-activity">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">
        登录活动
      </div>
      <div class="login-a-main">
        <div class="login-a-m-item">
          <img
            src="../../../assets/容器 249@4x.png"
            alt=""
            style="
              width: 30px;
              height: 30px;
              background-color: #e8edff;
              padding: 5px;
              border-radius: 5px;
              margin-right: 10px;
            "
          />
          <div class="login-a-m-i-main">
            <div>DESKTOP-P70KINV</div>
            <div style="color: #8f92a0">
              <span>115.199.130.168</span>
              <span style="margin: 0 10px">|</span>
              <span>CN</span>
            </div>
          </div>
          <div style="flex: 1; display: flex; justify-content: flex-end">
            <el-tag type="success">当前</el-tag>
          </div>
        </div>
      </div>
    </div>
    <div class="other">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">其他</div>
      <div class="other-main">
        <el-form :model="userForm">
          <el-form-item label="语言" prop="language">
            <el-select v-model="userForm.language" placeholder="请选择语言">
              <el-option label="中文" :value="1"></el-option>
              <el-option label="英文" :value="2"></el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="时区" prop="timeZone">
            <el-select v-model="userForm.timeZone" placeholder="请选择时区">
              <el-option label="GMT+08:00 Asia/Shanghai" :value="1"></el-option>
              <el-option label="GMT+10:00 Etc/GMT-10" :value="2"></el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="通知" prop="notification">
            <el-switch
              v-model="userForm.notification"
              :active-value="1"
              :inactive-value="0"
            ></el-switch>
            <div style="margin-left: 10px">订阅营销通知（邮件/短信）</div>
          </el-form-item>
        </el-form>
      </div>
    </div>
  </div>
  <!-- 修改用户名 -->
  <el-dialog
    v-model="editNameDialog"
    title="修改姓名"
    width="550px"
    style="padding: 20px; border-radius: 20px; height: 200px"
    class="edit-name-dialog"
    :close-on-click-modal="false"
  >
    <el-form :model="userForm">
      <el-form-item label="姓名" prop="username">
        <el-input v-model="userForm.username" placeholder="请输入用户名" />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button
          @click="
            (editNameDialog = false), (userForm.username = initUser.username)
          "
          >取消</el-button
        >
        <el-button type="primary" @click="submitEditName"> 确定 </el-button>
      </div>
    </template>
  </el-dialog>
  <!-- 绑定邮箱  -->
  <el-dialog
    v-model="bindEmailDialog"
    title="绑定邮箱"
    width="550px"
    :close-on-click-modal="false"
    class="bind-email-dialog"
    @close="handleCloseEmail"
    style="padding: 20px; border-radius: 20px; height: auto"
  >
    <el-form :model="emailForm" ref="emailFormRef" :rules="emailRules">
      <el-form-item label="邮箱" prop="email">
        <el-input v-model="emailForm.email" placeholder="请输入邮箱" />
      </el-form-item>
      <el-form-item label="验证码" prop="code">
        <div style="display: flex; align-items: center; width: 100%">
          <el-input
            v-model="emailForm.code"
            placeholder="请输入验证码"
            style="flex: 1"
          />
          <el-button
            type="primary"
            plain
            :disabled="isCounting"
            @click="sendCode"
            :text="buttonText"
            style="width: 100px; height: 40px; margin-left: 10px"
            >{{ buttonText }}</el-button
          >
        </div>
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleCloseEmail">取消</el-button>
        <el-button type="primary" @click="submitEmail(emailFormRef)">
          确定
        </el-button>
      </div>
    </template>
  </el-dialog>
  <!-- 修改手机号 -->
  <el-dialog
    v-model="editPhoneDialog"
    title="修改手机"
    width="550px"
    :close-on-click-modal="false"
    class="edit-phone-dialog"
    @close="handleClosePhone"
    style="padding: 20px; border-radius: 20px; height: auto"
  >
    <el-form :model="phoneForm" ref="phoneFormRef" :rules="phoneRules">
      <el-form-item label="手机" prop="phone">
        <el-input v-model="phoneForm.phone" placeholder="请输入手机号" />
      </el-form-item>
      <el-form-item label="验证码" prop="code">
        <div style="display: flex; align-items: center; width: 100%">
          <el-input
            v-model="phoneForm.code"
            placeholder="请输入验证码"
            style="flex: 1"
          />
          <el-button
            type="primary"
            plain
            :disabled="isCounting"
            @click="sendCode"
            :text="buttonText"
            style="width: 100px; height: 40px; margin-left: 10px"
            >{{ buttonText }}</el-button
          >
        </div>
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClosePhone">取消</el-button>
        <el-button type="primary" @click="submitPhone(phoneFormRef)">
          确定
        </el-button>
      </div>
    </template>
  </el-dialog>
  <!-- 修改密码 -->
  <el-dialog
    v-model="editPasswordDialog"
    title="修改密码"
    width="550px"
    :close-on-click-modal="false"
    class="edit-password-dialog"
    @close="handleClosePassword"
    style="padding: 20px; border-radius: 20px; height: auto"
  >
    <el-form :model="passwordForm" ref="passwordFormRef" :rules="passwordRules">
      <el-form-item label="旧密码" prop="oldPassword">
        <el-input
          v-model="passwordForm.oldPassword"
          placeholder="请输入旧密码"
        />
      </el-form-item>
      <el-form-item label="新密码" prop="newPassword">
        <el-input
          v-model="passwordForm.newPassword"
          placeholder="请输入新密码"
        />
      </el-form-item>
      <el-form-item label="确认密码" prop="confirmPassword">
        <el-input
          v-model="passwordForm.confirmPassword"
          placeholder="请输入确认密码"
        />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClosePassword">取消</el-button>
        <el-button type="primary" @click="submitPassword(passwordFormRef)">
          确定
        </el-button>
      </div>
    </template>
  </el-dialog>
  <!-- 实名认证 -->
  <el-dialog
    v-model="editRealNameDialog"
    title="实名认证"
    width="550px"
    :close-on-click-modal="false"
    class="edit-real-name-dialog"
    @close="handleCloseRealName"
    style="padding: 20px; border-radius: 20px; height: auto"
  >
    <el-form :model="realNameForm" ref="realNameFormRef" :rules="realNameRules">
      <el-form-item label="姓名" prop="name">
        <el-input v-model="realNameForm.name" placeholder="请输入姓名" />
      </el-form-item>
      <el-form-item label="身份证" prop="idCard">
        <el-input v-model="realNameForm.idCard" placeholder="请输入身份证" />
      </el-form-item>
      <el-form-item label="手机号" prop="phone">
        <el-input v-model="realNameForm.phone" placeholder="请输入手机号" />
      </el-form-item>
      <el-form-item label="验证码" prop="code">
        <div style="display: flex; align-items: center; width: 100%">
          <el-input
            v-model="realNameForm.code"
            placeholder="请输入验证码"
            style="flex: 1"
          />
          <el-button
            type="primary"
            plain
            :disabled="isCounting"
            @click="sendCode"
            :text="buttonText"
            style="width: 100px; height: 40px; margin-left: 10px"
            >{{ buttonText }}</el-button
          >
        </div>
      </el-form-item>
      <el-form-item label="" prop="agree">
        <el-checkbox v-model="agree" style="height: auto">
          <div style="font-size: 12px; line-height: 1.5">
            本人确认所提供的信息是完整、准确和有效的，且同意将上述信息发送给AdsPower，以遵守<br />适用的法规和法律。本人应对因未提供完整、准确、有效的信息而造成的任何及所有损失和后<br />果负责。
          </div>
        </el-checkbox>
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleCloseRealName">取消</el-button>
        <el-button type="primary" @click="submitRealName(realNameFormRef)">
          确定
        </el-button>
      </div>
    </template>
  </el-dialog>
  <!-- 身份验证器 -->
  <el-dialog
    v-model="editAuthenticatorDialog"
    title="身份验证器"
    width="750px"
    :close-on-click-modal="false"
    class="edit-authenticator-dialog"
    @close="handleCloseAuthenticator"
    style="padding: 20px; border-radius: 20px; height: auto"
  >
    <el-steps :active="activeStep" finish-status="success" align-center>
      <el-step title="Step 1">
        <template #title> 扫描二维码 </template>
      </el-step>
      <el-step title="输入动态验证码"></el-step>
    </el-steps>
    <div class="step-description-1" v-show="activeStep === 1">
      <div style="font-size: 20px">使用身份验证器扫描二维码</div>
      <QrCode :value="qrText" />
      <el-link type="primary" v-show="isScan" @click="isScan = false"
        >无法扫描二维码？</el-link
      >
      <div v-show="!isScan">请复制这串密钥并在APP端输入进行绑定</div>
      <div v-show="!isScan" style="color: #1e4cfc">
        5SZYRXTNSMDQ3Y3ER6CAWKUGUHPFPMWY
      </div>
      <div class="step-d-1-foot">
        <div>推荐的身份验证器</div>
        <div>
          <ul>
            <li>Google Authenticator</li>
            <li>Microsoft Authenticator</li>
            <li>LastPass Authenticator</li>
          </ul>
        </div>
      </div>
    </div>
    <div class="step-description-2" v-show="activeStep === 2">
      <el-form
        :model="authenticatorForm"
        ref="authenticatorFormRef"
        :rules="authenticatorRules"
      >
        <el-form-item label="手机" prop="phone">
          +86 {{ maskedPhone }}
        </el-form-item>
        <el-form-item label="验证码" prop="code">
          <div style="display: flex; align-items: center; width: 100%">
            <el-input
              v-model="authenticatorForm.code"
              placeholder="请输入验证码"
              style="flex: 1"
            />
            <el-button
              type="primary"
              plain
              :disabled="isCounting"
              @click="sendCode"
              :text="buttonText"
              style="width: 100px; height: 40px; margin-left: 10px"
              >{{ buttonText }}</el-button
            >
          </div>
          <div>手机不可以？请联系管理员处理</div>
        </el-form-item>
        <el-form-item label="动态验证码" prop="dynamicCode">
          <el-input
            v-model="authenticatorForm.dynamicCode"
            placeholder="请输入动态验证码"
          />
          <div>请输入身份验证器上的动态验证码</div>
        </el-form-item>
      </el-form>
    </div>
    <el-button
      @click="activeStep++"
      v-show="activeStep === 1"
      style="
        width: 100px;
        height: 40px;
        background-color: #1e4cfc;
        color: #fff;
        font-size: 18px;
        border-radius: 10px;
      "
      >下一步</el-button
    >
    <el-button
      v-show="activeStep === 2"
      @click="submitAuthenticator(authenticatorFormRef)"
      style="
        width: 100px;
        height: 40px;
        background-color: #1e4cfc;
        color: #fff;
        font-size: 18px;
        border-radius: 10px;
      "
      >绑定</el-button
    >
    <el-button
      type="primary"
      text
      v-show="activeStep === 2"
      @click="activeStep--"
      style="width: 100px; height: 40px; font-size: 18px"
      >上一步</el-button
    >
  </el-dialog>
  <!-- 查看实名认证 -->
  <el-dialog
    v-model="viewRealNameDialog"
    title="实名认证"
    width="550px"
    :close-on-click-modal="false"
    class="view-real-name-dialog"
    @close="viewRealNameDialog = false"
    style="padding: 20px; border-radius: 20px; height: auto"
  >
    <div style="height: 300px;display: flex;flex-direction: column;align-items: center;justify-content: space-around;">
      <img src="../../../assets/real_naem.png" alt="" style="width: 204px;height: 142px;" />
      <div style="font-size: 20px;font-weight: 600">您已通过实名认证</div>
      <div>姓名：{{ userInfo.username }}</div>
      <div>手机号：{{ userInfo.phone }}</div>
      <div>身份证：{{ userInfo.idCard }}</div>
    </div>
  </el-dialog>
</template>

<script setup lang="ts">
import { reactive, ref, watch, onMounted, computed } from "vue";
import {
  CircleCloseFilled,
  SuccessFilled,
  Star,
  Edit,
} from "@element-plus/icons-vue";
import { type FormInstance, type FormRules, ElMessage } from "element-plus";
import circleUrl from "@/assets/avatar/bd7ba5efc6cb20ce6db32d1cedb8da6d5b4b5f10.jpg";
import QrCode from "@/components/Myqrcode/index.vue";

const qrText = ref("https://example.com");
const userInfo = reactive({
  //用户信息
  username: "19858152311", // 用户名
  email: null, // 邮箱
  is_real_name: true, //是否实名认证
  user_id: "user_h197twq", // 用户ID
  team_name: "19858152311", // 团队名称
  team_id: "team_h11kaom", // 团队ID
  phone: "1958152311", // 手机号
  password: "12345678", // 密码
  idCard: "410105194906231234", // 身份证
  language: 1, // 语言
  timeZone: 1, // 时区
  notification: 0, // 通知
});

interface UserForm {
  // 修改用户名表单类型
  id?: number;
  user_id: string;
  team_name: string;
  team_id: string;
  username: string;
  language: number;
  timeZone: number;
  notification: number;
  phone: string;
  password: string;
}
const userForm = reactive<UserForm>({
  user_id: "", // 用户ID
  team_name: "", // 团队名称
  team_id: "", // 团队ID
  username: "", // 用户名
  language: 1, // 语言
  timeZone: 1, // 时区
  notification: 0, // 通知
  phone: "", // 手机号
  password: "", // 密码
});
const agree = ref(false); // 是否同意
const editNameDialog = ref(false); // 修改用户名弹窗
const bindEmailDialog = ref(false); // 绑定邮箱弹窗
const editPhoneDialog = ref(false); // 修改手机弹窗
const editPasswordDialog = ref(false); // 修改密码弹窗
const editRealNameDialog = ref(false); // 实名认证弹窗
const editAuthenticatorDialog = ref(false); // 身份验证器弹窗
const viewRealNameDialog = ref(false); // 查看实名认证弹窗
const emailFormRef = ref<FormInstance>(); // 绑定邮箱表单
const phoneFormRef = ref<FormInstance>(); // 修改手机表单
const passwordFormRef = ref<FormInstance>(); // 修改密码表单
const realNameFormRef = ref<FormInstance>(); // 实名认证表单
const authenticatorFormRef = ref<FormInstance>(); // 身份验证器表单
const initUser = ref<any>({}); // 初始用户信息
const isCounting = ref(false); // 是否正在发送验证码
const timeLeft = ref(60); // 倒计时
const buttonText = ref("发送验证码"); // 按钮文字
const isScan = ref(true); // 是否扫描二维码
const maskedPhone = ref(""); // 脱敏后的手机号
const activeStep = ref(1);

watch(bindEmailDialog, (newVal) => {
  // 监听绑定邮箱弹窗-
  resetTimer();
  clearEmailForm();
});
watch(editPhoneDialog, (newVal) => {
  // 监听修改手机弹窗
  if (newVal === false) {
    resetTimer();
  }
  clearPhoneForm();
});
watch(editRealNameDialog, (newVal) => {
  // 监听实名认证弹窗
  resetTimer();
  clearRealNameForm();
});
watch(editAuthenticatorDialog, (newVal) => {
  // 监听身份验证器弹窗
  resetTimer();
  clearAuthenticatorForm();
});
interface EmailForm {
  // 绑定邮箱表单类型
  email: string;
  code: string;
}
const emailForm = reactive<EmailForm>({
  //绑定邮箱表单
  email: "", // 邮箱
  code: "", // 验证码
});
interface PhoneForm {
  // 修改手机表单类型
  phone: string;
  code: string;
}
const phoneForm = reactive<PhoneForm>({
  //修改手机表单
  phone: "", // 手机号
  code: "", // 验证码
});
interface PasswordForm {
  // 修改密码表单类型
  oldPassword: string;
  newPassword: string;
  confirmPassword: string;
}
const passwordForm = reactive<PasswordForm>({
  oldPassword: "", // 旧密码
  newPassword: "", // 新密码
  confirmPassword: "", // 确认密码
});
interface RealNameForm {
  // 实名认证表单类型
  name: string;
  idCard: string;
  phone: string;
  code: string;
}
const realNameForm = reactive<RealNameForm>({
  name: "", // 姓名
  idCard: "", // 身份证
  phone: "", // 手机号
  code: "", // 验证码
});
interface AuthenticatorForm {
  phone: string;
  code: string;
  dynamicCode: string;
}
const authenticatorForm = reactive<AuthenticatorForm>({
  phone: "", // 手机号
  code: "", // 验证码
  dynamicCode: "", // 动态验证码
});
//清空邮箱表单
const clearEmailForm = () => {
  emailForm.email = "";
  emailForm.code = "";
};
//清空手机表单
const clearPhoneForm = () => {
  phoneForm.phone = "";
  phoneForm.code = "";
};
const clearRealNameForm = () => {
  realNameForm.name = "";
  realNameForm.idCard = "";
  realNameForm.phone = "";
  realNameForm.code = "";
  agree.value = false;
};
//清空密码表单
const clearPasswordForm = () => {
  passwordForm.oldPassword = "";
  passwordForm.newPassword = "";
  passwordForm.confirmPassword = "";
};
const clearAuthenticatorForm = () => {
  authenticatorForm.phone = "";
  authenticatorForm.code = "";
  authenticatorForm.dynamicCode = "";
};
const emailRules = reactive<FormRules<EmailForm>>({
  //绑定邮箱表单验证
  email: [{ required: true, message: "请输入邮箱", trigger: "blur" }],
  code: [{ required: true, message: "请输入验证码", trigger: "blur" }],
});
const phoneRules = reactive<FormRules<PhoneForm>>({
  //修改手机表单验证
  phone: [{ required: true, message: "请输入手机号", trigger: "blur" }],
  code: [{ required: true, message: "请输入验证码", trigger: "blur" }],
});
const passwordRules = reactive<FormRules<PasswordForm>>({
  //修改密码表单验证
  oldPassword: [{ required: true, message: "请输入旧密码", trigger: "blur" }],
  newPassword: [{ required: true, message: "请输入新密码", trigger: "blur" }],
  confirmPassword: [
    { required: true, message: "请输入确认密码", trigger: "blur" },
  ],
});
const realNameRules = reactive<FormRules<RealNameForm>>({
  //实名认证表单验证
  name: [{ required: true, message: "请输入姓名", trigger: "blur" }],
  idCard: [{ required: true, message: "请输入身份证", trigger: "blur" }],
  phone: [{ required: true, message: "请输入手机号", trigger: "blur" }],
  code: [{ required: true, message: "请输入验证码", trigger: "blur" }],
});
const authenticatorRules = reactive<FormRules<AuthenticatorForm>>({
  //身份验证器表单验证
  code: [{ required: true, message: "请输入验证码", trigger: "blur" }],
  dynamicCode: [
    { required: true, message: "请输入动态验证码", trigger: "blur" },
  ],
});
watch(userForm, (newVal) => {
  console.log("newVal", newVal);
});
//记录初始用户名
const initUsername = ref("");
const editName = () => {
  // 修改用户名
  initUsername.value = userForm.username;
  editNameDialog.value = true;
};
const bindEmail = () => {
  // 绑定邮箱
  console.log("bindEmail");
  bindEmailDialog.value = true;
};
const editPhone = () => {
  // 修改手机
  // console.log("editPhone");
  editPhoneDialog.value = true;
};
const editPassword = () => {
  // 修改密码
  editPasswordDialog.value = true;
};
const editRealName = () => {
  // 实名认证
  editRealNameDialog.value = true;
};
const editAuthenticator = () => {
  // 身份验证器
  editAuthenticatorDialog.value = true;
};
const bindAccount = () => {
  // 绑定Google
  ElMessage.warning("跳转浏览器绑定对应账号");
};
const viewRealName = () => {
  // 查看实名认证
  ElMessage.warning("查看实名认证");
  viewRealNameDialog.value = true;
};
const submitEditName = () => {
  // 提交修改用户名
  //TODO发送修改名称请求
  ElMessage.success("修改成功");
  editNameDialog.value = false; // 关闭弹窗
};
const submitEmail = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log("submit!");
      bindEmailDialog.value = false; // 关闭弹窗
      //TODO发送绑定邮箱请求
      ElMessage.success("绑定成功");
    } else {
      console.log("error submit!", fields);
    }
  });
};
const submitPassword = async (formEl: FormInstance | undefined) => {
  // 提交修改密码
  if (!formEl) return;
  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log("submit!");

      // 判断旧密码是否正确
      if (userForm.password === passwordForm.oldPassword) {
        if (passwordForm.newPassword === passwordForm.confirmPassword) {
          ElMessage.success("修改成功");
          clearPasswordForm();
          editPasswordDialog.value = false; // 关闭弹窗
          //TODO发送修改密码请求
        } else {
          ElMessage.error("两次密码不一致");
        }
      } else {
        ElMessage.error("密码错误");
      }
    } else {
      console.log("error submit!", fields);
    }
  });
};
const submitPhone = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log("submit!");
      editPhoneDialog.value = false; // 关闭弹窗
      //TODO发送修改手机请求
      ElMessage.success("修改成功");
    } else {
      console.log("error submit!", fields);
    }
  });
};
const submitRealName = async (formEl: FormInstance | undefined) => {
  // 提交实名认证
  if (!formEl) return;
  await formEl.validate((valid, fields) => {
    if (valid) {
      //TODO发送实名认证请求
      if (agree.value) {
        ElMessage.success("认证成功");
        console.log("submit!");
        editRealNameDialog.value = false; // 关闭弹窗
      } else {
        ElMessage.error("请同意协议");
      }
    } else {
      console.log("error submit!", fields);
    }
  });
};
const submitAuthenticator = async (formEl: FormInstance | undefined) => {
  // 提交身份验证器
  if (!formEl) return;
  await formEl.validate((valid, fields) => {
    if (valid) {
      //TODO发送
      ElMessage.success("绑定成功");
      console.log("submit!");
      editAuthenticatorDialog.value = false; // 关闭弹窗
    } else {
      console.log("error submit!", fields);
    }
  });
};
const submit = async () => {
  //提交其他
  ElMessage.warning("提交语言、时区、通知的修改");
};
const handleCloseEmail = () => {
  // 关闭邮箱弹窗(取消)
  bindEmailDialog.value = false;
};
const handleClosePhone = () => {
  // 关闭手机弹窗(取消)
  editPhoneDialog.value = false;
};
const handleClosePassword = () => {
  // 关闭密码弹窗(取消)
  clearPasswordForm();
  editPasswordDialog.value = false;
};
const handleCloseRealName = () => {
  // 关闭实名认证弹窗(取消)
  editRealNameDialog.value = false;
};
const handleCloseAuthenticator = () => {
  // 关闭身份验证器弹窗(取消)
  editAuthenticatorDialog.value = false;
};
const intervalId = ref<number>();
const sendCode = () => {
  // 发送验证码
  console.log("sendCode");
  if (timeLeft.value <= 0 || isCounting.value) return; // 如果已经计时或时间为0，则不执行

  isCounting.value = true;
  ElMessage.success("验证码发送成功");
  intervalId.value = setInterval(() => {
    timeLeft.value--;
    buttonText.value = ` ${timeLeft.value} 秒后再次发送`;

    if (timeLeft.value <= 0) {
      clearInterval(intervalId.value);
      resetTimer();
    }
  }, 1000);
};
const resetTimer = () => {
  timeLeft.value = 60; // 重置倒计时
  isCounting.value = false;
  buttonText.value = "发送验证码";
  clearInterval(intervalId.value); // 清除定时器
};
// 计算属性，用于生成脱敏后的手机号
const maskedPhoneNumber = (phone: string) => {
  if (phone) {
    return phone.replace(/(\d{2})\d+(\d{2})/, "$1******$2");
  }
  return "";
};
//密码脱敏
const maskedPassword = (password: string) => {
  if (password) {
    return password.replace(/./g, "*");
  }
  return "";
};

// 获取用户信息
onMounted(() => {
  // 获取用户信息
  // console.log("获取用户信息");
  userForm.username = userInfo.username;
  userForm.user_id = userInfo.user_id;
  userForm.team_name = userInfo.team_name;
  userForm.team_id = userInfo.team_id;
  userForm.phone = userInfo.phone;
  phoneForm.phone = userInfo.phone;
  userForm.password = userInfo.password;

  initUser.value = userInfo;

  maskedPhone.value = maskedPhoneNumber(userForm.phone);
});
</script>

<style scoped lang="less">
.footer {
  position: fixed;
  bottom: 0;
  width: 100%;
  height: 80px;
  display: flex;
  align-items: center;
  // left: 300px;
  background-color: #ffffff;
  padding-left: 50px;
  :deep(.el-button) {
    width: 100px;
    height: 40px;
    background-color: #1e4cfc;
    color: #fff;
    font-size: 18px;
    border-radius: 10px;
  }
}
.account-main {
  height: auto;
  max-width: 1000px;
  min-width: 700px;
  padding: 20px;
  //   overflow: hidden;

  .account-information {
    width: 100%;
    height: auto;
    padding: 5px 10px;

    .account-i-top {
      width: 100%;
      height: 100px;
      padding: 5px 25px;
      display: flex;
      align-items: center;
      background-color: #f8faff;

      .account-i-t-main1 {
        height: 70%;
        margin: 0 30px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
      }
    }

    .account-i-info {
      width: 600px;
      padding: 30px 20px;
      //   background-color: antiquewhite;

      :deep(.el-row) {
        height: 60px;
        display: flex;
        align-items: center;
        justify-content: center;

        .acc-i-col1 {
          display: flex;
          justify-content: flex-end;
        }
        .acc-i-col2 {
          .el-button {
            height: 40px;
            width: 80px;
            border: 1px solid #1e4cfc;
            color: #1e4cfc;
            border-radius: 10px;
          }

          .el-link {
            margin-left: 20px;
          }
        }
      }
    }
  }

  .account-linkage {
    height: 120px;
    .account-l-main {
      .acc-l-m-item {
        width: 100%;
        background: #f4f6ff;
        height: 38px;
        line-height: 38px;
        padding: 0 10px;
        display: flex;
        justify-content: space-between;
        border-radius: 5px;
      }
    }
  }

  .login-activity {
    min-height: 150px;
    .login-a-main {
      .login-a-m-item {
        width: 100%;
        height: 50px;
        padding: 10px;
        display: flex;
        align-items: center;
        background-color: #f4f6ff;
        border-radius: 10px;

        .login-a-m-i-main {
          width: 200px;
        }
      }
    }
  }

  .other {
    .other-main {
      width: 500px;
      display: flex;
      justify-content: center;
      padding-left: 100px;
      // align-items: center;

      :deep(.el-form) {
        width: 100%;

        .el-form-item {
          height: 50px;
          display: flex;
          align-items: center;

          .el-select {
            width: 100%;
            height: 40px;

            .el-select__wrapper {
              height: 45px;
              border-radius: 10px;
            }
          }
        }
      }
    }
  }
}
//修改名称
.edit-name-dialog {
  // .el-dialog__header {
  //   .el-dialog__title {
  //     font-size: 28px;
  //     font-weight: 800;
  //   }
  // }

  .el-dialog__body {
    .el-form {
      width: 100%;
      height: 60px;
      display: flex;
      align-items: center;

      .el-form-item {
        width: 100%;

        :deep(.el-input) {
          height: 40px;
          .el-input__wrapper {
            border-radius: 10px;
          }
        }
      }
    }
  }

  :deep(.dialog__footer) {
    .dialog-footer {
      display: flex;
      justify-content: flex-start;
    }
  }
}
//绑定邮箱
.bind-email-dialog {
  .el-dialog__body {
    .el-form {
      width: 100%;
      .el-form-item {
        display: flex;
        align-items: center;
        min-height: 40px;
        :deep(.el-form-item__label) {
          width: 70px;
        }

        .el-input {
          :deep(.el-input__wrapper) {
            height: 40px;
          }
        }
      }
    }
  }
}
.edit-phone-dialog {
  .el-dialog__body {
    .el-form {
      width: 100%;
      .el-form-item {
        display: flex;
        align-items: center;
        min-height: 40px;
        :deep(.el-form-item__label) {
          width: 70px;
        }

        .el-input {
          :deep(.el-input__wrapper) {
            height: 40px;
          }
        }
      }
    }
  }
}
// 修改密码
.edit-password-dialog {
  .el-dialog__body {
    .el-form {
      width: 100%;
      .el-form-item {
        display: flex;
        align-items: center;
        min-height: 40px;
        :deep(.el-form-item__label) {
          width: 80px;
        }

        .el-input {
          :deep(.el-input__wrapper) {
            height: 40px;
          }
        }
      }
    }
  }
}

// 实名认证
.edit-real-name-dialog {
  .el-dialog__body {
    .el-form {
      width: 100%;
      .el-form-item {
        display: flex;
        align-items: center;
        min-height: 40px;
        :deep(.el-form-item__label) {
          width: 80px;
        }

        .el-input {
          :deep(.el-input__wrapper) {
            height: 40px;
          }
        }
      }
    }
  }
}

// 身份验证器
.edit-authenticator-dialog {
  .el-dialog__body {
    display: flex;
    flex-direction: column;

    .el-steps {
      width: 100%;
      height: 100px;
    }
    .step-description-1 {
      height: 350px;
      display: flex;
      flex-direction: column;
      justify-content: space-around;
      align-items: center;

      .step-d-1-foot {
        width: 550px;
        background-color: #f8faff;
        padding: 15px 20px;
        border-radius: 10px;

        ul {
          display: flex;
          padding-left: 15px;
          justify-content: space-between;
        }
      }
    }

    .step-description-2 {
      .el-form {
        width: 100%;
        .el-form-item {
          display: flex;
          // align-items: center;
          min-height: 40px;
          :deep(.el-form-item__label) {
            width: 100px;
          }

          .el-input {
            :deep(.el-input__wrapper) {
              height: 40px;
            }
          }
        }
      }
    }
  }
}

// 查看实名认证
// .view-real-name-dialog {
//   .el-dialog__header {
//     font-size: 30px;
//   }
//   .el-dialog__body {
//     display: flex;
//     flex-direction: column;
//     align-items: center;
//     justify-content: center;
//   }
// }
</style>
