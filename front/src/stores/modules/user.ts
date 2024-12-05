import { defineStore } from "pinia";
// import { PersistedStateOptions } from "pinia-plugin-persistedstate";

// const piniaPersistConfig = (key: string, paths?: string[]) => {
//   const persist: PersistedStateOptions = {
//     key,
//     storage: localStorage,
//     // storage: sessionStorage,
//     paths
//   };
//   return persist;
// };

export const useUserStore = defineStore({
  
  id: "geeker-user",
  state: (): UserState => ({
    token: "",
    userInfo: { name: "Geeker" }
  }),
  
  getters: {},
  actions: {
   
    setToken(token: string) {
      this.token = token;
    },
    
    setUserInfo(userInfo: UserState["userInfo"]) {
      this.userInfo = userInfo;
    }
  },
  
});

export interface UserState {
  token: string;
  userInfo: { name: string };
}




