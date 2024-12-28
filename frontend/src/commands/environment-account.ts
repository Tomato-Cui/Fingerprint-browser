import { invoke } from '@tauri-apps/api/core'

export type EnvironmentAccount = {
    platform: string;
    platform_url: string;
    platform_account: string;
    platform_password: string;
    platform_description?: string;
    environment_uuid: string;
    user_uuid: string;
}

export const environment_account_query_id = async (id: number,) => {
    return await invoke('environment_account_query_id', { id })
}


export const environment_account_query = async (
    pageNum: number,
    pageSize: number,
) => {
    return await invoke('environment_account_query', {
        pageNum,
        pageSize
    })
}

export const environment_account_query_current = async (
    environmnet_uuid: string,
    pageNum: number,
    pageSize: number,
) => {
    return await invoke('environment_account_query_current', {
        environmnet_uuid,
        pageNum,
        pageSize
    })
}

export const environment_account_create = async (payload: EnvironmentAccount) => {
    return await invoke('environment_account_create', { payload })
}

export const environment_account_modify = async (id: number, payload: EnvironmentAccount) => {
    return await invoke('environment_account_modify', { id, payload })
}

export const environment_account_delete = async (id: number) => {
    return await invoke('environment_account_delete', { id })
}

export const environment_account_batch_delete = async (ids: number[]) => {
    return await invoke('environment_account_batch_delete ', { ids })
}
