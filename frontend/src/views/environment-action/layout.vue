<script setup>
import { useRoute, useRouter } from "vue-router";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ShuffleIcon, WaypointsIcon } from "lucide-vue-next";
import RightDetail from "./right-detail.vue";
import { useEnvironmentCreateFromStore } from "@/stores/form/environment-create";
import { useEnvironmentCreatesFromStore } from "@/stores/form/environment-creates";
import { useImportCenterFromStore } from "@/stores/form/import-center";
import {
  environment_create,
  environment_batch_create,
  environment_modify_info,
  environment_detail_create,
} from "@/commands/environment";
import { toast } from "vue-sonner";

const tabs = [
  { id: "single", name: "单个创建", href: "/environment-action/create" },
  { id: "batch", name: "批量创建", href: "/environment-action/creates" },
  { id: "import", name: "窗口导入", href: "/environment-action/imports" },
];

const route = useRoute();
const router = useRouter();

const environmentCreateFrom = useEnvironmentCreateFromStore();
const environmentCreatesFrom = useEnvironmentCreatesFromStore();
const importFrom = useImportCenterFromStore();
const onSubmit = () => {
  if (route.path === "/environment-action/create") {
    environmentCreateFrom.handleSubmit(async (values) => {
      try {
        let { name, description, browser, os, ua, kind } = values;
        const action = route.query.action;
        console.log("这是action:  " + action);
        if (action === "edit") {
          const apiResponse = route.query.environment
            ? JSON.parse(route.query.environment)
            : null;
          console.log("这是环境" + apiResponse.uuid);
          // 修改逻辑
          apiResponse.name = name;
          apiResponse.description = description;
          apiResponse.browser = browser;
          apiResponse.os = os;
          apiResponse.ua = ua;
          apiResponse.kind = kind;

          const EnvironmentInfo = {
            id: apiResponse.id, // 必填项
            uuid: apiResponse.uuid, // 必填项，确保 apiResponse 有 uuid
            user_uuid: apiResponse.user_uuid || "", // 可选项，给一个默认值 ''
            team_id: apiResponse.team_id || 0, // 可选项，给一个默认值 0
            proxy: apiResponse.proxy_id
              ? {
                  // proxy 部分是可选的，根据 apiResponse 来判断
                  id: apiResponse.proxy_id || 0,
                  kind: apiResponse.kind || "", // 默认为空字符串
                  host: apiResponse.proxy_host || "", // 默认为空字符串
                  port: apiResponse.proxy_port || "", // 默认为空字符串
                  username: apiResponse.username || "", // 默认为空字符串
                  password: apiResponse.password || "", // 默认为空字符串
                  user_uuid: apiResponse.user_uuid || "", // 默认为空字符串
                  environment_group_id: apiResponse.group_id || 0, // 默认为 null
                }
              : undefined, // 如果 proxy 部分是 null 或 undefined，就不给值
            fp_info: {
              id: apiResponse.fp_info_id || 0, // 默认为 0
              user_uuid: apiResponse.user_uuid || 0, // 确保为字符串类型，默认为空字符串
              browser: apiResponse.browser || "", // 默认为空字符串
              ua: apiResponse.ua || "", // 默认为空字符串
              os: apiResponse.os || "", // 默认为空字符串
              country: apiResponse.country || "", // 默认为空字符串
              region: apiResponse.region || "", // 默认为空字符串
              city: apiResponse.city || "", // 默认为空字符串
              language_type: apiResponse.language_type || 0, // 默认为 0
              languages: apiResponse.languages || "", // 默认为空字符串
              gmt: apiResponse.gmt || "", // 默认为空字符串
              geography: apiResponse.geography || "", // 默认为空字符串
              geo_tips: apiResponse.geo_tips || 0, // 默认为 0
              geo_rule: apiResponse.geo_rule || 0, // 默认为 0
              longitude: apiResponse.longitude || 0, // 默认为空字符串
              latitude: apiResponse.latitude || 0, // 默认为空字符串
              radius: apiResponse.radius || 0, // 默认为 0
              height: apiResponse.height || 0, // 默认为 0
              width: apiResponse.width || 0, // 默认为 0
              fonts_type: apiResponse.fonts_type || 0, // 默认为 0
              fonts: apiResponse.fonts || "", // 默认为空字符串
              font_fingerprint: apiResponse.font_fingerprint || 0, // 默认为 0
              web_rtc: apiResponse.web_rtc || 0, // 默认为 0
              web_rtc_local_ip: apiResponse.web_rtc_local_ip || "", // 默认为空字符串
              canvas: apiResponse.canvas || 0, // 默认为 0
              webgl: apiResponse.webgl || 0, // 默认为 0
              hardware_acceleration: apiResponse.hardware_acceleration || 0, // 默认为 0
              webgl_info: apiResponse.webgl_info || 0, // 默认为 0
              audio_context: apiResponse.audio_context || 0, // 默认为 0
              speech_voices: apiResponse.speech_voices || 0, // 默认为 0
              media: apiResponse.media || 0, // 默认为 0
              cpu: apiResponse.cpu || 0, // 默认为 0
              memory: apiResponse.memory || 0, // 默认为 0
              do_not_track: apiResponse.do_not_track || 0, // 默认为 0
              battery: apiResponse.battery || 0, // 默认为 0
              port_scan: apiResponse.port_scan || 0, // 默认为 0
              white_list: apiResponse.white_list || "", // 默认为空字符串
            },
            name: apiResponse.name + "" || "", // 必填项，确保 name 存在，默认为空字符串
            description: apiResponse.description || "", // 可选项，默认为空字符串
            default_urls: apiResponse.default_urls || "", // 可选项，默认为空字符串
            proxy_enable: apiResponse.proxy_enable || 0, // 必填项，默认为 0
          };

          let res = await environment_modify_info(
            apiResponse.uuid,
            EnvironmentInfo
          );
          toast.success("环境信息更新成功：" + res.message);
        } else {
          // 创建逻辑
          let res = await environment_create(name);
          toast.success("环境创建成功：" + res.message);
        }
      } catch (error) {
        toast.warning("创建失败:" + error);
      }
    })();
  } else if (route.path === "/environment-action/creates") {
    environmentCreatesFrom.handleSubmit(async (values) => {
      try {
        let { name, numbers } = values;
        let names = [...new Array(numbers)].map(
          (_, index) => name + "-" + (index + 1)
        );

        let res = await environment_batch_create(names);
        toast.success(res.message);
      } catch (error) {
        toast.warning("创建失败:" + error);
      }
    })();
  } else {
    importFrom.handleSubmit(async (values) => {
      try {
        let { file } = values;
        // 遍历每一行并将数据映射到 EnvironmentInfo 类型
        const environmentInfoList = file.map((row, index) => {
          return {
            id: row.id, // 必填项
            uuid: row.uuid, // 必填项
            user_uuid: row.user_uuid || "", // 可选项，默认为 ''
            team_id: row.team_id || 0, // 可选项，默认为 0
            proxy: row["proxy.id"]
              ? {
                  id: row["proxy.id"] || 0,
                  kind: row["proxy.kind"] || "",
                  host: row["proxy.host"] || "",
                  port: row["proxy.port"] || "",
                  username: row["proxy.username"] || "",
                  password: row["proxy.password"] || "",
                  user_uuid: row["proxy.user_uuid"] || "",
                  environment_group_id: row["proxy.environment_group_id"] || 0,
                }
              : undefined, // 如果 proxy 部分为 null 或 undefined，就不给值
            fp_info: {
              id: row["fp_info.id"] || 0,
              user_uuid: row["fp_info.user_uuid"] || "",
              browser: row["fp_info.browser"] || "",
              ua: row["fp_info.ua"] || "",
              os: row["fp_info.os"] || "",
              country: row["fp_info.country"] || "",
              region: row["fp_info.region"] || "",
              city: row["fp_info.city"] || "",
              language_type: row["fp_info.language_type"] || 0,
              languages: row["fp_info.languages"] || "",
              gmt: row["fp_info.gmt"] || "",
              geography: row["fp_info.geography"] || "",
              geo_tips: row["fp_info.geo_tips"] || 0,
              geo_rule: row["fp_info.geo_rule"] || 0,
              longitude: row["fp_info.longitude"] || 0,
              latitude: row["fp_info.latitude"] || 0,
              radius: row["fp_info.radius"] || 0,
              height: row["fp_info.height"] || 0,
              width: row["fp_info.width"] || 0,
              fonts_type: row["fp_info.fonts_type"] || 0,
              fonts: row["fp_info.fonts"] || "",
              font_fingerprint: row["fp_info.font_fingerprint"] || 0,
              web_rtc: row["fp_info.web_rtc"] || 0,
              web_rtc_local_ip: row["fp_info.web_rtc_local_ip"] || "",
              canvas: row["fp_info.canvas"] || 0,
              webgl: row["fp_info.webgl"] || 0,
              hardware_acceleration: row["fp_info.hardware_acceleration"] || 0,
              webgl_info: row["fp_info.webgl_info"] || 0,
              audio_context: row["fp_info.audio_context"] || 0,
              speech_voices: row["fp_info.speech_voices"] || 0,
              media: row["fp_info.media"] || 0,
              cpu: row["fp_info.cpu"] || 0,
              memory: row["fp_info.memory"] || 0,
              do_not_track: row["fp_info.do_not_track"] || 0,
              battery: row["fp_info.battery"] || 0,
              port_scan: row["fp_info.port_scan"] || 0,
              white_list: row["fp_info.white_list"] || "",
            },
            name: row.name || "", // 必填项
            description: row.description || "", // 可选项
            default_urls: row.default_urls || "", // 可选项
            proxy_enable: row.proxy_enable || 0, // 必填项
          };
        });

        environmentInfoList.forEach((info, index) => {
          console.log(info);
          environment_detail_create(info);
        });

        toast.success("成功传入,创建成功");
      } catch (error) {
        toast.warning("创建失败:" + error);
      }
    })();
  }
};
</script>
<template>
  <div class="h-main bg-white m-4 flex flex-col rounded-md">
    <div class="flex border-b mb-2 justify-between pr-4">
      <nav class="flex">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="() => router.push(tab.href)"
          class="px-6 py-3 -mb-px font-medium"
          :class="
            route.path === tab.href
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-600'
          "
        >
          {{ tab.name }}
        </button>
      </nav>
    </div>

    <form
      class="flex flex-col overflow-hidden gap-x-2 flex-auto"
      @submit.prevent="onSubmit"
    >
      <div class="flex overflow-hidden gap-x-2 flex-auto">
        <div class="overflow-y-auto flex-auto">
          <router-view />
        </div>

        <RightDetail />
      </div>
      <div class="flex gap-x-4 pt-3 px-6 mb-8 bg-white border-t">
        <CancelButton @click="router.go(-1)">取消</CancelButton>
        <PrimaryButton type="submit">确定</PrimaryButton>
      </div>
    </form>
  </div>
</template>
