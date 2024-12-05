import { defineStore } from "pinia";

export const useStateStore = defineStore('MyButton', {
  // 其他配置项，后面逐一说明
  state: () => ({
    isActive:true,
  })
})