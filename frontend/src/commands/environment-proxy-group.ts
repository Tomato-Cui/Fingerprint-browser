import { invoke } from './index'

export interface ProxyGroup {
    name: string;                    // 分组名称
    description?: string;            // 分组描述
}
export const environment_proxy_group_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_proxy_group_query_id', { id })
};

export const environment_proxy_group_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_proxy_group_query', { pageNum, pageSize })
};

export const environment_proxy_group_create = async (payload: ProxyGroup): Promise<any> => {
    return await invoke('environment_proxy_group_create', { payload })
};

export const environment_proxy_group_modify = async (id: number, payload: ProxyGroup): Promise<any> => {
    return await invoke('environment_proxy_group_modify', { id, payload })
};


export const environment_proxy_group_delete = async (id: number): Promise<any> => {
    return await invoke('environment_proxy_group_delete', { id })
};