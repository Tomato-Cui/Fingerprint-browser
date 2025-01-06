import { invoke } from './index'

export const environment_cookie_query_environment_uuid = async (environmentUuid: string) => {
    return await invoke('environment_cookie_query_environment_uuid', { environmentUuid })
}

export const environment_cookie_create = async (
    environmentUuid: string,
    cookie_str: string,
) => {
    return await invoke('environment_cookie_create', {
        environmentUuid,
        cookie_str
    })
}

export const environment_account_create = async (
    environmentUuid: string,
    cookie_str: string,
) => {
    return await invoke('environment_account_create', {
        environmentUuid,
        cookie_str
    })
}

export const environment_account_modify = async (
    environmentUuid: string,
    cookie_str: string,
) => {
    return await invoke('environment_account_modify', {
        environmentUuid,
        cookie_str
    })
}

export const environment_account_delete = async (environmentUuid: string) => {
    return await invoke('environment_account_delete', { environmentUuid })
}
