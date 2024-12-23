import { invoke } from '@tauri-apps/api/core'

export const browser_start = async (environmentId: number, groupId?: number,): Promise<any> => {
    return await invoke('browser_start', { groupId, environmentId })
};

export const browser_starts = async (payload: Array<{ environment_id: number, group_id?: number }>): Promise<any> => {
    return await invoke('browser_starts', { payload })
};

export const browser_stops = async (ids: Array<number>): Promise<any> => {
    return await invoke('browser_stops', { ids })
};

export const browser_status = async (): Promise<any> => {
    return await invoke('browser_status')
};