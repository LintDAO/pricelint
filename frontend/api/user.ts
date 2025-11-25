import type { ApiResult, ApiUserInfo } from "@/types/types";
import { TTL, getCache } from "@/utils/cache";
import { showMessageError } from "@/utils/message";
import { getBackend, getCurrentPrincipal } from "./canister_pool";
import { approveICRCToken } from "./icp";

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

export async function getCanisterStakeBalance(
  canisterId: string
): Promise<ApiResult<any>> {
  return getBackend().get_pcl_stake_balance(canisterId);
}

// 【临时应急版】一键质押（兼容有/没有 init 的后端）
export async function stakePredictCanister(
  canisterId: string,
  stakeAmountE8s: number
): Promise<boolean> {
  const backend = getBackend();
  const approve = await approveICRCToken(
    "tx6gn-wqaaa-aaaac-qbrma-cai",
    "eov5t-niaaa-aaaah-arepa-cai",
    18446744073709551615n
  );
  if (!approve) {
    return false;
  }
  // Step 1: 尝试一下老的 init（后端删掉 init 之后这一段直接 404 或报错，我们直接走 catch）
  try {
    //canister_Id  ,token_name, 质押后锁定时间，默认写0
    const initRes = await backend.stake_init(canisterId, "ICPUSDT", 0n);
    if ("Ok" in initRes) {
    }
    if ("Err" in initRes) {
      throw initRes.Err;
    }
    console.log("oldversion：init 成功", initRes);
  } catch (err) {
    // 只要 init 报任何错（包括方法不存在、already initialized、参数不对等等），我们都认为可以直接 stake
    // 什么都不用管，直接往下走就行
    console.error("stake_init", err);
    console.warn(
      "Initialization failed or user has already initialized, try staking directly (compatible with new versions)"
    );
  }

  // Step 2: 直接调用 stake（新老版本都支持这一个）
  try {
    const result = await backend.pcl_stake(canisterId, stakeAmountE8s + 0.0);

    if ("Ok" in result) {
      // 质押成功
      console.log("success stake");
      return true;
    } else {
      throw new Error(result.Err);
    }
  } catch (err) {
    console.error("stake failed", err);
    showMessageError("stake failed: " + err);
    throw new Error("stake failed" + err);
  }
}

export async function unstakePredictCanister(canisterId: string): Promise<any> {
  return getBackend().pcl_unstake(canisterId);
}
