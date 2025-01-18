<script setup lang="ts">
import { CardFooter, CardContent } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select/index";
import { Label } from "@/components/ui/label";
import {
  IconChrome,
  IconFrame,
  IconFrame1,
  IconFrame2,
  IconFrame3,
  IconFrame4,
} from "@/assets/icons/environment-creates/simple-create/index";
import {
  NumberField,
  NumberFieldContent,
  NumberFieldInput,
} from "@/components/ui/number-field";
import { ChevronDownIcon, ChevronUpIcon } from "lucide-vue-next";
import Radio, { type RadioInterface } from "../components/radio.vue";
import EncryptModel from "../components/encrypt-model.vue";
import { onMounted, onUnmounted, reactive, ref } from "vue";
import { useRouter } from "vue-router";
import Loading from "@/components/loading/index.vue";
import LoadingIcon from "@/components/loading.vue";
import { environment_simple_create } from "@/commands/environment";
import { environment_group_query } from "@/commands/environment-group";
import { toast } from "vue-sonner";
import { convertToCSV, downloadCSV } from "@/util/lib";

const router = useRouter();
const data = {
  browserTypes: [
    { value: "chrome", icon: IconChrome },
    { value: "firefox", icon: IconFrame },
  ] as RadioInterface[],
  osTypes: [
    { value: "Windows", icon: IconFrame1 },
    { value: "MacOs", icon: IconFrame2 },
    { value: "Android", icon: IconFrame3 },
    { value: "IOS", icon: IconFrame4 },
  ] as RadioInterface[],
  encrypts: [{ value: "开启" }, { value: "关闭" }] as RadioInterface[],
  groups: [{ id: 0, value: "未知分组" }] as any[],
};
const numberBound = { min: 1, max: 19 };

const forms = reactive({
  browserType: ref<RadioInterface>({
    value: "chrome",
  }),
  osType: ref<RadioInterface>({
    value: "Windows",
  }),
  encryPtSwitch: ref<RadioInterface>({
    value: "关闭",
  }),
  numbers: ref<number>(1),
  group: ref<any>(0),
});
const createing = ref(false);
const encrypts = ref<any[]>([]);

const handleInput = (value: number) => {
  if (numberBound.min !== undefined && value < numberBound.min) {
    forms.numbers = numberBound.min;
    return;
  }
  if (numberBound.max !== undefined && value > numberBound.max) {
    forms.numbers = numberBound.max;
    return;
  }
  forms.numbers = value;
};

const enterHandle = (e: KeyboardEvent) => {
  if (e.key === "Enter") {
    handleSubmit();
    e.preventDefault();
    return;
  }
};
onMounted(() => {
  document.addEventListener("keydown", enterHandle);

  environment_group_query(1, 100).then((res) => {
    if (res.code == 1) {
      data.groups = [
        ...data.groups,
        ...res.data.data.map((item: any) => ({
          id: item.id,
          value: item.name,
        })),
      ];
    }
  });
});
onUnmounted(() => {
  document.removeEventListener("keydown", enterHandle);
});
const handleSubmit = async () => {
  let { browserType, osType, encryPtSwitch, numbers, group } = forms;
  group = group ? group : undefined;
  let encryPt = encryPtSwitch.value == "开启";

  const loadingTimeout = setTimeout(() => {
    createing.value = true;
  }, 1000);

  environment_simple_create(
    browserType.value,
    osType.value,
    Number(numbers),
    encryPt,
    group
  )
    .then((res) => {
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
};
</script>
<template>
  <CardContent className="p-4 grow space-y-4 select-none">
    <div className="flex items-center">
      <Label class="w-48 text-left">浏览器</Label>
      <Radio
        :check="forms.browserType"
        :data="data.browserTypes"
        @update:check="(v) => (forms.browserType = v)"
      />
    </div>

    <div className="flex items-center">
      <Label class="w-48 text-left">操作系统</Label>

      <Radio
        :check="forms.osType"
        :data="data.osTypes"
        @update:check="(v) => (forms.osType = v)"
      />
    </div>

    <div className="flex items-center">
      <Label class="w-48 text-left">创建环境数</Label>
      <NumberField
        id="numbers"
        :min="numberBound.min"
        :max="numberBound.max"
        :default-value="forms.numbers"
        class="w-28 number-group"
      >
        <NumberFieldContent
          class="group flex border-[1.4px] border-[#C5CEE0] rounded-lg h-9 overflow-hidden focus-within:border-blue-500 focus-within:ring-2 ring-blue-200 transition-all ease-in-out"
        >
          <NumberFieldInput
            class="outline-none ring-none focus:outline-none focus-visible:ring-0 focus-visible:outline-0 focus-visible:ring-offset-0 border-none h-full text-xs"
            :value="forms.numbers"
            v-model="forms.numbers"
            @input="(v) => handleInput(v.target.value)"
          />
          <div class="border-l flex flex-col w-1/3">
            <div
              class="flex items-center flex-1 justify-center cursor-pointer"
              @click="forms.numbers < numberBound.max && ++forms.numbers"
            >
              <ChevronUpIcon class="text-[#C5CEE0] size-4" />
            </div>
            <div
              class="flex items-center border-t flex-1 justify-center cursor-pointer"
              @click="forms.numbers > numberBound.min && --forms.numbers"
            >
              <ChevronDownIcon class="text-[#C5CEE0] size-4" />
            </div>
          </div>
        </NumberFieldContent>
      </NumberField>
    </div>

    <div className="flex items-center">
      <p class="w-48 text-left text-sm">环境分组</p>
      <div class="w-[30rem]">
        <Select class="flex" v-model="forms.group" :default-value="forms.group">
          <SelectTrigger
            className="w-full rounded-md flex p-2 border-[1.4px] border-[#C5CEE0] items-center justify-between"
          >
            <SelectValue placeholder="选择或新建分组" class="text-sm" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="item in data.groups" :value="item.id">{{
              item.value
            }}</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>

    <div className="flex flex-col">
      <div class="flex">
        <p class="w-48 text-left text-sm">端对端密码</p>

        <Radio
          :check="forms.encryPtSwitch"
          :data="data.encrypts"
          @update:check="(v) => (forms.encryPtSwitch = v)"
        />
      </div>
      <div className="text-muted-foreground pl-48 pt-2 text-xs">
        开启后，只有通过密钥授权的账号和设备可以解密和访问此环境
      </div>
    </div>
  </CardContent>
  <CardFooter class="border-t flex gap-x-4 justify-end items-center p-3">
    <button
      @click="router.go(-1)"
      class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 active:scale-[.98]"
    >
      取消
    </button>
    <button
      @click="handleSubmit"
      class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
    >
      确认
    </button>
  </CardFooter>
  <Loading :open="createing">
    <LoadingIcon />
    <span class="text-xs whitespace-nowrap">创建浏览器环境中, 稍等.</span>
  </Loading>
  <EncryptModel
    :open="encrypts.length > 0"
    title="密码本"
    @close="encrypts = []"
  >
    <div
      class="grid grid-cols-9 mx-8 py-6 rounded-md max-h-[24rem] overflow-auto"
    >
      <div class="col-span-2 border-b text-sm text-right py-1">环境名称</div>
      <div class="col-span-1 border-b text-sm text-right py-1"></div>
      <div class="col-span-6 border-b text-sm text-left py-1">密码</div>

      <template v-for="item in encrypts" :key="item.name">
        <div
          class="col-span-2 text-xs flex items-center h-8 justify-end border-b"
        >
          {{ item.name }}
        </div>
        <div class="col-span-1 text-sm text-right border-b"></div>
        <div
          class="col-span-6 truncate overflow-hidden text-xs flex items-center h-8 justify-start border-b"
        >
          {{ item.encrypt }}
        </div>
      </template>
    </div>

    <div class="border-t py-4 flex justify-end px-4">
      <button
        @click="downloadCSV(convertToCSV(encrypts))"
        class="w-28 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
      >
        下载
      </button>
    </div>
  </EncryptModel>
</template>
