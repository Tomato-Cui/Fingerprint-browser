import { invoke } from '@tauri-apps/api/core'


export const environment_trash_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_trash_query_id', { id })
};

export const environment_trash_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_trash_query', { pageNum, pageSize })
};

export const environment_trash_recover = async (id: number): Promise<any> => {
    return await invoke('environment_trash_recover', { id })
};

export const environment_trash_recovers = async (environmentIds: number[]): Promise<any> => {
    return await invoke('environment_trash_recovers', { environmentIds })
};

export const environment_trash_recover_all = async (): Promise<any> => {
    return await invoke('environment_trash_recover_all', {})
};

export const environment_trash_delete_again = async (id: number): Promise<any> => {
    return await invoke('environment_trash_delete_again', { id })
};

export const environment_trash_clean = async (
): Promise<any> => {
    return await invoke('environment_trash_clean', {})
};