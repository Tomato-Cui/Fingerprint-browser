import { invoke } from '@tauri-apps/api/core';

// 该方法用于将状态信息注册到状态.
export const login = async (token: String): Promise<any> => {
    let response = invoke && await invoke('login', { token });
    return response
}
