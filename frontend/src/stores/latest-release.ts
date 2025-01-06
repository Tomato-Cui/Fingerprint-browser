import { defineStore } from "pinia";
import { reactive } from "vue";

type LatestStateProps = {
    is_latest: boolean,
    is_download_latest: boolean,
    is_get_lateset: boolean,
};

export const useLatestReleaseStore = defineStore('latest-release-store', () => {
    let latestState = reactive<LatestStateProps>({
        is_latest: false,
        is_download_latest: false,
        is_get_lateset: false,
    });


    return {
        latestState
    }
})