// src/api/environmentService.js
import axiosInstance from './axiosConfig';
import { ElMessage, ElMessageBox } from 'element-plus';
import axios from 'axios';
import { reactive, ref } from 'vue';

/**
 * 获取所有环境记录
 * @returns {Promise} 返回环境记录列表
 */
export const getAllEnvironments = async () => {
  const response = await axiosInstance.get('/environments');


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
export const createEnvironment = async (data: any) => {
  const response = await axiosInstance.post('/environments', data);
  return response.data;
};

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

/**
 * 删除环境记录
 * @param {number} id 要删除的记录 ID
 * @returns {Promise} 返回删除成功的消息
 */


/**
 * 加载表格数据
 * @param {...any} params 分页参数
 * @returns {Object} 返回表格数据和总条数
 */
export const loadData = async (...params: any) => {
  console.log("修改ip后重新加载数据")
  // 定义返回数据对象
  const obj = ref({
    tableData: '', // 表格数据
    total: '', // 总条数
  })
  try {
    // 发送GET请求获取数据
    const response = await axiosInstance.get("/environments/getbypage", {
      params: {
        page: params[0], // 当前页码
        page_size: params[1], // 每页大小
      },
    });

    // 处理返回的数据
    if (response.data && response.data.data) {
      // 格式化表格数据
      obj.value.tableData = response.data.data.map((item: any, index: number) => ({
        id: item.ID,
        id_code: (params[0] - 1) * 10 + index + 1,
        group_name: item.group_id,
        name: item.name,
        ip: item.ip,
        last_opened: new Date(item.last_opened).toLocaleString(), // 格式化时间
        // account_platform: item.account_platform,
        account_platform: item.domain_name,

        actions: item.actions,
      }));


      // 更新总条数
      obj.value.total = response.data.pagination.total;
    } else {
      console.error("后端返回的数据格式不正确:", response.data);
      ElMessage.error("数据加载失败：服务器返回格式异常");
    }
  } catch (error) {
    console.error("加载数据失败:", error);
    ElMessage.error("数据加载失败，请稍后重试");
  }
  return obj.value;
};


/**
 * 处理删除操作
 * @param {Object} row 要删除的行数据
 * @returns {Promise} 返回删除操作的结果
 */
export const handleDeleteClick = async (row: any) => {
  console.log(row.id);
  try {
    // 显示删除确认对话框
    await ElMessageBox.confirm("确定要删除这条数据吗？", "警告", {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
      type: "warning",
    });

    // 发送删除请求
    const response = await axiosInstance.delete(`/environments/${row.id}`);
    console.log("----------:", response);

    ElMessage.success("删除成功");

    // 刷新页面
    window.location.reload();
    return response.data;
  } catch (error) {
    ElMessage.error("删除失败，请稍后重试");
  }
};

/**
 * 更新环境IP地址
 * @param {number} id 环境ID
 * @param {string} ip 新的IP地址
 * @returns {Promise} 返回更新结果
 */
export const updateEnvironmentIP = async (id: number, ip: string) => {
  try {
    // 发送更新IP请求
    const response = await axiosInstance.put(`/environments/${id}/ip`, { ip });
    window.location.reload();
    return response.data;
  } catch (error) {
    console.error('更新 IP 失败:', error);
    throw error;
  }
};

/**
 * 更新环境名称
 * @param {number} id 环境ID
 * @param {string} name 新的名称
 * @returns {Promise} 返回更新结果
 */
export const updateEnvironmentName = async (id: number, name: string) => {
  console.log("调用updateEnvironmentName");
  try {
    // 发送更新名称请求
    const response = await axiosInstance.put(`/environments/${id}/name`, { name });
    ElMessage.success('名称更新成功');
    return response.data;
  } catch (error) {
    console.error('更新名称失败:', error);
    ElMessage.error('更新名称失败，请稍后重试');
    throw error;
  }
};

/**
 * 更新环境账号平台
 * @param {number} id 环境ID
 * @param {string} accountPlatform 新的账号平台
 * @returns {Promise} 返回更新结果
 */
export const updateEnvironmentAccountPlatform = async (id: number, accountPlatform: string) => {

  try {
    // 发送更新账号平台请求
    const response = await axiosInstance.put(`/environments/${id}/account_platform`, {


      account_platform: accountPlatform,
    });
    ElMessage.success("账号平台更新成功！");
    return response.data;
  } catch (error) {
    console.error("更新账号平台失败:", error);
    ElMessage.error("更新账号平台失败，请稍后重试！");
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
};
