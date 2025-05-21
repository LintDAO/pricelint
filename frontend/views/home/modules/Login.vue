<template>
  <div class="color-mask">
    <div class="container login-container">
      <div class="description row">
        <div class="col-12">
          <q-card flat bordered>
            <q-item>
              <q-item-section>
                <q-item-label caption style="font-size: 1rem">
                  Holdings
                </q-item-label>
                <q-table
                  title="Predictions"
                  :rows="rows"
                  :columns="columns"
                  row-key="id"
                  flat
                >
                  <template #body-cell-asset="props">
                    <q-td :props="props">
                      <q-icon
                        v-if="props.row.source.icon"
                        :name="props.row.source.icon"
                        size="sm"
                        class="q-mr-sm"
                      />
                      <q-icon
                        v-else
                        name="mdi-help-circle"
                        size="sm"
                        class="q-mr-sm"
                      />
                      {{ props.row.name }} ({{ props.row.source.name }})
                    </q-td>
                  </template>
                  <template #body-cell-stake="props">
                    <q-td
                      :props="props"
                      :class="
                        props.row.stake.change > 0
                          ? 'text-positive'
                          : 'text-negative'
                      "
                    >
                      {{ props.value.amount.toFixed(2) }} USDT (
                      <q-icon
                        :name="
                          props.value.change > 0
                            ? 'mdi-arrow-up'
                            : 'mdi-arrow-down'
                        "
                        size="xs"
                      />
                      {{ props.value.change > 0 ? "+" : ""
                      }}{{ props.value.change.toFixed(1) }}%)
                    </q-td>
                  </template>
                </q-table>
              </q-item-section>
            </q-item>
          </q-card>
        </div>
        <div class="logo">
          <img alt="logo" src="@/assets/on-chain.svg" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { IdentityInfo, initAuth, signIn } from "@/api/auth";
import { setCurrentIdentity } from "@/api/canister_pool";
import { useUserStore } from "@/stores/user";
import type { TableColumn } from "@/types/model";
import { debounce } from "quasar";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const userStore = useUserStore();

interface RowData {
  id: number;
  name: string;
  source: { name: string; icon: string };
  last_2: TimePoint;
  last_1: TimePoint;
  now: TimePoint;
  next: Omit<TimePoint, "price"> | null;
  accuracy: number;
  stake: { amount: number; change: number };
}

const columns = ref<TableColumn[]>([]);
const rows = ref<RowData[]>([]);

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
      label: times[0],
      field: "last_2",
      align: "center",
      format: (val) =>
        `${val.price.toFixed(2)} ${val.trend} ${
          val.pred
            ? `(PredUp ${val.pred.up}% Up, ${val.pred.down}% Down, Staked ${val.pred.staked} USDT)`
            : ""
        }`,
    },
    {
      name: "last_1",
      label: times[1],
      field: "last_1",
      align: "center",
      format: (val) =>
        `${val.price.toFixed(2)} ${val.trend} ${
          val.pred
            ? `(PredUp ${val.pred.up}% Up, ${val.pred.down}% Down, Staked ${val.pred.staked} USDT)`
            : ""
        }`,
    },
    {
      name: "now",
      label: times[2],
      field: "now",
      align: "center",
      format: (val) =>
        `${val.price.toFixed(2)} ${val.trend} ${
          val.pred
            ? `(PredUp ${val.pred.up}% Up, ${val.pred.down}% Down, Staked ${val.pred.staked} USDT)`
            : ""
        }`,
    },
    {
      name: "next",
      label: `${times[3]} (Predictions)`,
      field: "next",
      align: "center",
      format: (val) =>
        val ? `${val.trend} staked (Pred ${val.pred.staked} USDT)` : "?",
    },
    {
      name: "accuracy",
      label: "Accuracy (1 week)",
      field: "accuracy",
      align: "center",
      sortable: true,
      format: (val) => `${val}%`,
    },
    {
      name: "stake",
      label: "Stake (24h)",
      field: "stake",
      align: "center",
      format: (val) =>
        `${val.amount.toFixed(2)} USDT (${
          val.change > 0 ? "+" : ""
        }${val.change.toFixed(1)}%)`,
    },
  ];

  // 假数据（实际从 API 获取）
  rows.value = [
    {
      id: 1,
      name: "BTC-USDT",
      source: { name: "BINANCE", icon: "img:https://example.com/binance.png" },
      last_2: {
        price: 105133.25,
        trend: "Down",
        pred: { staked: 100.5, up: 39.091, down: 36.109 },
      },
      last_1: {
        price: 105223.01,
        trend: "Down",
        pred: { staked: 100.5, up: 39.091, down: 36.109 },
      },
      now: { price: 105248.65, trend: "Up", pred: null },
      next: { trend: "Up", pred: { staked: 76.2, up: 45.5, down: 30.2 } },
      accuracy: 51.9,
      stake: { amount: 23786.0, change: 5.2 },
    },
    {
      id: 2,
      name: "ETH-USDT",
      source: { name: "BINANCE", icon: "" },
      last_2: {
        price: 3200.45,
        trend: "Up",
        pred: { staked: 76.2, up: 45.5, down: 30.2 },
      },
      last_1: {
        price: 3210.12,
        trend: "Up",
        pred: { staked: 76.2, up: 46.0, down: 29.8 },
      },
      now: { price: 3195.78, trend: "Down", pred: null },
      next: { trend: "Up", pred: { staked: 76.2, up: 45.5, down: 30.2 } },
      accuracy: 62.3,
      stake: { amount: 16000, change: 5.2 },
    },
  ];
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
  setInterval(updateTable, 60 * 1000); // 每分钟更新
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
  padding-top: 100px;
  padding-bottom: 250px;

  .description > div {
    position: relative;
  }
  .login-button {
    margin-top: 50px;
  }
  .logo {
    position: absolute;
    bottom: 0;
    left: 0;
  }
}
</style>
