<script setup lang="ts">
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
import {
  useEnvironmentAdvancedFormStore,
  templaeFormtData as data,
} from "@/stores/form/environment-advanced";
import { ref, watch } from "vue";

const currentPlatform = ref<string>("");
const forms = useEnvironmentAdvancedFormStore().forms;

watch(currentPlatform, (newV) => {
  let current = data.platforms.find((item) => item.value == newV);
  forms.account.platform = current?.value || "";
  forms.account.platform_url = current?.href || "";
});
</script>

<template>
  <Accordion type="single" class="w-full px-4" collapsible>
    <AccordionItem value="account-setting" class="border-0">
      <AccordionTrigger
        class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
        >账号平台
      </AccordionTrigger>
      <AccordionContent class="px-10 space-y-4 py-1">
        <div className="flex items-center">
          <p class="w-32 text-left text-sm">账号平台</p>
          <Select v-model="currentPlatform">
            <SelectTrigger class="w-1/3">
              <SelectValue
                placeholder="无"
                class="w-full p-2 rounded-lg outline-none"
              />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem v-for="item in data.platforms" :value="item.value">
                  <div class="flex justify-start items-center gap-x-2">
                    <img :src="item.avatar" class="w-4 h-4" />
                    {{ item.value }}
                  </div>
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>

        <div className="flex items-center">
          <p class="w-32 text-left text-sm relative">
            <span class="text-red-500 absolute -left-2">*</span>用户名
          </p>
          <Input v-model="forms.account.platform_account" class="w-1/2" />
        </div>

        <div className="flex items-center">
          <p class="w-32 text-left text-sm relative">
            <span class="text-red-500 absolute -left-2">*</span>密码
          </p>
          <Input v-model="forms.account.platform_password" class="w-1/2" />
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
