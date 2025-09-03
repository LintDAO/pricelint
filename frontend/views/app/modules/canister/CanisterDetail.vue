<template>
  <div class="canister-detail">
    Header Section
    <div class="header-section">
      <div class="header-content">
        <div class="canister-info">
          <div class="canister-avatar">
            <q-icon name="memory" size="48px" color="primary" />
          </div>
          <div class="canister-details">
            <h1 class="canister-title">Prediction Model Canister</h1>
            <div class="canister-meta">
              <span class="canister-id">{{ canisterData.id }}</span>
              <span class="separator">•</span>
              <span class="owner">Owner: {{ canisterData.owner }}</span>
              <span class="separator">•</span>
              <span class="last-updated"
                >Last updated {{ canisterData.lastUpdated }}</span
              >
            </div>
          </div>
        </div>
        <div class="header-actions">
          <q-btn outline color="grey-8" label="Edit" class="action-btn" />
          <q-btn color="primary" label="Deploy" class="action-btn" />
        </div>
      </div>

      Status Badge
      <div class="status-section">
        <q-chip
          :color="getStatusColor(canisterData.status)"
          text-color="white"
          :label="canisterData.status"
          class="status-chip"
        />
        <q-chip
          color="green"
          text-color="white"
          label="On Sale"
          class="sale-chip"
          v-if="canisterData.tradingPair"
        />
      </div>
    </div>

    Stats Section
    <div class="stats-section">
      <div class="stat-item">
        <div class="stat-label">Cycles Balance</div>
        <div class="stat-value">
          {{ formatCycles(canisterData.cyclesBalance) }}
        </div>
        <div class="stat-change" :class="getCyclesChangeClass()">
          {{ canisterData.cyclesChange }}% from last week
        </div>
      </div>

      <div class="stat-item">
        <div class="stat-label">Total Stake</div>
        <div class="stat-value">{{ canisterData.totalStake }}</div>
        <div class="stat-change positive">
          +{{ canisterData.stakeChange }}% from last week
        </div>
      </div>

      <div class="stat-item">
        <div class="stat-label">Model Accuracy</div>
        <div class="stat-value">{{ canisterData.accuracy }}%</div>
        <div class="stat-change" :class="getAccuracyChangeClass()">
          {{ canisterData.accuracyChange }}% from last week
        </div>
      </div>
    </div>

    Model Info Section
    <div class="model-info-section">
      <div class="info-grid">
        <div class="info-item">
          <span class="info-label">Trading Pair:</span>
          <span class="info-value">{{ canisterData.tradingPair }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Model Version:</span>
          <span class="info-value">{{ canisterData.modelVersion }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Next Prediction:</span>
          <span
            class="info-value prediction-result"
            :class="getPredictionClass()"
          >
            {{ canisterData.nextPrediction }}
          </span>
        </div>
      </div>
    </div>

    Recent Predictions Table
    <div class="predictions-section">
      <h2 class="section-title">Recent Predictions</h2>

      <div class="table-container">
        <table class="predictions-table">
          <thead>
            <tr>
              <th>Date</th>
              <th>Predicted</th>
              <th>Actual</th>
              <th>Probability</th>
              <th>Result</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="prediction in predictionHistory" :key="prediction.date">
              <td class="date-cell">{{ formatDate(prediction.date) }}</td>
              <td
                class="prediction-cell"
                :class="getPredictionCellClass(prediction.predicted)"
              >
                {{ prediction.predicted }}
              </td>
              <td
                class="actual-cell"
                :class="getActualCellClass(prediction.actual)"
              >
                {{ prediction.actual }}
              </td>
              <td class="probability-cell">{{ prediction.probability }}%</td>
              <td class="result-cell">
                <q-icon
                  :name="prediction.correct ? 'check_circle' : 'cancel'"
                  :color="prediction.correct ? 'positive' : 'negative'"
                  size="20px"
                />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface CanisterData {
  id: string;
  owner: string;
  status: "Running" | "Stopped" | "Installing";
  cyclesBalance: number;
  cyclesChange: number;
  totalStake: string;
  stakeChange: number;
  accuracy: number;
  accuracyChange: number;
  tradingPair: string;
  modelVersion: string;
  nextPrediction: "Bullish" | "Bearish";
  lastUpdated: string;
}

interface PredictionRecord {
  date: string;
  predicted: "Bullish" | "Bearish";
  actual: "Bullish" | "Bearish";
  probability: number;
  correct: boolean;
}

const canisterData = ref<CanisterData>({
  id: "rrkah-fqaaa-aaaaa-aaaaq-cai",
  owner: "alice-principal-id",
  status: "Running",
  cyclesBalance: 1250000000000,
  cyclesChange: -2.1,
  totalStake: "45,230 ICP",
  stakeChange: 8.3,
  accuracy: 87.5,
  accuracyChange: 1.2,
  tradingPair: "ICP/USDT",
  modelVersion: "v2.1.3",
  nextPrediction: "Bullish",
  lastUpdated: "May 15, 2024 at 2:30 PM",
});

const predictionHistory = ref<PredictionRecord[]>([
  {
    date: "2024-05-15",
    predicted: "Bullish",
    actual: "Bullish",
    probability: 85,
    correct: true,
  },
  {
    date: "2024-05-14",
    predicted: "Bearish",
    actual: "Bearish",
    probability: 78,
    correct: true,
  },
  {
    date: "2024-05-13",
    predicted: "Bullish",
    actual: "Bearish",
    probability: 72,
    correct: false,
  },
  {
    date: "2024-05-12",
    predicted: "Bearish",
    actual: "Bearish",
    probability: 89,
    correct: true,
  },
  {
    date: "2024-05-11",
    predicted: "Bullish",
    actual: "Bullish",
    probability: 91,
    correct: true,
  },
  {
    date: "2024-05-10",
    predicted: "Bearish",
    actual: "Bullish",
    probability: 68,
    correct: false,
  },
  {
    date: "2024-05-09",
    predicted: "Bullish",
    actual: "Bullish",
    probability: 83,
    correct: true,
  },
  {
    date: "2024-05-08",
    predicted: "Bearish",
    actual: "Bearish",
    probability: 76,
    correct: true,
  },
]);

const getStatusColor = (status: string) => {
  switch (status) {
    case "Running":
      return "positive";
    case "Stopped":
      return "negative";
    case "Installing":
      return "warning";
    default:
      return "grey";
  }
};

const getCyclesChangeClass = () => {
  return canisterData.value.cyclesChange >= 0 ? "positive" : "negative";
};

const getAccuracyChangeClass = () => {
  return canisterData.value.accuracyChange >= 0 ? "positive" : "negative";
};

const getPredictionClass = () => {
  return canisterData.value.nextPrediction === "Bullish"
    ? "bullish"
    : "bearish";
};

const getPredictionCellClass = (prediction: string) => {
  return prediction === "Bullish" ? "bullish" : "bearish";
};

const getActualCellClass = (actual: string) => {
  return actual === "Bullish" ? "bullish" : "bearish";
};

const formatCycles = (cycles: number) => {
  return (cycles / 1000000000000).toFixed(2) + "T";
};

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
};
</script>

<style scoped>
.canister-detail {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
  background: #ffffff;
  min-height: 100vh;
}

.breadcrumb {
  margin-bottom: 24px;
}

.header-section {
  margin-bottom: 32px;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
}

.canister-info {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}

.canister-avatar {
  width: 64px;
  height: 64px;
  background: #f5f5f5;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.canister-title {
  font-size: 28px;
  font-weight: 600;
  color: #1f2937;
  margin: 0 0 8px 0;
  line-height: 1.2;
}

.canister-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #6b7280;
  font-size: 14px;
}

.canister-id {
  font-family: monospace;
  background: #f3f4f6;
  padding: 2px 6px;
  border-radius: 4px;
}

.separator {
  color: #d1d5db;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.action-btn {
  min-width: 80px;
}

.status-section {
  display: flex;
  gap: 8px;
}

.status-chip,
.sale-chip {
  font-weight: 500;
}

.stats-section {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 32px;
  margin-bottom: 32px;
  padding-bottom: 32px;
  border-bottom: 1px solid #e5e7eb;
}

.stat-item {
  text-align: left;
}

.stat-label {
  font-size: 14px;
  color: #6b7280;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 32px;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 4px;
  line-height: 1.2;
}

.stat-change {
  font-size: 14px;
  font-weight: 500;
}

.stat-change.positive {
  color: #059669;
}

.stat-change.negative {
  color: #dc2626;
}

.model-info-section {
  margin-bottom: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid #e5e7eb;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-label {
  font-size: 14px;
  color: #6b7280;
  font-weight: 500;
}

.info-value {
  font-size: 16px;
  color: #1f2937;
  font-weight: 600;
}

.prediction-result.bullish {
  color: #059669;
}

.prediction-result.bearish {
  color: #dc2626;
}

.predictions-section {
  margin-bottom: 32px;
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 16px;
}

.table-container {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow: hidden;
}

.predictions-table {
  width: 100%;
  border-collapse: collapse;
}

.predictions-table th {
  background: #f9fafb;
  padding: 12px 16px;
  text-align: left;
  font-weight: 600;
  color: #374151;
  font-size: 14px;
  border-bottom: 1px solid #e5e7eb;
}

.predictions-table td {
  padding: 12px 16px;
  border-bottom: 1px solid #f3f4f6;
  font-size: 14px;
}

.predictions-table tbody tr:last-child td {
  border-bottom: none;
}

.predictions-table tbody tr:hover {
  background: #f9fafb;
}

.date-cell {
  color: #6b7280;
  font-weight: 500;
}

.prediction-cell,
.actual-cell {
  font-weight: 600;
}

.prediction-cell.bullish,
.actual-cell.bullish {
  color: #059669;
}

.prediction-cell.bearish,
.actual-cell.bearish {
  color: #dc2626;
}

.probability-cell {
  color: #374151;
  font-weight: 500;
}

.result-cell {
  text-align: center;
}

@media (max-width: 768px) {
  .canister-detail {
    padding: 16px;
  }

  .header-content {
    flex-direction: column;
    gap: 16px;
  }

  .stats-section {
    grid-template-columns: 1fr;
    gap: 24px;
  }

  .info-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .table-container {
    overflow-x: auto;
  }

  .predictions-table {
    min-width: 600px;
  }
}
</style>
