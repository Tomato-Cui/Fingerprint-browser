<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Switch } from "@/components/ui/switch";
import { useEnvironmentAdvancedFormStore } from "@/stores/form/environment-advanced";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";

const forms = useEnvironmentAdvancedFormStore().forms;
</script>

<template>
  <Accordion type="single" class="w-full px-4" collapsible>
    <AccordionItem value="startup-page-setting" class="border-0">
      <AccordionTrigger
        class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
        >浏览器启动页
      </AccordionTrigger>
      <AccordionContent class="px-10 space-y-4 py-1">
        <div className="flex">
          <div class="w-32">
            <p class="text-left text-sm">启动后</p>
          </div>
          <div class="grow">
            <RadioGroup
              :defaultValue="forms.setting.startedBrowserDefaultUrl"
              v-model="forms.setting.startedBrowserDefaultUrl"
            >
              <div className="flex items-center space-x-2">
                <RadioGroupItem
                  value="0"
                  id="option-one"
                  class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]"
                />
                <Label htmlFor="option-one">继续浏览上次打开的网页</Label>
              </div>
              <div className="flex items-center space-x-2">
                <RadioGroupItem
                  value="1"
                  id="option-two"
                  class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]"
                />
                <Label htmlFor="option-two">打开指定网页</Label>
              </div>
            </RadioGroup>
          </div>
        </div>
        <div className="flex">
          <div class="w-32">
            <p class="text-left text-sm">其他设置</p>
          </div>
          <div class="grow flex flex-col gap-y-1">
            <div class="flex gap-x-2">
              <Switch
                :checked="forms.setting.startedBrowserAccountUrlIsOpen"
                @update:checked="
                  forms.setting.startedBrowserAccountUrlIsOpen =
                    !forms.setting.startedBrowserAccountUrlIsOpen
                "
                class="data-[state=checked]:bg-[#5050FA]"
              />
              <p>同时打开「账号」已配置的平台网页</p>
            </div>
            <div class="flex gap-x-2">
              <Switch
                class="data-[state=checked]:bg-[#5050FA]"
                :checked="forms.setting.startedBrowserStayCheckHome"
                @update:checked="
                  forms.setting.startedBrowserStayCheckHome =
                    !forms.setting.startedBrowserStayCheckHome
                "
              />
              <p>打开后默认停留在检测页</p>
            </div>
          </div>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
