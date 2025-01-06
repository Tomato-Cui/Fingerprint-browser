import { invoke } from '@tauri-apps/api/core'

export const platform = async (): Promise<String> => {
    return await invoke('platfrom',)
}