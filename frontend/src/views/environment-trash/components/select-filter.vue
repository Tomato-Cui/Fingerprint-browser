<script setup lang="ts">
import {
  IconFilter,
  IconRefreshSquare,
} from "@/assets/icons/environment-trash/index.ts";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import { Button } from "@/components/ui/button";
import TooltipButton from "@/components/tooltip-button.vue";
import { computed } from "vue";

const props = defineProps<{
  data: any;
  selects: any;
}>();

const emits = defineEmits<{
  "update:onProxySelect": [boolean];
  "update:onAccountSelect": [string];
  "update:onTagSelect": [string];
  "update:onGroupSelect": [string];
  "update:reset": [];
}>();

const accounts = computed(() => {
  return [
    ...new Set(
      props.data
        .filter((item: any) => item.account != "--")
        .map((item: any) => item.account)
    ),
  ];
}) as any;
const tags = computed(() => {
  return [
    ...new Set(
      props.data
        .filter((item: any) => item.tag != "--")
        .map((item: any) => item.tag)
    ),
  ];
}) as any;
const groups = computed(() => {
  return [
    ...new Set(
      props.data
        .filter((item: any) => item.group != "--")
        .map((item: any) => item.group)
    ),
  ];
}) as any;
</script>
<template>
  <Popover>
    <PopoverTrigger>
      <Button
        variant="outline"
        class="h-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]"
      >
        <component :is="IconFilter" />
        筛选
      </Button>
    </PopoverTrigger>
    <PopoverContent align="end" class="p-4">
      <div class="flex justify-between">
        <span>筛选</span>
        <TooltipButton title="重置">
          <IconRefreshSquare
            class="cursor-pointer"
            @click="() => emits('update:reset')"
          />
        </TooltipButton>
      </div>
      <div class="pt-4 space-y-2">
        <Select
          :default-value="selects.proxy ? '1' : '0'"
          @update:model-value="(v) => emits('update:onProxySelect', v == '1')"
        >
          <SelectTrigger class="h-8 select-none">
            <SelectValue
              placeholder="绑定代理情况"
              class="px-0 w-full rounded-lg outline-none text-xs"
            />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem
                v-for="item in [
                  { title: '已绑定', value: '1' },
                  { title: '未绑定', value: '0' },
                ]"
                :key="item.value"
                :value="item.value"
                >{{ item.title }}</SelectItem
              >
            </SelectGroup>
          </SelectContent>
        </Select>
        <Select
          :disabled="!(accounts.length > 0)"
          :default-value="selects.account"
          @update:model-value="(v) => emits('update:onAccountSelect', v)"
        >
          <SelectTrigger class="h-8 select-none">
            <SelectValue
              placeholder="账号平台"
              class="px-0 w-full rounded-lg outline-none text-xs"
            />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem :key="item" :value="item" v-for="item in accounts">{{
                item
              }}</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
        <Select
          :disabled="!(tags.length > 0)"
          :default-value="selects.tag"
          @update:model-value="(v) => emits('update:onTagSelect', v)"
        >
          <SelectTrigger class="h-8 select-none">
            <SelectValue
              placeholder="请选择标签"
              class="px-0 w-full rounded-lg outline-none text-xs"
            />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem :key="item" :value="item" v-for="item in tags">{{
                item
              }}</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
        <Select
          :disabled="!(groups.length > 0)"
          :default-value="selects.group"
          @update:model-value="(v) => emits('update:onGroupSelect', v)"
        >
          <SelectTrigger class="h-8 select-none">
            <SelectValue
              placeholder="请选择分组"
              class="px-0 w-full rounded-lg outline-none text-xs"
            />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem :key="item" :value="item" v-for="item in groups">{{
                item
              }}</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
    </PopoverContent>
  </Popover>
</template>
