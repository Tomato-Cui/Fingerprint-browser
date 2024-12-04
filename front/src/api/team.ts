import axios from "axios";
import axiosInstance from "./axiosConfig";
import { ref } from "vue";

//创建团队 ------------取消
export const createTeamApi = (data: any) => {
  const request = axiosInstance.post("/teamMg/createteam", {
    DeletedAt: null,
    TeamName: data.teamName,
    Description: data.remark,
  });
  return request;
};

/**
 * @param data.groupName: 分组名称
 * @param data.remark: 备注
 * @param data.role[]: 权限？(数组)
 */
//创建分组 -- 复制分组
export const createGroupApi = (data: any) => {
  const url = "/?*?/";
  axiosInstance
    .post(url, {
      p1: data.groupName,
      p2: data.remark,
      p3: data.role,
    })
    .then((res: any) => {
      console.log("创建分组成功");
    });
};

// 根据团队id获取分组列表 （分页查询）---------------无
export const getGroupListApi = async (teamId: number) => {
  //   return axiosInstance.get("/team/group");
  axiosInstance.post("/teamMg/GetTeamGroups", {
    teamId: teamId,
    ownerID: "1",
  });
};
//打开默认团队
export const getDefaultTeam = async () => {
  //默认进入第一个团队---查询当前用户下的所有团队
  const teams = await axiosInstance.get("/teamMg/GetTeams");
  console.log("teams:", teams);

  const response = await axiosInstance.post("/teamMg/GetTeamGroups", {
    teamID: String(teams.data.data[0].ID),
    ownerID: "1",
  });
  console.log("response:", response);

  return response;
};

/**
 * 删除分组
 * @param teamId 团队ID
 * @param groupId 分组ID
 */
export const deleteGroupApi = (teamId: number, groupId: number) => {
  //   return axiosInstance.delete("/team/group");
  // console.log("删除分组", id);
  axiosInstance
    .delete(`/teamMg/DeleteTeamGroup?teamID=${teamId}&groupID=${groupId}`)
    .then((res) => {
      console.log("删除分组成功", res);
      return res;
    })
    .catch((err) => {
      console.log("删除分组失败", err);
      return err;
    });
};

/**
 * 修改分组
 * @param data.groupID: 分组ID
 * @param data.groupName: 分组名称
 * @param data.remark: 备注
 * @param data.role[]: 权限？（数组）
 */
export const updateGroup = (data: any) => {
  const url = "/?*?/";
  axiosInstance
    .put(url, {
      p1: data.groupID,
      p2: data.groupName,
      p3: data.remark,
      p4: data.role,
    })
    .then((res: any) => {
      console.log("修改成功");
    });
};

//TODO 根据电话或邮箱判断当前成员是否已经是本团成员
export const isMemberApi = (phoneOrEmail: string) => {
  //   return axiosInstance.get("/team/group");
  console.log("根据电话或邮箱判断当前成员是否已经是本团成员", phoneOrEmail);
  return true;
};

//TODO 根据团队id查询所有环境
export const getEnvironmentsApi = (teamId: number) => {
  //   return axiosInstance.get("/team/group");
  console.log("根据团队id查询所有环境", teamId);
  return [];
};

/**
 * 添加成员到对应的分组和团队 --- 输入登录账号自动查询用户，获取用户名，若用户已在本团队则禁止提交，若无此用户则创建新用户
 * @param data.groupID: 添加到的分组ID
 * @param data.userID: 添加的成员ID
 * @param data.environmentList?: 授权环境列表
 * @returns
 */
export const addMemberApi = (data: any) => {
  const url = "/?*?/";
  axiosInstance
    .post(url, {
      p1: data.groupID,
      p2: data.userID,
      p3: data.environmentList,
    })
    .then((res: any) => {
      console.log("添加成功");
    });
};

/**
 * 根据登录账号查询用户，返回用户是否已经存在于本团的boolean值
 * @param params：登录账号
 * @returns
 */
export const searchByEmail = (params: any) => {
  const url = "/?*?/";

  axiosInstance
    .get(url, {
      params: {
        p1: params,
      },
    })
    .then((res: any) => {
      console.log("是否存在：", res);
    });
};

/**
 * 查询团队下所有环境
 * @param params 
 * @returns 
 */

//根据id查询分组信息
export const getGroupInfoApi = (params: any) => {
  // console.log("根据id查询分组信息", id);
  const request = axiosInstance.post("/teamMg/GetTeamGroups", {
    teamID: params.teamId,
    ownerID: params.ownerId,
  });
  return request;
};

//根据分组id查询所有成员
export const getMembersApi = (groupId: number) => {
  // console.log("根据分组id查询所有成员", groupId);
  const request = axiosInstance.get(`/teamMg/groups/${groupId}/users`);
  return request;
};

//根据成员id为成员授权环境
export const addEnvironmentApi = (data: any) => {
  //   return axiosInstance.get("/team/group");
  console.log("根据成员id为成员授权环境", data);
};
//根据成员id查询成员授权的环境
export const getMemberEnvironmentsApi = (id: number) => {
  //   return axiosInstance.get("/team/group");
  console.log("根据成员id查询成员授权的环境", id);
  return [];
};

//修改成员信息
export const updateMemberApi = (data: any) => {
  // console.log("修改成员信息", data);
  const request = axiosInstance.put(
    `/teamMg/${data.groupId}/users/${data.userId}`,
    {
      role: data.role,
      permission: data.permission,
    }
  );
  return request;
};

//TODO根据id查询成员信息
export const getMemberInfoApi = (id: number) => {
  //   return axiosInstance.get("/team/group");
  console.log("根据id查询成员信息", id);
  return {};
};

//根据id删除成员
export const deleteMemberApi = (data: any) => {
  // console.log("根据id删除成员", id);
  const request = axiosInstance.delete(
    `/teamMg/groups/${data.groupId}/users/${data.userId}`
  );
  return request;
};

//TODO 根据id禁用成员
export const disableMemberApi = (id: number) => {
  //   return axiosInstance.get("/team/group");
  console.log("根据id禁用成员", id);
};

//TODO根据id启用成员
export const enableMemberApi = (id: number) => {
  //   return axiosInstance.get("/team/group");
  console.log("根据id启用成员", id);
};

//  TODO
export const agreeOrRefuseJoinTeamApi = (data: any) => {
  //   return axiosInstance.get("/team/group");
  console.log("同意或拒绝加入团队", data);
};

//根据团队ID查询所有成员
export const getAllMemberByTeamId = async (teamId: number) => {
  const resList = ref<any[]>([]);
  //查询所有团队下所有分组
  const res = await axiosInstance.post("/teamMg/GetTeamGroups", {
    teamID: String(teamId),
    ownerID: "1",
  });
  // console.log("_+_+_+_:", res);

  res.data.data.forEach(async (group: any) => {
    //查询每个组的成员
    const result = await axiosInstance.get(
      `/teamMg/groups/${group.group_id}/users`
    );
    result.data.data.forEach((user: any) => {
      //每个成员添加到resList
      if (resList.value.find((item) => item.ID === user.ID) === undefined) {
        resList.value.push(user);
      }
    });
  });
  return resList.value;
};

// 查询当前用户的所有团队
export const getAllTeam = async () => {
  // const result = ref();
  return await axiosInstance.get("/teamMg/GetTeams");
  // console.log("getAllTeam---:", result.value);

  // return result.value;
};

export default {
  createGroupApi,
  getGroupListApi,
  deleteGroupApi,
  isMemberApi,
  getEnvironmentsApi,
  getGroupInfoApi,
  addMemberApi,
  getMembersApi,
  addEnvironmentApi,
  updateMemberApi,
  getMemberInfoApi,
  deleteMemberApi,
  disableMemberApi,
  enableMemberApi,
  getMemberEnvironmentsApi,
  agreeOrRefuseJoinTeamApi,
  getAllMemberByTeamId,
  getDefaultTeam,
  getAllTeam,
  updateGroup,
  searchByEmail,
};
