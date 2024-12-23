import { invoke } from '@tauri-apps/api/core'

export type Environment = {
    owner_id?: number; // 所有者ID
    id?: number; // 自增ID
    name: string; // 环境名称
    description?: string; // 环境名称
    domain_name?: string; // 账号平台的域名
    open_urls?: string; // 其他URL
    repeat_config?: string; // 去重配置
    username: string; // 账号
    password: string; // 密码
    fakey?: string; // 2FA密钥
    cookie?: string; // Cookie
    ignore_cookie_error?: number; // 校验Cookie失败时的行为
    group_id?: number; // 分组ID
    fp_info_id?: number; // 指纹信息ID
    ua: string; // 用户代理
    os: string; // 操作系统
    country?: string; // 国家/地区
    region?: string; // 省/州
    city?: string; // 城市
    remark?: string; // 备注
    ipchecker: string; // IP查询渠道
    sys_app_cate_id: string; // 应用分类ID
    user_proxy_config?: string; // 环境代理配置
    proxy_enable: number; // 代理启用
    is_tz: number; // 是否启用时区
    is_pos: number; // 是否启用地理位置
    user_data_file: string; // 用户数据文件路径
    driver_location?: string; // 浏览器驱动位置
    status: number; // 浏览器状态
};

export const environment_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_query_id', { id })
};

export const environment_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query', { pageNum, pageSize })
};

export const environment_query_by_group = async (groupId: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_group', { groupId, pageNum, pageSize })
};

export const environment_create = async (payload: Environment): Promise<any> => {
    return await invoke('environment_create', { payload })
};

export const environment_batch = async (payload: Array<Environment>): Promise<any> => {
    return await invoke('environment_batch', { payload })
};

export const environment_modify = async (id: number, payload: Environment): Promise<any> => {
    return await invoke('environment_modify', { id, payload })
};

export const environment_move_to_group = async (
    environmentId: number,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_move_to_group', { environmentId, groupId })
};

export const environment_batch_move_to_group = async (
    environmentIds: Array<number>,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_batch_move_to_group', {
        environmentIds, groupId
    })
};

export const environment_delete = async (id: number): Promise<any> => {
    return await invoke('environment_delete', { id })
};

//
export const environment_batch_delete = async (ids: Array<number>): Promise<any> => {
    return await invoke('environment_batch_delete', { ids })
};