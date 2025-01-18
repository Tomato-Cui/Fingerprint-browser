<script setup lang="ts">
import { onMounted } from "vue";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import Input from "@/components/input.vue";
import {
  NumberField,
  NumberFieldContent,
  NumberFieldInput,
} from "@/components/ui/number-field";
import Radio from "../radio.vue";
import { Iconrefresh } from "@/assets/icons";
import { Textarea } from "@/components/ui/textarea";
import {
  templaeFormtData as data,
  useEnvironmentAdvancedFormStore,
} from "@/stores/form/environment-advanced";
import { environment_group_query } from "@/commands/environment-group";
import { environment_tag_query } from "@/commands/environment-tag";
import { ChevronDownIcon, ChevronUpIcon } from "lucide-vue-next";

const numberBound = { min: 1, max: 19 };
const { forms, randomUA } = useEnvironmentAdvancedFormStore();

onMounted(() => {
  environment_group_query(1, 100).then((res) => {
    if (res.code == 1) {
      let tempGroups = [
        ...data.groups,
        ...res.data.data.map((item: any) => ({
          id: item.id,
          value: item.name,
        })),
      ];
      data.groups = Array.from(
        new Map(tempGroups.map((item) => [item.id, item])).values()
      );
    }
  });
  environment_tag_query(1, 100).then((res) => {
    if (res.code == 1) {
      let tempTags = [
        ...data.tags,
        ...res.data.data.map((item: any) => ({
          id: item.id,
          value: item.name,
        })),
      ];

      data.tags = Array.from(
        new Map(tempTags.map((item) => [item.id, item])).values()
      );
    }
  });
});

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
</script>

<template>
  <Accordion
    type="single"
    class="w-full px-4"
    collapsible
    default-value="basic-setting"
  >
    <AccordionItem class="border-0" value="basic-setting">
      <AccordionTrigger
        class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
        >基础设置</AccordionTrigger
      >
      <AccordionContent class="px-10 space-y-4 py-4">
        <div class="flex items-center">
          <p class="w-32 text-left">名称</p>
          <div class="relative text-sm w-1/2 flex flex-col">
            <Input
              v-model="forms.name"
              type="text"
              :placeholder="data.name.placeholder"
              maxlength="20"
            />
          </div>
        </div>
        <div class="flex items-center">
          <p class="w-32 text-left">浏览器</p>
          <Radio
            :check="forms.browser"
            :data="data.browserTypes"
            @update:check="(v:any) => (forms.browser = {value: v.value})"
          />
        </div>

        <div class="space-y-2 flex items-center">
          <p class="w-32 text-left">操作系统</p>
          <Radio
            :check="forms.os"
            :data="data.osTypes"
            @update:check="(v:any) => (forms.os = {value: v.value})"
          />
        </div>

        <div class="flex">
          <div class="flex items-start justify-start">
            <p class="w-32 text-left text-sm">UA</p>
          </div>
          <div class="w-full relative">
            <Textarea class="w-full" :placeholder="forms.ua" disabled />
            <div class="absolute right-2 top-2 flex gap-2" @click="randomUA">
              <Iconrefresh class="w-4 h-4 fill-gray-800 cursor-pointer" />
            </div>
          </div>
        </div>

        <div className="flex flex-col">
          <div class="flex">
            <p class="w-32 text-left text-sm">端对端加密</p>
            <Radio
              :check="forms.encryPtSwitch"
              :data="data.encrypts"
              @update:check="(v) => (forms.encryPtSwitch = v)"
            />
          </div>
          <div className="text-muted-foreground pl-32 pt-2 text-xs">
            开启后，只有通过密钥授权的账号和设备可以解密和访问此环境
          </div>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-left text-sm">环境分组</p>
          <div class="w-1/2">
            <Select
              class="flex"
              v-model="forms.group"
              :default-value="forms.group"
            >
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

        <div className="flex items-center">
          <p class="w-32 text-left text-sm">环境标签</p>
          <div class="w-1/2">
            <Select class="flex" v-model="forms.tag" :default-value="forms.tag">
              <SelectTrigger
                className="w-full rounded-md flex p-2 border-[1.4px] border-[#C5CEE0] items-center justify-between"
              >
                <SelectValue placeholder="选择或新建标签" class="text-sm" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem v-for="item in data.tags" :value="item.id">{{
                  item.value
                }}</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>

        <div class="flex items-center">
          <p class="w-32 text-left">备注</p>
          <div class="relative text-sm w-2/3 pt-1">
            <Input
              v-model="forms.description"
              type="text"
              :placeholder="data.description.placeholder"
              class="w-full text-gray-600 px-[0.45rem]"
              maxlength="28"
            />
            <span class="absolute right-3 top-2 text-xs text-gray-400"
              >{{ forms.description.length || 0 }}/28</span
            >
          </div>
        </div>

        <div className="flex items-center">
          <Label class="w-32 text-left">创建环境数</Label>
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
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
