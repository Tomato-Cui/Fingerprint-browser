import { toast } from 'vue-sonner'
import { fetch_update, install_update } from '@/commands/update'
import { useSwitchStore } from '@/stores/switch'

export const update = async (isQuiet: boolean = true) => {
    fetch_update().then(_res => {
        let canUpdate = false;
        if (canUpdate) {
            install_update().then(_res => {
                useSwitchStore().open();
                console.log(_res)
            }).catch(_err => {
                toast.error('无法连接服务器，请重新下载最新的软件')
            })
        } else {
            useSwitchStore().open();
        }
    }).catch(_ => {
        if (!isQuiet)
            toast.error('无法连接服务器，请重新下载最新的软件')
    });
}


export default {
    install: () => {
        update(false)
    },
}