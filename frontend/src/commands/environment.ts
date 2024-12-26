import { invoke } from '@tauri-apps/api/core'

export interface Environment {
    name: string;                    // 环境名称
    description?: string;            // 环境描述
    default_urls?: string;           // 默认打开网页
    proxy_enable: number;            // 代理启用
    team_id?: number;                 // 团队ID
    proxy_id?: number;               // 代理ID
    fp_info_id?: number;             // 指纹信息ID
    group_id?: number;               // 分组ID
    status: number;                  // 状态
}
export const environment_query_id = async (environmentUuid: string): Promise<any> => {
    return await invoke('environment_query_id', { environmentUuid})
};

export const environment_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query', { pageNum, pageSize })
};

export const environment_query_by_group = async (id: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_group', { id, pageNum, pageSize })
};

export const environment_create = async (payload: Environment): Promise<any> => {
    return await invoke('environment_create', { payload })
};

export const environment_batch_create = async (payload: Array<Environment>): Promise<any> => {
    return await invoke('environment_batch_create', { payload })
}

export const environment_modify_info = async (environmentUuid: string, payload: Environment): Promise<any> => {
    return await invoke('environment_modify_info ', {environmentUuid, payload })
};

export const environment_move_to_group = async (
    environmentUuid: string,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_move_to_group', { environmentUuid, groupId })
};

export const environment_batch_move_to_group = async (
    environmentUuids: Array<string>,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_batch_move_to_group', {
        environmentUuids, groupId
    })
};

export const environment_delete = async (environmentUuid: String,): Promise<any> => {
    return await invoke('environment_delete', { environmentUuid})
};

export const environment_batch_delete = async (
    environmentUuids: Array<string>,
): Promise<any> => {
    return await invoke('environment_batch_delete', { environmentUuids })
};