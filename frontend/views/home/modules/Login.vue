<template>
  <div class="color-mask">
    <div class="container login-container">
      <div class="description row">
        <div class="col-12">
          <h5>Accurate price predictions for your favorite crypto assets</h5>
          <q-card flat bordered>
            <q-item>
              <q-item-section>
                <q-table
                  :rows="rows"
                  :columns="columns"
                  row-key="id"
                  flat
                  :pagination="{ rowsPerPage: 0 }"
                  hide-bottom
                >
                  <template
                    v-for="col in columns"
                    :key="col.name"
                    v-slot:[`header-cell-${col.name}`]="props"
                  >
                    <q-th
                      :props="props"
                      class="text-weight-bolder"
                      style="font-size: 16px"
                    >
                      {{ props.col.label }}!
                      <q-tooltip>{{
                        columnTooltips[col.name] || "无描述"
                      }}</q-tooltip>
                    </q-th>
                  </template>
                  <template #body-cell-asset="props">
                    <q-td :props="props" class="token">
                      <q-item dense>
                        <q-item-section class="token-logo">
                          <q-icon
                            :name="'img:' + props.row.token.logo"
                            size="28px"
                          />
                        </q-item-section>

                        <q-item-section>
                          <q-item-label class="text-subtitle2">
                            {{ props.row.token.name }}
                          </q-item-label>
                          <q-item-label caption>
                            <q-icon
                              :name="'img:' + props.row.source.logo"
                              size="10px"
                            />
                            {{ props.row.source.name }}
                          </q-item-label>
                        </q-item-section>
                      </q-item>
                    </q-td>
                  </template>
                  <!-- 预测插槽 -->
                  <template
                    v-for="colName in ['last_2', 'last_1', 'now', 'next']"
                    :key="colName"
                    v-slot:[`body-cell-${colName}`]="props"
                  >
                    <q-td :props="props" class="text-center q-py-xs">
                      <!-- 上面的一行：价格或stake -->
                      <div class="flex-y-center justify-center">
                        <span
                          v-if="colName !== 'next'"
                          class="flex-y-center text-subtitle1"
                        >
                          ${{ props.row[colName].price.toFixed(2) }}
                          <ArrowIcon
                            :direction="props.row[colName].trend"
                            size="12px"
                          />
                        </span>
                        <span v-else>
                          {{
                            props.row[colName]?.pred?.staked.toFixed(2) ?? "0"
                          }}
                          staked
                        </span>
                      </div>
                      <!-- 下面的一行：预测（now 除外） -->
                      <div v-if="colName !== 'now'" class="text-caption">
                        Pred
                        <ArrowIcon
                          :direction="props.row[colName]?.pred?.trend"
                          size="12px"
                        />
                        <span>
                          {{
                            props.row[colName]?.pred
                              ? `${props.row[colName].pred.up}↑ ${props.row[colName].pred.down}↓`
                              : "-"
                          }}
                        </span>
                      </div>
                    </q-td>
                  </template>
                  <template #body-cell-stake="props">
                    <q-td :props="props">
                      <span class="text-subtitle1">
                        {{ props.row.stake.amount.toFixed(1) }}&nbsp;
                      </span>
                      <span
                        :class="
                          props.row.stake.change > 0
                            ? 'text-positive'
                            : 'text-negative'
                        "
                      >
                        {{ props.row.stake.change > 0 ? "+" : "" }}
                        {{ props.row.stake.change }}%
                      </span>
                    </q-td>
                  </template>
                  <template #body-cell-accuracy="props">
                    <q-td :props="props">
                      <span class="text-subtitle1">
                        {{ props.row.accuracy + " %" }}
                      </span>
                    </q-td>
                  </template>
                </q-table>
              </q-item-section>
            </q-item>
          </q-card>
        </div>
        <div class="logo q-mt-md flex justify-center">
          <img alt="logo" src="@/assets/on-chain.svg" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { IdentityInfo, initAuth, signIn } from "@/api/auth";
import { setCurrentIdentity } from "@/api/canister_pool";
import { MARKETS } from "@/api/constants/markets";
import ArrowIcon from "@/components/ArrowIcon.vue";
import { useUserStore } from "@/stores/user";
import type { TableColumn } from "@/types/model";
import type { TimePoint } from "@/types/predict";
import { debounce } from "quasar";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const userStore = useUserStore();

interface RowData {
  id: number;
  token: { name: string; logo: string };
  source: { name: string; logo: string };
  last_2: TimePoint;
  last_1: TimePoint;
  now: TimePoint;
  next: Omit<TimePoint, "price"> | null;
  accuracy: number;
  stake: { amount: number; change: number };
}

const columns = ref<TableColumn[]>([]);
const rows = ref<RowData[]>([]);
const columnTooltips: Record<string, string> = {
  asset: "显示交易对和来源交易所",
  last_2: "10分钟前的价格和预测（02:20）",
  last_1: "5分钟前的价格和预测（02:25）",
  now: "当前价格（02:32）",
  next: "下一时间点的预测（02:35）",
  accuracy: "过去一周预测准确率",
  stake: "最近24小时的质押金额和变化",
};

onMounted(() => {
  // getOKInfo()
});
// 与 II 认证相关的信息
const signedIn = ref(false); // 是否登录
const loading = ref(false);

// 更新表格数据
const updateTable = debounce(() => {
  const now = new Date(); // 2025-05-21 15:42 PDT
  const times = getTimeLabels(now); // [15:35, 15:40, 15:42, 15:45]

  columns.value = [
    {
      name: "asset",
      label: "Asset",
      field: "name",
      align: "left",
    },
    {
      name: "last_2",
      label: times[0], // 01:35
      field: "last_2",
      align: "center",
    },
    {
      name: "last_1",
      label: times[1], // 01:40
      field: "last_1",
      align: "center",
    },
    {
      name: "now",
      label: times[2], // 01:50
      field: "now",
      align: "center",
    },
    {
      name: "next",
      label: `${times[3]}`, // 01:50 (预测)
      field: "next",
      align: "center",
    },
    {
      name: "accuracy",
      label: "Accuracy (1 week)",
      field: "accuracy",
      align: "center",
    },
    {
      name: "stake",
      label: "Stake (24h)",
      field: "stake",
      align: "center",
    },
  ];

  // 假数据（实际从 API 获取）
  rows.value = [
    {
      id: 1,
      token: { name: "BTC-USDT", logo: "/frontend/assets/icons/BTC.svg" },
      source: { name: "BINANCE" },
      last_2: {
        price: 105133.25,
        trend: "Down",
        pred: { staked: 100.5, up: 39.091, down: 36.109, trend: "Up" },
      },
      last_1: {
        price: 105223.01,
        trend: "Down",
        pred: { staked: 100.5, up: 39.091, down: 36.109, trend: "Up" },
      },
      now: { price: 105248.65, trend: "Up", pred: null },
      next: {
        trend: "Up",
        pred: { staked: 76.2, up: 45.5, down: 30.2, trend: "Up" },
      },
      accuracy: 51.9,
      stake: { amount: 23786.0, change: 5.2 },
    },
    {
      id: 2,
      token: { name: "ETH-USDT", logo: "/frontend/assets/icons/ETH.svg" },
      source: { name: "BINANCE" },
      last_2: {
        price: 3200.45,
        trend: "Up",
        pred: { staked: 76.2, up: 45.5, down: 30.2, trend: "Up" },
      },
      last_1: {
        price: 3210.12,
        trend: "Up",
        pred: { staked: 76.2, up: 46.0, down: 29.8, trend: "Up" },
      },
      now: { price: 3195.78, trend: "Down", pred: null },
      next: {
        trend: "Up",
        pred: { staked: 76.2, up: 45.5, down: 30.2, trend: "Up" },
      },
      accuracy: 62.3,
      stake: { amount: 16000, change: 5.2 },
    },
  ].map((item) => ({
    ...item,
    source: {
      name: item.source.name,
      logo: MARKETS[item.source.name]?.logo,
    },
  }));
}, 500);

// 计算时间标签
const getTimeLabels = (now: Date) => {
  const minute = now.getMinutes();
  const baseMinute = Math.floor(minute / 5) * 5; // 取整到最近的5分钟
  const baseTime = new Date(now);
  baseTime.setMinutes(baseMinute, 0, 0);

  return [
    new Date(baseTime.getTime() - 10 * 60 * 1000).toLocaleTimeString("en-US", {
      hour12: false,
      hour: "2-digit",
      minute: "2-digit",
    }), // last_2
    new Date(baseTime.getTime() - 5 * 60 * 1000).toLocaleTimeString("en-US", {
      hour12: false,
      hour: "2-digit",
      minute: "2-digit",
    }), // last_1
    now.toLocaleTimeString("en-US", {
      hour12: false,
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    }), // now
    new Date(baseTime.getTime() + 5 * 60 * 1000).toLocaleTimeString("en-US", {
      hour12: false,
      hour: "2-digit",
      minute: "2-digit",
    }), // next
  ];
};

// 初始化和定时更新
onMounted(() => {
  updateTable();
  // setInterval(updateTable, 60 * 1000); // 每分钟更新
});

const onLogin = async () => {
  const auth = await initAuth();
  loading.value = true;
  //TODO 先不使用登录缓存，有点问题
  // if (!auth.info) {
  //检查用户是否已登录，未登录就登录
  signIn(auth.client) // 理论上有链接对象才会进入这个方法
    .then((ii) => {
      signedIn.value = true;
      auth.info = ii;
      loginSuccess(ii);
    })
    .catch((e) => {
      console.error("e", e);
    })
    .finally(() => {
      loading.value = false;
    });
  // } else {
  //   //存在auth.info，说明用户已登录，不需要再登录
  //   loginSuccess(auth.info)
  // }
};

const enableTwitterAds = () => {
  // 调用 Twitter 广告跟踪事件
  //@ts-ignore
  window.twq("event", "tw-opr1q-opr2m", {});
};

const loginSuccess = (ii: IdentityInfo) => {
  // 保存登录状态到actor，方便调用
  setCurrentIdentity(ii.identity, ii.principal);
  // 保存 principal 到状态
  userStore.setPrincipal(ii.principal).then(() => {
    enableTwitterAds();
    //直接跳转到应用中，在应用里获取userInfo，加快速度。
    router.push({
      path: "/app",
    });
  });
};
</script>

<style lang="scss" scoped>
.color-mask {
  overflow: hidden;
  position: relative;
  &::before {
    content: "";
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 50%; // 你可以根据需要调整大小
    height: 200px; // 半圆的高度是圆的半径
    background: radial-gradient(
      circle,
      rgba(192, 217, 255, 0.5),
      rgba(240, 185, 229, 0.5)
    );
    border-radius: 100px;
    filter: blur(300px); // 模糊效果
  }
}
.login-container {
  padding-top: 50px;
  padding-bottom: 250px;
  .logo {
    width: 100%;
  }
}
.token {
  .q-item {
    padding-left: 0;
    .token-logo {
      max-width: 28px;
    }
  }
}
</style>
