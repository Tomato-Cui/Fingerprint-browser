<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import ImportProxyModel from "../components/import-proxy-model.vue";
import { useRouter, useRoute } from "vue-router";
import BasicSetting from "../components/advanced-create/basic-setting.vue";
import ProxySetting from "../components/advanced-create/proxy-setting.vue";
import AccountSetting from "../components/advanced-create/account-setting.vue";
import CookieSetting from "../components/advanced-create/cookie-setting.vue";
import OpenedBrowserSetting from "../components/advanced-create/opened-browser-setting.vue";
import AdvancedSetting from "../components/advanced-create/advanced-setting.vue";
import LoadingIcon from "@/components/loading.vue";
import { advancedCreateHandle, updateEnvironment } from "../index";
import { convertToCSV, downloadCSV } from "@/util/lib";
import EncryptModel from "../components/encrypt-model.vue";
import Loading from "@/components/loading/index.vue";
import { toast } from "vue-sonner";
import {
  templaeFormtData as data,
  useEnvironmentAdvancedFormStore,
} from "@/stores/form/environment-advanced";

const { forms, resetForm } = useEnvironmentAdvancedFormStore();

const importProxyOpen = ref(false);
const router = useRouter();
const route = useRoute();
const proxyModelCloseHandle = () => (importProxyOpen.value = false);

const createing = ref(false);
const encrypts = ref<any[]>([]);

const handleSubmit = () => {
  if (route.query.env !== undefined) {  //修改操作
    updateEnvironment(JSON.parse(JSON.parse(JSON.stringify(route.query.env))).uuid).then((res: any) => {
      toast.success(res.message)
      resetForm();
      router.push('/environment/0')
    }).catch((res) => {
      toast.warning(res)
    })
  } else {  //创建操作
    const loadingTimeout = setTimeout(() => {
      createing.value = true;
    }, 1000);
    advancedCreateHandle()
      .then((res: any) => {
        if (res.code == 1) {
          toast.success("创建成功", { position: "top-right" });
          encrypts.value = res.data
            .filter((item: any) => item.encrypt)
            .map((item: any) => ({ name: item.name, encrypt: item.encrypt }));
        }
      })
      .catch((res) => {
        toast.warning(res);
      })
      .finally(() => {
        clearTimeout(loadingTimeout);
        createing.value = false;
      });
  }
};
watch(() => route.path, () => {
  // resetForm();
})
onMounted(() => {
  //
  if (route.query.env !== undefined) {
    const env = JSON.parse(JSON.parse(JSON.stringify(route.query.env)));
    // console.log("env:", env);
    
    forms.id = env.id;
    forms.name = env.name;
    forms.browser = { value: env.browser };
    forms.os = { value: env.os };
    forms.ua = env.ua;
    forms.encryPtSwitch = { value: "关闭" }
    forms.group = env.group_id || '';
    forms.tag = env.tag_id || '';
    forms.description = env.description || '';
    forms.proxy.kind = env.proxy_kind || '';
    forms.proxy.host = env.proxy_host || '';
    forms.proxy.port = env.proxy_port || '';
    forms.proxy.username = env.proxy_username || '';
    forms.proxy.password = env.proxy_password || '';
    forms.account.platform = env.accounts.platform || '';
    forms.account.platform_account = env.accounts.platform_account || '';
    forms.account.platform_password = env.accounts.platform_password || '';
    forms.cookie = env.cookies || '';
    forms.setting.startedBrowserDefaultUrl = env.startedBrowserDefaultUrl || '';
    forms.setting.startedBrowserAccountUrlIsOpen = env.startedBrowserAccountUrlIsOpen || '';
    forms.fingerprint.timezone = { value: env.timezone || 'ip' };
    forms.fingerprint.webRTC = { value: env.web_rtc };
    forms.fingerprint.geolocation = { value: env.geography };
    forms.fingerprint.language = { value: env.language };
    forms.fingerprint.resolution = { value: env.region }
    forms.fingerprint.fonts = { value: env.fonts };
    forms.fingerprint.canvas = { value: env.canvas };
    forms.fingerprint.webGL = { value: env.webgl }
    forms.fingerprint.webGPU = { value: 'disabled' }
    forms.fingerprint.audioContext = { value: env.audio_context };
    forms.fingerprint.mediaDevices = { value: env.media }
    forms.fingerprint.clientRects = { value: 'real' }
    forms.fingerprint.speechVoices = { value: env.speech_voices }
    forms.fingerprint.hardwareConcurrency = { value: env.cpu }
    forms.fingerprint.deviceMemory = { value: env.memory }
    forms.fingerprint.deviceName = { value: 'real' }
    forms.fingerprint.macAddress = { value: 'real' }
    forms.fingerprint.doNotTrack = { value: env.do_not_track}
    forms.fingerprint.bluetooth = { value: 'random' }
    forms.fingerprint.battery = { value: env.battery }
    forms.fingerprint.portScanning = { value: env.port_scan }
    forms.fingerprint.startupParams.value = ''
  }
});
</script>
<template>
  <!-- {{ route.query }} -->
  <div class="grow overflow-hidden">
    <div class="overflow-y-auto h-full my-3">
      <BasicSetting />
      <ProxySetting />
      <AccountSetting />
      <CookieSetting />
      <OpenedBrowserSetting />
      <AdvancedSetting />
    </div>
  </div>
  <div class="border-t flex gap-x-4 justify-end items-center p-3">
    <button @click="router.go(-1)"
      class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 active:scale-[.98]">
      取消
    </button>
    <button @click="handleSubmit"
      class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]">
      确认
    </button>
  </div>
  <ImportProxyModel :open="importProxyOpen" @close="proxyModelCloseHandle" />

  <Loading :open="createing">
    <LoadingIcon />
    <span class="text-xs whitespace-nowrap">创建浏览器环境中, 稍等.</span>
  </Loading>
  <EncryptModel :open="encrypts.length > 0" title="密码本" @close="encrypts = []">
    <div class="grid grid-cols-9 mx-8 py-6 rounded-md max-h-[24rem] overflow-auto">
      <div class="col-span-2 border-b text-sm text-right py-1">环境名称</div>
      <div class="col-span-1 border-b text-sm text-right py-1"></div>
      <div class="col-span-6 border-b text-sm text-left py-1">密码</div>

      <template v-for="item in encrypts" :key="item.name">
        <div class="col-span-2 text-xs flex items-center h-8 justify-end border-b">
          {{ item.name }}
        </div>
        <div class="col-span-1 text-sm text-right border-b"></div>
        <div class="col-span-6 truncate overflow-hidden text-xs flex items-center h-8 justify-start border-b">
          {{ item.encrypt }}
        </div>
      </template>
    </div>

    <div class="border-t py-4 flex justify-end px-4">
      <button @click="downloadCSV(convertToCSV(encrypts))"
        class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]">
        下载
      </button>
    </div>
  </EncryptModel>
</template>
