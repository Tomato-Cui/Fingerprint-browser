import { defineStore } from "pinia";
import { reactive, ref } from "vue";


export interface UpdateState {
    version: string,
    note: string,
}

export const useUpdateStore = defineStore('update-store', () => {
    const updateDialog = ref<boolean>(false);

    const state = reactive<UpdateState | any>({
        version: "",
        note: "",
    });

    const setVersion = (version: string, note: string) => {
        state.version = version;
        state.note = note;
    }

    const updateDialogOpen = () => updateDialog.value = true;
    const updateDialogClose = () => updateDialog.value = false;


    return {
        state, setVersion, updateDialog, updateDialogOpen, updateDialogClose
    }
})