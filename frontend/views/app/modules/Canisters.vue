<template>
  <q-btn color="primary" @click="createNew()"> New Canister</q-btn>
  <div class="q-mt-md">
    <span v-if="userCanisterIds.length === 0">
      You don't have any canister.
    </span>
    <q-table
      v-else
      title="Canister List"
      :rows="canisterData"
      :columns="columns"
      row-key="canisterId"
      :loading="loading"
      :pagination="pagination"
    >
      <!-- Operation column -->
      <template v-slot:body-cell-actions="props">
        <q-td :props="props" class="q-gutter-xs">
          <q-btn
            v-if="props.row.status === 'stopped'"
            color="primary"
            label="Start"
            :loading="loadingActions[props.row.canisterId]?.start"
            @click="startThisCanister(props.row.canisterId)"
          />
          <q-btn
            v-if="props.row.status === 'running'"
            color="negative"
            label="Stop"
            :loading="loadingActions[props.row.canisterId]?.stop"
            @click="stopThisCanister(props.row.canisterId)"
          />
          <q-btn-dropdown
            split
            color="secondary"
            :label="
              props.row.module_hash.length === 0
                ? 'Install Code'
                : 'Configuration'
            "
            :loading="loadingActions[props.row.canisterId]?.install"
            @click="
              props.row.module_hash.length === 0
                ? installCanisterCode(props.row.canisterId)
                : openConfigurationDialog(props.row.canisterId)
            "
          >
            <q-list>
              <q-item
                clickable
                v-close-popup
                @click="showTopupCycles(props.row.canisterId)"
              >
                <q-item-section>
                  <q-item-label>Top-up Cycles</q-item-label>
                </q-item-section>
              </q-item>
            </q-list>
          </q-btn-dropdown>
        </q-td>
      </template>
    </q-table>

    <!-- Install Code dialog -->
    <q-dialog v-model="showConfigurationDialog">
      <q-card>
        <q-card-section>
          <div class="text-h6">
            Recommended Configuration for Canister {{ selectedCanisterId }}
          </div>
        </q-card-section>
        <q-card-section>
          The model will now run using the recommended configuration.
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" color="negative" v-close-popup />
          <q-btn
            label="Use Recommend"
            color="primary"
            @click="useRecommend()"
            :loading="loadingActions[selectedCanisterId]?.use"
          />
        </q-card-actions>
      </q-card>
    </q-dialog>
    <TopUpCycles
      v-model:visible="topUpDialog"
      :operation="operation"
      :targetCanisterId="selectedCanisterId"
    />
  </div>
</template>

<script setup lang="ts">
import {
  CanisterData,
  callTargetCanister,
  getCanisterList,
  installCode,
  queryCanisterStatus,
  startCanister,
  stopCanister,
} from "@/api/canisters";
import TopUpCycles from "@/components/TopUpCycles.vue";
import type { TableColumn } from "@/types/model";
import { fromTokenAmount } from "@/utils/common";
import { showMessageError } from "@/utils/message";
import { onMounted, ref } from "vue";

const topUpDialog = ref(false);
const operation = ref<"createCanister" | "topUp">("createCanister");

const columns: TableColumn[] = [
  {
    name: "canisterId",
    label: "Canister ID",
    field: "canisterId",
    align: "left",
    sortable: true,
  },
  {
    name: "cycles",
    label: "Remaining Cycles (T)",
    field: (row) =>
      `${fromTokenAmount(row.cycles.toString(), 12).toFixed(2)} T`, // Convert Cycles to T,
    align: "right",
    sortable: true,
  },
  {
    name: "cyclesConsumptionRate",
    label: "Cycles Consumption Rate (Cycles/ Day)",
    field: "cyclesConsumptionRate",
    align: "right",
    sortable: true,
  },
  {
    name: "predictionAccuracy",
    label: "Prediction Accuracy (%)",
    field: "predictionAccuracy",
    align: "right",
    sortable: true,
  },
  {
    name: "tokenBalance",
    label: "Token Balance",
    field: (row) => row.tokenBalance?.toString() ?? "N/A",
    align: "right",
    sortable: true,
  },
  {
    name: "status",
    label: "Status",
    field: "status",
    align: "center",
    sortable: true,
  },
  {
    name: "profitEarned",
    label: "Profit Earned",
    field: (row) => row.profitEarned?.toString() ?? "N/A",
    align: "right",
    sortable: true,
  },
  { name: "actions", label: "Actions", field: "canisterId", align: "center" },
];
// 表格数据和状态
const canisterData = ref<CanisterData[]>([]);
const loading = ref(false);
const userCanisterIds = ref<string[]>([]);
const pagination = ref({
  sortBy: "canisterId",
  descending: false,
  page: 1,
  rowsPerPage: 10,
});
// Operation loading states
const loadingActions = ref<
  Record<
    string,
    { start?: boolean; stop?: boolean; install?: boolean; use?: boolean }
  >
>({});
// Install code dialog state
const showConfigurationDialog = ref(false);
const selectedCanisterId = ref("");
const wasmFile = ref<File | null>(null);

// 初始化时获取数据
onMounted(async () => {
  userCanisterIds.value = await getCanisterList();
  await getCanisterInfo();
});

const createNew = async () => {
  topUpDialog.value = true;
  operation.value = "createCanister";
};

const getCanisterInfo = async () => {
  loading.value = true;
  canisterData.value = [];

  for (const canisterId of userCanisterIds.value) {
    try {
      const status = await queryCanisterStatus(canisterId);
      if (!status) {
        throw new Error(`Canister ${canisterId} status is undefined`);
      }
      canisterData.value.push({
        canisterId,
        status: Object.keys(status.status)[0] as
          | "running"
          | "stopping"
          | "stopped",
        module_hash: status.module_hash,
        cycles: status.cycles,
        controllers: status.settings.controllers.map((p) => p.toText()),
        // 占位字段（接口未提供，预留 N/A 或默认值）
        cyclesConsumptionRate: 0, // 占位
        tokenBalance: 0,
        predictionAccuracy: 0, // 占位
        profitEarned: BigInt(0), // 占位
      });
    } catch (error) {
      console.error(`Error fetching status for canister ${canisterId}:`, error);
      // 错误时仍添加占位数据
      canisterData.value.push({
        canisterId,
        status: "unknown",
        module_hash: [],
        cycles: BigInt(0),
        controllers: [],
        cyclesConsumptionRate: 0,
        tokenBalance: 0,
        predictionAccuracy: 0,
        profitEarned: BigInt(0),
      });
    }
  }

  loading.value = false;
};

// Start Canister
const startThisCanister = async (canisterId: string) => {
  loadingActions.value[canisterId] = {
    ...loadingActions.value[canisterId],
    start: true,
  };
  try {
    await startCanister(canisterId);
    await getCanisterInfo(); // Refresh data
  } catch (error) {
    console.error(`Error starting canister ${canisterId}:`, error);
    showMessageError(
      `Failed to start canister: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
  } finally {
    loadingActions.value[canisterId] = {
      ...loadingActions.value[canisterId],
      start: false,
    };
  }
};

// Stop Canister
const stopThisCanister = async (canisterId: string) => {
  loadingActions.value[canisterId] = {
    ...loadingActions.value[canisterId],
    stop: true,
  };
  try {
    await stopCanister(canisterId);
    await getCanisterInfo(); // Refresh data
  } catch (error) {
    console.error(`Error stopping canister ${canisterId}:`, error);
    showMessageError(
      `Failed to stop canister: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
  } finally {
    loadingActions.value[canisterId] = {
      ...loadingActions.value[canisterId],
      stop: false,
    };
  }
};

//当canister没有初始化环境配置时，使用installCode为canister安装代码
const installCanisterCode = async (canisterId: string) => {
  loadingActions.value[canisterId] = {
    ...loadingActions.value[canisterId],
    use: true,
  };
  await installCode(canisterId, "0.0.1");
  loadingActions.value[canisterId] = {
    ...loadingActions.value[canisterId],
    use: false,
  };
};

const showTopupCycles = (canisterId: string) => {
  topUpDialog.value = true;
  operation.value = "topUp";
  selectedCanisterId.value = canisterId;
};

// Open install code dialog
const openConfigurationDialog = (canisterId: string) => {
  selectedCanisterId.value = canisterId;
  wasmFile.value = null;
  showConfigurationDialog.value = true;
};

const useRecommend = async () => {
  const canisterId = selectedCanisterId.value;
  loadingActions.value[selectedCanisterId.value] = {
    ...loadingActions.value[selectedCanisterId.value],
    use: true,
  };
  await callTargetCanister(canisterId);
  loadingActions.value[selectedCanisterId.value] = {
    ...loadingActions.value[selectedCanisterId.value],
    use: false,
  };
};
</script>

<style lang="scss" scoped></style>
