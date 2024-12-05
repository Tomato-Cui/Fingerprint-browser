import axiosInstance from "./axiosConfig";
import { ElMessage, ElMessageBox, linkEmits } from "element-plus";
import axios from "axios";
import { reactive, ref } from "vue";
import { useRouter } from "vue-router";



// export const createEnvironment = async (data: any) => {
//   const response = await axiosInstance.post("/environments", data);
//   return response.data;
// };




export const createEnvironment = async (data: any) => {
  try {
    const response = await axiosInstance.post('environments/CreateEnvironment', data);
    return response.data;
  } catch (error) {
    console.error('Error creating environment:', error);
    throw error;
  }
};


export const fetchGroupIds = async (page: number, pageSize: number) => {
  console.log("fetchGroupIds被调用了");
  try {


    const response = await axiosInstance.get('/environments/GetEnvironmentGroupBypage', {
      params: {
        page: 1,
        limit: 1000,
      },
    });

    // 确保 response.data 和 response.data.data 存在
    if (response.data && response.data.data && response.data.data.data) {


      // 提取所有的 ID
      const groupIds = response.data.data.data.map(group => group.ID);
      return groupIds;
    } else {
      throw new Error('响应数据格式错误或数据缺失');
    }
  } catch (error) {
    console.error('Error fetching group IDs:', error);
    throw error; // 向上抛出错误，方便调用者处理
  }
};



export default {

  // fetchGroups
  fetchGroupIds,
  createEnvironment,
};
