import { getCurrentPrincipal } from "@/api/canister_pool";
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
 * 按 不同用户的 principalId 向 localStorage 中的数组追加字符串，并避免重复
 * @param principalId 用户的 principalId
 * @param key 存储的键名
 * @param string 要存储的字符串（字符串类型）
 * @returns 是否成功追加并存储（如果项已存在，返回 false）
 */
export const setStringArrayByPrincipal = (
  key: string,
  string: string
): boolean => {
  try {
    // 获取当前用户principal ID
    const principalId = getCurrentPrincipal();
    if (!principalId)
      throw new Error("User not authenticated: Principal not found");
    // 1. 从 localStorage 获取现有数据
    const existingData = getCanisterArrayByPrincipal(key); // 使用内部 get 函数获取整个 map，但不过滤
    // 2. 确保返回值为对象，如果不是或为空，返回空对象
    const existingMap =
      typeof existingData === "object" && existingData !== null
        ? existingData
        : {};

    // 3. 获取当前 principalId 对应的数组，如果不存在则初始化为空数组
    const existingArray = Array.isArray(existingMap[principalId])
      ? existingMap[principalId]
      : [];

    // 4. 检查是否已存在该项（无论是否屏蔽），避免重复添加
    if (
      existingArray.some((item: { string: string }) => item.string === string)
    ) {
      console.log(
        `${string} already exists for principal ${principalId} in ${key}, skipping storage`
      );
      return false; // 未添加新项
    }

    // 5. 追加新项到对应 principalId 的数组，初始为未屏蔽
    const updatedArray = [...existingArray, { string }];

    // 6. 更新 principalId 对应的数组
    existingMap[principalId] = updatedArray;

    // 7. 存储更新后的对象
    setStorage(key, existingMap);
    console.log(
      `Successfully stored ${string} for principal ${principalId} in ${key}, updated map:`,
      existingMap
    );
    return true; // 成功添加新项
  } catch (error) {
    console.error(
      `Failed to store string ${string} for principal in ${key}:`,
      error
    );
    return false; // 存储失败
  }
};

/**
 * 从 localStorage 获取 当前 principal ID 存储的 字符串 数组
 * @param key 存储的键名
 * @returns 指定 principalId 的字符串数组，或 null
 */
export const getStringArrayByPrincipal = (key: string): string[] | null => {
  const principalId = getCurrentPrincipal();
  if (!principalId) {
    throw new Error("User not authenticated: Principal not found");
  }
  const value = localStorage.getItem(key);
  if (value === null) return null;

  try {
    const parsed = JSON.parse(value);
    // 返回对应 principalId 的数组
    const array = Array.isArray(parsed?.[principalId])
      ? parsed[principalId]
      : [];
    // 直接映射为字符串数组（假设存储结构为 { string: string }）
    return array.map((item: { string: string }) => item.string);
  } catch (e) {
    console.error(`Failed to read ${key} info:`, e);
    return null;
  }
};

/**
 * 按用户的 principalId 向 localStorage 存储字符串
 * @param key 存储的键名
 * @param string 要存储的字符串
 * @returns 是否成功存储（如果已存在相同字符串，返回 false）
 */
export const setStringByPrincipal = (key: string, string: string): boolean => {
  try {
    const principalId = getCurrentPrincipal();
    if (!principalId)
      throw new Error("User not authenticated: Principal not found");

    const existingData = getCanisterArrayByPrincipal(key);
    const existingMap =
      typeof existingData === "object" && existingData !== null
        ? existingData
        : {};

    // 检查是否已存在相同字符串
    if (existingMap[principalId] === string) {
      console.log(
        `${string} already exists for principal ${principalId} in ${key}, skipping storage`
      );
      return false;
    }

    // 存储字符串
    existingMap[principalId] = string;
    setStorage(key, existingMap);
    console.log(
      `Successfully stored ${string} for principal ${principalId} in ${key}`
    );
    return true;
  } catch (error) {
    console.error(
      `Failed to store string ${string} for principal in ${key}:`,
      error
    );
    return false;
  }
};

/**
 * 从 localStorage 获取当前 principalId 存储的字符串
 * @param key 存储的键名
 * @returns 指定 principalId 的字符串，或 null
 */
export const getStringByPrincipal = (key: string): string | null => {
  const principalId = getCurrentPrincipal();
  if (!principalId)
    throw new Error("User not authenticated: Principal not found");

  const value = localStorage.getItem(key);
  if (value === null) return null;

  try {
    const parsed = JSON.parse(value);
    return typeof parsed?.[principalId] === "string"
      ? parsed[principalId]
      : null;
  } catch (e) {
    console.error(`Failed to read ${key} info:`, e);
    return null;
  }
};

/**
 * 按 principalId 向 localStorage 中的数组追加 Canister ID，并避免重复，默认不拉黑，也就是会显示
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
    const existingData = getCanisterArrayByPrincipal(key); // 使用内部 get 函数获取整个 map，但不过滤
    // 2. 确保返回值为对象，如果不是或为空，返回空对象
    const existingMap =
      typeof existingData === "object" && existingData !== null
        ? existingData
        : {};

    // 3. 获取当前 principalId 对应的数组，如果不存在则初始化为空数组
    const existingArray = Array.isArray(existingMap[principalId])
      ? existingMap[principalId]
      : [];

    // 4. 检查是否已存在该项（无论是否屏蔽），避免重复添加
    if (
      existingArray.some(
        (item: { canisterId: string; blocked: boolean }) =>
          item.canisterId === canisterId
      )
    ) {
      console.log(
        `Canister ID ${canisterId} already exists for principal ${principalId} in ${key}, skipping storage`
      );
      return false; // 未添加新项
    }

    // 5. 追加新项到对应 principalId 的数组，初始为未屏蔽
    const updatedArray = [...existingArray, { canisterId, blocked: false }];

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
 * Block (shield) a canister ID for the principal in localStorage, marking it as hidden without removing it
 * @param principalId The principal ID
 * @param key Storage key (e.g., CONTROLLER_CANISTERS_KEY)
 * @param canisterId The canister ID to block
 * @returns True if blocked successfully, false if the canisterId doesn't exist, already blocked, or error occurs
 */
export function blockCanisterArrayByPrincipal(
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
    const itemIndex = existingArray.findIndex(
      (item: { canisterId: string; blocked: boolean }) =>
        item.canisterId === canisterId
    );
    if (itemIndex === -1) {
      showMessageError(`Canister ID ${canisterId} does not exist`);
      return false;
    }
    if (existingArray[itemIndex].blocked) {
      showMessageError(`Canister ID ${canisterId} is already blocked`);
      return false;
    }

    // Block the canisterId by setting blocked to true
    existingArray[itemIndex].blocked = true;
    existingMap[principalId] = existingArray;

    // Update storage
    localStorage.setItem(key, JSON.stringify(existingMap));
    console.log(
      `Successfully blocked ${canisterId} for principal ${principalId} in ${key}`,
      existingMap
    );
    showMessageSuccess(`Canister ID ${canisterId} blocked successfully`);
    return true;
  } catch (error) {
    console.error(
      `Failed to block canisterId ${canisterId} for principal ${principalId} in ${key}:`,
      error
    );
    showMessageError("Failed to block Canister ID");
    return false;
  }
}

/**
 * 按 principalId 从 localStorage 获取 Canister ID 数组（默认只返回未屏蔽的 ID）
 * @param key 存储的键名
 * @param principalId 用户的 principalId（可选，如果不提供则返回整个对象）
 * @param includeBlocked 是否包含屏蔽的 ID（默认 false，只返回活跃的）
 * @returns 指定 principalId 的 Canister ID 数组（字符串数组），或整个存储对象，或 null
 */
export const getCanisterArrayByPrincipal = (
  key: string,
  principalId?: string,
  includeBlocked: boolean = false
): any | null => {
  const value = localStorage.getItem(key);
  if (value === null) return null;

  try {
    const parsed = JSON.parse(value);
    // 如果提供了 principalId，返回对应数组（过滤屏蔽的，除非 includeBlocked 为 true）
    if (principalId) {
      const array = Array.isArray(parsed?.[principalId])
        ? parsed[principalId]
        : [];
      const filteredArray = includeBlocked
        ? array
        : array.filter(
            (item: { canisterId: string; blocked: boolean }) => !item.blocked
          );
      return filteredArray.map(
        (item: { canisterId: string; blocked: boolean }) => item.canisterId
      );
    }
    return parsed; // 返回整个对象时不过滤
  } catch (e) {
    console.error(`Failed to read ${key} info:`, e);
    return null;
  }
};

/**
 * 从 localStorage 获取 当前 principal ID 和 canister ID 存储的字符串
 * @param key 存储的键名
 * @param canisterId canister 的 ID
 * @returns 指定 principalId 和 canisterId 的字符串，或 null
 */
export const getStringByPrincipalAndCanister = (
  key: string,
  canisterId: string
): string | null => {
  const principalId = getCurrentPrincipal();
  if (!principalId) {
    throw new Error("User not authenticated: Principal not found");
  }
  const value = localStorage.getItem(key);
  if (value === null) return null;

  try {
    const parsed = JSON.parse(value);
    // 返回对应 principalId 和 canisterId 的字符串
    return typeof parsed?.[principalId]?.[canisterId] === "string"
      ? parsed[principalId][canisterId]
      : null;
  } catch (e) {
    console.error(`Failed to read ${key} info for canister ${canisterId}:`, e);
    return null;
  }
};

/**
 * 按用户的 principalId 和 canisterId 向 localStorage 存储字符串
 * @param key 存储的键名
 * @param canisterId canister 的 ID
 * @param string 要存储的字符串
 * @returns 是否成功存储（如果已存在相同字符串，返回 false）
 */
export const setStringByPrincipalAndCanister = (
  key: string,
  canisterId: string,
  string: string
): boolean => {
  try {
    const principalId = getCurrentPrincipal();
    if (!principalId)
      throw new Error("User not authenticated: Principal not found");

    const existingData = localStorage.getItem(key);
    let existingMap: Record<string, Record<string, string>> = {};

    // 解析现有数据
    if (existingData) {
      try {
        existingMap = JSON.parse(existingData);
      } catch (e) {
        console.error(`Failed to parse existing data for ${key}:`, e);
      }
    }

    // 初始化 principalId 的结构
    if (!existingMap[principalId]) {
      existingMap[principalId] = {};
    }

    // 检查是否已存在相同字符串
    if (existingMap[principalId][canisterId] === string) {
      console.log(
        `String ${string} already exists for principal ${principalId} and canister ${canisterId} in ${key}, skipping storage`
      );
      return false;
    }

    // 存储字符串（覆盖旧值）
    existingMap[principalId][canisterId] = string;
    setStorage(key, existingMap);
    console.log(
      `Successfully stored string ${string} for principal ${principalId} and canister ${canisterId} in ${key}`
    );
    return true;
  } catch (error) {
    console.error(
      `Failed to store string ${string} for principal and canister ${canisterId} in ${key}:`,
      error
    );
    return false;
  }
};
