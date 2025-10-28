import { showMessageError } from "@/utils/message";
import { IDL } from "@dfinity/candid";
import axios from "axios";
import { getBackend } from "./canister_pool";
import { BINANCE_URL } from "./constants/ic";

export interface YearTimestamp {
  year: number | string;
  timestamp: { start: number; end: number };
}

export const getTokenPriceHistory = async (
  tokenSymbol: string
): Promise<any> => {
  try {
    //获取binance的所有ICP价格历史数据，目前coingecko只允许调用一年以内的数据，无法使用。
    const url = `${BINANCE_URL}/api/v3/klines`;
    let priceData = [];
    //由于币安一次只能请求500条数据，所以这里就分别请求每年的ICP价格历史再组装。
    for (const {
      timestamp: { start, end },
    } of getYearTimestamps()) {
      //获取从2021年开始的每年数据
      const params = {
        symbol: tokenSymbol.toUpperCase() + "USDT",
        startTime: start,
        endTime: end,
        interval: "1d",
      };
      const response = await axios.get(url, { params });
      if (response.status === 200) {
        // 解析响应数据response.data
        // [
        //   1499040000000, // k线开盘时间
        //   "0.01634790", // 开盘价
        //   "0.80000000", // 最高价
        //   "0.01575800", // 最低价
        //   "0.01577100", // 收盘价(当前K线未结束的即为最新价)
        //   "148976.11427815", // 成交量
        //   1499644799999, // k线收盘时间
        //   "2434.19055334", // 成交额
        //   308, // 成交笔数
        //   "1756.87402397", // 主动买入成交量
        //   "28.46694368", // 主动买入成交额
        //   "17928899.62484339", // 请忽略该参数
        // ]

        // priceData 目前只筛选使用开盘时间戳和开盘价
        const timestampAndPrice = response.data.map((item) => [
          item[0],
          Number(item[1]),
        ]);
        priceData = priceData.concat(timestampAndPrice);
      }
    }
    return priceData;
  } catch (error) {
    showMessageError(
      "Can not connect Binance api, please check if you have access to Binance or try later"
    );
    console.error(`Error fetching ${tokenSymbol} price data:`, error);
    throw error;
  }
};
// 可以根据年份返回对应的时间戳数组，以此来满足API的timerange
export const getYearTimestamps = (): YearTimestamp[] => {
  const currentYear = new Date().getFullYear();
  const timestamps: YearTimestamp[] = [];
  //从2021年上线开始，至今。
  for (let year = 2021; year <= currentYear; year++) {
    const startOfYear = new Date(year, 0, 1); // January 1st
    const endOfYear = new Date(year, 11, 31, 23, 59, 59); // December 31st, 23:59:59
    timestamps.push({
      year: year,
      timestamp: {
        start: startOfYear.getTime(),
        end: endOfYear.getTime(),
      },
    });
  }

  return timestamps;
};

// 定义 record 和 vec
const HistoryRecord = IDL.Record({
  time: IDL.Nat64,
  price: IDL.Float64,
});

/**
 * 按顺序分批导入历史记录（每批 200 条）
 * @param symbol 交易对，例如 ICPUSDT 或 BTCUSDT
 * @param history 原始历史数据，必须按时间升序排列：[time, price][]
 */
export async function importHistoryRecords(
  symbol: string,
  history: [number, number][]
): Promise<void> {
  const BATCH_SIZE = 200;
  const results: any[] = [];
  for (let i = 0; i < history.length; i += BATCH_SIZE) {
    const batch = history.slice(i, i + BATCH_SIZE);
    console.log("batch", batch);
    const blob = encodeHistoryToCandidBlob(batch); // 转为 blob
    console.log(blob);
    // 转换 time 为 BigInt，保持 price 不变
    // 串行调用：等待当前批次完成后再进行下一批
    const result = await getBackend().import_history_records(symbol, blob);
    console.log("res", i, result);
    results.push(result);
  }
  console.log("importReocrd", results);
  // 无返回值，或返回 void
}

/**
 * 把 [[number, number], ...] 序列化为 Candid blob
 */
function encodeHistoryToCandidBlob(history: [number, number][]): Uint8Array {
  // 转为 Candid 需要的格式: [{ time: bigint, price: number }, ...]
  const candidData = history.map(([time, price]) => ({
    time: BigInt(time),
    price: price,
  }));
  const HistoryVec = IDL.Vec(HistoryRecord);
  // 正确方式：使用 IDL.encode([type], [value])
  const buffer = IDL.encode([HistoryVec], [candidData]);

  return new Uint8Array(buffer);
}
