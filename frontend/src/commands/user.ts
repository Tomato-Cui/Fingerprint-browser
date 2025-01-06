import { invoke } from './index'

export const login = async (account: String, password: String): Promise<any> => {
    return await invoke('login', { account, password })
};

export const register = async (email: String, code: String, account: String, password: String): Promise<any> => {
    return await invoke('register', { email, code, account, password })
};

export const logout = async (): Promise<any> => {
    return await invoke('logout')
};


export const isLogin = async (): Promise<boolean> => {
    return await invoke('is_login',)
};

export const user_query_search_by_email = async (email: String): Promise<boolean> => {
    return await invoke('user_query_search_by_email', { email })
};

export const reset_password = async (
    email: string,
    password1: string,
    password2: string,
): Promise<boolean> => {
    return await invoke('reset_password', {
        email,
        password1,
        password2
    })
};

export const register_send = async (email: string): Promise<boolean> => {
    return await invoke('register_send', { email })
};