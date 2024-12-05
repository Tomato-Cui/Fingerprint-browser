import axiosInstance from './axiosConfig';
import { ElMessage, ElMessageBox } from 'element-plus';
import axios, { AxiosError } from 'axios';

// 登录操作

// 登录
export const login = async (credentials: any) => {
  try {
    const response = await axiosInstance.post('/auth/login', credentials); // 使用 POST 请求，直接用相对路径
    return response.data; // 返回登录成功的数据
  } catch (error: unknown) {
    if (error instanceof AxiosError) {
      throw new Error(error.response?.data?.message || '登录失败');
    } else {
      throw new Error('登录失败');
    }
  }
};

// 注册
export const register = async (formData: any) => {
  try {
    // 发送注册请求
    const response = await axiosInstance.post('/auth/register', formData);
    return response.data; // 假设返回的是注册成功的相关数据
  } catch (error: unknown) {
    if (error instanceof AxiosError) {
      throw new Error(error.response?.data?.message || '注册失败');
    } else {
      throw new Error('注册失败');
    }
  }
};

// 发送验证码
export const sendCode = async (email: string) => {
  try {
    const response = await axiosInstance.post('/auth/send', { email });
    return response.data;
  } catch (error: unknown) {
    if (error instanceof AxiosError) {
      throw new Error(error.response?.data?.message || '验证码发送失败');
    } else {
      throw new Error('验证码发送失败');
    }
  }
};

export default {
  login,
  register,
  sendCode
};
