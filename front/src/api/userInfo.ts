import axiosInstance from "./axiosConfig";

//TODO 获取当前登录用户信息
export const getUserInfoApi = () => {
//   const request = axiosInstance.get("/user/info");
//   return request;
};

//TODO 绑定邮箱
export const bindEmailApi = (data: any) => {
//   const request = axiosInstance.post("/user/bindemail", data);
//   return request;
};

// TODO 修改用户信息
export const updateUserInfoApi = (data: any) => {
//   const request = axiosInstance.put("/user/update", data);
//   return request;
};

// TODO 创建用户偏好
export const createUserPreferenceApi = (data: any) => {
//   const request = axiosInstance.post("/user/preference", data);
//   return request;
};

// TODO 本地设置
export const localSettingApi = (data: any) => {
//   const request = axiosInstance.post("/user/localSetting", data);
//   return request;
};


export default {
  getUserInfoApi,
  bindEmailApi,
  updateUserInfoApi,
  createUserPreferenceApi,
  localSettingApi,
};
