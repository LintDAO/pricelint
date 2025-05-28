interface Market {
  name: string;
  logo: string;
}

export const MARKETS: Record<string, Market> = {
  BINANCE: {
    name: "BINANCE",
    logo: "/frontend/assets/icons/binance.svg",
  },
} as const;
