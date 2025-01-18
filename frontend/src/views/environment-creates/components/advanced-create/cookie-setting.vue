<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import {
  useEnvironmentAdvancedFormStore,
  templaeFormtData as data,
} from "@/stores/form/environment-advanced";
import { Textarea } from "@/components/ui/textarea/index";

const forms = useEnvironmentAdvancedFormStore().forms;
</script>

<template>
  <Accordion type="single" class="w-full px-4" collapsible>
    <AccordionItem value="cookie-setting" class="border-0">
      <AccordionTrigger
        class="hover:no-underline rounded-md text-sm p-3 bg-gray-50 mb-2"
        >Cookie
      </AccordionTrigger>
      <AccordionContent class="px-10 space-y-4 py-1">
        <div className="flex">
          <div class="w-32">
            <p class="text-left text-sm">Cookie</p>
          </div>
          <div class="grow">
            <div class="w-full space-y-2">
              <button
                class="w-24 text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
              >
                导入Cookie
              </button>
              <Textarea
                rows="4"
                :placeholder="data.cookie.placeholder"
                class="w-1/2 px-3 py-2 border rounded-md placeholder-gray-400"
                v-model="forms.cookie"
                :value="forms.cookie"
              ></Textarea>
            </div>
          </div>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
