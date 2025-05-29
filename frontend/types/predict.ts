export type PredictionData = {
  staked: number;
  up: number;
  down: number;
  trend: string; //预测中的趋势是上涨还是下跌
};

export type TimePoint = {
  price: number; //实际价格
  trend: string; //实际上的趋势是上涨还是下跌
  pred: PredictionData | null;
};
