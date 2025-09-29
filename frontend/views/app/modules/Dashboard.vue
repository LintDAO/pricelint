<template>
  <div>
    <div class="q-gutter-md">
      <div class="overview-container">
        <div class="text-h6 text-grey-8 q-mb-md">Overview</div>
        <div class="row q-col-gutter-md">
          <div
            v-for="item in overviewData"
            :key="item.title"
            class="col-12 col-sm-6 col-md-6 col-lg-3"
          >
            <div class="overview-item">
              <q-separator class="q-mb-lg" />
              <div class="text-subtitle2">{{ item.title }}</div>
              <div class="text-h5 text-weight-bold q-my-sm">
                {{ item.value }}
              </div>
              <div class="change-text">
                <span
                  class="change-value"
                  :class="
                    item.change >= 0
                      ? 'text-positive bg-positive-1'
                      : 'text-negative bg-negative-1'
                  "
                >
                  {{ item.change >= 0 ? "+" : "" }}{{ item.change }}%
                </span>
                <span class="text-grey-7 q-ml-sm">from last week</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="text-h6 text-grey-8 q-pt-xl">Data</div>
      <q-separator />
      <div class="card-row row q-col-gutter-sm">
        <!-- 用户地址 -->
        <div class="col-12 col-md-6">
          <q-card class="dashboard-card">
            <q-card-section>
              <div class="text-h6">Treasury</div>
              <div class="text-subtitle2 text-grey">ICP wallet info</div>
              <div class="text-caption text-grey">Principal Id</div>
              <div class="row items-center">
                <div class="text-body2">
                  {{ userData.principal || "N/A" }}
                </div>
                <q-icon
                  name="content_copy"
                  color="primary"
                  class="cursor-pointer"
                  @click="copyToClipboard(userData.principal)"
                >
                  <q-tooltip>Copy address</q-tooltip>
                </q-icon>
              </div>
              <div class="row items-center">
                <div class="text-caption text-grey">Account Id</div>
                <div class="text-body2 text-break">
                  {{ userData.accountId || "N/A" }}
                </div>
                <q-icon
                  name="content_copy"
                  color="primary"
                  class="cursor-pointer"
                  @click="copyToClipboard(userData.accountId)"
                >
                  <q-tooltip>Copy AccountId</q-tooltip>
                </q-icon>
              </div>
              <div class="row items-center"></div>
            </q-card-section>
            <q-card-section class="q-pt-none q-gutter-sm row">
              <div class="text-subtitle2 text-grey">Balance</div>
              <q-list dense padding class="token-list q-pt-none q-mt-none">
                <q-item
                  v-for="(token, index) in userData.balances"
                  :key="index"
                >
                  <!-- 遍历已添加的token -->
                  <q-item-section avatar>
                    <q-avatar size="40px" font-size="12px">
                      <img :src="token.logo" />
                    </q-avatar>
                  </q-item-section>
                  <q-item-section>
                    <q-item-label>{{ token.name }}</q-item-label>
                  </q-item-section>
                  <q-item-section side>
                    <div style="display: flex; align-items: center; gap: 8px">
                      <q-item-label>
                        {{ token.amount }} {{ token.symbol }}
                      </q-item-label>
                      <q-btn-dropdown flat round dense>
                        <q-list dense>
                          <q-item clickable v-close-popup>
                            <q-item-section>
                              <q-item-label @click="openSendDialog(token)">
                                Send
                              </q-item-label>
                            </q-item-section>
                          </q-item>
                        </q-list>
                      </q-btn-dropdown>
                    </div>
                  </q-item-section>
                </q-item>
              </q-list>
              <!-- 发送代币对话框 -->
              <q-dialog v-model="showSendDialog" @hide="closeSendDialog()">
                <q-card>
                  <q-card-section>
                    <div class="text-h6">Send {{ selectedToken?.name }}</div>
                  </q-card-section>
                  <q-card-section>
                    <q-input
                      v-model="sendForm.principal"
                      label="To This Principal"
                      filled
                      class="q-mb-md"
                      :rules="[
                        (val) => !!val || 'Principal ID is required',
                        (val) => isPrincipal(val) || 'Invalid Principal ID',
                      ]"
                    />
                    <q-input
                      v-model.number="sendForm.amount"
                      label="amount"
                      type="number"
                      filled
                      :suffix="selectedToken?.symbol"
                      :rules="[
                (val: number) => val > 0 && val <= (selectedToken?.amount ?? 0) || 'insufficient balance'
              ]"
                    />
                  </q-card-section>
                  <q-card-actions align="right">
                    <q-btn flat label="cancel" v-close-popup />
                    <q-btn
                      color="primary"
                      label="confirm"
                      :loading="loadingSend"
                      @click="sendToken"
                      :disable="!sendForm.principal || sendForm.amount == null"
                    />
                  </q-card-actions>
                </q-card>
              </q-dialog>
            </q-card-section>
          </q-card>
        </div>

        <!-- 钱包Cycles余额 -->
        <div class="col-12 col-md-6">
          <q-card class="dashboard-card">
            <q-card-section>
              <div class="text-h6">Cycles Balance</div>
              <div class="text-subtitle2 text-grey">
                Canister consumes cycles as fuel every day
              </div>
            </q-card-section>
            <q-card-section class="q-pt-none">
              <div class="text-body1">{{ userData.cycles.amount }} Cycles</div>
              <div class="text-body1">0.00000T / Day</div>
              <q-linear-progress
                stripe
                :value="cyclesPercentage / 100"
                style="height: 10px"
                color="positive"
              />
            </q-card-section>
          </q-card>
        </div>
      </div>
      <div class="card-row row q-col-gutter-sm">
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
                    <q-item-label caption>Cycle: {{ pool.cycle }}</q-item-label>
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
              <div class="text-body1">Rank #{{ userData.leaderboardRank }}</div>
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

      <div class="text-h6 text-grey-8 q-pt-md">Quick Start</div>
      <q-separator />
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
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ICP_LOGO } from "@/api/constants/tokens";
import { getCyclesBalance, getICPBalance, transferICP } from "@/api/icp";
import { useUserStore } from "@/stores/user";
import { isPrincipal, p2a } from "@/utils/common";
import { useQuasar } from "quasar";
import { computed, onMounted, ref } from "vue";

// 初始化 Quasar
const $q = useQuasar();
const userStore = useUserStore();
const loading = ref(true);
const loadingSend = ref(false);
const showSendDialog = ref(false);
const selectedToken = ref<{
  name: string;
  symbol: string;
  amount: number;
} | null>(null);

const sendForm = ref<{
  principal: string;
  amount: number;
}>({
  principal: "",
  amount: 0,
});

// 初始化时获取数据
onMounted(async () => {
  // 如果不存在用户信息则进行同步
  if (!userStore.principal) {
    console.log("userStore");
    await userStore.fetchUserInfo();
  }
  getUserInfo();
  loading.value = false;
});

const overviewData = [
  {
    title: "Running Canisters",
    value: "12",
    change: 4.5,
  },
  {
    title: "Profit",
    value: "$455",
    change: -0.5,
  },
  {
    title: "Prediction Accuracy",
    value: "51%",
    change: 4.5,
  },
  {
    title: "Cycles Burn",
    value: "20 T",
    change: 21.2,
  },
];

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
  principal: "",
  accountId: "",
  balances: {
    icp: {
      name: "Internet Computer",
      symbol: "ICP",
      logo: ICP_LOGO,
      amount: 0,
    },
    // cycles: {
    //   name: "Cycles",
    //   symbol: "CYC",
    //   logo: "",
    //   amount: 0,
    // },
    // pcl: {
    //   name: "PriceLint",
    //   symbol: "PCL",
    //   logo: "",
    //   amount: 0,
    // },
  },
  cycles: {
    amount: 0,
  },
  runningCanisters: [
    { id: "can1", name: "Prediction Model", status: "Running" },
    { id: "can2", name: "Data Processor", status: "Active" },
  ],
  stakedTokens: 0,
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

const getUserInfo = () => {
  userData.value.principal = userStore.principal;
  userData.value.accountId = p2a(userStore.principal);
  getICPBalance(userData.value.accountId).then((res) => {
    userData.value.balances.icp.amount = res;
  });
  getCyclesBalance(userData.value.principal).then((res) => {
    userData.value.cycles.amount = Number(res);
  });
};

const sendToken = async () => {
  loadingSend.value = true;
  try {
    await transferICP(sendForm.value.principal, sendForm.value.amount);
  } catch (error) {}
  loadingSend.value = false;
  showSendDialog.value = false;
};

// 计算 Cycles 余额进度条百分比（假设最大值为 1000 万）
const cyclesPercentage = computed(() =>
  Math.min((userData.value.cycles.amount / 10000000) * 100, 100)
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

// 打开发送对话框
const openSendDialog = (token) => {
  selectedToken.value = { ...token };
  sendForm.value.principal = "";
  sendForm.value.amount = 0;
  showSendDialog.value = true;
  console.log(
    "  selectedToken.value ",
    selectedToken.value,
    showSendDialog.value
  );
};
// 不清理状态的话会导致切换代币时无法正常打开dialog
const closeSendDialog = () => {
  showSendDialog.value = false;
  selectedToken.value = null;
  sendForm.value = { principal: "", amount: 0 };
};

// 处理列表项点击事件
const handleItemClick = (item) => {
  $q.notify({
    message: `Coming Soon: ${item.title}`,
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
  min-height: 180px;
  width: 100%;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.1),
    /* 内部高光 */ 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.08);
}

.dashboard-card:hover {
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.2),
    0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  transform: translateY(-2px);
  border-color: rgba(0, 0, 0, 0.12);
}
.token-list {
  width: 100%;
  .q-item {
    padding-left: 0;
  }
}
.card-row {
  margin-left: 8px;
}
</style>
