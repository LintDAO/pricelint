<template>
  <div class="canister-detail">
    <div class="header-section">
      <div class="header-content">
        <div class="canister-info">
          <div class="canister-avatar">
            <q-icon name="memory" size="48px" color="primary" />
          </div>
          <div class="canister-details">
            <h1 class="canister-title">Prediction Model Canister</h1>
            <div class="canister-meta">
              <span class="canister-id">{{ canisterId }}</span>
              <span class="separator">•</span>
              <span class="last-updated">
                Last updated {{ canisterData.lastUpdated }}
              </span>
            </div>
          </div>
        </div>
        <div class="header-actions">
          <q-btn
            @click="toCanisterEdit()"
            outline
            color="grey-8"
            label="Edit"
            class="action-btn"
          />
          <q-btn color="primary" label="Stake" class="action-btn" />
        </div>
      </div>

      <div class="status-section">
        <q-chip
          :color="getStatusColor(canisterData.status)"
          text-color="white"
          :label="canisterData.status"
          class="status-chip"
        />
      </div>
    </div>

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

    <!-- Prediction Chart -->
    <div class="chart-section">
      <div class="chart-header">
        <h2>Prediction Analysis</h2>
      </div>
      <div class="chart-container">
        <div ref="chartRef" class="chart"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import * as echarts from "echarts";
import { nextTick, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";

const router = useRouter();
const route = useRoute();

const canisterId = ref(route.params.canisterId as string);
// Reactive data
const chartView = ref("accuracy");
const chartRef = ref<HTMLElement>();
let chart: echarts.ECharts | null = null;
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
const chartData = ref({
  dates: [
    "Jan 6",
    "Jan 7",
    "Jan 8",
    "Jan 9",
    "Jan 10",
    "Jan 11",
    "Jan 12",
    "Jan 13",
    "Jan 14",
    "Jan 15",
  ],
  predictedValues: [12.5, 13.2, 12.8, 14.1, 13.9, 15.2, 14.7, 15.8, 15.1, 16.2],
  actualValues: [12.8, 13.1, 13.2, 14.3, 13.7, 15.1, 14.9, 15.6, 15.3, 16.0],
  correctPredictions: [1, 1, 0, 1, 0, 1, 1, 1, 1, 1], // 1 = correct, 0 = incorrect
  accuracyRate: [82, 85, 83, 87, 84, 86, 88, 87, 89, 87],
});

const toCanisterEdit = () => {
  router.push({
    path: `/app/canisters/edit/${canisterId.value}`,
  });
};

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

const formatCycles = (cycles: number) => {
  return (cycles / 1000000000000).toFixed(2) + "T";
};
const initChart = () => {
  if (!chartRef.value) return;

  chart = echarts.init(chartRef.value);

  const option = {
    tooltip: {
      trigger: "axis",
      backgroundColor: "rgba(255, 255, 255, 0.95)",
      borderColor: "#e5e7eb",
      textStyle: { color: "#374151" },
      formatter: function (params: any) {
        const dataIndex = params[0].dataIndex;
        const date = chartData.value.dates[dataIndex];
        const predicted = chartData.value.predictedValues[dataIndex];
        const actual = chartData.value.actualValues[dataIndex];
        const correct = chartData.value.correctPredictions[dataIndex]
          ? "Correct"
          : "Incorrect";
        const accuracy = chartData.value.accuracyRate[dataIndex];

        return `
          <div style="padding: 8px;">
            <div style="font-weight: 600; margin-bottom: 4px;">${date}</div>
            <div>Predicted: $${predicted}</div>
            <div>Actual: $${actual}</div>
            <div>Trend: <span style="color: ${
              chartData.value.correctPredictions[dataIndex]
                ? "#059669"
                : "#dc2626"
            }">${correct}</span></div>
            <div>Accuracy: ${accuracy}%</div>
          </div>
        `;
      },
    },
    legend: {
      data: ["Predicted Values", "Actual Values", "Accuracy Rate"],
      top: 10,
      textStyle: { color: "#374151" },
    },
    grid: {
      left: "3%",
      right: "8%",
      bottom: "3%",
      top: "15%",
      containLabel: true,
    },
    xAxis: {
      type: "category",
      data: chartData.value.dates,
      axisLine: { lineStyle: { color: "#e5e7eb" } },
      axisTick: { show: false },
      axisLabel: { color: "#6b7280" },
    },
    yAxis: [
      {
        type: "value",
        name: "Price ($)",
        position: "left",
        axisLine: { show: false },
        axisTick: { show: false },
        axisLabel: { color: "#6b7280" },
        splitLine: { lineStyle: { color: "#f3f4f6" } },
      },
      {
        type: "value",
        name: "Accuracy (%)",
        position: "right",
        min: 0,
        max: 100,
        axisLine: { show: false },
        axisTick: { show: false },
        axisLabel: { color: "#6b7280" },
        splitLine: { show: false },
      },
    ],
    series: [
      {
        name: "Predicted Values",
        type: "line",
        yAxisIndex: 0,
        data: chartData.value.predictedValues,
        smooth: true,
        lineStyle: { color: "#3b82f6", width: 2 },
        itemStyle: {
          color: "#3b82f6",
          borderWidth: 2,
          borderColor: "#ffffff",
        },
        symbol: "circle",
        symbolSize: 6,
      },
      {
        name: "Actual Values",
        type: "line",
        yAxisIndex: 0,
        data: chartData.value.actualValues,
        smooth: true,
        lineStyle: { color: "#10b981", width: 2 },
        itemStyle: {
          color: "#10b981",
          borderWidth: 2,
          borderColor: "#ffffff",
        },
        symbol: "circle",
        symbolSize: 6,
      },
      {
        name: "Accuracy Rate",
        type: "bar",
        yAxisIndex: 1,
        data: chartData.value.accuracyRate,
        itemStyle: {
          color: "rgba(168, 85, 247, 0.3)",
          borderColor: "#a855f7",
          borderWidth: 1,
        },
        barWidth: "40%",
      },
      {
        name: "Prediction Correctness",
        type: "scatter",
        yAxisIndex: 0,
        data: chartData.value.predictedValues.map((val, index) => ({
          value: [index, val],
          itemStyle: {
            color: chartData.value.correctPredictions[index]
              ? "#059669"
              : "#dc2626",
          },
        })),
        symbol: chartData.value.correctPredictions.map((correct) =>
          correct ? "circle" : "rect"
        ),
        symbolSize: 8,
        tooltip: { show: false },
      },
    ],
  };

  chart.setOption(option);
};
// Lifecycle
onMounted(() => {
  nextTick(() => {
    initChart();
  });
});

// Watch chart view changes
watch(chartView, () => {
  if (chart) {
    initChart();
  }
});
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

.chart-section {
  margin-bottom: 32px;
  padding: 24px;
  background: #ffffff;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.chart-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #111827;
  margin: 0;
}

.chart-container {
  height: 300px;
}

.chart {
  width: 100%;
  height: 100%;
}

.activity-section {
  padding: 24px;
  background: #ffffff;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
}

.activity-section h2 {
  font-size: 18px;
  font-weight: 600;
  color: #111827;
  margin: 0 0 24px 0;
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: #f9fafb;
  border-radius: 6px;
}

.activity-time {
  font-size: 12px;
  color: #6b7280;
  min-width: 80px;
}

.activity-content {
  flex: 1;
}

.activity-title {
  font-size: 14px;
  font-weight: 500;
  color: #111827;
  margin-bottom: 2px;
}

.activity-description {
  font-size: 13px;
  color: #6b7280;
}

.activity-result {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
}

.activity-result.correct {
  background: #dcfce7;
  color: #059669;
}

.activity-result.incorrect {
  background: #fee2e2;
  color: #dc2626;
}
</style>
