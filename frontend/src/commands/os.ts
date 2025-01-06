import { invoke } from '@tauri-apps/api/core'

export const fetch_update = async (): Promise<String> => {
    return await invoke('fetch_update',)
};
export const install_update = async (): Promise<String> => {
    return await invoke('install_update',)
};