interface PredictionData {
  staked: number;
  up: number;
  down: number;
}

interface TimePoint {
  price: number;
  trend: "Up" | "Down";
  pred: PredictionData | null;
}
