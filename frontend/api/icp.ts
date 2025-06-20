import { currencyCalculate, p2a } from "@/utils/common";
import { showMessageError } from "@/utils/message";
import { Actor, HttpAgent } from "@dfinity/agent";
import { IcrcLedgerCanister } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";
import axios from "axios";
import { getCurrentPrincipal } from "./canister_pool";
import { CYCLES_LEDGER_CANISTER, IC_LEDGER_URL } from "./constants/ic";

const currency = { decimals: 8, symbol: "ICP" };

// 最小化的Cycles Ledger Candid 接口
const cyclesLedgerIdlFactory = ({ IDL }) => {
  return IDL.Service({
    deposit: IDL.Func(
      [
        IDL.Record({
          to: IDL.Record({
            owner: IDL.Principal,
            subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
          }),
          memo: IDL.Opt(IDL.Vec(IDL.Nat8)),
          amount: IDL.Nat,
        }),
      ],
      [IDL.Nat],
      []
    ),
    create_canister: IDL.Func(
      [IDL.Record({ settings: IDL.Opt(IDL.Record({})) })],
      [IDL.Record({ canister_id: IDL.Principal })],
      []
    ),
  });
};

// 初始化 ICRC Ledger
const initIcrcLedger = (canisterId: string) => {
  const agent = new HttpAgent({ host: "https://ic0.app" });
  return IcrcLedgerCanister.create({
    agent,
    canisterId: Principal.fromText(canisterId),
  });
};

const initCyclesLedger = () => {
  const agent = new HttpAgent({ host: "https://ic0.app" });
  const canisterId = Principal.fromText(CYCLES_LEDGER_CANISTER);
  return Actor.createActor(cyclesLedgerIdlFactory, { agent, canisterId });
};

//获得当前account id所持有的icp balance
// 必须是account id，不支持principal id的查询
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
  try {
    const cyclesLedger = initIcrcLedger(CYCLES_LEDGER_CANISTER);
    const balance = await cyclesLedger.balance({
      owner: Principal.fromText(principal),
    });
    return Number(balance) / 1_000_000_000_000; // Convert to T Cycles
  } catch (error) {
    console.error("Failed to get Cycles balance:", error);
    throw new Error(`Failed to get Cycles balance: ${error}`);
  }
};

//将当前principal id的ICP转换为Cycles
export const burnICP = async (icpAmount: number): Promise<boolean> => {
  try {
    const principal = getCurrentPrincipal();
    console.log("princ", principal);
    if (!principal) throw new Error("未登录，无法获取 Principal");

    // 检查 ICP 余额
    const icpBalance = await getICPBalance(p2a(principal));
    if (icpBalance < icpAmount)
      throw new Error(
        `ICP 余额不足: 当前 ${icpBalance} ICP，需 ${icpAmount} ICP`
      );

    const cyclesLedger = initCyclesLedger();
    const amount = BigInt(Math.floor(icpAmount * 1_000_000_000_000)); // 1 ICP = 10^12 e8s
    console.log("amount", amount);
    if (amount < BigInt(100_000_000))
      throw new Error("金额需至少 0.0001 ICP 以覆盖手续费");

    const result = await cyclesLedger.deposit({
      to: {
        owner: Principal.fromText(principal),
        subaccount: [],
      },
      memo: [],
      amount,
    });
    console.log("Deposit result:", result);
    return true;
  } catch (error) {
    console.error("Failed to burn ICP:", error);
    throw error;
  }
};

// 创建新 Canister
export const createNewCanister = async (): Promise<boolean> => {
  try {
    const principal = getCurrentPrincipal();
    if (!principal) throw new Error("未登录，无法获取 Principal");

    // 检查 Cycles 余额
    const cyclesBalance = await getCyclesBalance(principal);
    if (cyclesBalance < 1) {
      // 如果 Cycles 不足，转换 1 ICP
      await burnICP(1);
    }

    const cyclesLedger = initCyclesLedger();
    const result = await cyclesLedger.create_canister({
      settings: [],
    });
    console.log("create result", result);
    return true;
  } catch (error) {
    console.error("Failed to create canister:", error);
    throw error;
  }
};
