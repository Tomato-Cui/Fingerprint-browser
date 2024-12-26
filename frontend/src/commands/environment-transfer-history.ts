import { invoke } from '@tauri-apps/api/core'

export const environment_transfer_history_query_id = async (environmentUuid: string): Promise<any> => {
    return await invoke('environment_transfer_history_query_id', { environmentUuid })
};

export const environment_transfer_history_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_transfer_history_query', { pageNum, pageSize })
};

export const environment_transfer_history_batch_create = async (
    environmentUuids: Array<String>,
    to_user_uuid: String,
): Promise<any> => {
    return await invoke('environment_transfer_history_batch_create', {
        environmentUuids,
        to_user_uuid,
    })
};

export const environment_transfer_history_delete = async (environmentUuid: String): Promise<any> => {
    return await invoke(' environment_transfer_history_delete', { environmentUuid })
};
