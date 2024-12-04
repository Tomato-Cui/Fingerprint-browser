<!-- 偏好设置 -->
<template>
  <div class="footer">
    <el-button @click="submitSetting">保存设置</el-button>
    <el-link style="margin-left: 20px">恢复默认</el-link>
  </div>
  <el-form :model="form">
    <el-form-item label="浏览器" prop="browser">
      <el-radio-group
        v-model="form.browser"
        size="large"
        @change="handleChange"
        style="width: 100%; height: 100%; gap: 20px"
      >
        <el-radio-button :value="browserRadio.ChromeCore">
          <div
            style="
              width: 100%;
              display: flex;
              align-items: center;
              justify-content: center;
              font-size: 16px;
            "
          >
            {{ browserRadio.ChromeCore }}
          </div>
          <el-select
            v-model="browserRadio.ChromeCore"
            placeholder="请选择浏览器"
          >
            <el-option
              v-for="item in browserList1"
              :label="item"
              :value="item"
            />
          </el-select>
        </el-radio-button>
        <el-radio-button :value="browserRadio.FirefoxCore">
          <div
            style="
              width: 100%;
              display: flex;
              align-items: center;
              justify-content: center;
              font-size: 16px;
            "
          >
            {{ browserRadio.FirefoxCore }}
          </div>
          <el-select
            v-model="browserRadio.FirefoxCore"
            placeholder="请选择浏览器"
          >
            <el-option
              v-for="item in browserList2"
              :label="item"
              :value="item"
            />
          </el-select>
        </el-radio-button>
      </el-radio-group>
    </el-form-item>
    <el-form-item label="操作系统" prop="optionSystem">
      <div style="display: flex; flex-direction: column">
        <el-switch v-model="form.optionSystem" active-text="基于电脑匹配" />
        <el-checkbox-group
          v-model="form.customSystem"
          size="large"
          v-if="form.optionSystem == 0"
        >
          <el-checkbox-button
            v-for="system in systemList"
            :key="system"
            :value="system"
          >
            {{ system }}
          </el-checkbox-button>
        </el-checkbox-group>
      </div>
    </el-form-item>
    <el-form-item label="userAgent" prop="userAgent">
      <el-select
        v-model="form.userAgent"
        multiple
        collapse-tags
        collapse-tags-tooltip
        :max-collapse-tags="3"
        placeholder="Select"
        style="width: 240px"
      >
        <el-option
          v-for="item in userAgentList"
          :key="item"
          :label="item"
          :value="item"
        />
      </el-select>
    </el-form-item>
    <el-divider content-position="left">指纹配置</el-divider>
    <el-form-item label="WebRTC">
      <el-radio-group v-model="form.WebRTC" size="large">
        <el-radio-button v-for="item in WebRTCList" :key="item" :value="item">
          {{ item }}
        </el-radio-button>
      </el-radio-group>
    </el-form-item>
    <el-form-item label="时区">
      <el-radio-group v-model="form.timezone" size="large">
        <el-radio-button
          v-for="item in isCustomTimezone"
          :key="item.value"
          :value="item.value"
        >
          {{ item.label }}
        </el-radio-button>
      </el-radio-group>
      <el-select
        v-model="form.customTimezone"
        placeholder="请选择时区"
        v-if="form.timezone === 2"
      >
        <el-option
          v-for="item in timezoneList"
          :key="item"
          :label="item"
          :value="item"
        />
      </el-select>
    </el-form-item>
    <el-form-item label="地理位置">
      <el-radio-group v-model="form.location" size="large">
        <el-radio-button v-for="item in locationList" :key="item" :value="item">
          {{ item.label }}
        </el-radio-button>
      </el-radio-group>
      <div>
        <el-checkbox
          v-model="form.useIPLocation"
          label="使用IP地理位置"
          size="large"
        />
        <div>
          <el-form :model="form.customLocation">
            <el-form-item label="经度/纬度">
              <el-input v-model="form.customLocation.longitude" />
              <!--经度-->
              <el-input v-model="form.customLocation.latitude" />
              <!--纬度-->
            </el-form-item>
            <el-form-item label="精度">
              <el-input v-model="form.customLocation.precision" />
            </el-form-item>
          </el-form>
        </div>
      </div>
    </el-form-item>
    <el-form-item label="语言">
      <el-radio-group v-model="form.language" size="large">
        <el-radio-button :value="0">基于IP</el-radio-button>
        <el-radio-button :value="1">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.language === 1">
        <div v-for="item in form.customLanguage" :key="item.value">{{ item.label }}</div>
        <el-link :icon="Plus" @click="languageDialogVisible = true">添加语言</el-link>
      </div>
    </el-form-item>
    <el-form-item label="界面语言">
      <el-radio-group v-model="form.interfaceLanguage" size="large">
        <el-radio-button :value="0">基于语言</el-radio-button>
        <el-radio-button :value="1">真实</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.interfaceLanguage === 2">
        <el-select
          v-model="form.customInterfaceLanguage"
          placeholder="请选择界面语言"
        >
          <el-option
            v-for="item in interfaceLanguageList"
            :key="item"
            :label="item"
            :value="item"
          />
        </el-select>
      </div>
    </el-form-item>
    <el-form-item label="分辨率">
      <el-radio-group v-model="form.resolution" size="large">
        <el-radio-button :value="0">随机</el-radio-button>
        <el-radio-button :value="1">预定义</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.resolution === 1">
        <el-select v-model="form.customResolution" placeholder="请选择分辨率">
          <el-option
            v-for="item in resolutionList"
            :key="item"
            :label="item"
            :value="item"
          />
        </el-select>
      </div>
      <div v-if="form.resolution === 2">
        <el-input v-model="form.customResolution" placeholder="请输入分辨率" />
      </div>
    </el-form-item>
    <el-form-item label="字体">
      <el-radio-group v-model="form.font" size="large">
        <el-radio-button :value="0">随机</el-radio-button>
        <el-radio-button :value="1">默认</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.font === 2">
        <el-link :icon="Refresh" @click="getRandomArrayElements(fontList, 3)"
          >换一换</el-link
        >
        <div v-if="form.font === 2">
          <div v-for="item in randomList" :key="item">{{ item }}</div>
        </div>
      </div>
    </el-form-item>
    <el-form-item label="硬件噪音">
      <el-switch v-model="form.hardwareNoise.Canvas" active-text="Canvas" />
      <el-switch v-model="form.hardwareNoise.WebGL" active-text="WebGL图像" />
      <el-switch
        v-model="form.hardwareNoise.AudioContext"
        active-text="AudioContext"
      />
      <el-switch
        v-model="form.hardwareNoise.MediaDevices"
        active-text="媒体设备[auto]"
      />
      <el-link type="primary">编辑</el-link>
      <el-switch
        v-model="form.hardwareNoise.ClientRects"
        active-text="ClientRects"
      />
      <el-switch
        v-model="form.hardwareNoise.SpeechVoices"
        active-text="SpeechVoices"
      />
    </el-form-item>
    <el-form-item label="WebGL元数据">
      <el-radio-group v-model="form.WebGL" size="large">
        <el-radio-button :value="0">基于User-Agent</el-radio-button>
        <el-radio-button :value="1">真实</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.WebGL === 2">
        <el-form :model="form.customWebGL">
          <el-form-item label="制造商">
            <el-input v-model="form.customWebGL.manufacturer" />
          </el-form-item>
          <el-form-item label="渲染器">
            <el-input v-model="form.customWebGL.renderer" />
          </el-form-item>
        </el-form>
      </div>
    </el-form-item>
    <el-form-item label="WebGPU">
      <el-radio-group v-model="form.WebGPU" size="large">
        <el-radio-button :value="0">基于WebGL</el-radio-button>
        <el-radio-button :value="1">真实</el-radio-button>
        <el-radio-button :value="2">禁用</el-radio-button>
      </el-radio-group>
    </el-form-item>
    <el-form-item label="CPU">
      <el-radio-group v-model="form.CPU" size="large">
        <el-radio-button :value="0">随机</el-radio-button>
        <el-radio-button :value="1">真实</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.CPU === 2">
        <el-select v-model="form.customCPU" placeholder="请选择CPU">
          <el-option
            v-for="item in CPUList"
            :key="item"
            :label="item"
            :value="item"
          />
        </el-select>
      </div>
    </el-form-item>
    <el-form-item label="RAM">
      <el-radio-group v-model="form.RAM" size="large">
        <el-radio-button :value="0">随机</el-radio-button>
        <el-radio-button :value="1">真实</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.RAM === 2">
        <el-select v-model="form.customRAM" placeholder="请选择RAM">
          <el-option
            v-for="item in RAMList"
            :key="item"
            :label="item"
            :value="item"
          />
        </el-select>
      </div>
    </el-form-item>
    <el-form-item label="设备名称">
      <el-radio-group v-model="form.deviceName" size="large">
        <el-radio-button :value="0">基于User-Agent</el-radio-button>
        <el-radio-button :value="1">真实</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.deviceName === 2">
        <el-input
          v-model="form.customDeviceName"
          placeholder="请输入设备名称"
        />
      </div>
    </el-form-item>
    <el-form-item label="MAC地址">
      <el-radio-group v-model="form.MACaddress" size="large">
        <el-radio-button :value="0">基于User-Agent</el-radio-button>
        <el-radio-button :value="1">真实</el-radio-button>
        <el-radio-button :value="2">自定义</el-radio-button>
      </el-radio-group>
      <div v-if="form.MACaddress === 2">
        <el-input v-model="form.customMACaddress" placeholder="请输入MAC地址" />
      </div>
    </el-form-item>
    <el-form-item label="doNotTrack">
      <el-radio-group v-model="form.doNotTrack" size="large">
        <el-radio-button :value="0">默认</el-radio-button>
        <el-radio-button :value="1">开启</el-radio-button>
        <el-radio-button :value="2">关闭</el-radio-button>
      </el-radio-group>
    </el-form-item>
    <el-form-item label="端口扫描保护">
      <el-radio-group v-model="form.postScan" size="large">
        <el-radio-button :value="0">开启</el-radio-button>
        <el-radio-button :value="1">关闭</el-radio-button>
      </el-radio-group>
      <div v-if="form.postScan === 1">
        <el-input
          v-model="form.customPostScan"
          placeholder="选填，请输入端口扫描保护"
        />
      </div>
    </el-form-item>
    <el-form-item label="硬件加速">
      <el-radio-group v-model="form.hardwareAcceleration" size="large">
        <el-radio-button :value="0">默认</el-radio-button>
        <el-radio-button :value="1">开启</el-radio-button>
        <el-radio-button :value="2">关闭</el-radio-button>
      </el-radio-group>
    </el-form-item>
    <el-form-item label="启动参数">
      <el-input
        v-model="form.startupArguments"
        placeholder="example: --disable-web-security --disable-gpu"
        type="textarea"
      />
    </el-form-item>
    <!-- 添加语言弹出框 -->
    <el-dialog v-model="languageDialogVisible" title="添加语言" width="550px" @close="languageDialogVisible = false">
      <div class="language-dialog">
        <el-input v-model="searchLanguage" placeholder="请输入语言" style="height: 50px;"/>
        <el-checkbox-group v-model="selectedLanguage" style="display: flex;flex-direction: column;padding: 0px 15px;">
          <el-checkbox v-for="item in filterLanguageList" :key="item.value" :value="item">
            {{ item.label }}
          </el-checkbox>
        </el-checkbox-group>
      </div>
      <template #footer>
        <el-button type="primary" @click="submitLanguage">确定</el-button>
        <el-button @click="languageDialogVisible = false">取消</el-button>
      </template>
    </el-dialog>
  </el-form>
</template>

<script setup lang="ts">
import { ElMessage } from "element-plus";
import { ref, reactive, watch, onMounted, computed } from "vue";
import { Plus, Refresh } from "@element-plus/icons-vue";

const browserList1 = ref(["Chrome 131", "Chrome 130", "Chrome 132"]);
const browserList2 = ref(["Firefox 132", "Firefox 130", "Firefox 132"]);
const systemList = ["Windows", "MacOS", "Linux", "Android", "iOS"];
const userAgentList = ["UA 131", "UA 130", "UA 132", "UA 133", "UA 134"];
const WebRTCList = ["转发", "替换", "真实", "禁用"];
const isCustomTimezone = [
  //是否自定义时区
  {
    label: "基于IP",
    value: 0,
  },
  {
    label: "真实",
    value: 1,
  },
  {
    label: "自定义",
    value: 2,
  },
];
const timezoneList = ["Asia/Shanghai", "Asia/Tokyo", "Europe/London"];
const locationList = [
  {
    label: "询问",
    value: 0,
  },
  {
    label: "允许",
    value: 1,
  },
  {
    label: "禁止",
    value: 2,
  },
];
const resolutionList = [
  "1920x1080",
  "1920x1080",
  "1920x1080",
  "1920x1080",
  "1920x1080",
]; //分辨率列表
const fontList = [
  "Arial",
  "Arial Black",
  "Arial Narrow",
  "Arial Rounded MT Bold",
  "Arial Unicode MS",
]; //字体列表
const CPUList = ["2核", "3核心", "4核心", "5核心", "6核心", "7核心"]; //CPU列表
const RAMList = ["2GB", "3GB", "4GB", "5GB", "6GB", "7GB"]; //RAM列表
const languageList = ref([
  { label: "English", value: 1 },
  { label: "French", value: 2 },
  { label: "German", value: 3 },
  { label: "Spanish", value: 4 },
  { label: "Italian", value: 5 },
  { label: "Portuguese", value: 6 },
  { label: "Russian", value: 7 },
  { label: "Japanese", value: 8 },
  { label: "Chinese", value: 9 },
]); //语言列表
const selectedLanguage = ref([]); //选中的语言
const interfaceLanguageList = [
  "eesti",
  "suomi",
  "latviešu",
  "lithuanian",
  "latvian",
];
const browserFlag = ref(0); //当前选中的浏览器类型 0: ChromeCore 1: FirefoxCore
const form = reactive({
  browser: "Firefox 132",
  optionSystem: 0, //0: 系统操作系统 1: 自定义操作系统
  customSystem: "", //自定义操作系统列表
  userAgent: "", //自定义userAgent
  WebRTC: "", //自定义WebRTC
  timezone: 0, //时区 0: 基于IP 1: 真实 2: 自定义
  customTimezone: "", //自定义时区
  location: 0, //地理位置
  useIPLocation: false, //使用IP地理位置
  customLocation: { precision: "", longitude: "", latitude: "" }, //自定义地理位置  经度 纬度 精度
  language: 0, //是否自定义语言  0：基于IP 1：自定义
  customLanguage: [
    { label: "English", value: 1 },
    { label: "Italian", value: 5 },
  ], //自定义语言
  interfaceLanguage: 0, //界面语言  0：基于语言 1：真实 2：自定义
  customInterfaceLanguage: "", //自定义界面语言
  resolution: 0, //分辨率  0：随机 1：预定义 2：自定义
  customResolution: "", //自定义分辨率
  font: 0, //字体  0：随机 1：默认 2：自定义
  customFont: "", //自定义字体
  hardwareNoise: {
    Canvas: 0,
    WebGL: 0,
    AudioContext: 0,
    MediaDevices: 0,
    ClientRects: 0,
    SpeechVoices: 0,
  }, //硬件噪音
  WebGL: 0, //WebGL  0：基于User-Agent 1：真实 2：自定义
  customWebGL: { manufacturer: "", renderer: "" }, //自定义WebGL 制造商 渲染器
  WebGPU: 0, //WebCPU  0：基于WebGL 1：真实 2：禁用
  CPU: 0, //CPU  0：随机 1：真实 2：自定义
  customCPU: "", //自定义CPU
  RAM: 0, //RAM  0：随机 1：真实 2：自定义
  customRAM: "", //自定义RAM
  deviceName: 0, //设备名称  0：基于User-Agent 1：真实 2：自定义
  customDeviceName: "", //自定义设备名称
  MACaddress: 0, //MAC地址  0：基于User-Agent 1：真实 2：自定义
  customMACaddress: "", //自定义MAC地址
  doNotTrack: 0, //doNotTrack  0：默认 1：开启 2：关闭
  postScan: 0, //端口扫描保护  0：开启 1：关闭
  customPostScan: "", //自定义端口扫描保护
  hardwareAcceleration: 0, //硬件加速  0：默认 1：开启 2：关闭
  startupArguments: "", //自定义启动参数
});
const languageDialogVisible = ref(false); //添加语言弹出框
//浏览器单选框
const browserRadio = reactive({
  ChromeCore: "Chrome 131",
  FirefoxCore: "Firefox 132",
});
watch(
  () => browserRadio.ChromeCore,
  (newVal) => {
    form.browser = newVal;
    browserFlag.value = 0;
  }
);
watch(
  () => browserRadio.FirefoxCore,
  (newVal) => {
    form.browser = newVal;
    browserFlag.value = 1;
  }
);
const handleChange = (val: string) => {
  console.log(val);
};

const submitSetting = () => {
  // ElMessage.success("确定");
  console.log("-----------:", form.browser);
};
// 获取数组中随机几个元素
const randomList = ref<any[]>([]);
const getRandomArrayElements = (arr: any[], count: number) => {
  if (count > arr.length) {
    throw new Error("Requested more elements than the array contains.");
  }

  let copy = [...arr]; // 创建数组的一个浅拷贝
  for (let i = 0; i < count; i++) {
    const randomIndex = Math.floor(Math.random() * (copy.length - i)) + i;
    [copy[i], copy[randomIndex]] = [copy[randomIndex], copy[i]]; // 交换元素
  }
  randomList.value = copy.slice(0, count);
};

const searchLanguage = ref(""); //搜索语言
//过滤语言列表
const filterLanguageList = computed(() => {
  return languageList.value.filter((language) => {
    //如果form.customLanguage中包含此项，则过滤
    const matchGroup = form.customLanguage.find(item => item.value === language.value)
    // console.log("matchGroup:", matchGroup);
    
    let matchQuery = language.label.includes(searchLanguage.value)
    return matchQuery && !matchGroup;
    });
});
const submitLanguage = () => { //确定添加语言
  // form.customLanguage = selectedLanguage.value;
  selectedLanguage.value.forEach(item => {
    form.customLanguage.push(item)
  })
  selectedLanguage.value = []; //清空选中的语言
  console.log("form.customLanguage:", form.customLanguage);
  
  languageDialogVisible.value = false;
};

onMounted(() => {
  getRandomArrayElements(fontList, 3);
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
  background-color: #fff;
  padding-left: 50px;
  // left: 300px;
  :deep(.el-button) {
    width: 100px;
    height: 40px;
    background-color: #1e4cfc;
    color: #fff;
    font-size: 18px;
    border-radius: 10px;
  }
}

.el-form {
  // height: 2000px;
  min-width: 600px;
  .el-form-item {
    min-height: 50px;
    display: flex;
    align-items: center;

    :deep(.el-form-item__content) {
      //height: 40px;
      // width: 300px;

      .el-radio-button {
        width: 230px;
        height: 100%;

        .el-radio-button__inner {
          padding: 0;
          width: 100%;
          height: 100%;
          display: flex;
          justify-content: flex-end;
          border-radius: 5px;
          border: 1.5px solid #bcbec6;

          .el-select {
            width: 50px;
            display: flex;
            justify-content: flex-end;
          }
        }
      }
    }
  }
}

.language-dialog{
  border: 1px solid #1e4cfc;
  border-radius: 10px;

  .el-input{
    :deep(.el-input__wrapper){
      border-radius: 10px 10px 0 0;
    }
  }
}
</style>
