import { defineStore } from "pinia";
import { reactive, watchEffect } from "vue";

const default_avatar = 'https://img.freepik.com/free-vector/beautiful-woman-has-long-hair-pink-dress_1308-125946.jpg?t=st=1734769015~exp=1734772615~hmac=dfbb76ec0391f86c02c79773423791880b882012d3c3cb636daf171ecd61087d&w=2000';

export interface UserState {
  id?: any,
  account: string,
  email: string,
  avatar?: string,
}

export const useUserStore = defineStore('user-store', () => {
  let userInfo = reactive({
    id: 0,
    account: "",
    email: "",
    avatar: default_avatar
  });

  const loadFromLocalStorage = () => {
    const userInfoTemp = JSON.parse(localStorage.getItem('user-infos') || '[]')
    userInfo = { ...userInfoTemp };
  }

  loadFromLocalStorage()

  watchEffect(() => {
    window.addEventListener('storage', () => {
      loadFromLocalStorage()
    })
  })

  const setUserInfo = (payload: UserState) => {
    localStorage.setItem('user-infos', JSON.stringify(payload));
    userInfo.id = payload.id || 0;
    userInfo.account = payload.account;
    userInfo.email = payload.email;
    userInfo.avatar = payload.avatar || default_avatar;
  }

  const getUserInfo = (): UserState => userInfo;

  return {
    userInfo,
    getUserInfo,
    setUserInfo,
  }
})