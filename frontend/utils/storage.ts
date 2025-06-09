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
