import { invoke } from '@tauri-apps/api/core'

// 代理接口定义
export interface Proxy {
    id?: number;                     // 代理ID
    kind: string;                    // 代理类型
    host: string;                    // 代理主机地址
    port: string;                    // 代理端口
    username?: string;               // 代理用户名（可选）
    password?: string;               // 代理密码（可选）
    user_uuid?: string;              // 用户UUID（可选）
    environment_group_id?: number;   // 环境组ID（可选）
}

// 根据ID查询代理
export const environment_proxies_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_proxies_query_id', { id })
};

// 分页查询代理列表
export const environment_proxies_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_proxies_query', { pageNum, pageSize })
};

// 根据代理组ID分页查询代理列表
export const environment_proxies_query_by_group = async (proxyGroupId: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_proxies_query_by_group', { proxyGroupId, pageNum, pageSize })
};

// 创建新代理
export const environment_proxies_create = async (payload: Proxy): Promise<any> => {
    return await invoke('environment_proxies_create', {
        payload
    })
};

// 修改代理信息
export const environment_proxies_modify = async (id: number, payload: Proxy): Promise<any> => {
    return await invoke('environment_proxies_modify', { id, payload })
};

// 修改环境代理设置
export const environment_modify_proxy = async (environmentUuid: string, payload: Proxy): Promise<any> => {
    return await invoke('environment_modify_proxy', { environmentUuid, payload })
};

// 删除单个代理
export const environment_proxies_delete = async (id: number): Promise<any> => {
    return await invoke('environment_proxies_delete', { id })
};

// 批量删除代理
export const environment_proxies_batch_delete = async (ids: number[]) => {
    return await invoke('environment_proxies_batch_delete', { ids })
}
