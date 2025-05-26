export type PredictionData = {
  staked: number;
  up: number;
  down: number;
};

export type TimePoint = {
  price: number;
  trend: string;
  pred: PredictionData | null;
};
