import icpLogo from "@/assets/dfinity.svg";
import btcLogo from "@/assets/icons/BTC.svg";
import ethLogo from "@/assets/icons/ETH.svg";
import { LEDGER_CANISTER_ID } from "./ic";

export const ICP_LOGO = icpLogo;
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
    decimals: 8,
    fee: 10000,
    meta: {
      description: "",
      logo: icpLogo,
      name: "Internet Computer",
      url: "https://dashboard.internetcomputer.org/",
    },
  },
  BTC: {
    symbol: "BTC",
    canisterId: "",
    meta: {
      description: "",
      logo: btcLogo,
      name: "Bitcoin",
      url: "",
    },
  },
  ETH: {
    symbol: "ETH",
    canisterId: "",
    meta: {
      description: "",
      logo: ethLogo,
      name: "Ethereum",
      url: "",
    },
  },
};
