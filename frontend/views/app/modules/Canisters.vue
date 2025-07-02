<template>
  <q-btn color="primary" :loading="createLoading" @click="createNew()">
    Create New
  </q-btn>
  <div>
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
          <q-btn
            color="secondary"
            label="Install Code"
            :loading="loadingActions[props.row.canisterId]?.install"
            @click="openInstallCodeDialog(props.row.canisterId)"
          />
        </q-td>
      </template>
    </q-table>

    <!-- Install Code dialog -->
    <q-dialog v-model="showInstallDialog">
      <q-card>
        <q-card-section>
          <div class="text-h6">
            Install Code for Canister {{ selectedCanisterId }}
          </div>
        </q-card-section>
        <q-card-section>
          <q-file v-model="wasmFile" label="Select .wasm file" accept=".wasm" />
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" color="negative" v-close-popup />
          <q-btn
            flat
            label="Install"
            color="primary"
            :disable="!wasmFile"
            :loading="loadingActions[selectedCanisterId]?.install"
          />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<script setup lang="ts">
import {
  CanisterData,
  queryCanisterStatus,
  startCanister,
  stopCanister,
} from "@/api/canisters";
import type { TableColumn } from "@/types/model";
import { fromTokenAmount } from "@/utils/common";
import { showMessageError, showMessageSuccess } from "@/utils/message";
import { onMounted, ref } from "vue";

const createLoading = ref(false);

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
const userCanisterIds = ["dxegq-jyaaa-aaaab-qb2wq-cai"];
const pagination = ref({
  sortBy: "canisterId",
  descending: false,
  page: 1,
  rowsPerPage: 10,
});
// Operation loading states
const loadingActions = ref<
  Record<string, { start?: boolean; stop?: boolean; install?: boolean }>
>({});
// Install code dialog state
const showInstallDialog = ref(false);
const selectedCanisterId = ref("");
const wasmFile = ref<File | null>(null);

// 初始化时获取数据
onMounted(async () => {
  getCanisterInfo();
});

const createNew = async () => {
  createLoading.value = true;
  // await burnICPcreateCanister(0.25);
  createLoading.value = false;
};

const getCanisterInfo = async () => {
  queryCanisterStatus();
  loading.value = true;
  canisterData.value = [];

  for (const canisterId of userCanisterIds) {
    try {
      const status = await queryCanisterStatus();
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
    showMessageSuccess(`Canister ${canisterId} started successfully`);
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
    showMessageSuccess(`Canister ${canisterId} stopped successfully`);
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

// Open install code dialog
const openInstallCodeDialog = (canisterId: string) => {
  selectedCanisterId.value = canisterId;
  wasmFile.value = null;
  showInstallDialog.value = true;
};
</script>

<style lang="scss" scoped></style>
