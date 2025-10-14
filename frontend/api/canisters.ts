import type { ApiResult } from "@/types/types";
import { showMessageError, showMessageSuccess } from "@/utils/message";
import {
  blockCanisterArrayByPrincipal,
  getCanisterArrayByPrincipal,
  setCanisterArrayByPrincipal,
} from "@/utils/storage";
import { Actor } from "@dfinity/agent";
import {
  ICManagementCanister,
  canister_install_mode,
  chunk_hash,
} from "@dfinity/ic-management";
import { Principal } from "@dfinity/principal";
import {
  createIIAgent,
  getBackend,
  getCurrentPrincipal,
} from "./canister_pool";
import { CONTROLLER_CANISTERS_KEY } from "./icp";

// 定义 Canister 数据接口
export interface CanisterData {
  canisterId: string;
  status: "running" | "stopping" | "stopped" | "unknown";
  cycles: bigint;
  module_hash: [] | [Uint8Array | number[]];
  controllers: string[];
  cyclesConsumptionRate?: number;
  predictionAccuracy?: number;
  tokenBalance?: bigint | number;
  profitEarned?: bigint;
}

// 初始化 管理罐子，可操作拥有控制权的罐子
const initManage = () => {
  const agent = createIIAgent();
  return ICManagementCanister.create({
    agent,
  });
};

// 手动定义 Candid 接口
const userCanisterIdlFactory = ({ IDL }) => {
  const Result = IDL.Variant({
    Ok: IDL.Null, // 成功时无返回值
    Err: IDL.Text, // 失败时返回错误信息
  });
  return IDL.Service({
    set_train_params: IDL.Func([], [], []), // 无参数，无返回值
    set_start_predict: IDL.Func([], [Result], []), // 返回 Result
    set_stop_predict: IDL.Func([], [Result], []), // 返回 Result
    is_predict_running: IDL.Func([], [IDL.Bool], []),
  });
};

// 初始化 用户罐子，可操作用户拥有的罐子
const initTargetCanister = async (canisterId: string) => {
  const agent = createIIAgent();
  return Actor.createActor(userCanisterIdlFactory, {
    agent,
    canisterId: Principal.fromText(canisterId),
  });
};

//检查最新的系统版本号是多少，用于通知用户更新，添加新的api
export async function checkSystemLatestVersion() {
  // FunctionUpdate 功能性更新，返回系统的版本号 ; ModelUpdate 模型更新，返回模型的版本号
  const version = await getBackend().get_latest_version({
    FunctionUpdate: null,
  });
  return version;
}

/**
 * Set the canister list for the current principal
 * @param canisterId The canister ID to store
 */
export async function importCanisterList(canisterId: string): Promise<boolean> {
  // Trim and validate input
  const trimmedCanisterId = canisterId.trim();
  if (!trimmedCanisterId) {
    showMessageError("Canister ID cannot be empty");
    return false;
  }

  // 检查容器控制人是否是用户
  const status = await queryCanisterStatus(trimmedCanisterId);
  if (!status) {
    // 如果 status 为 null，说明查询失败或用户不是控制器
    return false;
  }

  try {
    Principal.fromText(trimmedCanisterId);
  } catch {
    showMessageError("Invalid Canister ID format");
    return false;
  }

  // Get current principal
  const principalId = getCurrentPrincipal();
  if (!principalId) {
    showMessageError("No valid principal ID found");
    return false;
  }

  // Store canister ID
  try {
    const success = setCanisterArrayByPrincipal(
      principalId,
      CONTROLLER_CANISTERS_KEY,
      trimmedCanisterId
    );
    if (success) {
      showMessageSuccess(
        `Canister ID ${trimmedCanisterId} stored successfully`
      );
    } else {
      showMessageError(`Canister ID ${trimmedCanisterId} already exists`);
    }
    return true;
  } catch (error) {
    console.error("Failed to store canister ID:", error);
    showMessageError("Failed to store Canister ID");
    return false;
  }
}

//获取用户的canister列表
export async function getCanisterList(): Promise<string[]> {
  const canisters = getCanisterArrayByPrincipal(
    CONTROLLER_CANISTERS_KEY,
    getCurrentPrincipal()
  );
  console.log("getCanisterList", getCurrentPrincipal(), canisters);
  if (canisters) return canisters;
  //TODO 调用后端api，查询线上的canister list是否一致

  // 没有结果就返回空
  return [];
}

//删除指定canister
export function blockCanisterIdFromList(canisterId: string) {
  const principalId = getCurrentPrincipal();
  blockCanisterArrayByPrincipal(
    principalId,
    CONTROLLER_CANISTERS_KEY,
    canisterId
  );
}

// 查询目标 Canister 的状态，目标canister的controller必须是用户本人
export async function queryCanisterStatus(
  canisterId: string
): Promise<any | null> {
  // 创建管理 Canister 的 Actor
  const managementCanister = initManage();
  try {
    const status = await managementCanister.canisterStatus(
      Principal.fromText(canisterId)
    );
    console.log("Canister Status:", status);
    return status;
  } catch (error: any) {
    console.error("Error querying canister status:", error);
    // Default error message for users
    let userMessage =
      "Failed to query Canister status. Please try again later.";

    // Parse error message for user-friendly feedback
    if (error.message) {
      if (error.message.includes("Only controllers of canister")) {
        userMessage = `You do not have permission to query the status of Canister ${canisterId}. Only the canister's controller can perform this action.`;
      } else if (error.message.includes("Invalid principal")) {
        userMessage = `The provided Canister ID ${canisterId} is invalid. Please check the ID and try again.`;
      } else {
        userMessage = `An error occurred while querying Canister status: ${error.message}`;
      }
    }
    showMessageError(userMessage);
    return null; // 返回 null 表示查询失败
  }
}

// 启用canister
export async function startCanister(canisterId: string) {
  // 创建管理 Canister 的 Actor
  const managementCanister = initManage();
  try {
    const res = await managementCanister.startCanister(
      Principal.fromText(canisterId)
    );
    console.log(`Canister ${canisterId} started successfully`, res);
    showMessageSuccess(`Canister ${canisterId} started successfully`);
  } catch (error) {
    console.error("Error querying canister status:", error);
  }
}

// Stop Canister
export async function stopCanister(canisterId: string): Promise<void> {
  const managementCanister = initManage();
  try {
    const res = await managementCanister.stopCanister(
      Principal.fromText(canisterId)
    );
    console.log(`Canister ${canisterId} stopped successfully`, res);
    showMessageSuccess(`Canister ${canisterId} stopped successfully`);
  } catch (error) {
    console.error(`Error stopping canister ${canisterId}:`, error);
    throw error;
  }
}

//防止直接使用返回值导致ts报错：不存在属性“Err”。类型“{ Ok: [] | [User]; }”上不存在属性“Err
export async function getWasmFile(
  name: string,
  version: string
): Promise<ApiResult<any>> {
  return getBackend().get_wasm_bin(name, version);
}

// 计算 SHA256 哈希并转换为十六进制字符串
async function computeSha256Hex(data: Uint8Array): Promise<string> {
  const hashBuffer = await crypto.subtle.digest("SHA-256", data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");
}

// 分块大小（1MB，低于 2MB 限制）
const CHUNK_SIZE = 1024 * 1024; // 1MB

/**
 * 分块上传 WASM 文件并安装到目标 Canister
 * @param canisterId 目标 Canister ID（字符串）
 * @param version WASM 文件版本号
 * @returns Promise<void> 安装成功返回 void，否则抛出错误
 */
export async function installCode(
  canisterId: string,
  wasm_name: string,
  version: string,
  mode: "install" | "upgrade" = "install" // 字符串参数，用于指定模式
): Promise<void> {
  const managementCanister = initManage();
  try {
    // 获取 WASM 文件
    const wasmResult = await getWasmFile(wasm_name, version);
    if (!wasmResult.Ok) {
      throw new Error(`Failed to retrieve WASM file: ${wasmResult.Err}`);
    }

    const wasmModule = wasmResult.Ok.wasm_bin[0];
    if (!wasmModule || wasmModule.length === 0) {
      throw new Error("Retrieved WASM file is empty");
    }
    console.log(
      `Retrieved WASM file for version ${version}, size: ${wasmModule.length} bytes`
    );

    const targetCanisterId = Principal.fromText(canisterId);
    // 确定安装代码的模式
    const installMode: canister_install_mode =
      mode === "install" ? { install: null } : { upgrade: [] };
    console.log("installMode", installMode);
    // 如果 WASM 小于 2MB，直接使用 installCode
    if (wasmModule.length <= 2097152) {
      await managementCanister.installCode({
        canisterId: targetCanisterId,
        wasmModule: wasmModule,
        arg: new Uint8Array([]),
        mode: installMode,
      });
      console.log(
        `Code installed successfully on canister ${canisterId} using installCode`
      );
      return;
    }

    // 分块上传
    const chunkHashesList: chunk_hash[] = [];
    const chunks: Uint8Array[] = [];
    for (let offset = 0; offset < wasmModule.length; offset += CHUNK_SIZE) {
      const chunk = wasmModule.subarray(offset, offset + CHUNK_SIZE);
      chunks.push(chunk);

      // 上传分块并获取管理 Canister 返回的哈希
      const chunkHash: chunk_hash = await managementCanister.uploadChunk({
        canisterId: targetCanisterId,
        chunk: chunk,
      });
      chunkHashesList.push(chunkHash);

      console.log(
        `Uploaded chunk ${chunks.length} of ${Math.ceil(
          wasmModule.length / CHUNK_SIZE
        )}, hash: ${chunkHash.hash}`
      );
    }

    // 计算整个 WASM 的哈希（十六进制字符串）
    const wasmModuleHash = await computeSha256Hex(wasmModule);

    // 调用 install_chunked_code
    await managementCanister.installChunkedCode({
      targetCanisterId,
      wasmModuleHash,
      chunkHashesList,
      arg: new Uint8Array([]),
      mode: installMode,
    });

    console.log(
      `Code installed successfully on canister ${canisterId} using install_chunked_code`
    );
  } catch (error) {
    console.error(`Error installing code on canister ${canisterId}:`, error);
    throw error;
  }
}

export const useRecommendSetTrainParam = async (
  canisterId: string
): Promise<void> => {
  console.log(
    `Calling set_train_params for canister ${canisterId} with params:`
  );
  try {
    const canisterActor = await initTargetCanister(canisterId);
    const res = await canisterActor.set_train_params();
    console.log("useRecommendSetTrainParam", res);
  } catch (error) {
    console.error(
      `Error calling set_train_params on canister ${canisterId}:`,
      error
    );
    throw error;
  }
};

//启动用户canister的预测，true为开始，false为停止
export const onPredict = async (
  canisterId: string,
  start: boolean
): Promise<void> => {
  try {
    const canisterActor = await initTargetCanister(canisterId);
    if (start) {
      const res = await canisterActor.set_start_predict();
      console.log("set_start_predict", res);
    } else {
      const res = await canisterActor.set_stop_predict();
      console.log("set_stop_predict", res);
    }
  } catch (error) {
    console.error(`Error calling onPredict on canister ${canisterId}:`, error);
    throw error;
  }
};

export async function checkIsPredictRunning(
  canisterId: string
): Promise<boolean> {
  try {
    const canisterActor = await initTargetCanister(canisterId);
    const res = await canisterActor.is_predict_running();
    return res as boolean;
  } catch (error) {
    console.error(
      `Error calling checkIsPredictRunning on canister ${canisterId}:`,
      error
    );
    const errorMessage = error instanceof Error ? error.message : String(error);
    if (
      errorMessage.includes(
        "Canister has no update method 'is_predict_running'"
      )
    ) {
      showMessageError(
        "Your canister version is outdated and requires updating."
      );
    } else {
      showMessageError(
        `Error calling checkIsPredictRunning on canister ${canisterId}: ${errorMessage}`
      );
    }
    throw error;
  }
}
