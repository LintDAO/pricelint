import axios from "axios";
import { BINANCE_URL } from "./constants/ic";
import { showMessageError } from "@/utils/message";

export interface YearTimestamp {
  year: number | string;
  timestamp: { start: number; end: number };
}

export const getTokenPriceHistory = async (): Promise<any> => {
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
        symbol: "ICPUSDT",
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

        // priceData 包含时间戳和价格数据
        const timestampAndPrice = response.data.map((item) => [
          item[0],
          Number(item[1]),
        ]);
        priceData = priceData.concat(timestampAndPrice);
      }
    }
    console.log("priceData", priceData);
    return priceData;
  } catch (error) {
    showMessageError(
      "Can not connect Binance api, please check if you have access to Binance or try later"
    );
    console.error("Error fetching ICP price data:", error);
    throw error;
  }
};

export const getYearTimestamps = (): YearTimestamp[] => {
  const currentYear = new Date().getFullYear();
  const timestamps: YearTimestamp[] = [];
  //从ICP的2021年上线开始，至今。
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
