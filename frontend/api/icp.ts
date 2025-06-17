import axios from "axios";
import { IC_LEDGER_URL } from "./constants/ic";
import { showMessageError } from "@/utils/message";
import { currencyCalculate } from "@/utils/common";

const currency = { decimals: 8, symbol: "ICP" };

//获得当前account id所持有的icp balance
export const getICPBalance = async (accountId: string): Promise<number> => {
  try {
    const url = `${IC_LEDGER_URL}/accounts/${accountId}`;
    const res = await axios.get(url);
    console.log("getICPBalance", res.data);
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
