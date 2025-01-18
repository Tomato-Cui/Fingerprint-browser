<script setup lang="ts">
import { ref } from "vue";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import Input from "@/components/input.vue";
import { useEnvironmentAdvancedFormStore } from "@/stores/form/environment-advanced";
import { ip_info } from "@/commands";
import { Model } from "@/components/model/index";
import Loading from "@/components/loading/index.vue";
import { toast } from "vue-sonner";

const forms = useEnvironmentAdvancedFormStore().forms;

const useProxyOpen = ref(false);
const proxyTypes = ["NO Proxy (本地直连)", "HTTP", "HTTPS", "SOCKS5", "SSH"];
const loading = ref(false);
const checkOk = ref(false);

const checkHandle = async () => {
  let { kind, host, port, username, password } = forms.proxy;
  if (kind == "NO Proxy (本地直连)") {
    kind = "no";
  }
  let timer = setTimeout(() => {
    loading.value = true;
  }, 300);

  ip_info(kind, host, port, username, password)
    .then((res) => {
      if (res.code != 1) {
        toast.warning(res.message);
        checkOk.value = false;
      } else {
        checkOk.value = true;
        toast.success("检测通过");
      }
    })
    .catch((res) => {
      checkOk.value = false;
      toast.warning(res);
    })
    .finally(() => {
      clearTimeout(timer);
      loading.value = false;
    });
};
</script>

<template>
  <Accordion type="single" class="w-full px-4" collapsible>
    <AccordionItem class="border-0" value="proxy-setting">
      <AccordionTrigger
        class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
        >代理信息</AccordionTrigger
      >
      <AccordionContent class="px-10 space-y-4 py-1">
        <div className="flex items-center">
          <p class="w-32 text-left text-sm">代理方式</p>
          <div
            class="flex gap-x-2 border-[1.4px] border-[#C5CEE0] p-1 rounded-lg"
          >
            <div
              class="flex items-center space-x-1 py-1.5 px-2 rounded-lg cursor-pointer bg-[#5050FA] text-white fill-white"
            >
              <span>新代理</span>
            </div>
            <div
              class="flex items-center space-x-1 py-1.5 px-2 rounded-lg cursor-pointer"
              @click="useProxyOpen = true"
            >
              <span>使用现有代理</span>
            </div>
          </div>
        </div>
        <div className="flex items-center">
          <p class="w-32 text-left text-sm">代理类型</p>
          <Select :default-value="forms.proxy.kind" v-model="forms.proxy.kind">
            <SelectTrigger class="w-1/3">
              <SelectValue class="w-full p-2 rounded-lg outline-none" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem :value="item" v-for="item in proxyTypes">
                  {{ item }}
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>

        <div
          className="flex flex-col space-y-4"
          v-if="forms.proxy.kind != 'NO Proxy (本地直连)'"
        >
          <div className="flex items-center">
            <p class="w-32 text-left text-sm relative">
              <span class="text-red-500 absolute -left-2">*</span>代理主机
            </p>
            <Input v-model="forms.proxy.host" class="w-1/2" />
          </div>
          <div className="flex items-center">
            <p class="w-32 text-left text-sm relative">
              <span class="text-red-500 absolute -left-2">*</span>
              代理端口
            </p>
            <Input v-model="forms.proxy.port" class="w-1/2" />
          </div>
          <div className="flex items-center">
            <p class="w-32 text-left text-sm">代理账号</p>
            <Input v-model="forms.proxy.username" class="w-1/2" />
          </div>
          <div className="flex items-center">
            <p class="w-32 text-left text-sm">代理密码</p>
            <Input v-model="forms.proxy.password" class="w-1/2" />
          </div>
        </div>

        <div class="flex">
          <div class="flex items-start justify-start">
            <p class="w-32 text-left text-sm">IP查询渠道</p>
          </div>
          <div class="w-1/2 space-y-2">
            <Select>
              <SelectTrigger class="w-2/3" disabled>
                <SelectValue
                  placeholder="IP2Location"
                  class="w-full p-2 rounded-lg outline-none"
                />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="IP2Location"> IP2Location </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <button
              class="text-sm border whitespace-nowrap rounded-md px-2 py-2 flex justify-center items-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
              @click="checkHandle"
            >
              {{ checkOk ? "网络检测成功" : "网络检测" }}
            </button>
          </div>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
  <Model :open="useProxyOpen" @close="useProxyOpen = false"> </Model>
  <Loading :open="loading" />
</template>
