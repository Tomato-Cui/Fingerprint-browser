// src/api/axiosConfig.js
import axios from 'axios';

const axiosInstance = axios.create({
  baseURL: 'http://192.168.6.116:8080/api', // 替换为后端 API 的基础地址
  timeout: 5000, // 请求超时时间
  headers: { 'Content-Type': 'application/json' },
});

axiosInstance.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token'); // 从 localStorage 获取 token
    if (token) {
      // 如果存在 token，将它设置到 Authorization header 中
      config.headers['Authorization'] = `${token}`;
    }
    return config; // 返回请求配置对象，继续发送请求
  },
  (error) => {
    return Promise.reject(error); // 错误处理
  }
);

axiosInstance.interceptors.response.use(
  (response) => response, // 响应成功处理
  (error) => {
    console.error('API Error:', error); // 打印错误信息
    return Promise.reject(error); // 响应错误处理
  }
);

export default axiosInstance; // 导出 axios 实例
