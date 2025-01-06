import { defineStore } from "pinia";
import { ref } from "vue";


export const useSwitchStore = defineStore('switch-store', () => {
    const switch_ = ref(false);


    const open = () => switch_.value = true

    const close = () => switch_.value = false

    const isCan = () => switch_.value


    return {
        switch_, open, close, isCan
    }
})