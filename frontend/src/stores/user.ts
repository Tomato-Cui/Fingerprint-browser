import { defineStore } from "pinia";
import { reactive } from "vue";

const default_avatar = 'https://img.freepik.com/free-vector/beautiful-woman-has-long-hair-pink-dress_1308-125946.jpg?t=st=1734769015~exp=1734772615~hmac=dfbb76ec0391f86c02c79773423791880b882012d3c3cb636daf171ecd61087d&w=2000';

export interface UserState {
  id?: any,
  account: string,
  avatar?: string,
}

export const useUserStore = defineStore('user-store', () => {
  const userInfo = reactive({
    id: 0,
    account: "",
    avatar: default_avatar
  });

  const login = (payload: UserState) => {
    localStorage.setItem('nickname', payload.account);
    userInfo.id = payload.id || 0;
    userInfo.account = payload.account;
    userInfo.avatar = payload.avatar || default_avatar;
  }

  const logout = () => {
    localStorage.clear();
    userInfo.id = 0;
    userInfo.account = '';
    userInfo.avatar = default_avatar;
  }

  const getUserInfo = (): UserState => {
    let id = userInfo.id || (localStorage.getItem('id') || 0);
    let account = userInfo.account || localStorage.getItem('nickname') || '';
    let avatar = userInfo.avatar || default_avatar;

    return { id, account, avatar }
  }

  return {
    userInfo,
    getUserInfo,
    login,
    logout
  }
})