import { getStorage } from "@/utils/storage";
import { ICManagementCanister } from "@dfinity/ic-management";
import { Principal } from "@dfinity/principal";
import { createIIAgent } from "./canister_pool";
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

//获取用户的canister列表
export async function getCanisterList(): Promise<string[]> {
  const canisters = getStorage(CONTROLLER_CANISTERS_KEY);
  if (canisters) return canisters;
  //TODO 调用后端api，查询线上的canister list是否一致

  // 没有结果就返回空
  return [];
}

// 查询目标 Canister 的状态，目标canister的controller必须是用户本人
export async function queryCanisterStatus() {
  // 创建管理 Canister 的 Actor
  const managementCanister = initManage();
  try {
    const status = await managementCanister.canisterStatus(
      Principal.fromText("dxegq-jyaaa-aaaab-qb2wq-cai")
    );
    console.log("Canister Status:", status);
    return status;
  } catch (error) {
    console.error("Error querying canister status:", error);
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
  } catch (error) {
    console.error(`Error stopping canister ${canisterId}:`, error);
    throw error;
  }
}

// Install Code
export async function installCode(
  canisterId: string,
  wasmFile: File
): Promise<void> {
  const managementCanister = initManage();
  try {
    const wasmModule = await wasmFile.arrayBuffer();
    const wasmBuffer = new Uint8Array(wasmModule);
    await managementCanister.installCode({
      canisterId: Principal.fromText(canisterId),
      wasmModule: wasmBuffer,
      arg: new Uint8Array([]),
      mode: { install: null }, // or { reinstall: null } or { upgrade: null }
    });
    console.log(`Code installed successfully on canister ${canisterId}`);
  } catch (error) {
    console.error(`Error installing code on canister ${canisterId}:`, error);
    throw error;
  }
}
