import type { ApiResult, ApiUserInfo } from "@/types/types";
import { TTL, getCache } from "@/utils/cache";
import { showMessageError } from "@/utils/message";
import { getBackend, getCurrentPrincipal } from "./canister_pool";

const userTTL = TTL.hour12; //用户自身信息缓存时长。

//防止直接使用返回值导致ts报错：不存在属性“Err”。类型“{ Ok: [] | [User]; }”上不存在属性“Err
export async function userLogin(): Promise<ApiResult<any>> {
  return getBackend().user_login();
}

export async function userRegister(): Promise<ApiResult<any>> {
  return getBackend().user_register();
}

// （后端自动注册）并登录，如果有注册，就获取当前登录用户信息，如果没注册，就注册完了再获取信息
export async function getUserAutoRegister(): Promise<ApiResult<ApiUserInfo>> {
  return await getCache({
    key: "USER_INFO_" + getCurrentPrincipal().toUpperCase(),
    execute: async () => {
      const loginResult = await userLogin();
      if (loginResult.Err === "UserIsNotExist") {
        const registerResult = await userRegister();
        if (registerResult.Ok) return registerResult; // 注册成功后直接返回registerResult，因为注册和登录返回值是一样的。

        return registerResult; // 注册失败，返回错误
      }
      return loginResult; // 登录成功或其他错误
    },
    ttl: userTTL,
    isLocal: true, // 启用本地存储（根据需要）
    update: true, // 异步更新缓存（可选）
  }).catch((error) => {
    console.error("getUserAutoRegister Error:", error);
    showMessageError("Login Failed: " + error);
  });
}
