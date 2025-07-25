// 指定一些缓存时间
export class TTL {
  static minute1 = 60;
  static minute10 = 60 * 10;
  static minute15 = 60 * 15;
  static minute30 = 60 * 30;
  static hour1 = 60 * 60;
  static hour2 = 60 * 60 * 2;
  static hour4 = 60 * 60 * 4;
  static hour6 = 60 * 60 * 6;
  static hour12 = 60 * 60 * 12;
  static day1 = 60 * 60 * 24;
  static day3 = 60 * 60 * 24 * 3;
  static day7 = 60 * 60 * 24 * 7;

  static minute = (minute: number) => 60 * minute;
  static hour = (hour: number) => 60 * 60 * hour;
  static day = (day: number) => 60 * 60 * 24 * day;
}

type CacheItem = {
  value: any;
  expired: number; // 毫秒过期时间
};
// 存储进行中的请求，以避免重复请求
const ongoingRequests: Record<string, Promise<any> | null> = {};

//内存变量
const CACHE_DATA = {};

// 递归规范化数据中的 Principal 和 bigint， 将所有 Principal 全改为 string
function normalizeData(value: any): any {
  if (value === null || value === undefined) {
    return value;
  }
  if (typeof value === "bigint") {
    return Number(value);
  }
  if (
    value &&
    (value._isPrincipal || value?.constructor?.name === "Principal")
  ) {
    return value.toString();
  }
  if (value && typeof value === "object" && "__principal__" in value) {
    return value.__principal__;
  }
  if (Array.isArray(value)) {
    return value.map(normalizeData);
  }
  if (typeof value === "object") {
    const result: Record<string, any> = {};
    for (const key in value) {
      result[key] = normalizeData(value[key]);
    }
    return result;
  }
  return value;
}

// 直接获取本地储存的带过期时间的数据进行验证，如果没有才发出网络请求
//TODO 方法返回值必须是res.Ok才会激活本地缓存。注意，现在好像不是硬需求res.Ok也行了
export async function getCache(info: {
  key: string; // key: 设置storage里的key，注意：key 里面应当包含执行方法的参数信息，不同的参数不能共用一个 key
  execute: () => Promise<any>; // execute: 传入执行方法
  ttl?: number; // ttl: Time To Live 过期时长，秒为单位，比如xx秒以后过期
  isLocal?: boolean; //是否存在LocalStorage中，如果否，则存在内存中。
  timeout?: number; // 超时限制，如果网络请求时间实在太长，就提示错误
  refresh?: boolean; // 是否刷新旧缓存，如果 refresh 是 true，那么表明不使用缓存而是加载新的数据，再将新的数据缓存
  console?: boolean; // 是否打印返回值，方便查看
  notice?: (_fromCaching: boolean) => void; // 万一上级需要判断是否从缓存中读取，因此需要额外通知数据
  update?: boolean; // 当update为true时，立即返回之前缓存的数据，并且在后台异步地加载新的数据，并在加载完成后更新缓存
  updatedCallback?: (_data: any) => void; // 异步更新成功是否需要回调
}): Promise<any> {
  const key = "CACHE_" + info.key;
  let data;

  // 判断是否需要刷新缓存
  if (info.refresh) {
    data = null;
  } else {
    data = getExpiredData(key, info.isLocal || false);
    if (data) {
      data = normalizeData(data); // 规范化缓存数据
    }
  }
  // data = null; // 启用则关闭缓存功能，方便测试
  // 如果缓存中已有数据，直接返回缓存
  if (data) {
    if (info.console)
      console.log(info.execute.toString() + " cache have data", data);
    if (info.notice) info.notice(true);
    // 如果允许异步更新，则返回数据并异步更新
    if (info.update) {
      setTimeout(async () => {
        const d = await info.execute();
        const normalizedD = normalizeData(d);
        if (d) {
          setExpiredData(
            key,
            normalizedD,
            info.ttl || 60 * 60,
            info.isLocal || false
          );
          if (info.updatedCallback) info.updatedCallback(normalizedD);
        }
      }, 0);
    }
    return data;
  }

  // 检查是否已有正在进行的请求，免得重复请求一种api链接，例如一百条历史交易记录同时请求100次ghost的价格API，现在只会请求一次
  if (ongoingRequests[key]) {
    return ongoingRequests[key];
  }
  // 发起新的请求，并设置超时机制
  const timeout = info.timeout ?? 30000;
  ongoingRequests[key] = new Promise((resolve, reject) => {
    const start = Date.now();
    let flag = false;

    const timeoutCallback = () => {
      delete ongoingRequests[key];
      reject(`getCache timeout: ${key}`);
    };

    setTimeout(() => !flag && timeoutCallback(), timeout);

    info
      .execute()
      .then((d) => {
        if (Date.now() <= start + timeout) {
          const normalizedD = normalizeData(d); // 规范化执行结果
          if (d) {
            setExpiredData(
              key,
              normalizedD,
              info.ttl || 60 * 60,
              info.isLocal || false
            );
          }
          resolve(normalizedD);
        } else {
          timeoutCallback(); // 即使获取到内容，超时了，也不接受
        }
      })
      .catch((error) => reject(error))
      .finally(() => {
        flag = true;
        delete ongoingRequests[key]; // 清除请求状态
      });
  });

  // 获取结果并返回
  try {
    data = await ongoingRequests[key];
    if (info.notice) info.notice(false);
    if (info.console)
      console.log(info.execute.toString() + " result data", data);
    return data;
  } catch (error) {
    if (info.notice) info.notice(false);
    throw error;
  }
}

// 保存值到缓存
// key: 设置storage里的key
// value: 传入要存储的值
// ttl: Time To Live 过期时长，秒为单位，比如xx秒以后过期
// isLocal: true为LocalStorage，false为非LocalStorage
const setExpiredData = (
  key: string,
  value: any,
  ttl: number,
  isLocal: boolean
): void => {
  const now = new Date().getTime();
  // 1. 加上过期时间
  const item: CacheItem = {
    value: value,
    expired: now + ttl * 1000,
  };
  // 2. 将数据存在内存变量里，以便不用每次从 localStorage 中读取
  CACHE_DATA[key] = item;
  // 3. 如果需要存储至 localStorage
  if (isLocal) {
    localStorage.setItem(key, JSON.stringify(item));
  }
};

// 获取包含过期时间和使用次数的localStorage
const getExpiredData = (key: string, isLocal: boolean): any => {
  const now = new Date().getTime();
  // 1. 先读取内存中的值
  let item = CACHE_DATA[key] as CacheItem;
  if (item) {
    // 比对是否过期
    if (item.expired < now) {
      // 已经过期 需要删除
      delete CACHE_DATA[key];
      localStorage.removeItem(key);
      return null;
    } else {
      return item.value;
    }
  }
  if (!isLocal) {
    // 未存本地则不继续找了
    return null;
  }
  // 2. 内存中没有，再读取持久化的信息
  const itemString = localStorage.getItem(key);
  if (!itemString) {
    return null;
  }
  item = JSON.parse(itemString) as CacheItem;
  if (item.expired == undefined || item.expired < now) {
    // 没有过期时间 直接删除
    // 已经过期 需要删除
    delete CACHE_DATA[key];
    localStorage.removeItem(key);
    return null;
  }
  // 2.1 将持久化的信息在内存中存一份
  CACHE_DATA[key] = item;
  return item.value;
};

// 清除指定缓存，有些数据需要更新，有缓存情况下，影响显示
export const clearCacheData = (key: string): void => {
  key = "CACHE_" + key;
  console.error("clear cache data for key: " + key);
  delete CACHE_DATA[key];
  localStorage.removeItem(key);
};
