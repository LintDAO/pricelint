import { fromTokenAmount, p2a, toTokenAmount } from "@/utils/common";
import { showMessageError, showMessageSuccess } from "@/utils/message";
import { setCanisterArrayByPrincipal } from "@/utils/storage";
import { Actor } from "@dfinity/agent";
import { CMCCanister } from "@dfinity/cmc";
import { SubAccount } from "@dfinity/ledger-icp";
import { IcrcLedgerCanister } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";
import axios from "axios";
import { createIIAgent, getCurrentPrincipal } from "./canister_pool";
import {
  BINANCE_URL,
  CMC_CANISTER,
  CYCLES_LEDGER_CANISTER,
  ICP_LEDGER_CANISTER,
  IC_LEDGER_URL,
} from "./constants/ic";

const currency = { decimals: 8, symbol: "ICP" };
export const CONTROLLER_CANISTERS_KEY = "CONTROLLER_CANISTERS"; // 存储 Canister ID 数组的键

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

/**
 * 生成 CMC 操作的 memo 值（8 字节 Uint8Array）。
 * @param method CMC 方法名称，支持以下三种：
 * - "CREA": 用于 notify_create_canister，将 ICP 转换为 Cycles 并创建新 Canister，memo 为 blob "\43\52\45\41\00\00\00\00" (ASCII "CREA" + 4 个零字节)。
 * - "TPUP": 用于 notify_top_up，将 ICP 转换为 Cycles 为现有 Canister 充值，memo 为 blob "\54\50\55\50\00\00\00\00" (ASCII "TPUP" + 4 个零字节)。
 * - "MINT": 用于 notify_mint_cycles，将 ICP 转换为 Cycles 存入目标账户，memo 为 blob "\4d\49\4e\54\00\00\00\00" (ASCII "MINT" + 4 个零字节)。
 * @returns 8 字节的 Uint8Array，表示对应方法的 memo。
 */
const getMemoCode = (method: string) => {
  // 生成 memo
  const encoder = new TextEncoder();
  const methodBytes = encoder.encode(method); // 编码方法名（如 "CREA" -> [0x43, 0x52, 0x45, 0x41]）
  const memo = new Uint8Array(8); // 创建 8 字节数组
  memo.set(methodBytes, 0); // 将方法字节放入前 4 位，后 4 位自动为 0
  return memo;
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
    return fromTokenAmount(res.data.balance, currency.decimals);
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

//获得当前principal id的cycles，单位为T，注意，只能获取principal的，不能获取canister id的
export const getCyclesBalance = async (principal: string): Promise<number> => {
  try {
    const cyclesLedger = initIcrcLedger(CYCLES_LEDGER_CANISTER);
    const balance = await cyclesLedger.balance({
      owner: Principal.fromText(principal),
    });
    return Number(balance) / 1_000_000_000_000; // Convert to T Cycles
  } catch (error) {
    console.error("Failed to get Cycles balance:", error);
    showMessageError(`Failed to get Cycles balance: ${error}`);
    throw new Error(`Failed to get Cycles balance: ${error}`);
  }
};

//获取ICP转换为cycles的比率
export const getICPtoCyclesRate = async (): Promise<number> => {
  try {
    const cmc = initCmc();
    const rate = await cmc.getIcpToCyclesConversionRate();
    // 将 xdr_permyriad_per_icp 转换为 XDR/ICP
    const xdrPerIcp = fromTokenAmount(Number(rate), 4); // 精度为 4，因为是 per 10,000 ICP
    // 1 XDR = 1 万亿 cycles
    const cyclesPerIcpInTrillions = xdrPerIcp * 1; // 1 XDR = 1 T cycles
    return cyclesPerIcpInTrillions;
  } catch (error) {
    console.error(
      `[getICPtoCyclesRate] Failed to fetch ICP to cycles rate: ${error}`
    );
    showMessageError(
      `[getICPtoCyclesRate] Failed to fetch ICP to cycles rate: ${error}`
    );
    throw new Error("Unable to fetch ICP to cycles conversion rate");
  }
};

//将当前principal id的ICP转换为Cycles
export const burnICPcreateCanister = async (
  icpAmount: number
): Promise<boolean> => {
  try {
    const principal = getCurrentPrincipal();
    if (!principal)
      throw new Error("User not authenticated: Principal not found");
    const controller = Principal.fromText(principal);

    // 检查 ICP 余额够不够
    const icpBalance = await getICPBalance(p2a(principal));
    if (icpBalance < icpAmount) {
      throw new Error(
        `Insufficient ICP balance: ${icpBalance} ICP available, ${icpAmount} ICP required`
      );
    }

    // 将 ICP 转换为 e8s（1 ICP = 10^8 e8s）
    const amountE8s = toTokenAmount(icpAmount, currency.decimals);
    const subaccount = SubAccount.fromPrincipal(controller).toUint8Array(); // 生成 subaccount 的 Uint8Array
    const ledger = initIcrcLedger(ICP_LEDGER_CANISTER);

    // 调用 ICP ledger 的 transfer 方法，将 ICP 转到 CMC
    const blockIndex = await ledger.transfer({
      to: {
        owner: Principal.fromText(CMC_CANISTER),
        subaccount: [subaccount], // subaccount为本人的principal id
      },
      amount: amountE8s,
      memo: getMemoCode("CREA"), //表示此次转账用途是调用 notify_create_canister 方法
    });

    console.log(
      `Successfully converted ${icpAmount} ICP to Cycles, block index: ${blockIndex}`
    );
    const cmc = initCmc();
    const notifyResult = await cmc.notifyCreateCanister({
      block_index: blockIndex,
      controller,
      subnet_selection: [], // Default subnet
      settings: [], // Default settings
      subnet_type: [], // Default subnet type
    });
    console.log("create new", notifyResult);
    return true;
  } catch (error) {
    console.error("Failed to burn ICP:", error);
    showMessageError("Failed to burn ICP:" + error);
    throw error;
  }
};

/**
 * 为指定 Canister 充值 Cycles，通过将 ICP 转换为 Cycles。
 * @param canisterId 要充值的 Canister ID（字符串格式）
 * @param icpAmount 要转换的 ICP 数量
 * @returns Promise<boolean> 充值成功返回 true，否则抛出错误
 */
export const topupCycles = async (
  icpAmount: number,
  canisterId: string
): Promise<boolean> => {
  try {
    const principal = getCurrentPrincipal();
    if (!principal)
      throw new Error("User not authenticated: Principal not found");

    // 检查 ICP 余额
    const icpBalance = await getICPBalance(p2a(principal));
    if (icpBalance < icpAmount) {
      throw new Error(
        `Insufficient ICP balance: ${icpBalance} ICP available, ${icpAmount} ICP required`
      );
    }

    // 将 ICP 转换为 e8s（1 ICP = 10^8 e8s）
    const amountE8s = toTokenAmount(icpAmount, currency.decimals);
    const subaccount = SubAccount.fromPrincipal(
      Principal.fromText(principal)
    ).toUint8Array();
    const ledger = initIcrcLedger(ICP_LEDGER_CANISTER);

    // 调用 ICP ledger 的 transfer 方法，将 ICP 转到 CMC
    const blockIndex = await ledger.transfer({
      to: {
        owner: Principal.fromText(CMC_CANISTER),
        subaccount: [subaccount],
      },
      amount: amountE8s,
      memo: getMemoCode("TPUP"), // 表示此次转账用途是调用 notify_top_up 方法
    });

    console.log(
      `Successfully transferred ${icpAmount} ICP to CMC for top-up, block index: ${blockIndex}`
    );

    // 调用 CMC 的 notify_top_up 方法
    const cmc = initCmc();
    await cmc.notifyTopUp({
      block_index: blockIndex,
      canister_id: Principal.fromText(canisterId),
    });

    console.log(
      `Successfully topped up ${canisterId} with ${icpAmount} ICP worth of Cycles`
    );
    return true;
  } catch (error) {
    console.error(`Failed to top up Cycles for canister ${canisterId}:`, error);
    throw error;
  }
};

/**
 * 向指定 Principal 转账 ICP。
 * @param toPrincipal 接收方的 Principal ID（字符串格式）
 * @param icpAmount 要转账的 ICP 数量
 * @returns Promise<boolean> 转账成功返回 true，否则抛出错误
 */
export const transferICP = async (
  toPrincipal: string,
  icpAmount: number
): Promise<boolean> => {
  try {
    const principal = getCurrentPrincipal();
    if (!principal) {
      throw new Error("User not authenticated: Principal not found");
    }

    // 检查 ICP 余额
    const icpBalance = await getICPBalance(p2a(principal));
    if (icpBalance < icpAmount) {
      throw new Error(
        `Insufficient ICP balance: ${icpBalance} ICP available, ${icpAmount} ICP required`
      );
    }

    // 将 ICP 转换为 e8s（1 ICP = 10^8 e8s）
    const amountE8s = toTokenAmount(icpAmount, currency.decimals);
    const ledger = initIcrcLedger(ICP_LEDGER_CANISTER);

    // 调用 ICP ledger 的 transfer 方法
    const blockIndex = await ledger.transfer({
      to: {
        owner: Principal.fromText(toPrincipal),
        subaccount: [], // 默认使用接收方的主账户
      },
      amount: amountE8s,
    });

    console.log(
      `Successfully transferred ${icpAmount} ICP to ${toPrincipal}, block index: ${blockIndex}`
    );
    showMessageSuccess(
      `Successfully transferred ${icpAmount} ICP to ${toPrincipal}, block index: ${blockIndex}`
    );
    return true;
  } catch (error) {
    console.error(`Failed to transfer ICP to ${toPrincipal}:`, error);
    showMessageError(`Failed to transfer ICP to ${toPrincipal}:` + error);
    throw error;
  }
};

export const getTokenNowPrice = async (
  tokenSymbol: string
): Promise<number> => {
  try {
    //获取Binance的当前ICP历史数据。
    const url = `${BINANCE_URL}/api/v3/ticker/price`;
    //Symbol: ICP BTC...
    const params = { symbol: tokenSymbol.toUpperCase() + "USDT" };

    const response = await axios.get(url, { params });

    if (response.status === 200) {
      //直接返回价格
      return Number(response.data.price);
    } else {
      showMessageError(
        "Can not connect Binance api, please check if you have access to Binance or try later"
      );
      throw new Error("Failed to fetch ICP price data");
    }
  } catch (error) {
    showMessageError(
      "Can not connect Binance api, please check if you have access to Binance or try later"
    );
    console.error("Error fetching ICP price data:", error);
    throw error;
  }
};
