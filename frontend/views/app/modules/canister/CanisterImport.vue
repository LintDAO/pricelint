<template>
  <div>
    <q-card class="q-ma-md">
      <q-card-section>
        <div ref="chartRef" style="width: 100%; height: 400px"></div>
      </q-card-section>
    </q-card>
    <q-input v-model="tokenSymbol" label="Token Symbol" />
    <div class="q-mt-md q-gutter-md">
      <q-btn
        label="Get History"
        :loading="isLoadingHistory"
        @click="getTokenHistoryData"
      />
      <q-btn
        label="Import Data"
        color="primary"
        :loading="isLoadingImport"
        @click="importData"
      />
      <q-btn label="Export as JSON Data" color="primary" @click="exportJson" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { getTokenPriceHistory } from "@/api/token";
import * as echarts from "echarts";
import { ref } from "vue";

// Loading states
const isLoadingHistory = ref(false);
const isLoadingImport = ref(false);

const tokenSymbol = ref("");
const priceData = ref([]);

// 图表 DOM 引用和 ECharts 实例
const chartRef = ref<HTMLElement | null>(null);

// 初始化图表
const initChart = (data: [number, number][]) => {
  if (!chartRef.value) return;

  // 初始化 ECharts
  let chartInstance = echarts.init(chartRef.value);

  // 准备数据
  const dates = data.map((item) => new Date(item[0]).toLocaleDateString());
  const prices = data.map((item) => item[1]);

  // ECharts 配置
  const option = {
    title: {
      text: "Token Price History",
      left: "center",
    },
    tooltip: {
      trigger: "axis",
      formatter: (params: any) => {
        const param = params[0];
        return `${param.name}<br/>Price: $${param.value.toFixed(2)}`;
      },
    },
    xAxis: {
      type: "category",
      data: dates,
      name: "Date",
      axisLabel: {
        rotate: 45,
        interval: Math.floor(dates.length / 10) || "auto",
      },
    },
    yAxis: {
      type: "value",
      name: "Price (USDT)",
      axisLabel: {
        formatter: "${value}",
      },
    },
    series: [
      {
        name: "Price",
        type: "line",
        data: prices,
        smooth: true,
        lineStyle: {
          color: "#5470C6",
        },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: "rgba(84, 112, 198, 0.3)" },
            { offset: 1, color: "rgba(84, 112, 198, 0)" },
          ]),
        },
      },
    ],
    dataZoom: [
      {
        type: "slider",
        xAxisIndex: 0,
        start: 0,
        end: 100,
      },
      {
        type: "inside",
        xAxisIndex: 0,
      },
    ],
  };

  // 设置图表
  chartInstance.setOption(option);
};

const getTokenHistoryData = async () => {
  try {
    isLoadingHistory.value = true;
    priceData.value = await getTokenPriceHistory(tokenSymbol.value);
    console.log("price", priceData.value);
    initChart(priceData.value);
  } finally {
    isLoadingHistory.value = false;
  }
};

const importData = async () => {
  try {
    isLoadingImport.value = true;
    // Add your import data logic here
    // For example: await someImportFunction();
  } finally {
    isLoadingImport.value = false;
  }
};
//将数据导出为json文件
const exportJson = async () => {
  try {
    // Convert priceData ref to JSON string
    const jsonData = JSON.stringify(priceData.value, null, 2);

    // Create a Blob with the JSON data
    const blob = new Blob([jsonData], { type: "application/json" });

    // Create a temporary URL for the Blob
    const url = window.URL.createObjectURL(blob);

    // Create a temporary link element for downloading
    const link = document.createElement("a");
    link.href = url;
    link.download = "priceData.json"; // File name for download
    document.body.appendChild(link);
    link.click();

    // Clean up
    document.body.removeChild(link);
    window.URL.revokeObjectURL(url);
  } catch (error) {
    console.error("Error exporting JSON:", error);
  }
};
</script>

<style lang="scss" scoped></style>
