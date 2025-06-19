import { currencyCalculate } from "@/utils/common";
import { showMessageError } from "@/utils/message";
import axios from "axios";
import { ic } from "./canister_pool";
import { IC_LEDGER_URL } from "./constants/ic";

const currency = { decimals: 8, symbol: "ICP" };

//获得当前account id所持有的icp balance
export const getICPBalance = async (accountId: string): Promise<number> => {
  try {
    const url = `${IC_LEDGER_URL}/accounts/${accountId}`;
    const res = await axios.get(url);
    return currencyCalculate(res.data.balance, currency.decimals);
  } catch (error) {
    if (error instanceof Error) {
      console.error("getICPBalance Error:", error);
      showMessageError("getICPBalance Error: " + error.message);
    } else {
      console.error("getICPBalance Error:", error);
      showMessageError("getICPBalance Error: An unknown error occurred");
    }
    return 0;
  }
};

//获得当前principal id的cycles
export const getCyclesBalance = async (principal: string): Promise<number> => {
  // 查询 Cycles 余额
  let cyclesBalance = 0;
  try {
    const cyclesActor = await ic("aanaa-xaaaa-aaaah-aaeiq-cai");
    const cyclesResult = cyclesActor.balance({ account: principal });
    console.log("ccyclesResult", cyclesResult);
    cyclesBalance = Number(cyclesResult) / 1_000_000_000_000; // Convert to T Cycles
  } catch (error) {
    console.error("Failed to get Cycles balance:", error);
  }
  return cyclesBalance;
};
