// src/api/environmentService.js
import axiosInstance from "./axiosConfig";
import { ElMessage, ElMessageBox, linkEmits } from "element-plus";
import axios from "axios";
import { reactive, ref } from "vue";

/**
 * 获取所有环境记录
 * @returns {Promise} 返回环境记录列表
 */
export const getAllEnvironments = async () => {
  const response = await axiosInstance.get("/environments");

  return response.data;
};

/**
 * 根据 ID 获取单个环境记录
 * @param {number} id 环境记录的 ID
 * @returns {Promise} 返回指定 ID 的环境记录
 */
export const getEnvironmentByID = async (id: number) => {
  const response = await axiosInstance.get(`/environments/${id}`);
  return response.data;
};

/**
 * 创建新的环境记录
 * @param {Object} data 新的环境数据
 * @returns {Promise} 返回创建的记录
 */

/**
 * 更新现有的环境记录
 * @param {number} id 要更新的记录 ID
 * @param {Object} data 更新的数据
 * @returns {Promise} 返回更新后的记录
 */
export const updateEnvironment = async (id: number, data: any) => {
  const response = await axiosInstance.put(`/environments/${id}`, data);
  return response.data;
};

interface TableDataItem {
  id: number;
  id_code: number;
  group_name: number;
  name: string;
  ip: string;
  last_opened: string;
  account_platform: string;
  actions: string | null; // 假设 actions 是可选字段
  creation_time: string;
  remarks: string;
  label: string | null; // 假设 label 是可选字段
  // group_count: number;
}

export const loadData = async (...params: any) => {
  // 使用泛型来显式指定 tableData 的类型
  const obj = ref<{
    tableData: TableDataItem[]; // 这里指定 tableData 为 TableDataItem 类型数组
    total: number;
  }>({
    tableData: [], // 表格数据
    total: 0, // 总条数
  });

  try {
    // 发送 GET 请求获取数据
    const response = await axiosInstance.get("/environments/getbypage", {
      params: {
        page: params[0], // 当前页码
        limit: params[1], // 每页大小
      },
    });

    // 检查返回数据的结构
    if (response.data && response.data.code === 1) {

      return response.data;

    } else {
      console.error(
        "后端返回的状态码不是 1，错误信息：",
        response.data.message
      );
      ElMessage.error("数据加载失败：服务器返回异常");
    }
  } catch (error) {
    console.error("加载数据失败:", error);
    ElMessage.error("数据加载失败，请稍后重试");
  }
  return obj.value;
};




export const handleDeleteClick = async (row: any): Promise<boolean> => {
  console.log("loadData 类型: ", typeof loadData); // 打印 loadData 的类型

  try {
    // 删除确认对话框
    await ElMessageBox.confirm("确定要删除这条数据吗？", "警告", {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
      type: "warning",
    });

    const id = row.ID;

    if (!id) {
      throw new Error("删除失败：无效的 ID");
    }

    const deleteUrl = `/environments/${id}`;
    const response = await axiosInstance.delete(deleteUrl);
    ElMessage.success("删除成功");
    return true;

  } catch (error: any) {
    if (error.isAxiosError) {
      console.error("请求失败:", error.response);
      ElMessage.error(`删除失败：服务器错误(${error.response.status})`);
    } else {
      console.error("删除失败:", error);
      ElMessage.error(error.message || "删除失败，请稍后重试");
    }
    return false;
  }
};

export const updateEnvironmentIP = async (ID: number, ip: string) => {
  console.log("调用updateEnvironmentIP，传递的ID:", ID, "IP:", ip);

  if (!ID || !ip) {
    throw new Error("ID 或 IP 参数缺失");
  }

  try {
    // 发送更新IP请求，确保 IP 以 JSON 格式传送
    const requestData = {
      proxy: ip,
    }; // 这里将 ip 包装在一个 JSON 对象中

    // 配置请求头，确保发送 JSON 格式
    const response = await axiosInstance.put(
      `/environments/${ID}`,
      requestData,
      {}
    );

    // 打印响应内容，确保接收到的数据是你期望的
    console.log("更新成功，返回的数据:", response.data);
    window.location.reload();
    // loadData(1, 10);

    return response.data;
  } catch (error) {
    console.error("更新 IP 失败:", error);
    throw error;
  }
};

export const updateEnvironmentName = async (id: number, name: string) => {
  console.log("name:", name);
  console.log("调用updateEnvironmentName");
  try {
    // 发送更新名称请求
    const response = await axiosInstance.put(`/environments/${id}`, { name });

    // 判断响应状态码
    if (response.status === 200) {
      ElMessage.success("名称更新成功");
      return response.data; // 返回更新后的数据
    } else {
      ElMessage.error("名称更新失败，请稍后重试");
      throw new Error("更新失败，状态码：" + response.status);
    }
  } catch (error) {
    console.error("更新名称失败:", error);
    ElMessage.error("更新名称失败，请稍后重试");
    throw error;
  }
};

/**
 * 更新环境账号平台
 * @param {number} id 环境ID
 * @param {string} accountPlatform 新的账号平台
 * @returns {Promise} 返回更新结果
 */
// export const updateEnvironmentAccountPlatform = async (id: number, accountPlatform: string) => {
//   console.log("调用updateEnvironmentAccountPlatform");

//   try {
//     // 发送更新账号平台请求
//     const response = await axiosInstance.put(`/environments/${id}`, {

//       account_platform: accountPlatform,
//     });
//     ElMessage.success("账号平台更新成功！");
//     return response.data;
//   } catch (error) {
//     console.error("更新账号平台失败:", error);
//     ElMessage.error("更新账号平台失败，请稍后重试！");
//     throw error;
//   }
// };

export const updateEnvironmentAccountPlatform = async (
  id: number,
  accountPlatform: string
) => {
  console.log("调用updateEnvironmentAccountPlatform");

  try {
    // 发送更新账号平台请求
    const response = await axiosInstance.put(`/environments/${id}`, {
      domain_name: accountPlatform,
    });

    // 判断响应状态码
    if (response.status === 200) {
      ElMessage.success("账号平台更新成功");
      return response.data; // 返回更新后的数据
      // window.location.reload();
    } else {
      ElMessage.error("账号平台更新失败，请稍后重试");
      throw new Error("更新失败，状态码：" + response.status);
    }
  } catch (error) {
    console.error("更新账号平台失败:", error);
    ElMessage.error("更新账号平台失败，请稍后重试");
    throw error;
  }
};

export const getEnvironmentGroup = async (id: number, params: any) => {
  console.log("调用updateEnvironmentAccountPlatform");

  try {
    // 发送更新账号平台请求
    const response = await axiosInstance.get(
      `/environments/GetEnvironmentGroup/${id}`,
      {
        params: {
          page: params[0], // 当前页码
          limit: params[1], // 每页大小
        },
      }
    );

    // 判断响应状态码
    if (response.status === 200) {
      ElMessage.success("账号平台更新成功");
      return response.data; // 返回更新后的数据
      // window.location.reload();
    } else {
      ElMessage.error("账号平台更新失败，请稍后重试");
      throw new Error("更新失败，状态码：" + response.status);
    }
  } catch (error) {
    console.error("更新账号平台失败:", error);
    ElMessage.error("更新账号平台失败，请稍后重试");
    throw error;
  }
};




// 导出所有方法
export default {
  loadData,
  handleDeleteClick,
  updateEnvironmentIP,
  updateEnvironmentName,
  updateEnvironmentAccountPlatform,
  getEnvironmentGroup,

};
