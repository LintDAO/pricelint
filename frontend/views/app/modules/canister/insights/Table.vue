<template>
  <div>
    <div class="text-h4">Table</div>
    <div class="chart-container">
      <q-skeleton v-if="isChartLoading" height="390px" />
      <div v-else ref="chartRef" class="chart"></div>
    </div>
    <q-card class="terminal-panel">
      <q-card-section class="q-pa-none">
        <pre
          ref="terminalOutput"
        ><code><span v-for="(log, index) in logs" :key="index" class="log-entry"><span class="canister-id">[{{ log.canisterId }}]</span><span class="timestamp">[{{ log.timestamp }}]</span><span class="message">{{ log.message }}</span></span></code></pre>
      </q-card-section>
    </q-card>
  </div>
</template>

<script setup lang="ts">
import * as echarts from "echarts";
import { onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";

// 获取 canisterId 从路由参数
const route = useRoute();
const chartRef = ref<HTMLElement>();
const canisterId = ref(route.params.canisterId as string);
const chartView = ref("accuracy");
let chart: echarts.ECharts | null = null;
const isChartLoading = ref(false);

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
  accuracyRate: [51, 49, 51, 52, 54, 55, 57, 56, 55, 57],
});

// 日志数据结构
const logs = ref([
  {
    canisterId: canisterId.value || "Unknown",
    timestamp: new Date().toISOString(),
    message: "Loading...",
  },
  {
    canisterId: canisterId.value || "Unknown",
    timestamp: new Date().toISOString(),
    message: "Loading... ",
  },
]);

// 添加日志
const addLog = (message, timestamp = new Date().toISOString()) => {
  logs.value.push({
    canisterId: canisterId.value || "Unknown",
    timestamp,
    message,
  });
  // 限制日志数量
  if (logs.value.length > 100) logs.value.shift();
  // // 自动滚动到底部
  // nextTick(() => {
  //   const output = terminalOutput.value;
  //   output.scrollTop = output.scrollHeight;
  // });
};

const terminalOutput = ref(null);

const initChart = () => {
  if (!chartRef.value) return;

  chart = echarts.init(chartRef.value);

  // 动态计算 y 轴范围
  const accuracyValues = chartData.value.accuracyRate;
  const minAccuracy = Math.floor(Math.min(...accuracyValues) - 2); // 减少缓冲区
  const maxAccuracy = Math.ceil(Math.max(...accuracyValues) + 2);

  const option = {
    title: {
      text: "Accuracy Rate Trend",
      subtext: "Last 30 Days",
      left: "center",
      top: 16,
      textAlign: "center",

      // === 主标题：深色、粗体、现代感 ===
      textStyle: {
        color: "#1f2937", // 深灰（与坐标轴标签协调）
        fontSize: 15,
        fontWeight: "bold",
        fontFamily: '"Inter", "Segoe UI", sans-serif',
      },

      // === 副标题：浅灰、小号、轻量 ===
      subtextStyle: {
        color: "#9ca3af",
        fontSize: 11,
        fontWeight: "normal",
      },

      // === 可选：轻微阴影（现代 UI 风格）===
      // 注意：ECharts 5+ 支持 shadow
      shadowBlur: 6,
      shadowColor: "rgba(99, 102, 241, 0.15)",
      shadowOffsetY: 2,
    },
    tooltip: {
      trigger: "axis",
      backgroundColor: "rgba(255, 255, 255, 0.95)",
      borderColor: "#d1d5db",
      borderRadius: 6,
      extraCssText: "box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);", // 使用 extraCssText 替代 boxShadow
      textStyle: { color: "#4b5563", fontSize: 11 },
      formatter: (params: any) => {
        const param = Array.isArray(params) ? params[0] : params;
        const dataIndex = param.dataIndex;
        const date = chartData.value.dates[dataIndex];
        const accuracy = chartData.value.accuracyRate[dataIndex];
        return `${date}<br/>Accuracy Rate: ${accuracy}%`;
      },
    },
    grid: {
      left: "5%", // 原来 8% → 改成对称
      right: "5%", // 原来 4% → 改成对称
      top: 60,
      bottom: "10%",
      containLabel: true,
    },
    xAxis: {
      type: "category",
      data: chartData.value.dates,
      axisLine: {
        lineStyle: { color: "#e5e7eb", width: 1 },
      },
      axisTick: { show: false },
      axisLabel: {
        color: "#6b7280",
        fontSize: 9,
        interval: 0,
        rotate: 45, // 旋转标签适配小屏幕
      },
    },
    yAxis: {
      type: "value",
      min: Math.max(0, minAccuracy),
      max: Math.min(100, maxAccuracy),
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: {
        color: "#6b7280",
        fontSize: 9,
        formatter: "{value}%",
      },
      splitLine: {
        lineStyle: {
          color: "#f3f4f6",
          type: "dashed",
        },
      },
    },
    series: [
      {
        name: "Accuracy Rate",
        type: "line",
        data: chartData.value.accuracyRate,
        smooth: true,
        lineStyle: {
          color: "#6366f1",
          width: 2,
          shadowColor: "rgba(99, 102, 241, 0.2)",
          shadowBlur: 3,
          shadowOffsetY: 1,
        },
        symbol: "none",
        areaStyle: {
          color: {
            type: "linear",
            x: 0,
            y: 0,
            x2: 0,
            y2: 1,
            colorStops: [
              { offset: 0, color: "rgba(99, 102, 241, 0.1)" },
              { offset: 1, color: "rgba(99, 102, 241, 0.02)" },
            ],
          },
        },
      },
    ],
    animation: true,
    animationDuration: 800,
    animationEasing: "cubicOut",
  } as echarts.EChartsOption; // 显式声明类型

  chart.setOption(option);
};

onMounted(() => {
  initChart();
});
watch(chartView, () => {
  if (chart) {
    initChart();
  }
});
</script>

<style scoped>
.terminal-panel {
  background-color: #0c0c0c; /* Campbell 背景色 */
  color: #cccccc; /* Campbell 默认文本色 */
  font-family: "Consolas", "Courier New", monospace;
  max-height: 400px;
  overflow: hidden;
  border-radius: 4px;
}

pre {
  height: 400px; /* 固定高度，模拟终端面板 */
  overflow-y: auto;
  margin: 0;
  padding: 16px;
  white-space: pre-wrap;
  line-height: 1.2; /* 紧凑行高 */
  box-sizing: border-box; /* 确保 padding 不影响高度 */
}
code {
  color: #cccccc;
}
.log-entry {
  display: block;
  margin: 0;
}
.canister-id {
  color: #4caf50; /* Campbell 绿色 */
}
.timestamp {
  color: #ffc107; /* Campbell 亮黄色 */
}
.message {
  color: #cccccc; /* Campbell 浅灰色 */
}

.chart-container {
  height: 400px;
}

.chart {
  width: 100%;
  height: 100%;
}
</style>
