<template>
  <div class="color-mask predictions-table" style="min-height: 100vh">
    <div class="container login-container">
      <div class="description row">
        <div class="col-12">
          <div class="text-h5 q-my-md">Crypto Trends: Up or Down</div>
          <q-card>
            <q-item style="padding-bottom: 16px">
              <q-item-section>
                <q-table
                  :rows="rows"
                  :columns="columns"
                  row-key="id"
                  flat
                  :pagination="{ rowsPerPage: 0 }"
                  hide-bottom
                  :loading="loading"
                >
                  <!-- 表头 -->
                  <template
                    v-for="col in columns"
                    :key="col.name"
                    v-slot:[`header-cell-${col.name}`]="props"
                  >
                    <q-th
                      :props="props"
                      class="text-bold"
                      style="font-size: 16px"
                    >
                      <div
                        class="row items-center"
                        :class="{
                          'justify-center': props.col.align === 'center',
                          'justify-start': props.col.align === 'left',
                        }"
                        style="gap: 8px"
                      >
                        <!-- 左侧文字部分 -->
                        <div class="column" style="line-height: 1">
                          <span>{{ props.col.label }}</span>
                          <span
                            v-if="props.col.subtitle"
                            class="text-caption"
                            style="font-size: 11px"
                          >
                            {{ props.col.subtitle }}
                          </span>
                        </div>
                        <!-- 右侧图标，仅当有 tooltip 时显示 -->
                        <el-tooltip
                          v-if="columnTooltips[col.name]"
                          effect="dark"
                          placement="bottom"
                        >
                          <template #content>
                            <span style="white-space: pre-wrap">
                              {{ columnTooltips[col.name] }}
                            </span>
                          </template>
                          <q-icon
                            name="error_outline"
                            size="16px"
                            class="cursor-pointer"
                          />
                        </el-tooltip>
                      </div>
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
        <div class="logo q-mt-xl flex justify-center">
          <img alt="logo" src="@/assets/on-chain.svg" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { showPredictions } from "@/api/canisters";
import { MARKETS } from "@/api/constants/markets";
import { TOKENS } from "@/api/constants/tokens";
import { getTokenPrice } from "@/api/token";
import ArrowIcon from "@/components/ArrowIcon.vue";
import { useUserStore } from "@/stores/user";
import type { TableColumn } from "@/types/model";
import type { TimePoint } from "@/types/predict";
import { showMessageError } from "@/utils/message";
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
  last_2:
    "Price at the specified time with direction compared to previous prediction time price. \nIf subscribed, the prediction data it's going to be displayed.",
  last_1:
    "Price at the specified time with direction compared to previous prediction time price. \nIf subscribed, the prediction data it's going to be displayed.",
  now: "Live price and direction compared to previous prediction time price.",
  next: "Predicted price direction for the time specified time. Check PCL staked and stake directions for a higher confidence.",
  accuracy: "Percentage of accurate predictions over the last 1 week.",
  stake:
    "The percentage difference in the total number of PCL tokens invested in the past 24 hours compared to the previous day.",
};

const loading = ref(false);
let priceTimer: any = null;

const updateTable = debounce(async () => {
  loading.value = true;

  const now = new Date();
  const times = getTimeLabels(now);

  try {
    // 1. 调用 API，获取 ApiResult<ApiItem[]>
    const result = await showPredictions();
    console.log("showPredictions result:", result);

    // ---- 关键：处理 Ok / Err ----
    if (result.Err) {
      console.error("API Error:", result.Err);
      showMessageError("Failed to obtain predicted data" + result.Err);
      loading.value = false;
      return;
    }

    if (!result.Ok || result.Ok.length === 0) {
      console.warn("No data in Ok");
      rows.value = [];
      loading.value = false;
      return;
    }

    const apiData = result.Ok; // 现在安全使用

    // 2. 设置 columns
    columns.value = [
      { name: "asset", label: "Asset", field: "token", align: "left" },
      { name: "last_2", label: times[0], field: "last_2", align: "center" },
      { name: "last_1", label: times[1], field: "last_1", align: "center" },
      { name: "now", label: times[2], field: "now", align: "center" },
      {
        name: "next",
        label: `${times[3]}`,
        field: "next",
        align: "center",
        subtitle: "Predictions",
      },
      {
        name: "accuracy",
        label: "Accuracy",
        field: "accuracy",
        align: "center",
        subtitle: "1 week",
      },
      {
        name: "stake",
        label: "Stake",
        field: "stake",
        align: "center",
        subtitle: "24h",
      },
    ];
    // 直接映射，类型用 any 绕过检查（简单粗暴但有效）
    const rawRows: any[] = apiData.map((item: any, i: number) => {
      const l2 = item.last_2[0] || {};
      const l1 = item.last_1[0] || {};
      const nx = item.next[0] || {};

      // 辅助函数：把 BigInt 转 Number
      const n = (val: any) => (typeof val === "bigint" ? Number(val) : val);

      return {
        id: i + 1,
        token: { name: item.token_name.replace("USDT", "-USDT"), logo: "" },
        source: { name: "BINANCE", logo: "" },
        last_2: {
          price: l2.price?.[0] || 0,
          trend: (l2.trend?.[0] || "").toUpperCase(),
          pred: {
            up: n(l2.pred?.up ?? 0),
            down: n(l2.pred?.down ?? 0),
            staked: n(l2.pred?.staked ?? 0),
            trend: l2.pred?.trend || "",
          },
        },
        last_1: {
          price: l1.price?.[0] || 0,
          trend: (l1.trend?.[0] || "").toUpperCase(),
          pred: {
            up: n(l1.pred?.up ?? 0),
            down: n(l1.pred?.down ?? 0),
            staked: n(l1.pred?.staked ?? 0),
            trend: l1.pred?.trend || "",
          },
        },
        now: { price: 0, trend: "", pred: null },
        next: {
          trend: (nx.trend?.[0] || "").toUpperCase(),
          pred: {
            up: n(nx.pred?.up ?? 0),
            down: n(nx.pred?.down ?? 0),
            staked: n(nx.pred?.staked ?? 0),
            trend: nx.pred?.trend || "",
          },
        },
        accuracy: item.accuracy || 0,
        stake: {
          amount: n(item.stake[0]) || 0,
          change: n(item.stake[1]) || 0,
        },
      };
    });

    // 4. 批量获取实时价格
    const symbols = rawRows.map((r) => getTokenSymbol(r.token.name));
    const pricePromises = symbols.map((sym) =>
      getTokenPrice(sym).catch(() => 0)
    );
    const priceResults = await Promise.all(pricePromises);

    // 5. 合并 logo + 实时价格
    rows.value = rawRows.map((row, i) => {
      const symbol = symbols[i];
      const currentPrice = priceResults[i];

      return {
        ...row,
        token: {
          ...row.token,
          logo: TOKENS[symbol]?.meta.logo || "/assets/default-icon.png",
        },
        source: {
          name: row.source.name,
          logo: MARKETS[row.source.name]?.logo || "/assets/default-icon.png",
        },
        now: {
          price: currentPrice || row.last_1.price,
          trend:
            currentPrice > row.last_1.price
              ? "up"
              : currentPrice < row.last_1.price
              ? "down"
              : "",
          pred: null,
        },
      };
    });
  } catch (err) {
    console.error("updateTable error:", err);
    showMessageError("Failed to obtain predicted data: " + err);
  } finally {
    loading.value = false;
  }
}, 500);
// 新增：每 60 秒刷新实时价格
const refreshNowPrice = async () => {
  if (!rows.value.length) return; // 没数据就不刷

  const symbols = rows.value.map((row) => getTokenSymbol(row.token.name));
  const prices = await Promise.all(
    symbols.map((sym) => getTokenPrice(sym).catch(() => 0))
  );

  rows.value = rows.value.map((row, i) => {
    const currentPrice = prices[i] || row.last_1.price;
    return {
      ...row,
      now: {
        price: currentPrice,
        trend:
          currentPrice > row.last_1.price
            ? "up"
            : currentPrice < row.last_1.price
            ? "down"
            : "",
        pred: null,
      },
    };
  });
};

// 提取代币符号的辅助函数
const getTokenSymbol = (pair: string): string => {
  // 假设 token.name 是如 "BTC-USDT" 的格式，返回 "BTC"
  return pair.split("-")[0].toUpperCase();
};

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
