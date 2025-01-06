import { toast } from 'vue-sonner'
import { fetch_update, install_update } from '@/commands/update'
import { useLatestReleaseStore } from '@/stores/latest-release'

export const update = async (isQuiet: boolean = true) => {
    fetch_update().then(_res => {
        let canUpdate = false;
        const latestRelease = useLatestReleaseStore();
        if (canUpdate) {
            install_update().then(_res => {
                latestRelease.latestState.is_get_lateset = true;
                latestRelease.latestState.is_download_latest = true;
                latestRelease.latestState.is_latest = true;
            }).catch(_err => {
                toast.error('无法连接服务器，请重新进入或者下载最新的软件')
            })
        } else {
            latestRelease.latestState.is_get_lateset = true;
            latestRelease.latestState.is_download_latest = true;
            latestRelease.latestState.is_latest = true;
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