import { invoke } from '@tauri-apps/api/core';

// 该方法用于将状态信息注册到状态.
export const login = async (token: String): Promise<void> => {
    let response = await invoke('login', { token });
}
