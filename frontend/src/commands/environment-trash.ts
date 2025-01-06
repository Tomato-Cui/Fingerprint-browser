import { invoke } from './index'

export const environment_trash_query_id = async (environmentUuid: String): Promise<any> => {
    return await invoke('environment_trash_query_id', { environmentUuid })
};

export const environment_trash_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_trash_query', { pageNum, pageSize })
};

export const environment_trash_recover = async (environmentUuid: string): Promise<any> => {
    return await invoke('environment_trash_recover', { environmentUuid })
};

export const environment_trash_recovers = async (environmentUuids: String[]): Promise<any> => {
    return await invoke('environment_trash_recovers', { environmentUuids })
};

export const environment_trash_recover_all = async (): Promise<any> => {
    return await invoke('environment_trash_recover_all', {})
};

export const environment_trash_delete_batch = async (environmentUuids: String[]): Promise<any> => {
    return await invoke('environment_trash_delete_batch', { environmentUuids })
};

export const environment_trash_clean = async (
): Promise<any> => {
    return await invoke('environment_trash_clean', {})
};