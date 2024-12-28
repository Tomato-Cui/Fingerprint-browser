import { invoke } from '@tauri-apps/api/core'

export const login = async (account: String, password: String): Promise<any> => {
    return await (window as any).new_invoke('login', { account, password })
    // return await invoke('login', { account, password })
};

export const register = async (email: String, account: String, password: String): Promise<any> => {
    return await invoke('register', { email, account, password })
};

export const logout = async (): Promise<any> => {
    return await invoke('logout')
};


export const isLogin = async (): Promise<boolean> => {
    return await invoke('is_login',)
};