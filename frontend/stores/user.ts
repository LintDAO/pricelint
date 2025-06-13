import { getUserAutoRegister } from "@/api/user";
import type { ApiResult, ApiUserInfo } from "@/types/types";
import { defineStore } from "pinia";

interface UserState {
  principal: string;
  user: ApiUserInfo;
}

export const useUserStore = defineStore({
  id: "user",
  state: (): UserState => ({
    principal: "",
    user: {
      id: "",
      owner: "", // 初始化为 string
      name: "",
      created_at: BigInt(0),
    },
  }),
  getters: {
    // 直接返回 state.user，确保 owner 是 string
    getUserInfo: (state): ApiUserInfo => state.user,
  },
  actions: {
    // 设置 principal 并获取用户信息
    async setPrincipal(principal: string) {
      if (principal === "") {
        this.principal = "";
        this.user = {
          id: "",
          owner: "",
          name: "",
          created_at: BigInt(0),
        };
        return;
      }

      if (principal !== this.principal) {
        this.principal = principal;
        await this.fetchUserInfo();
      }
    },

    // 异步获取用户信息，规范化 owner 为 string
    async fetchUserInfo() {
      if (!this.principal) {
        this.user = {
          id: "",
          owner: "",
          name: "",
          created_at: BigInt(0),
        };
        return;
      }

      try {
        const response: ApiResult<ApiUserInfo> = await getUserAutoRegister();
        if (response.Ok) {
          this.user = {
            ...response.Ok,
          };
        } else {
          console.error("Failed to fetch user info:", response.Err);
          this.user = {
            id: "",
            owner: this.principal, // 使用 principal 作为默认 owner
            name: "",
            created_at: BigInt(0),
          };
        }
      } catch (error) {
        console.error("Error fetching user info:", error);
        this.user = {
          id: "",
          owner: this.principal,
          name: "",
          created_at: BigInt(0),
        };
      }
    },

    // 更新用户名
    async setUsername(username: string) {
      this.user.name = username;
      // 如果需要持久化，调用后端 API 并刷新缓存
      // await updateUserInfo({ ...this.user, name: username });
      await this.fetchUserInfo();
    },
  },
});
