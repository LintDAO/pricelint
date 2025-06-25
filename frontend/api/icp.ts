import { currencyCalculate, p2a } from "@/utils/common";
import { showMessageError } from "@/utils/message";
import { Actor } from "@dfinity/agent";
import { CMCCanister } from "@dfinity/cmc";
import { IcrcLedgerCanister } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";
import axios from "axios";
import { createIIAgent, getCurrentPrincipal } from "./canister_pool";
import {
  CMC_CANISTER,
  CYCLES_LEDGER_CANISTER,
  ICP_LEDGER_CANISTER,
  IC_LEDGER_URL,
} from "./constants/ic";

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
  const agent = createIIAgent();
  return IcrcLedgerCanister.create({
    agent,
    canisterId: Principal.fromText(canisterId),
  });
};

// 初始化 CMC
const initCmc = () => {
  const agent = createIIAgent();
  return CMCCanister.create({
    agent,
    canisterId: Principal.fromText(CMC_CANISTER),
  });
};

const initCyclesLedger = () => {
  const agent = createIIAgent();
  return Actor.createActor(cyclesLedgerIdlFactory, {
    agent,
    canisterId: Principal.fromText(CYCLES_LEDGER_CANISTER),
  });
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

    //TODO 这数字不对，这是除，不是乘，应该乘
    const amount = currencyCalculate(icpAmount, currency.decimals);
    // 将 ICP 转换为 e8s（1 ICP = 10^8 e8s）
    const amountE8s = BigInt(Math.floor(icpAmount * 100_000_000));

    console.log("amount", amount);

    if (amountE8s < currencyCalculate(0.0001, currency.decimals))
      throw new Error("金额需至少 0.0001 ICP 以覆盖手续费");
    const ledger = initIcrcLedger(ICP_LEDGER_CANISTER);

    // 调用 ICP ledger 的 transfer 方法，将 ICP 转到 CMC
    const blockindex = await ledger.transfer({
      to: {
        owner: Principal.fromText(CMC_CANISTER),
        subaccount: [],
      },
      amount: amountE8s,
      memo: new Uint8Array([80, 80]), // 0x5050
      created_at_time: BigInt(Date.now() * 1_000_000),
    });

    console.log(`Converted ${icpAmount} ICP to cycles`, blockindex);
    const cmc = initCmc();
    const notifyResult = await cmc.notifyCreateCanister({
      block_index: blockindex,
      controller: Principal.fromText(principal),
      subnet_selection: [], // Default subnet
      settings: [], // Default settings
      subnet_type: [], // Default subnet type
    });
    console.log("create new", notifyResult);
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
      await burnICP(0.001);
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
