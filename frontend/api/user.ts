import type { ApiResult, ApiUserInfo } from "@/types/types";
import { TTL, getCache } from "@/utils/cache";
import { getBackend, getCurrentPrincipal } from "./canister_pool";

const userTTL = TTL.hour12; //用户自身信息缓存时长。

// （后端自动注册）并登录，如果有注册，就获取当前登录用户信息，如果没注册，就注册完了再获取信息
export async function getUserAutoRegister(): Promise<ApiResult<ApiUserInfo>> {
  return await getCache({
    key: "USER_INFO_" + getCurrentPrincipal().toUpperCase(),
    execute: () => getBackend().get_state(),
    ttl: userTTL,
    isLocal: true, // 需要本地存储
  });
}
