<template>
  <div>
    <q-table
      flat
      title="Canister List"
      :rows="canisterData"
      :columns="columns"
      row-key="canisterId"
      :loading="loading"
      :pagination="pagination"
      class="canisters-list"
    >
      <template v-slot:top-right>
        <div class="q-gutter-md">
          <q-btn color="primary" @click="importDialogVisible = true" no-caps>
            Import Canister
          </q-btn>
          <q-btn color="primary" @click="createNew()" no-caps>
            New Canister</q-btn
          >
        </div>
      </template>
      <template v-slot:body-cell-canisterId="props">
        <q-td :props="props" class="text-grey-7">
          <span>{{ showUsername("", props.row.canisterId) }}</span>
          <q-icon
            name="content_copy"
            size="14px"
            class="q-ml-sm cursor-pointer"
            @click="copyText(props.row.canisterId)"
          >
            <q-tooltip>Copy Canister ID</q-tooltip>
          </q-icon>
        </q-td>
      </template>
      <!-- Operation column -->
      <template v-slot:body-cell-actions="props">
        <q-td :props="props" class="q-gutter-xs">
          <q-btn-dropdown
            split
            color="secondary"
            :label="props.row.module_hash.length === 0 ? 'Init' : 'Detail'"
            :loading="
              loadingActions[props.row.canisterId]?.install ||
              loadingActions[props.row.canisterId]?.start ||
              loadingActions[props.row.canisterId]?.stop
            "
            @click="
              props.row.module_hash.length === 0
                ? installCanisterCode(props.row.canisterId)
                : toCanisterDetail(props.row.canisterId)
            "
            no-caps
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
              <q-separator />
              <!-- Start 选项 -->
              <q-item
                v-if="props.row.status === 'stopped'"
                clickable
                v-close-popup
                @click="startThisCanister(props.row.canisterId)"
              >
                <q-item-section>
                  <q-item-label>Start Canister</q-item-label>
                </q-item-section>
              </q-item>
              <!-- Stop 选项 -->
              <q-item
                v-if="props.row.status === 'running'"
                clickable
                v-close-popup
                @click="stopThisCanister(props.row.canisterId)"
              >
                <q-item-section>
                  <q-item-label>Stop Canister</q-item-label>
                </q-item-section>
              </q-item>
              <!-- 将canisterid从已记录的列表中屏蔽，不是删除canister -->
              <!-- <q-item
                v-if="props.row.status !== 'unknown'"
                clickable
                v-close-popup
                @click="showBlockDialog(props.row.canisterId)"
              >
                Block Canister
              </q-item> -->
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
            @click="useRecommendParam()"
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
    <q-dialog v-model="importDialogVisible">
      <q-card style="min-width: 400px">
        <q-card-section>
          <div class="text-h6">Import Canister ID</div>
        </q-card-section>
        <q-card-section class="q-py-none">
          Import a canister currently controlled by the principal. Only canister
          controller can import the canister id.
        </q-card-section>
        <q-card-section>
          <q-input
            v-model="importCanisterId"
            label="Enter Canister ID"
            filled
            :rules="[
              (val) => !!val || 'Canister ID is required',
              (val) => isPrincipal(val) || 'Invalid Canister ID format',
            ]"
            @keyup.enter="importConfirm"
          />
        </q-card-section>

        <q-card-actions align="right">
          <q-btn flat label="Cancel" color="negative" v-close-popup />
          <q-btn
            :loading="importLoading"
            flat
            label="Import"
            color="primary"
            :disable="!importCanisterId || !isPrincipal(importCanisterId)"
            @click="importConfirm"
          />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<script setup lang="ts">
import {
  CanisterData,
  blockCanisterIdFromList,
  callTargetCanister,
  getCanisterList,
  importCanisterList,
  installCode,
  queryCanisterStatus,
  startCanister,
  stopCanister,
} from "@/api/canisters";
import TopUpCycles from "@/components/TopUpCycles.vue";
import type { TableColumn } from "@/types/model";
import { showUsername } from "@/utils/avatars";
import { copyText, fromTokenAmount, isPrincipal } from "@/utils/common";
import { showMessageError } from "@/utils/message";
import { useQuasar } from "quasar";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";

const $q = useQuasar();
const router = useRouter();
const topUpDialog = ref(false);
const importDialogVisible = ref(false);
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
  // {
  //   name: "cyclesConsumptionRate",
  //   label: "Cycles Consumption Rate (Cycles/ Day)",
  //   field: "cyclesConsumptionRate",
  //   align: "right",
  //   sortable: true,
  // },
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
const importLoading = ref(false);
const userCanisterIds = ref<string[]>([]);
const importCanisterId = ref<string>("");
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

// 初始化时获取数据
onMounted(async () => {
  await getCanisterInfo();
});

const createNew = async () => {
  topUpDialog.value = true;
  operation.value = "createCanister";
};

const getCanisterInfo = async () => {
  loading.value = true;
  userCanisterIds.value = await getCanisterList();
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
    install: true,
  };
  //TODO 版本号写死了，后面要可以选模型
  await installCode(canisterId, "lstm", "v2");
  loadingActions.value[canisterId] = {
    ...loadingActions.value[canisterId],
    install: false,
  };
  //安装完成后调用方法刷新数据
  getCanisterInfo();
};

const showTopupCycles = (canisterId: string) => {
  topUpDialog.value = true;
  operation.value = "topUp";
  selectedCanisterId.value = canisterId;
};

// jump to detail page
const toCanisterDetail = (canisterId: string) => {
  router.push({
    path: `/app/canisters/${canisterId}`,
  });
};

const useRecommendParam = async () => {
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

const importConfirm = async () => {
  importLoading.value = true;
  if (!isPrincipal(importCanisterId.value)) return;
  const success = await importCanisterList(importCanisterId.value);
  importLoading.value = false;
  if (success) {
    console.log("success");
    importDialogVisible.value = false;
    getCanisterInfo(); // Refresh data
  }
};

const showBlockDialog = (canisterId: string) => {
  $q.dialog({
    title: "Confirm Block",
    message: `This operation only block ${canisterId} from the list and does not actually delete the container.`,
    cancel: true, // Show cancel button
    ok: {
      label: "Block",
      color: "negative",
    },
  }).onOk(() => {
    // Show a second confirmation dialog to prevent accidental removal
    $q.dialog({
      title: "Are You Sure Block This Canister Id?",
      message: `This operation only block ${canisterId} from the list and does not actually delete the container.`,
      cancel: true,
      ok: {
        label: "Yes, Block It",
        color: "negative",
      },
    }).onOk(() => {
      // Only block if the second confirmation is accepted
      blockCanister(canisterId);
    });
  });
};

const blockCanister = (canisterId: string) => {
  blockCanisterIdFromList(canisterId);
  getCanisterInfo(); // Refresh data
};
</script>

<style lang="scss" scoped>
.canisters-list {
  :deep(.q-table__top) {
    padding: 0 !important;
  }
}
</style>
