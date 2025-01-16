<script setup lang="ts">
import Model from "@/components/model/model.vue";
import { reactive, ref, watch } from "vue";
import { PrimaryButton, CancelButton } from "@/components/button";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import Textarea from "@/components/ui/textarea/Textarea.vue";

const props = defineProps({
  createBookmark: {
    type: Boolean,
    default: false,
  },
});

const environmentUuid = ref("");

const text = ref(""); // 用于存储textarea的内容

const maxLength = 25000;

watch(text, (newValue) => {
  if (newValue.length > maxLength) {
    text.value = newValue.slice(0, maxLength); // 超过最大长度时截取文本
  }
});

//确认
const subMit = () => {
  console.log("nihao");
  emit("update:createBookmark", false);
  text.value = "";
};

const emit = defineEmits(["update:createBookmark"]);

const addBookmark = reactive({
  bookmarkName: "",
  description: "",
});
</script>

<template>
  <Model
    class=""
    title="添加书签"
    :open="props.createBookmark"
    @close="() => emit('update:createBookmark', false)"
  >
    <div class="flex flex-col gap-y-4 py-2 mb-2">
      <div class="flex gap-4 items-center px-9">
        <div>
          <label
            class="w-[50px] flex justify-end fl font-semibold text-[14px] leading-[22px]"
            >书签</label
          >
        </div>
        <div class="w-full">
          <Select>
            <SelectTrigger>
              <SelectValue
                v-model="environmentUuid"
                placeholder="所有环境"
                class="p-2 w-full rounded-lg outline-none"
              />
            </SelectTrigger>

            <SelectContent>
              <SelectGroup>
                <SelectItem value="SOCKS5"> SOCKS5 </SelectItem>
                <SelectItem value="SOCKS52"> SOCKS52 </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="flex gap-4 items-center px-9">
        <div>
          <label
            class="w-[50px] flex justify-end fl text-white font-semibold text-[14px] leading-[22px]"
            >&nbsp;&nbsp;</label
          >
        </div>
        <div class="w-full">
          <Textarea
            v-model="text"
            placeholder="NAME::URL&#10;可换行添加多个书签(书签名最多500字符;URL最多2000字符)"
            :maxlength="maxLength"
            class="font-sans font-normal text-[14px] placeholder:text-[12px] leading-[22px] flex-1 px-4 py-1 w-full text-lg rounded-lg border border-gray-300 h-[150px]"
          />
          <div>
            <p class="font-sans font-normal text-[12px] flex justify-end mt-2">
              <span class="text-gray-500"
                >{{ text.length }}/{{ maxLength }}</span
              >
            </p>
          </div>
        </div>
      </div>

      <div class="flex justify-end border-t border-gray-300">
        <div class="flex gap-x-4 justify-end px-2 pt-3">
          <CancelButton
            class="px-8 border border-gray-300"
            @click="() => emit('update:createBookmark', false)"
          >
            取消
          </CancelButton>

          <PrimaryButton
            class="px-8 !bg-[#5050FA] border border-gray-300"
            @click="subMit"
          >
            确定
          </PrimaryButton>
        </div>
      </div>
    </div>
  </Model>
</template>
