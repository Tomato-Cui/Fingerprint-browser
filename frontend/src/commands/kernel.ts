import { invoke } from "@tauri-apps/api/core";
import { download } from '@tauri-apps/plugin-upload';


export const latest_kernel = async (): Promise<any> => {
    return await invoke('latest_kernel')
};


export const kernel_location = async (
    version: string,
): Promise<any> => {
    return await invoke('kernel_location', {
        version
    })
};


interface ProgressPayload {
    progress: number;
    progressTotal: number;
    total: number;
    transferSpeed: number;
}

export const install_location = async (
    url: string, location: string, envent: (progress: ProgressPayload) => void
): Promise<any> => {
    let map = new Map<string, string>();
    map.set("Content-Type", "application/zip");

    await download(url, location + ".zip", envent, map);
    await new Promise(resolve => setTimeout(resolve, 10000));

    return await invoke('unzip_kernel_zip', {
        location: location + ".zip", extractTo: location
    })
};