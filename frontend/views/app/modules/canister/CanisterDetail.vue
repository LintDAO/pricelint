<template>
  <div class="project-overview color-mask">
    <q-banner v-if="showBanner" inline-actions class="banner-container q-mb-md">
      <template v-slot:avatar>
        <q-icon name="info" class="banner-icon" />
      </template>
      <div class="banner-text">
        Your Canister System version ({{ currentVersion }}) is outdated. Please
        upgrade to the latest version ({{ latestVersion }}) for better
        performance and security.
      </div>
      <template v-slot:action>
        <q-btn
          flat
          class="banner-btn-primary"
          label="Upgrade"
          @click="handleUpgrade"
        />
        <q-btn
          flat
          class="banner-btn-secondary"
          label="Dismiss"
          @click="handleDontShowAgain"
        />
      </template>
    </q-banner>
    <div class="overview-container">
      <!-- Left Sidebar - Project Info -->
      <div class="project-sidebar q-pt-md">
        <div class="project-card">
          <!-- Project Preview -->
          <div class="project-preview">
            <div class="chart-container">
              <q-skeleton v-if="isStatusLoading" height="200px" />
              <div v-else ref="chartRef" class="chart"></div>
            </div>
          </div>

          <!-- Project Details -->
          <div class="project-details">
            <q-skeleton
              v-if="isStatusLoading"
              type="text"
              class="q-mb-sm"
              width="150px"
            />
            <div v-else class="project-type">Model Version</div>
            <q-skeleton
              v-if="isStatusLoading"
              type="text"
              class="q-mb-sm"
              width="200px"
            />
            <div v-else class="text-h6 q-mb-sm">
              {{ canisterData.modelVersion }}
            </div>

            <div class="project-meta">
              <div class="meta-item">
                <span class="meta-label">Canister Id</span>
                <div class="meta-value">
                  <q-icon name="link" size="16px" class="link-icon" />
                  <a
                    :href="`${IC_DASHBOARD_URL}/canister/${canisterId}`"
                    target="_blank"
                    class="project-url"
                  >
                    {{ canisterId }}
                  </a>
                </div>
              </div>

              <div class="meta-row">
                <div class="meta-item">
                  <span class="meta-label">Status</span>
                  <div class="meta-value">
                    <q-skeleton v-if="isStatusLoading" type="QChip" />
                    <q-chip
                      v-else
                      :color="getStatusColor(canisterData.status)"
                      text-color="white"
                      size="sm"
                      :label="canisterData.status"
                    />
                  </div>
                </div>
                <div class="meta-item">
                  <span class="meta-label">Token Pair</span>
                  <div class="meta-value">
                    <q-skeleton v-if="isStatusLoading" type="QChip" />
                    <q-chip
                      v-else
                      color="blue"
                      text-color="white"
                      size="sm"
                      :label="canisterData.tradingPair"
                    />
                  </div>
                </div>
              </div>
              <div>
                <div class="meta-item">
                  <span class="meta-label">Next:</span>
                  <div class="meta-value">
                    <q-skeleton
                      v-if="isStatusLoading"
                      type="text"
                      width="100px"
                    />
                    <span v-else>{{ canisterData.nextPrediction }}</span>
                  </div>
                </div>
                <div class="meta-item">
                  <span class="meta-label">Update:</span>
                  <div class="meta-value">
                    <q-skeleton
                      v-if="isStatusLoading"
                      type="text"
                      width="100px"
                    />
                    <span v-else>{{ canisterData.lastUpdated }}</span>
                  </div>
                </div>
              </div>
            </div>

            <div class="site-structure">
              <div class="structure-header">
                <span class="structure-title">Actions</span>
              </div>
              <q-skeleton v-if="isStatusLoading" type="QBtn" width="120px" />
              <template v-else>
                <div class="q-gutter-md">
                  <q-btn
                    v-if="canisterData.status !== 'Predicting'"
                    label="Start Predict"
                    color="primary"
                    no-caps
                    :loading="startPredictLoading"
                    @click="startPredict(true)"
                  />
                  <q-btn
                    v-else
                    label="Stop Predict"
                    color="negative"
                    no-caps
                    :loading="startPredictLoading"
                    @click="startPredict(false)"
                  />
                  <q-btn
                    label="Stake"
                    color="primary"
                    no-caps
                    :loading="startPredictLoading"
                    @click="
                      isStakeMode = true;
                      openStakeDialog = true;
                    "
                  />
                  <q-btn
                    @click="
                      isStakeMode = false;
                      openStakeDialog = true;
                    "
                  >
                    Unstake
                  </q-btn>
                </div>
              </template>
            </div>
          </div>
        </div>
      </div>
      <div class="main-content">
        <!-- Insights Section -->
        <div>
          <div class="insights-header q-pt-md">
            <h4 class="section-title">
              Insights <span class="period-text">Last 7 days</span>
            </h4>

            <div class="insights-period">
              <q-btn
                flat
                dense
                color="primary"
                label="Setting"
                size="sm"
                icon-right="arrow_forward"
                :to="`/app/canisters/${canisterId}/edit`"
              />
            </div>
          </div>

          <div class="insights-grid">
            <!-- Model Accuracy Card -->
            <div class="insight-card">
              <div v-if="isDataLoading">
                <div class="insight-label">Model Accuracy</div>
                <q-skeleton type="text" width="60px" class="insight-value" />
                <q-skeleton type="text" width="80px" class="insight-change" />
              </div>
              <div v-else>
                <div class="insight-label">Model Accuracy</div>
                <div class="insight-value">{{ canisterData.accuracy }}%</div>
                <div
                  class="insight-change"
                  :class="getNumberChangeClass(canisterData.accuracyChange)"
                >
                  {{ canisterData.accuracyChange }}% from last week
                </div>
              </div>
            </div>

            <!-- Total Stake Card -->
            <div class="insight-card">
              <div v-if="isDataLoading">
                <div class="insight-label">Total Stake</div>
                <q-skeleton type="text" width="60px" class="insight-value" />
                <q-skeleton type="text" width="80px" class="insight-change" />
              </div>
              <div v-else>
                <div class="insight-label">Total Stake</div>
                <div class="insight-value">
                  {{ canisterData.totalStake }} {{ stakeToken }}
                </div>
                <div
                  class="insight-change"
                  :class="getNumberChangeClass(canisterData.stakeChange)"
                >
                  {{ canisterData.stakeChange }}% from last week
                </div>
              </div>
            </div>

            <!-- Cycles Balance Card -->
            <div class="insight-card">
              <div v-if="isDataLoading">
                <div class="insight-label">Cycles Balance</div>
                <q-skeleton type="text" width="60px" class="insight-value" />
                <q-skeleton type="text" width="80px" class="insight-change" />
              </div>
              <div v-else>
                <div class="insight-label">Cycles Balance</div>
                <div class="insight-value">
                  {{ canisterData.cyclesBalance }} T
                </div>
                <div
                  class="insight-change"
                  :class="getNumberChangeClass(canisterData.cyclesChange)"
                >
                  {{ canisterData.cyclesChange }}% from last week
                </div>
              </div>
            </div>

            <!-- Shutdowns Card -->
            <div class="insight-card">
              <div v-if="isDataLoading">
                <div class="insight-label">Shutdowns</div>
                <q-skeleton type="text" width="60px" class="insight-value" />
                <q-skeleton type="text" width="80px" class="insight-change" />
              </div>
              <div v-else>
                <div class="insight-label">Shutdowns</div>
                <div class="insight-value">0</div>
                <div class="insight-change">0% from last week</div>
              </div>
            </div>
          </div>
        </div>
        <!-- Quick Start Section -->
        <div>
          <div class="q-pt-md">
            <div class="header-left">
              <h4 class="section-title">Quick Start</h4>
            </div>
          </div>

          <q-stepper
            v-model="activeStep"
            ref="stepper"
            color="primary"
            animated
            flat
            header-nav
            vertical
          >
            <q-step
              v-for="item in quickStartItems"
              :key="item.id"
              :name="item.id"
              :title="item.title"
              :caption="item.description"
              :icon="item.icon"
              :done="item.id < activeStep"
              @click="handleItemClick(item)"
            >
              <div class="step-content">
                <q-chip :color="item.statusColor" text-color="white" size="sm">
                  {{ item.status }}
                </q-chip>
                <span>{{ item.description }}</span>
              </div>

              <div class="q-mt-md q-gutter-sm">
                <q-btn
                  v-if="item.id > 1"
                  flat
                  color="grey-7"
                  label="Back"
                  @click.stop="activeStep--"
                />
                <q-btn
                  v-if="item.id < quickStartItems.length"
                  color="primary"
                  label="Next"
                  @click.stop="activeStep++"
                />
                <!-- TODO 点击完成任务以后应该隐藏整个页面-->
                <q-btn
                  v-if="item.id === quickStartItems.length"
                  color="positive"
                  label="Complete"
                  @click.stop="completeQuickStart"
                />
              </div>
            </q-step>
          </q-stepper>
        </div>
      </div>
    </div>
    <!-- stake dialog -->
    <q-dialog v-model="openStakeDialog">
      <q-card style="width: 500px; max-width: 90vw">
        <q-card-section class="row items-center q-pb-none">
          <div class="text-h6">{{ isStakeMode ? "Stake" : "Unstake" }}</div>
          <q-space />
          <q-btn
            icon="close"
            flat
            round
            dense
            @click="openStakeDialog = false"
          />
        </q-card-section>

        <q-card-section class="q-pt-md">
          <q-item>
            <q-item-section>
              <q-item-label caption>
                {{ isStakeMode ? "Available Balance" : "Staked Amount" }}
              </q-item-label>
              <q-item-label class="text-weight-bold">
                {{ isStakeMode ? walletBalance : canisterData.totalStake }}
                {{ stakeToken }}
              </q-item-label>
            </q-item-section>
          </q-item>

          <q-separator class="q-my-md" />

          <q-banner
            v-if="isStakeMode"
            rounded
            class="bg-orange text-white q-mb-md"
          >
            <!-- Alpha Warning Banner -->
            <template v-slot:avatar>
              <q-icon name="warning" />
            </template>
            <div class="text-weight-bold">Alpha Phase Warning</div>
            <div>
              This is an <strong>alpha testnet</strong>. All staked tokens are
              test tokens and <strong>will NOT be carried over</strong> to
              mainnet or any future versions. Use them only for testing
              purposes.
            </div>
          </q-banner>
          <!-- 现在解除质押默认解除质押所有代币，所以先用if else粗暴解决 -->
          <q-input
            v-if="isStakeMode"
            v-model.number="stakeAmount"
            :label="isStakeMode ? 'Stake Amount' : 'Unstake Amount'"
            type="number"
            filled
            dense
            :rules="[
            (val: number | null) => val !== null || 'Please enter stake amount',
            (val: number | null) => (val && val > 0) || 'Amount must be greater than 0',
            (val: number | null) => (val && val <= walletBalance) || 'Insufficient balance'
            ]"
            class="q-mb-md"
            :hint="
              isStakeMode
                ? 'Enter the amount you want to stake'
                : 'Enter the amount you want to unstake'
            "
          >
            <template #append>
              <q-btn
                label="Max"
                flat
                dense
                no-caps
                size="sm"
                @click="
                  stakeAmount = isStakeMode
                    ? walletBalance
                    : canisterData.totalStake
                "
              />
            </template>
          </q-input>
          <!-- 解除质押时的“100% 进度条” -->
          <div v-else class="q-mb-md">
            <!-- Progress Bar with Label -->
            <div class="q-mb-md">
              <div class="row items-center justify-between q-mb-sm">
                <div>
                  <q-icon
                    name="info"
                    size="sm"
                    color="warning"
                    class="q-mr-sm"
                  />
                  <span class="text-subtitle2 text-weight-bold"
                    >Unstake Token Amount</span
                  >
                </div>

                <span class="text-caption text-weight-bold text-red">100%</span>
              </div>
              <q-linear-progress
                :value="1"
                color="red"
                size="12px"
                class="rounded-borders"
                stripe
                animated
              />
            </div>

            <!-- Info Text -->
            <q-banner
              class="bg-red-1 text-red-9 q-mb-md rounded-borders"
              dense
              flat
            >
              <template v-slot:avatar>
                <q-icon name="warning" />
              </template>
              All your staked tokens will be unstaked immediately
            </q-banner>
          </div>
        </q-card-section>

        <q-card-actions align="right">
          <q-btn
            label="Cancel"
            flat
            color="grey-8"
            @click="openStakeDialog = false"
          />
          <q-btn
            :label="isStakeMode ? 'Confirm Stake' : 'Confirm Unstake'"
            color="primary"
            :loading="stakeLoading"
            :disable="!isFormValid || stakeLoading"
            @click="handleStake"
          />
        </q-card-actions>
      </q-card>
    </q-dialog>

    <!-- Success notification -->
    <q-dialog v-model="showSuccessDialog">
      <q-card style="width: 400px">
        <q-card-section class="row items-center">
          <q-icon name="check_circle" size="lg" color="positive" />
          <span class="q-ml-md text-h6">Staking Successful!</span>
        </q-card-section>

        <q-card-section class="text-center text-grey-7">
          <p>
            Successfully staked
            <span class="text-weight-bold text-positive"
              >{{ stakeAmount }} TOKENS</span
            >
          </p>
        </q-card-section>

        <q-card-actions align="center">
          <q-btn
            label="Close"
            color="primary"
            @click="showSuccessDialog = false"
          />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<script setup lang="ts">
import {
  checkIsPredictRunning,
  checkSystemLatestVersion,
  getPrediction,
  getPredictionChartData,
  installCode,
  onPredict,
  queryCanisterStatus,
} from "@/api/canisters";
import { DOCS_URL } from "@/api/constants/docs";
import {
  DONT_SHOW_AGAIN_STORAGE_KEY,
  IC_DASHBOARD_URL,
} from "@/api/constants/ic";
import { getPCLBalance } from "@/api/icp";
import { getStakeBalance } from "@/api/token";
import { stakePredictCanister } from "@/api/user";
import { fromTokenAmount } from "@/utils/common";
import { showMessageError, showMessageSuccess } from "@/utils/message";
import {
  getStringByPrincipalAndCanister,
  setStringByPrincipalAndCanister,
} from "@/utils/storage";
import * as echarts from "echarts";
import { Notify } from "quasar";
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();

// 定义 Banner 是否显示
const showBanner = ref(false);
const isDataLoading = ref(true);
const isStatusLoading = ref(true);
const startPredictLoading = ref(false);
// Dialog 显示状态
const isStakeMode = ref(true); // true = stake，false = unstake（默认 stake）
const openStakeDialog = ref(false);
const showSuccessDialog = ref(false);

// 表单数据
const stakeAmount = ref<number>(0);

// 加载状态
const stakeLoading = ref(false);

// 常数
const walletBalance = ref(0);

const canisterId = ref(route.params.canisterId as string);
// 当前 Canister 版本和最新版本（示例数据）
const currentVersion = ref("0.0.0");
const latestVersion = ref("0.0.0");
const latestVersionName = ref("");
// Reactive data
const chartView = ref("accuracy");
const stakeToken = ref("testPCL");
const chartRef = ref<HTMLElement>();
let chart: echarts.ECharts | null = null;

const activeStep = ref(1);

const chartData = ref({
  dates: [
    "Nov 17",
    "Nov 18",
    "Nov 19",
    "Nov 20",
    "Nov 21",
    "Nov 22",
    "Nov 23",
    "Nov 24",
    "Nov 25",
    "Nov 26",
  ],
  accuracyRate: [51, 49, 51, 52, 54, 55, 57, 56, 55, 57],
});

const quickStartItems = [
  {
    id: 1,
    title: "Create your own canister",
    description: "Set up a new canister for your project",
    icon: "create_new_folder",
    status: "Required",
    statusColor: "primary",
    path: "/getting-started/quickstart",
  },
  {
    id: 2,
    title: "Configure model",
    description: "Customize and deploy your model settings",
    icon: "settings",
    status: "Optional",
    statusColor: "positive",
    path: "",
  },
  {
    id: 3,
    title: "Top up cycles",
    description: "Top up cycles to power your application",
    icon: "account_balance_wallet",
    status: "Optional",
    statusColor: "positive",
    path: "/getting-started/quickstart#topup-cycles",
  },
];

const canisterData = ref({
  owner: "",
  status: "Standby",
  module_hash: "",
  cyclesBalance: "0",
  cyclesChange: 0,
  totalStake: 0,
  stakeChange: 0,
  accuracy: 0,
  accuracyChange: 0,
  tradingPair: "ICP/USDT",
  modelVersion: "v0.0.1",
  nextPrediction: "None",
  lastUpdated: "None",
});

//获取canister信息
const getCanisterInfo = async () => {
  isDataLoading.value = true;
  try {
    const status = await queryCanisterStatus(canisterId.value);
    if (!status) {
      throw new Error(`Canister ${canisterId.value} status is undefined`);
    }
    canisterData.value = {
      ...canisterData.value,
      // status: Object.keys(status.status)[0] as
      //   | "running"
      //   | "stopping"
      //   | "stopped",
      module_hash: status.module_hash,
      cyclesBalance: `${fromTokenAmount(status.cycles.toString(), 12).toFixed(
        2
      )}`,
      owner: status.settings.controllers.map((p) => p.toText()),
    };
  } catch (error) {
    console.error(`Error fetching status for canister ${canisterId}:`, error);
  }
  isDataLoading.value = false;
};

//开始预测，true为开始，false为停止
const startPredict = async (start: boolean) => {
  startPredictLoading.value = true;
  try {
    await onPredict(canisterId.value, start);

    checkIsPredict();
  } catch (error) {}
  startPredictLoading.value = false;
};

const initChart = () => {
  if (!chartRef.value) return;

  chart = echarts.init(chartRef.value);

  // 动态计算 y 轴范围
  const accuracyValues = chartData.value.accuracyRate;
  const minAccuracy = Math.floor(Math.min(...accuracyValues) - 2); // 减少缓冲区
  const maxAccuracy = Math.ceil(Math.max(...accuracyValues) + 2);

  const option = {
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
      left: "8%",
      right: "4%",
      bottom: "12%",
      top: "8%",
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

const getTokenBalance = () => {
  getPCLBalance().then((res) => {
    walletBalance.value = res;
  });
  getStakeBalance(canisterId.value).then((res) => {
    canisterData.value.totalStake = res;
  });
};
const checkIsPredict = () => {
  checkIsPredictRunning(canisterId.value)
    .then((res) => {
      canisterData.value.status = res ? "Predicting" : "Standby";
      if (canisterData.value.status === "Predicting") {
      }
    })
    .finally(() => {
      isStatusLoading.value = false;
      nextTick(() => {
        initChart();
      });
    });
  getPrediction(canisterId.value).then((res) => {
    console.log("res", res);
  });
  getPredictPointChart();
};
const getPredictPointChart = () => {
  const nowNs = BigInt(Date.now()) * 1_000_000n;
  const sevenDaysAgoNs = nowNs - 7n * 86_400_000_000_000n; // 7 × 24 × 60 × 60 × 1e9
  getPredictionChartData(canisterId.value, sevenDaysAgoNs, nowNs).then(
    (data) => {
      console.log("getPredictPointChart", data);
      data.map((p) => ({
        time: new Date(Number(p.timestamp / 1_000_000n)), // 纳秒 → Date
        value: p.value,
      }));
    }
  );
};
// Lifecycle
onMounted(async () => {
  getCanisterInfo();
  getTokenBalance();
  checkIsPredict();
  // 检查是否启用升级版本的banner
  showBanner.value = await checkVersion();
});

// Watch chart view changes
watch(chartView, () => {
  if (chart) {
    initChart();
  }
});

const getStatusColor = (status: string) => {
  switch (status.toLowerCase()) {
    case "predicting":
      return "positive";
    case "stopped":
      return "negative";
    default:
      return "grey";
  }
};

const getNumberChangeClass = (number: number) => {
  return number >= 0 ? "positive" : "negative";
};

const completeQuickStart = () => {
  console.log("Quick Start Completed");
  // Handle completion logic here
};

// 检查是否有新版本需要提示更新
const checkVersion = async (): Promise<boolean> => {
  try {
    // 调用后端获取最新版本
    const latestVersionResult = await checkSystemLatestVersion();
    console.log("latestVersionResult", latestVersionResult);

    // 假设 checkSystemLatestVersion 返回 Result 类型，例如 { Ok: string } | { Err: string }
    if ("Err" in latestVersionResult) {
      console.error("Failed to fetch latest version:", latestVersionResult.Err);
      return false; // 如果后端返回错误，假设无需提示更新
    }

    latestVersion.value = latestVersionResult.version; // 获取最新版本号
    latestVersionName.value = latestVersionResult.name; // 获取最新版本号
    currentVersion.value = "0.0.9"; // TODO 临时占位符，替换为实际调用用户 canister 接口的逻辑
    // 示例：const currentVersion = await userCanister.getCurrentVersion();

    // 检查本地存储中用户屏蔽的版本
    const dismissedVersions = getStringByPrincipalAndCanister(
      DONT_SHOW_AGAIN_STORAGE_KEY,
      canisterId.value
    );
    //如果不存在屏蔽版本，直接显示
    if (!dismissedVersions) return true;

    // 比较版本号
    const isNewerVersion =
      compareVersions(latestVersion.value, currentVersion.value) > 0;

    if (isNewerVersion) {
      // 如果有新版本，检查用户是否屏蔽了 *最新版本*
      if (
        dismissedVersions !== null &&
        dismissedVersions.includes(latestVersion.value)
      ) {
        return false; // 用户已屏蔽最新版本的提示，不显示
      }
      return true; // 有新版本，且未被屏蔽，显示提示
    } else {
      // 没有新版本，检查是否屏蔽了当前版本
      if (
        dismissedVersions !== null &&
        dismissedVersions.includes(currentVersion.value)
      ) {
        return false; // 用户屏蔽了当前版本，不显示提示
      }
      return false; // 没有新版本，无需提示
    }
  } catch (error) {
    console.error("Error checking version update:", error);
    return false; // 发生错误，假设无需提示
  }
};

// 版本号比较函数
function compareVersions(latest: string, current: string): number {
  const latestParts = latest.split(".").map(Number);
  const currentParts = current.split(".").map(Number);
  for (let i = 0; i < Math.max(latestParts.length, currentParts.length); i++) {
    const latestNum = latestParts[i] || 0;
    const currentNum = currentParts[i] || 0;
    if (latestNum > currentNum) return 1;
    if (latestNum < currentNum) return -1;
  }
  return 0;
}
// 处理升级按钮点击事件
const handleUpgrade = async () => {
  console.log(
    "Initiating Canister upgrade...",
    latestVersionName.value,
    latestVersion.value
  );
  // Create a persistent notification with a progress bar
  const notify = Notify.create({
    spinner: true,
    message:
      "System upgrade in progress. Please do not leave this page until the update is complete.",
    type: "positive",
    position: "top",
    timeout: 0,
    group: false,
    actions: [
      {
        label: "Dismiss",
        color: "white",
        handler: () => false, // Prevent dismissal during upgrade
      },
    ],
  });

  try {
    await installCode(
      canisterId.value,
      latestVersionName.value,
      latestVersion.value,
      "upgrade"
    );
    //成功升级系统版本后自动不再提示当前版本。
    const success = setStringByPrincipalAndCanister(
      DONT_SHOW_AGAIN_STORAGE_KEY,
      canisterId.value,
      latestVersion.value
    );
    // 直接调用 notify() 触发 Api.dismiss() 关闭弹窗
    notify();
    showMessageSuccess(
      `System successfully installed with the latest version ${latestVersion.value}`
    );
  } catch (error) {
    notify();
    showMessageError(`Failed to upgrade system ${error}`);
    console.log(`Failed to upgrade system ${error}`);
  }

  // 升级后关闭 Banner
  showBanner.value = false;
};

// 处理关闭 Banner
const handleDontShowAgain = () => {
  const success = setStringByPrincipalAndCanister(
    DONT_SHOW_AGAIN_STORAGE_KEY,
    canisterId.value,
    latestVersion.value // 假设 latestVersion.value 是字符串
  );
  if (success) {
    showBanner.value = false;
    console.log(
      `String ${latestVersion.value} dismissed for canister ${canisterId}`
    );
  } else {
    console.log(
      `Failed to dismiss string ${latestVersion.value} for canister ${canisterId}`
    );
  }
};

// 处理列表项点击事件
const handleItemClick = (item) => {
  if (item.path) {
    const fullUrl = `${DOCS_URL}${item.path}`;
    window.open(fullUrl, "_blank", "noopener,noreferrer");
  } else {
    showMessageError(`Coming Soon: ${item.title}`);
  }
};

// 处理质押
const handleStake = async () => {
  if (!isFormValid.value) {
    showMessageError("Please check form information");
    return;
  }

  stakeLoading.value = true;

  try {
    if (isStakeMode.value) {
      console.log("stake", isStakeMode.value);
      const resBoolean = await stakePredictCanister(
        canisterId.value,
        stakeAmount.value
      );
      console.log("resBoolean", resBoolean);
      //方法不成功，跳出
      if (!resBoolean) return;
      openStakeDialog.value = false;
      showSuccessDialog.value = true;
      getTokenBalance();
    } else {
      console.log("unstake", isStakeMode.value);
    }
  } catch (error) {
  } finally {
    stakeLoading.value = false;
  }
};

const isFormValid = computed(() => {
  return (
    stakeAmount.value &&
    stakeAmount.value > 0 &&
    stakeAmount.value <= walletBalance.value
  );
});
</script>

<style lang="scss" scoped>
.unstake-progress-container {
  padding: 16px;
  background: linear-gradient(
    135deg,
    rgba(255, 0, 0, 0.02) 0%,
    rgba(255, 0, 0, 0.01) 100%
  );
  border-radius: 8px;
  border: 1px solid rgba(255, 0, 0, 0.1);
}
.chart-container {
  height: 200px;
}

.chart {
  padding: 10px;
  width: 100%;
  height: 100%;
}
.color-mask {
  position: relative;
  &::before {
    content: "";
    position: absolute;
    left: 0;
    transform: translateX(-50%);
    width: 10%; // 你可以根据需要调整大小
    height: 100px; // 半圆的高度是圆的半径
    background: radial-gradient(
      circle,
      rgba(100, 118, 234, 0.7),
      // 使用更深的蓝色，增加不透明度
      rgba(255, 105, 180, 0.7) // 使用更鲜艳的粉色，增加不透明度
    );
    border-radius: 100px;
    filter: blur(300px); // 模糊效果
  }
}

.q-stepper {
  border: 1px solid rgb(234, 235, 238);
}

.banner-container {
  background-color: #e0f2fe; /* 类似 Tailwind 的 blue-100，浅蓝色背景 */
  // color: #1e40af; /* 类似 Tailwind 的 blue-800，深蓝色文字 */
  border: 1px solid #bfdbfe; /* 类似 Tailwind 的 blue-200，浅蓝色边框 */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05); /* 类似 Tailwind 的 shadow-sm */
}

.banner-icon {
  color: #2563eb; /* 类似 Tailwind 的 blue-600，柔和的蓝色图标 */
}

.banner-text {
  font-size: 14px; /* 类似 Tailwind 的 text-sm */
  line-height: 1.5; /* 类似 Tailwind 的 leading-normal */
}

.project-overview {
  // min-height: 100vh;
}

.overview-container {
  max-width: 1400px;
  margin: 0 auto;
  display: grid;
  grid-template-columns: 320px 1fr;
  gap: 32px;
}

/* Project Sidebar */
.project-sidebar {
  position: sticky;
  top: 24px;
  height: fit-content;
}

.project-card {
  background: white;
  border-radius: 12px;
  border: 1px solid #e5e7eb;
  overflow: hidden;
}

.welcome-text {
  font-size: 12px;
  font-weight: 500;
  color: #374151;
  margin: 0;
}

.project-type {
  font-size: 12px;
  color: #6b7280;
  font-weight: 500;
}

.project-details {
  padding: 20px;
}
.project-meta {
  margin-bottom: 24px;
}

.meta-item {
  margin-bottom: 12px;
}

.meta-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.meta-label {
  font-size: 12px;
  color: #6b7280;
  font-weight: 500;
  display: block;
  margin-bottom: 4px;
}

.meta-value {
  display: flex;
  align-items: center;
  gap: 6px;
}

.link-icon {
  color: #6b7280;
}

.project-url {
  color: #2563eb;
  text-decoration: none;
  font-size: 14px;
}

.project-url:hover {
  text-decoration: underline;
}

.site-structure {
  border-top: 1px solid #f3f4f6;
  padding-top: 20px;
}

.structure-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.structure-title {
  font-size: 14px;
  font-weight: 600;
  color: #374151;
}

.manage-btn {
  font-size: 12px;
}

.structure-item {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #6b7280;
  font-size: 14px;
}

/* Main Content */
.main-content {
  display: flex;
  flex-direction: column;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  color: #111827;
  margin: 0;
}

.progress-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.progress-text {
  font-size: 14px;
  color: #059669;
  font-weight: 500;
}

.tasks-header {
  margin-bottom: 16px;
}

.tasks-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.task-item {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 16px;
  border: 1px solid #f3f4f6;
  border-radius: 8px;
  transition: all 0.2s;
  background: white;
}

.task-item:hover {
  border-color: #e5e7eb;
  background: #fafafa;
}

.task-item.completed .task-title {
  text-decoration: line-through;
}

.task-icon {
  width: 40px;
  height: 40px;
  background: #f3f4f6;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.task-content {
  flex: 1;
}

.task-title {
  font-size: 16px;
  font-weight: 600;
  color: #111827;
  margin: 0 0 4px 0;
}

.task-description {
  font-size: 14px;
  color: #6b7280;
  margin: 0 0 12px 0;
  line-height: 1.5;
}

.task-actions {
  display: flex;
  gap: 8px;
}

.task-expand {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.task-expanded-content {
  padding: 0 16px 16px 56px;
}

/* Insights Section */
.insights-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.insights-period {
  display: flex;
  align-items: center;
  gap: 16px;
}

.period-text {
  font-size: 14px;
  color: #6b7280;
}
.insights-grid {
  display: grid;
  border: 1px solid rgb(234, 235, 238);
  gap: 0;
  border-radius: 0.5rem;
  box-shadow: 0 1px 0 rgba(0, 0, 0, 0.1);
  .insight-card {
    padding: 24px;
  }
  @media screen and (max-width: 768px) {
    grid-template-columns: 1fr;
    .insight-card {
      border-bottom: 1px solid #f3f4f6;
      &:last-child {
        border-bottom: none;
      }
    }
  }
  @media screen and (min-width: 769px) {
    grid-template-columns: repeat(2, 1fr);
    .insight-card {
      &:not(:nth-child(2n)) {
        border-right: 1px solid #f3f4f6;
      }
      &:nth-child(-n + 2) {
        border-bottom: 1px solid #f3f4f6;
      }
    }
  }
}
.insight-card {
  box-sizing: border-box;
  border: none;
}
.insight-label {
  font-size: 14px;
  color: #111827;
  font-weight: 600;
  margin-bottom: 8px;
}

.insight-value {
  font-size: 32px;
  font-weight: 600;
  color: #111827;
  margin-bottom: 4px;
}

.insight-change {
  font-size: 12px;
  color: #6b7280;
  &.positive {
    color: #059669;
  }
  &.negative {
    color: #dc2626;
  }
}

/* Responsive */
@media (max-width: 1024px) {
  .overview-container {
    grid-template-columns: 1fr;
    gap: 24px;
  }

  .project-sidebar {
    position: static;
  }
}

@media (max-width: 768px) {
  .project-overview {
    padding: 16px;
  }
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
}
</style>
