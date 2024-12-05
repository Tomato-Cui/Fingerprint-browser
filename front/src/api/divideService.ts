import axiosInstance from './axiosConfig';



export const fetchDivides = async (params: any) => {
  console.log("fetchDivides:", params);

  const { page, pageSize, divides, totalItems } = params;

  try {
    const response = await axiosInstance.get("/divides", {
      params: {
        page: page.value,
        page_size: pageSize.value,
      },
    });
    divides.value = response.data.data;
    totalItems.value = response.data.meta.total_items;
    return response.data;
  } catch (error) {
    console.error("Error fetching divides:", error);
  }
};


export const getDivides = async (page: any, pageSize: any) => {

  try {
    const response = await axiosInstance.get('/divides', {
      params: {
        page: page,
        page_size: pageSize,
      },
    }); // 发送 GET 请求
    console.log("response.data:", response.data);
    return response.data; // 返回分组数据
  } catch (error) {
    console.error('获取分组数据失败', error);
    throw error; // 抛出错误
  }
};


export const createDivide = async (divideData: any) => {
  try {


    const response = await axiosInstance.post('/divides', divideData); // 发送 POST 请求
    return response.data; // 返回创建的分组数据
  } catch (error) {
    console.error('创建分组失败', error);
    throw error; // 抛出错误
  }
};


export const deleteDivide = async (id: any) => {
  try {
    const response = await axiosInstance.delete(`/divides/${id}`); // 发送 DELETE 请求
    return response.data; // 返回删除成功的响应数据
  } catch (error) {
    console.error('删除分组失败', error);
    throw error; // 抛出错误
  }
};


export default {
  getDivides,
  createDivide,
  deleteDivide,
  fetchDivides
};
