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
