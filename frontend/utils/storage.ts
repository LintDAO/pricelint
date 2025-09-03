import { isPrincipal } from "./common";
import { showMessageError, showMessageSuccess } from "./message";

//通用存储方法
export const setStorage = (key: string, value: any) => {
  try {
    // 检查 key 是否为字符串
    if (typeof key !== "string" || key.trim() === "") {
      throw new Error("Storage key must be a non-empty string");
    }
    // 检查 value 是否是可以序列化的有效类型
    if (typeof value === "undefined") {
      throw new Error("Cannot store undefined value");
    }
    localStorage.setItem(key, JSON.stringify(value));
  } catch (error) {
    console.error("Error setting storage:", error);
  }
};

/**
 * 通用方法，用于向 localStorage 中的数组追加项，并避免重复
 * @param key 存储的键名
 * @param item 要追加的项（字符串类型，例如 Canister ID）
 * @returns 是否成功追加并存储（如果项已存在，返回 false）
 */
export const setArrayStorage = (key: string, item: string): boolean => {
  try {
    // 1. 从 localStorage 获取现有数组
    const existingData = getStorage(key);
    // 2. 确保返回值为数组，如果不是或为空，返回空数组
    const existingArray = Array.isArray(existingData) ? existingData : [];

    // 3. 检查是否已存在该项，避免重复添加
    if (existingArray.includes(item)) {
      console.log(`Item ${item} already exists in ${key}, skipping storage`);
      return false; // 未添加新项
    }

    // 4. 追加新项
    const updatedArray = [...existingArray, item];

    // 5. 存储更新后的数组
    setStorage(key, updatedArray);
    console.log(
      `Successfully stored ${item} in ${key}, updated array:`,
      updatedArray
    );
    return true; // 成功添加新项
  } catch (error) {
    console.error(`Failed to store item in ${key}:`, error);
    return false; // 存储失败
  }
};

//通用读取存储方法
export const getStorage = (key: string): any | null => {
  const value = localStorage.getItem(key);
  if (null == value) return null;
  try {
    return JSON.parse(value);
  } catch (e) {
    console.error(`read ${key} info failed:`, e);
  }
  return null;
};

export const deleteUserInfoStorage = (principal: string): void => {
  localStorage.removeItem(`CACHE_USER_INFO_${principal.toUpperCase()}`);
};

/**
 * 按 principalId 向 localStorage 中的数组追加 Canister ID，并避免重复
 * @param principalId 用户的 principalId
 * @param key 存储的键名
 * @param canisterId 要追加的 Canister ID（字符串类型）
 * @returns 是否成功追加并存储（如果项已存在，返回 false）
 */
export const setCanisterArrayByPrincipal = (
  principalId: string,
  key: string,
  canisterId: string
): boolean => {
  try {
    // 1. 从 localStorage 获取现有数据
    const existingData = getCanisterArrayByPrincipal(key);
    // 2. 确保返回值为对象，如果不是或为空，返回空对象
    const existingMap =
      typeof existingData === "object" && existingData !== null
        ? existingData
        : {};

    // 3. 获取当前 principalId 对应的数组，如果不存在则初始化为空数组
    const existingArray = Array.isArray(existingMap[principalId])
      ? existingMap[principalId]
      : [];

    // 4. 检查是否已存在该项，避免重复添加
    if (existingArray.includes(canisterId)) {
      console.log(
        `Canister ID ${canisterId} already exists for principal ${principalId} in ${key}, skipping storage`
      );
      return false; // 未添加新项
    }

    // 5. 追加新项到对应 principalId 的数组
    const updatedArray = [...existingArray, canisterId];

    // 6. 更新 principalId 对应的数组
    existingMap[principalId] = updatedArray;

    // 7. 存储更新后的对象
    setStorage(key, existingMap);
    console.log(
      `Successfully stored ${canisterId} for principal ${principalId} in ${key}, updated map:`,
      existingMap
    );
    return true; // 成功添加新项
  } catch (error) {
    console.error(
      `Failed to store canisterId for principal ${principalId} in ${key}:`,
      error
    );
    return false; // 存储失败
  }
};

/**
 * Remove a canister ID from the principal's list in localStorage
 * @param principalId The principal ID
 * @param key Storage key (e.g., CONTROLLER_CANISTERS_KEY)
 * @param canisterId The canister ID to remove
 * @returns True if removed successfully, false if the canisterId doesn't exist or error occurs
 */
export function removeCanisterArrayByPrincipal(
  principalId: string,
  key: string,
  canisterId: string
): boolean {
  try {
    if (!isPrincipal(principalId)) return false;
    // Get existing data
    const rawData = localStorage.getItem(key);
    const existingMap = rawData ? JSON.parse(rawData) : {};

    // Check if principalId exists and has a valid array
    const existingArray = Array.isArray(existingMap[principalId])
      ? existingMap[principalId]
      : [];
    if (!existingArray.includes(canisterId)) {
      showMessageError(`Canister ID ${canisterId} does not exist`);
      return false;
    }

    // Remove the canisterId
    const updatedArray = existingArray.filter(
      (id: string) => id !== canisterId
    );
    existingMap[principalId] = updatedArray;

    // Update storage
    localStorage.setItem(key, JSON.stringify(existingMap));
    console.log(
      `Successfully removed ${canisterId} for principal ${principalId} in ${key}`,
      existingMap
    );
    showMessageSuccess(`Canister ID ${canisterId} removed successfully`);
    return true;
  } catch (error) {
    console.error(
      `Failed to remove canisterId ${canisterId} for principal ${principalId} in ${key}:`,
      error
    );
    showMessageError("Failed to remove Canister ID");
    return false;
  }
}

/**
 * 按 principalId 从 localStorage 获取 Canister ID 数组
 * @param key 存储的键名
 * @param principalId 用户的 principalId（可选，如果不提供则返回整个对象）
 * @returns 指定 principalId 的 Canister ID 数组，或整个存储对象，或 null
 */
export const getCanisterArrayByPrincipal = (
  key: string,
  principalId?: string
): any | null => {
  const value = localStorage.getItem(key);
  if (value === null) return null;

  try {
    const parsed = JSON.parse(value);
    // 如果提供了 principalId，返回对应数组；否则返回整个对象
    if (principalId) {
      return Array.isArray(parsed?.[principalId]) ? parsed[principalId] : [];
    }
    return parsed;
  } catch (e) {
    console.error(`Failed to read ${key} info:`, e);
    return null;
  }
};
