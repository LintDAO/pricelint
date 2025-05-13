import icpLogo from "@/assets/dfinity.svg"
import { LEDGER_CANISTER_ID } from "./ic"

export const ICP_LOGO = icpLogo
export const TOKENS = {
  ICP: {
    canisters: {
      governance: "",
      index: "",
      ledger: LEDGER_CANISTER_ID,
      root: "",
      swap: "",
    },
    symbol: "ICP",
    name: "Internet Computer",
    decimals: 8,
    fee: 10000,
    meta: {
      description: "",
      logo: icpLogo,
      name: "",
      url: "https://dashboard.internetcomputer.org/",
    },
  },
  XTC: {
    symbol: "XTC",
    canisterId: "aanaa-xaaaa-aaaah-aaeiq-cai",
    name: "Cycles",
    decimals: 12,
  },
  WTC: {
    symbol: "WTC",
    canisterId: "5ymop-yyaaa-aaaah-qaa4q-cai",
    name: "Wrapped Cycles",
    decimals: 12,
  },
  WICP: {
    symbol: "WICP",
    canisterId: "utozz-siaaa-aaaam-qaaxq-cai",
    name: "Wrapped ICP",
    decimals: 8,
  },
  BTKN: {
    symbol: "BTKN",
    canisterId: "cfoim-fqaaa-aaaai-qbcmq-cai",
    name: "Beta Token",
    decimals: 8,
  },
  DUST: {
    symbol: "DUST",
    canisterId: "4mvfv-piaaa-aaaak-aacia-cai",
    name: "Dust Token",
    decimals: 8,
  },
}
