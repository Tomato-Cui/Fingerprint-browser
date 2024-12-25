import { invoke } from '@tauri-apps/api/core'

export const environment_transfer_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_query_id', { id })
};

export const environment_transfer_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query', { pageNum, pageSize })
};

export const environment_transfer_delete = async (id: number): Promise<any> => {
    return await invoke('environment_delete', { id })
};

export const environment_transfer_batch_delete = async (ids: Array<number>): Promise<any> => {
    return await invoke('environment_batch_delete', { ids })
};
