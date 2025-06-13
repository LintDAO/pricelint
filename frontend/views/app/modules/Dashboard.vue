<template>
  <div class="dashboard-container q-gutter-md">
    <q-card class="total-card">
      <q-card-section>
        <div class="text-h5 q-mb-xs">Total</div>

        <div class="row q-col-gutter-sm dashboard-container">
          <!-- 用户地址 -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">User Address</div>
                <div class="text-subtitle2 text-grey">
                  Your ICP principal address
                </div>
              </q-card-section>
              <q-card-section
                class="q-pt-none q-gutter-sm row items-center no-wrap"
              >
                <q-icon
                  name="content_copy"
                  color="primary"
                  size="sm"
                  class="cursor-pointer"
                  @click="copyToClipboard(userData.address)"
                >
                  <q-tooltip>Copy address</q-tooltip>
                </q-icon>
                <div class="text-body1 text-break">
                  {{ userData.address || "N/A" }}
                </div>
              </q-card-section>
            </q-card>
          </div>

          <!-- 钱包Cycles余额 -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">Wallet Cycles Balance</div>
                <div class="text-subtitle2 text-grey">
                  Available cycles for canisters
                </div>
              </q-card-section>
              <q-card-section class="q-pt-none">
                <div class="text-body1">
                  {{ userData.cyclesBalance }} Cycles
                </div>
                <div class="text-body1">0.00001T / Day</div>
                <q-linear-progress
                  stripe
                  :value="cyclesPercentage / 100"
                  style="height: 10px"
                  color="positive"
                />
              </q-card-section>
            </q-card>
          </div>

          <!-- 运行中的Canister -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">Running Canisters</div>
                <div class="text-subtitle2 text-grey">
                  Active canisters in your account
                </div>
              </q-card-section>
              <q-card-section class="canister-list q-pt-none">
                <q-list dense>
                  <q-item
                    v-for="canister in userData.runningCanisters"
                    :key="canister.id"
                    clickable
                  >
                    <q-item-section>
                      <q-item-label>{{ canister.name }}</q-item-label>
                      <q-item-label caption
                        >Status: {{ canister.status }}</q-item-label
                      >
                    </q-item-section>
                  </q-item>
                  <q-item v-if="!userData.runningCanisters.length">
                    <q-item-label>No running canisters</q-item-label>
                  </q-item>
                </q-list>
              </q-card-section>
            </q-card>
          </div>

          <!-- 质押代币数量 -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">Staked Tokens</div>
                <div class="text-subtitle2 text-grey">
                  ICP tokens staked for predictions
                </div>
              </q-card-section>
              <q-card-section class="q-pt-none">
                <div class="text-body1">{{ userData.stakedTokens }} ICP</div>
                <q-chip color="primary" text-color="white" icon="lock">
                  Locked until {{ userData.stakeLockEnd }}
                </q-chip>
              </q-card-section>
            </q-card>
          </div>

          <!-- 预测准确率 -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">Prediction Accuracy</div>
                <div class="text-subtitle2 text-grey">
                  Your historical prediction accuracy
                </div>
              </q-card-section>
              <q-card-section class="q-pt-none q-gutter-md">
                <q-circular-progress
                  show-value
                  :value="userData.predictionAccuracy"
                  size="80px"
                  :thickness="0.2"
                  color="positive"
                  track-color="grey-3"
                >
                  {{ userData.predictionAccuracy }}%
                </q-circular-progress>
                <q-circular-progress
                  show-value
                  :value="userData.predictionAccuracy"
                  size="80px"
                  :thickness="0.2"
                  color="positive"
                  track-color="grey-3"
                >
                  {{ userData.predictionAccuracy }}%
                </q-circular-progress>
                <q-circular-progress
                  show-value
                  :value="userData.predictionAccuracy"
                  size="80px"
                  :thickness="0.2"
                  color="positive"
                  track-color="grey-3"
                >
                  {{ userData.predictionAccuracy }}%
                </q-circular-progress>
              </q-card-section>
            </q-card>
          </div>

          <!-- 预测收益 -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">Prediction Earnings</div>
                <div class="text-subtitle2 text-grey">
                  Earnings from successful predictions
                </div>
              </q-card-section>
              <q-card-section class="q-pt-none">
                <div class="text-body1">
                  {{ userData.predictionEarnings }} ICP
                </div>
                <q-btn
                  flat
                  color="positive"
                  label="View Trend"
                  @click="showEarningsTrend"
                />
              </q-card-section>
            </q-card>
          </div>

          <!-- 活跃预测池 -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">Active Prediction Pools</div>
                <div class="text-subtitle2 text-grey">
                  Ongoing prediction markets
                </div>
              </q-card-section>
              <q-card-section class="q-pt-none canister-list">
                <q-list dense>
                  <q-item
                    v-for="pool in userData.activePools"
                    :key="pool.id"
                    clickable
                  >
                    <q-item-section>
                      <q-item-label>{{ pool.asset }}</q-item-label>
                      <q-item-label caption
                        >Cycle: {{ pool.cycle }}</q-item-label
                      >
                    </q-item-section>
                  </q-item>
                  <q-item v-if="!userData.activePools.length">
                    <q-item-label>No active pools</q-item-label>
                  </q-item>
                </q-list>
              </q-card-section>
            </q-card>
          </div>

          <!-- 排行榜排名 -->
          <div class="col-12 col-md-4">
            <q-card class="dashboard-card">
              <q-card-section>
                <div class="text-h6">Leaderboard Rank</div>
                <div class="text-subtitle2 text-grey">
                  Your rank in prediction market
                </div>
              </q-card-section>
              <q-card-section class="q-pt-none">
                <div class="text-body1">
                  Rank #{{ userData.leaderboardRank }}
                </div>
                <q-icon
                  :name="
                    userData.rankTrend === 'up'
                      ? 'arrow_upward'
                      : 'arrow_downward'
                  "
                  :color="userData.rankTrend === 'up' ? 'positive' : 'negative'"
                  size="sm"
                />
              </q-card-section>
            </q-card>
          </div>
        </div>
      </q-card-section>
    </q-card>
    <q-card class="quick-start-card">
      <div class="text-h5 q-pa-md">Quick Start</div>
      <div>
        <q-list bordered separator>
          <q-item
            v-for="item in quickStartItems"
            :key="item.id"
            clickable
            v-ripple
            @click="handleItemClick(item)"
          >
            <q-item-section avatar>
              <q-icon :name="item.icon" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label class="text-h6">{{ item.title }}</q-item-label>
              <q-item-label caption>{{ item.description }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-badge :color="item.statusColor" :label="item.status" />
            </q-item-section>
          </q-item>
        </q-list>
      </div>
    </q-card>
  </div>
</template>

<script lang="ts" setup>
import { useUserStore } from "@/stores/user";
import { useQuasar } from "quasar";
import { computed, onMounted, ref } from "vue";

// 初始化 Quasar
const $q = useQuasar();
const userStore = useUserStore();
const loading = ref(true);

// 初始化时获取数据
onMounted(async () => {
  // 如果不存在用户信息则进行同步
  if (!userStore.principal) {
    console.log("userStore");
    await userStore.fetchUserInfo();
  }
  loading.value = false;
});

// 模拟 Quick Start 数据
const quickStartItems = ref([
  {
    id: 1,
    title: "Recharge cycles",
    description: "Top up cycles to power your application",
    icon: "account_balance_wallet",
    status: "Required",
    statusColor: "primary",
  },
  {
    id: 2,
    title: "Create your own canister",
    description: "Set up a new canister for your project",
    icon: "create_new_folder",
    status: "Required",
    statusColor: "primary",
  },
  {
    id: 3,
    title: "Configure model",
    description: "Customize and deploy your model settings",
    icon: "settings",
    status: "Optional",
    statusColor: "positive",
  },
]);

// 模拟用户数据
const userData = ref({
  address: computed(() => userStore.principal || ""),
  cyclesBalance: 5000000,
  runningCanisters: [
    { id: "can1", name: "Prediction Model", status: "Running" },
    { id: "can2", name: "Data Processor", status: "Active" },
  ],
  stakedTokens: 1000,
  stakeLockEnd: "2025-12-31",
  predictionAccuracy: 51,
  predictionEarnings: 250.5,
  activePools: [
    { id: "pool1", asset: "BTC/USD", cycle: "Hourly" },
    { id: "pool2", asset: "ETH/USD", cycle: "Daily" },
  ],
  leaderboardRank: 42,
  rankTrend: "up",
});

// 计算 Cycles 余额进度条百分比（假设最大值为 1000 万）
const cyclesPercentage = computed(() =>
  Math.min((userData.value.cyclesBalance / 10000000) * 100, 100)
);

// 复制地址到剪贴板
const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text);
    $q.notify({
      message: "Address copied to clipboard!",
      color: "positive",
      position: "top",
    });
  } catch (err) {
    $q.notify({
      message: "Failed to copy address",
      color: "negative",
      position: "top",
    });
  }
};
// 处理列表项点击事件
const handleItemClick = (item) => {
  $q.notify({
    message: `Clicked: ${item.title}`,
    color: "positive",
    position: "top",
  });
};

// 查看收益趋势（示例占位函数）
const showEarningsTrend = () => {
  $q.notify({
    message: "Earnings trend display not implemented yet",
    color: "warning",
    position: "top",
  });
};
</script>

<style lang="scss" scoped>
.dashboard-card {
  height: 180px; /* 固定卡片高度 */
  width: 100%; /* 填满分配的列宽 */
  transition: box-shadow 0.3s;
}
.dashboard-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}
.text-break {
  word-break: break-all; /* 长地址自动换行 */
}
</style>
