import axiosInstance from './axiosConfig';

export const getDivides = async (page: number, pageSize: number) => {
  console.log("getDivides:", page, pageSize);

  try {
    // 发起请求
    const response = await axiosInstance.get("/environments/GetEnvironmentGroupBypage", {
      params: {
        page: page,
        limit: pageSize,
      },
    });
    console.log("response.data:", response.data);

    // 根据返回的JSON格式重构返回数据
    return {
      code: response.data.code,
      message: response.data.message,
      data: {
        data: response.data.data.data.map((item: any) => ({
          id: item.ID,
          name: item.name,
          remark: item.remark,
          description: item.description,
          createdAt: item.CreatedAt,
          updatedAt: item.UpdatedAt,
          ownerId: item.OwnerID
        })),
        meta: {
          page: response.data.data.page,
          page_size: response.data.data.page_size,
          total_items: response.data.data.total_count
        }
      }
    };
  } catch (error: any) {
    console.error('获取分组数据失败', error);

    if (error.response) {
      // 请求发出去了，服务器响应了状态码
      console.error('Response error:', error.response.status);
      console.error('Response data:', error.response.data);
    } else if (error.request) {
      // 请求发出去了，但没有收到响应
      console.error('Request error:', error.request);
    } else {
      // 其他错误
      console.error('Error message:', error.message);
    }

    throw error;  // 抛出错误
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


// export const deleteDivide = async (id: any) => {
//   console.log("deleteDivide:", id);
//   try {
//     const response = await axiosInstance.delete(`/groups/environments/${id}`); // 发送 DELETE 请求
//     return response.data; // 返回删除成功的响应数据
//   } catch (error) {
//     console.error('删除分组失败', error);
//     throw error; // 抛出错误
//   }

// };

export const deleteDivide = async (id: any) => {
  console.log("deleteDivide:", id);
  try {
    const response = await axiosInstance.delete(`/environments/groups/${id}`);

    if (response.status === 200) {
      window.location.reload();
      console.log('删除成功');

      return true; // 或者你可以返回删除成功的某个状态
    }
    console.log('删除失败，状态码：', response.status);
    return false;
  } catch (error) {
    console.error('删除分组失败', error);
    throw error;
  }

};



export default {
  getDivides,
  createDivide,
  deleteDivide,
};
