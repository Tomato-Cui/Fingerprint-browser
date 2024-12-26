import { invoke } from '@tauri-apps/api/core'

export const browser_start = async (environmentUuid: string): Promise<any> => {
    return await invoke('browser_start', { environmentUuid })
};

export const browser_starts = async (environmentUuids: Array<String>): Promise<any> => {
    return await invoke('browser_starts', { environmentUuids })
};

export const browser_stops = async (environmentUuids: Array<string>): Promise<any> => {
    return await invoke('browser_stops', { environmentUuids })
};

export const browser_status = async (): Promise<any> => {
    return await invoke('browser_status')
};