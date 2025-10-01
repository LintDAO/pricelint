<!-- CyclesDialog.vue -->
<template>
  <q-dialog v-model="localDialogVisible" persistent>
    <q-card class="swap-card">
      <q-card-section class="q-pa-md">
        <div class="text-h6 text-center">
          {{ operation === "topUp" ? "Top Up Cycles" : "Create New Canister" }}
        </div>
        <div class="text-caption text-center q-mt-sm">
          {{
            operation === "topUp"
              ? `These ICP will be topped up to ${targetCanisterId} canister.`
              : `At least ${minimumCycles}T Cycles is required to create a new canister.`
          }}
        </div>
      </q-card-section>

      <q-card-section style="padding-top: 0">
        <!-- From: ICP 输入 -->
        <div class="swap-box q-mb-sm">
          <div class="balance-text">
            Balance: {{ userICP }} ICP
            <q-btn
              flat
              dense
              label="Max"
              color="primary"
              size="sm"
              class="q-ml-sm"
              @click="icpInput = userICP"
            />
          </div>
          <div class="swap-label">From</div>
          <q-input
            v-model.number="icpInput"
            type="number"
            placeholder="0.0"
            filled
            dense
            class="swap-input"
            :rules="[
              (val) => val >= 0 || 'Please enter a valid amount',
              (val) => val <= userICP || 'Insufficient ICP balance',
            ]"
          >
            <template v-slot:append>
              <span class="text-weight-bold">ICP</span>
            </template>
          </q-input>
        </div>

        <!-- 箭头图标 -->
        <div class="arrow-container q-my-sm">
          <q-icon name="arrow_downward" size="24px" color="primary" />
        </div>

        <!-- To: Cycles 输出 -->
        <div class="swap-box q-my-md">
          <div class="swap-label">To</div>
          <q-input
            :model-value="convertedCycles"
            type="number"
            placeholder="0.0"
            filled
            dense
            readonly
            :rules="[
              (val) =>
                val >= minimumCycles ||
                `At least ${minimumCycles}T cycles are required to successfully create.`,
            ]"
            class="swap-input"
          >
            <template v-slot:append>
              <span class="text-weight-bold">T Cycles</span>
            </template>
          </q-input>
          <div class="balance-text">1 ICP ≈ {{ icpToCyclesRate }} T Cycles</div>
        </div>
      </q-card-section>

      <q-card-actions align="right" class="q-pa-md">
        <q-btn
          flat
          label="Cancel"
          color="negative"
          class="action-btn"
          @click="closeDialog"
        />
        <q-btn
          unelevated
          :label="operation === 'topUp' ? 'Top Up Cycles' : 'Create Canister'"
          color="primary"
          class="action-btn"
          :disable="isConfirmDisabled"
          :loading="isLoading"
          @click="handleConfirm"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { getCurrentPrincipal } from "@/api/canister_pool";
import {
  burnICPcreateCanister,
  getICPBalance,
  getICPtoCyclesRate,
  topupCycles,
} from "@/api/icp";
import { p2a } from "@/utils/common";
import { showMessageError, showMessageSuccess } from "@/utils/message";
import { computed, onMounted, ref, watch } from "vue";

interface Props {
  visible: boolean; // 控制 Dialog 显示
  operation: "topUp" | "createCanister"; // 操作类型
  targetCanisterId?: string;
  onClose?: () => void; // 关闭回调
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: "update:visible", value: boolean): void;
}>();

const userICP = ref<number>(0); // 用户 ICP 余额
const icpToCyclesRate = ref<number>(0); // 转换比例
const minimumCycles = 0.5; //最低充值cycles的额度，0.5T
const icpInput = ref<number>(0); // 用户输入的 ICP 数量
const localDialogVisible = ref<boolean>(props.visible); // 本地 Dialog 显示状态
const isLoading = ref<boolean>(false); // 确认操作加载状态

// 同步外部 visible 属性
watch(
  () => props.visible,
  (newVal) => {
    localDialogVisible.value = newVal;
    if (newVal) {
      fetchUserICP(); // 每次打开 Dialog 时重新查询余额
    }
  }
);

// 同步本地 Dialog 状态到父组件
watch(localDialogVisible, (newVal) => {
  emit("update:visible", newVal);
});

// 同步 operation 变化，重置输入
watch(
  () => props.operation,
  () => {
    icpInput.value = 0;
  }
);

// 查询用户 ICP 余额
const fetchUserICP = async () => {
  try {
    const principal = getCurrentPrincipal();
    userICP.value = await getICPBalance(p2a(principal));
  } catch (error) {
    showMessageError("Failed to fetch ICP balance: " + error);
  }
};

//Top Up Cycles
const topUpCycles = async (icpAmount: number) => {
  isLoading.value = true;
  if (props.targetCanisterId) {
    await topupCycles(icpAmount, props.targetCanisterId);
  } else {
    showMessageError("Canister Id not undefined");
  }

  isLoading.value = false;
};

// 创建新容器
const createCanister = async (icpAmount: number) => {
  //使用用户指定的icp转换为cycles并创建容器
  isLoading.value = true;
  await burnICPcreateCanister(icpAmount);
  isLoading.value = false;
};

// 计算转换后的 Cycles
const convertedCycles = computed((): number => {
  return Number((icpInput.value * icpToCyclesRate.value).toFixed(5));
});

// 确认按钮禁用逻辑
const isConfirmDisabled = computed(() => {
  if (icpInput.value <= 0 || icpInput.value > userICP.value) return true;
  if (
    props.operation === "createCanister" &&
    convertedCycles.value < minimumCycles
  )
    return true;
  return false;
});

// 初始化数据
onMounted(async () => {
  try {
    icpToCyclesRate.value = await getICPtoCyclesRate();
    await fetchUserICP();
  } catch (error) {
    showMessageError("Failed to initialize data");
  }
});

// 处理确认操作
const handleConfirm = async () => {
  if (icpInput.value > userICP.value) {
    showMessageError("Insufficient ICP balance");
    return;
  }
  if (
    props.operation === "createCanister" &&
    convertedCycles.value < minimumCycles
  ) {
    showMessageError(
      `Minimum ${minimumCycles} TCycle required for canister creation`
    );
    return;
  }

  isLoading.value = true;
  try {
    if (props.operation === "topUp") {
      await topUpCycles(icpInput.value);
      showMessageSuccess(
        `Successfully topped up ${convertedCycles.value} T Cycles from ${icpInput.value} ICP`
      );
    } else {
      await createCanister(icpInput.value);
      showMessageSuccess(
        `Successfully created canister with ${convertedCycles.value} T Cycles`
      );
    }
    // 更新余额
    userICP.value -= icpInput.value;
  } catch (error) {
  } finally {
    isLoading.value = false;
  }

  // 重置输入
  icpInput.value = 0;
  localDialogVisible.value = false;
};

// 关闭 Dialog
const closeDialog = () => {
  localDialogVisible.value = false;
  props.onClose?.();
};
</script>

<style scoped></style>
