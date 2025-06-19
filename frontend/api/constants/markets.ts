import binanceLogo from "@/assets/icons/binance.svg";

interface Market {
  name: string;
  logo: string;
}

export const MARKETS: Record<string, Market> = {
  BINANCE: {
    name: "BINANCE",
    logo: binanceLogo,
  },
} as const;
