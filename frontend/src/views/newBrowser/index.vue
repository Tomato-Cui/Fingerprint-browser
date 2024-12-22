<template>
  <Layout>
    <!-- 导航栏 -->
    <div class="browser-header">
      <el-affix :offset="85">
        <el-menu mode="horizontal" class="horizontal-menu" active-text-color="#ffd04b">
          <el-menu-item @click="scrollToSection('base-info')">基础信息</el-menu-item>
          <el-menu-item @click="scrollToSection('proxy-info')">代理信息</el-menu-item>
          <el-menu-item @click="scrollToSection('account-info')">账号平台</el-menu-item>
          <el-menu-item @click="scrollToSection('fingerprint-info')">指纹配置</el-menu-item>
          <el-menu-item @click="scrollToSection('advanced-settings')">高级设置</el-menu-item>
        </el-menu>
      </el-affix>
    </div>
    <!-- 表单内容 -->
    <div class="browser-form">
      <el-form :model="form" class="browser-form-left">
        <!-- =========================================基础信息内容==================================================== -->
        <el-card id="base-info" class="box-card" shadow="never">
          <el-row :gutter="20">
            <el-col :span="12">
              <div>浏览器名称</div>
              <el-input v-model="form.browserName" placeholder="请输入浏览器名称"></el-input>
            </el-col>

            <el-col :span="6">
              <div>分组</div>
              <el-select v-model="form.groupId" placeholder="请选择分组" :disabled="loading" :style="{ width: '100%' }"
                :dropdown-style="{ maxHeight: '300px', overflowY: 'auto' }">
                <el-option v-for="groupId in groupIds" :key="groupId" :label="groupId" :value="groupId"></el-option>
              </el-select>
              <el-spin v-if="loading" />
            </el-col>

            <!-- <el-col :span="3">
              <br />
              <el-select v-model="form.priority" placeholder="便签">
                <el-option label="高" value="high"></el-option>
                <el-option label="中" value="medium"></el-option>
                <el-option label="低" value="low"></el-option>
              </el-select>
            </el-col> -->
          </el-row>
          <el-row :gutter="20">
            <el-col :span="6">
              <div>浏览器</div>
              <el-select v-model="form.goBrowser" placeholder="Google Chrome">
                <el-option label="1.0.0" value="1.0.0"></el-option>
                <el-option label="1.0.1" value="1.0.1"></el-option>
              </el-select>
            </el-col>
            <el-col :span="5">
              <br />
              <el-select v-model="form.fiBrowser" placeholder="firefox">
                <el-option label="1.0.0" value="1.0.0"></el-option>
                <el-option label="1.0.1" value="1.0.1"></el-option>
              </el-select>
            </el-col>
            <el-col :span="4" :offset="1">
              <div>操作系统</div>
              <el-select v-model="form.os" placeholder="操作系统">
                <el-option label="Windows 10" value="windows10"></el-option>
                <el-option label="Windows 7" value="windows7"></el-option>
                <el-option label="macOS" value="macos"></el-option>
              </el-select>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="3">
              <div>User Agent</div>
              <el-select v-model="form.userAgent" placeholder="全部">
                <el-option label="代理1" value="代理111"></el-option>
                <el-option label="代理2" value="代理222"></el-option>
              </el-select>
            </el-col>
            <el-col :span="9">
              <br />
              <el-input v-model="form.userAgent" placeholder="代理信息"></el-input>
            </el-col>
            <el-col :span="5">
              <div>cookie</div>
              <el-input v-model="form.cookie" placeholder="支持格式: JSON, Netscape, Name-Value"></el-input>
            </el-col>
          </el-row>
          <!-- 换行 -->
          <br />
          <!-- 换行 -->
          <el-form-item label="备注">
            <el-input type="textarea" v-model="form.remark" placeholder="请输入备注"></el-input>
          </el-form-item>
        </el-card>
        <!-- =========================================基础信息内容==================================================== -->
        <!-- 代理信息 -->
        <el-card id="proxy-info" class="box-card" shadow="never">
          <div slot="header">
            <span>代理信息</span>
          </div>
          <!-- =========================================代理信息内容==================================================== -->
          <el-row>
            <div style="margin-top: 20px">
              <div>代理信息</div>
              <el-radio-group v-model="proxy.proxyEnabled">
                <el-radio-button :label="true">自定义</el-radio-button>
                <el-radio-button :label="false">已添加代理</el-radio-button>
              </el-radio-group>
            </div>
            <el-col :span="6" :offset="8">
              <div style="margin-top: 20px">代理类型</div>
              <el-row>
                <el-col :span="18">
                  <el-select v-model="proxy.proxyMode" placeholder="选择代理类型">
                    <el-option label="No Proxy(本地直连)" value="noProxy"></el-option>
                    <el-option label="SOCKS5" value="socks5"></el-option>
                  </el-select>
                </el-col>
                <el-col :span="6">
                  <el-button type="primary" @click="checkNetwork">检查网络</el-button>
                </el-col>
              </el-row>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="6">
              <div>IP查询渠道</div>
              <el-select v-model="proxy.ipLocation" placeholder="IP2Location">
                <el-option label="IP2Location" value="IP2Location"></el-option>
                <el-option label="ip1" value="192.168.1.1"></el-option>
                <el-option label="ip2" value="192.168.1.2"></el-option>
              </el-select>
            </el-col>
          </el-row>
        </el-card>
        <!-- =======================================代理信息内容==================================================== -->
        <!-- 账号平台信息 -->
        <el-card id="account-info" class="box-card" shadow="never">
          <div slot="header">
            <span>帐号信息</span>
          </div>
          <!-- =======================================平台信息内容==================================================== -->
          <el-row style="margin-top: 20px">
            <el-col :span="12">
              <div>帐号平台</div>
              <el-select v-model="platform.platform" placeholder="请选择帐号平台">
                <el-option label="平台A" value="platformA"></el-option>
                <el-option label="平台B" value="platformB"></el-option>
              </el-select>
            </el-col>
            <el-col :span="3">
              <br />
              <el-input v-model="platform.accountUrl" placeholder="指定URL"></el-input>
            </el-col>
          </el-row>
        </el-card>
        <!-- =======================================平台信息内容==================================================== -->
        <!-- 指纹配置 -->
        <el-card id="fingerprint-info" class="box-card" shadow="never">
          <div class="header">
            <span>指纹配置</span>
          </div>
          <!-- =======================================指纹配置内容===================================================== -->
          <el-row :gutter="20">
            <el-col :span="12">
              <div>时区</div>
              <el-switch v-model="finge.timeZone" />
              <span>基于IP匹配</span>
            </el-col>
            <el-col :span="12">
              <div>WebRTC</div>
              <div>
                <div class="my-2 ml-4">
                  <el-radio-group v-model="finge.webRTC">
                    <el-radio value="1">转发</el-radio>
                    <el-radio value="2">隐私</el-radio>
                    <el-radio value="3">替换</el-radio>
                    <el-radio value="4">真实</el-radio>
                    <el-radio value="5">禁用</el-radio>
                  </el-radio-group>
                </div>
              </div>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>地理位置</div>
              <el-radio-group v-model="finge.geLocation">
                <el-radio :value="true">询问</el-radio>
                <el-radio :value="false">禁止</el-radio>
              </el-radio-group>
              <el-switch v-model="finge.geIp" />
              <span>基于IP匹配</span>
            </el-col>
            <el-col :span="12">
              <div>语言</div>
              <div>
                <el-switch v-model="finge.langIp" />
                <span>基于IP匹配</span>
              </div>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>分辨率</div>
              <el-radio-group v-model="finge.res">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">自定义</el-radio>
              </el-radio-group>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>字体</div>
              <el-radio-group v-model="finge.font">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">自定义</el-radio>
              </el-radio-group>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>canvas</div>
              <el-radio-group v-model="finge.canvas">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">自定义</el-radio>
              </el-radio-group>
            </el-col>
            <el-col :span="12">
              <div>WebGL图像</div>
              <el-radio-group v-model="finge.webGLimage">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">噪音</el-radio>
              </el-radio-group>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>WebGL元数据</div>
              <el-radio-group v-model="finge.webGLmetadata">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">噪音</el-radio>
              </el-radio-group>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>AudioContext</div>
              <el-radio-group v-model="finge.audioText">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">噪音</el-radio>
              </el-radio-group>
            </el-col>
            <el-col :span="12">
              <div>ClientRects</div>
              <el-radio-group v-model="finge.clientRects">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">噪音</el-radio>
              </el-radio-group>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>SpeechVoices</div>
              <el-radio-group v-model="finge.speech">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">噪音</el-radio>
              </el-radio-group>
            </el-col>
            <el-col :span="12">
              <el-col :span="12">
                <div>Do Not Track</div>
                <el-radio-group v-model="finge.track">
                  <el-radio value="1">默认</el-radio>
                  <el-radio value="2">开启</el-radio>
                  <el-radio value="3">禁止</el-radio>
                </el-radio-group>
              </el-col>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>硬件并发数</div>
              <el-space>
                <el-input-number v-model="finge.hardNum" controls-position="right" />
              </el-space>
            </el-col>
            <el-col :span="12">
              <div>设备内存</div>
              <el-space>
                <el-input-number v-model="finge.equiNum" controls-position="right" />
              </el-space>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>设备名称</div>
              <el-radio-group v-model="finge.equiName">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">自定义</el-radio>
              </el-radio-group>
            </el-col>
            <el-col :span="12">
              <div>MAC地址</div>
              <el-radio-group v-model="finge.macAddress">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">自定义</el-radio>
              </el-radio-group>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <div>端口扫描保护</div>
              <el-radio-group v-model="finge.portPro">
                <el-radio :value="true">关闭</el-radio>
                <el-radio :value="false">开启</el-radio>
              </el-radio-group>
            </el-col>
            <el-col :span="12">
              <div>Mac地址</div>
              <el-radio-group v-model="finge.macPro">
                <el-radio :value="true">真实</el-radio>
                <el-radio :value="false">自定义</el-radio>
              </el-radio-group>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="24">
              <el-row :gutter="20">
                <el-col :span="12">
                  <div>硬件加速</div>
                  <el-radio-group v-model="finge.hardAcc">
                    <el-radio value="1">默认</el-radio>
                    <el-radio value="2">开启</el-radio>
                    <el-radio value="3">禁止</el-radio>
                  </el-radio-group>
                </el-col>
              </el-row>
              <div>启动参数</div>
              <el-input type="textarea" v-model="finge.startup" placeholder="请输入备注"></el-input>
            </el-col>
          </el-row>
        </el-card>
        <!-- =======================================指纹配置内容===================================================== -->
        <!-- 高级配置 -->
        <el-card id="advanced-settings" class="box-card" shadow="never">
          <div slot="header">
            <span>高级配置</span>
          </div>
          <!-- =======================================高级配置内容===================================================== -->
          <el-row :gutter="20">
            <el-col :span="12">
              <div>User Agent</div>
              <el-select v-model="apply.appName" placeholder="跟随团队应用">
                <el-option label="1" value="应用1"></el-option>
                <el-option label="2" value="应用2"></el-option>
              </el-select>
            </el-col>
          </el-row>
        </el-card>
        <!-- =======================================高级配置内容===================================================== -->
        <!-- 提交按钮 -->
        <el-form-item>
          <el-button type="default" @click="onCancel">取消</el-button>
          <el-button type="primary" @click="onSubmit">确定</el-button>
        </el-form-item>
      </el-form>
      <el-form class="browser-form-right">
        <Summary />
      </el-form>
    </div>
  </Layout>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";
import Summary from "@/views/newBrowser/components/Summary.vue";
import Layout from "@/layouts/index.vue";
import { onMounted } from "vue";


const groupIds = ref([]); // 存储从后端获取的所有分组ID
const loading = ref(false); // 加载状态

// 获取分组ID数据的函数
const fetchGroupIdsData = async () => {
};

// 组件挂载时获取数据
onMounted(fetchGroupIdsData);

const checkNetwork = () => {
  // 随机生成 0 或 1 的布尔值
  const isConnected = Math.random() >= 0.5;

  if (isConnected) {
    // 成功提示

  } else {
    // 失败提示

  }
};


// 基础数据
const form = reactive({
  id: 1,
  browserName: "", //获取浏览器名称
  groupName: "", //获取分组名称
  priority: "", //获取便签信息
  goBrowser: "", //获取谷歌浏览器版本
  fiBrowser: "", //获取火狐浏览器版本
  os: "", //获取操作系统信息
  userAgent: "", //获取用户代理信息(User Agent)
  cookie: "", //获取cookie信息
  combineCookie: false, //是否合并cookie
  remark: "", //获取备注信息
  groupId: null, // 绑定选中的分组ID
});

// 代理数据
const proxy = reactive({
  proxyEnabled: true, //是否启用添加代理
  proxyMode: "", //获取代理模式
  primary: "", //检查网络（占位）
  ipLocation: "", //ip渠道查询
});

//平台数据
const platform = reactive({
  platform: "", //获取平台信息
  accountUrl: "", //获取账号地址
});
//指纹设置
const finge = reactive({
  timeZone: true, //获取时区关于ip匹配
  webRTC: "1", //获取webRTC信息
  geLocation: true, //获取地理位置询问或禁止
  geIp: true, //地理位置是否使用ip匹配
  langIp: true, //获取语言是否使用ip匹配
  res: true, //是否使用真实分辨率或者自定义分辨率
  font: true, //是否使用真实字体或者自定义字体
  canvas: true, //是否使用真实canvas或者自定义canvas
  webGLimage: true, //是否使用真实webGL图像或者自定义webGL图像
  webGLmetadata: true, //是否使用真实webGL元数据或者自定义webGL元数据
  audioText: true, //是否使用真实音频文本或者噪音音频文本
  clientRects: true, //是否使用真实ClientRects或者噪音ClientRects
  speech: true, //是否使用真实语音或者噪音语音
  track: "1", //是否开启追踪（默认1）
  hardNum: "1", //获取硬件并发数
  equiNum: "1", //获取设备内存大小
  equiName: true, //是否使用自定义设备名称
  equiNames: "", //获取设备名称
  macAddress: true, //是否使用自定义mac地址
  macNames: "", //获取mac地址
  portPro: true, //是否开启端口保护
  macPro: true, //是否开启mac地址保护
  hardAcc: "1", //是否使用真实硬件加速（默认1）
  startup: "", //填写启动参数
});

//高级配置
const apply = reactive({
  appName: "", //获取应用名称
});

// 滚动到页面的指定部分
const scrollToSection = (sectionId: string) => {
  const section = document.getElementById(sectionId);
  if (section) {
    section.scrollIntoView({ behavior: "smooth" });
  }
};

// 提交表单
const onSubmit = async () => {
  try {
    const data = {
      name: form.browserName,
      domain_name: platform.accountUrl,
      open_urls: "", // 需要根据实际情况设置
      repeat_config: "", // 需要根据实际情况设置
      username: "", // 需要根据实际情况设置
      password: "", // 需要根据实际情况设置
      fakey: "", // 需要根据实际情况设置
      cookie: form.cookie,
      ignore_cookie_error: 0,
      group_id: 0, // 需要根据form.groupName转换
      fp_info_id: 0, // 需要根据实际情况设置
      ua: form.userAgent,
      os: form.os,
      country: "", // 需要根据实际情况设置
      region: "", // 需要根据实际情况设置
      city: "", // 需要根据实际情况设置
      remark: form.remark,
      ipchecker: proxy.ipLocation,
      sys_app_cate_id: "0",
      user_proxy_config: "", // 需要根据proxy配置转换
      proxy: "", // 需要根据proxy配置转换
      proxy_enable: proxy.proxyEnabled ? 1 : 0,
      is_tz: finge.timeZone ? 1 : 0,
      is_pos: finge.geLocation ? 1 : 0,
      user_data_file: "", // 需要根据实际情况设置
      driver_location: "", // 需要根据实际情况设置
      status: 1,
    };


    console.log("创建环境成功:", data);
  } catch (error) {

    console.error("创建环境失败:", error);
  }
};

// 取消操作
const onCancel = () => {
  console.log("取消操作");
};

// 开关操作

//单选框默认
</script>

<style scoped>
.browser-form {
  display: flex;
}

.browser-form-left {
  width: 80%;
}

.browser-form-right {
  width: 20%;
  background-color: rgb(255, 255, 255);
}

.box-card {
  /* margin-bottom: 20px; */
  box-shadow: none;
  /* 取消卡片的阴影 */
}

.el-breadcrumb {
  margin-bottom: 20px;
}

/* 圆形选项框改成方形选项框 */
::v-deep .el-radio {
  .el-radio__input {
    .el-radio__inner {
      border-radius: 2px;
    }
  }

  .el-radio__input.is-checked .el-radio__inner::after {
    content: "";
    width: 8px;
    height: 3px;
    border: 2px solid white;
    border-top: transparent;
    border-right: transparent;
    text-align: center;
    display: block;
    position: absolute;
    top: 2px;
    left: 1px;
    /* vertical-align: middle; */
    transform: rotate(-45deg);
    border-radius: 0px;
    background: none;
  }
}
</style>
