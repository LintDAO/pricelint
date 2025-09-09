<template>
  <div class="q-pt-lg">
    <!-- 页面头部 -->
    <div class="q-mb-xl">
      <div class="text-h5 q-ma-none">Canister Details</div>
      <p class="text-grey-6 q-ma-none q-mt-xs">
        Configure your prediction model settings
      </p>
    </div>
    <q-separator />
    <!-- 主要内容 -->
    <div class="q-gutter-y-xl">
      <!-- 基本信息 -->
      <div class="row q-gutter-y-lg">
        <div class="col-12">
          <h2 class="text-h6 q-ma-none">Basic Information</h2>
        </div>
        <!-- Canister Name -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Canister Name
              </div>
              <div class="text-grey-6 text-body2">
                The display name for this prediction model canister.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-input
                v-model="canisterData.name"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>

        <!-- Principal ID -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Principal ID
              </div>
              <div class="text-grey-6 text-body2">
                Unique identifier for this canister on the Internet Computer.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-input
                :model-value="canisterId"
                outlined
                dense
                readonly
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>

        <!-- Status -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Status
              </div>
              <div class="text-grey-6 text-body2">
                Current operational status of the canister.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-select
                v-model="canisterData.status"
                :options="statusOptions"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>
      </div>
      <q-separator class="q-my-xl" />
      <!-- 预测模型配置 -->
      <div class="row q-gutter-y-lg">
        <div class="col-12">
          <h2 class="text-h6 q-ma-none">Prediction Model Configuration</h2>
        </div>

        <!-- Algorithm -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Algorithm
              </div>
              <div class="text-grey-6 text-body2">
                Select the machine learning algorithm for predictions.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-select
                v-model="modelConfig.algorithm"
                :options="algorithmOptions"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>

        <!-- Confidence Threshold -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Confidence Threshold
              </div>
              <div class="text-grey-6 text-body2">
                Minimum confidence level required for predictions (0.0 - 1.0).
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-input
                v-model.number="modelConfig.confidenceThreshold"
                type="number"
                min="0"
                max="1"
                step="0.01"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>

        <!-- Max Predictions -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Max Predictions
              </div>
              <div class="text-grey-6 text-body2">
                Maximum number of predictions to generate per request.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-input
                v-model.number="modelConfig.maxPredictions"
                type="number"
                min="1"
                max="1000"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>

        <!-- Timeout -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Timeout (seconds)
              </div>
              <div class="text-grey-6 text-body2">
                Maximum time to wait for prediction results.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-input
                v-model.number="modelConfig.timeout"
                type="number"
                min="1"
                max="300"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>
      </div>
      <q-separator />
      <!-- 数据源配置 -->
      <div class="row q-gutter-y-lg">
        <div class="col-12">
          <h2 class="text-h6 q-ma-none">Data Source Configuration</h2>
        </div>

        <!-- Data Source URL -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Data Source URL
              </div>
              <div class="text-grey-6 text-body2">
                URL endpoint for fetching training and prediction data.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-input
                v-model="dataSource.url"
                outlined
                dense
                placeholder="https://api.example.com/data"
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>

        <!-- Data Format -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Data Format
              </div>
              <div class="text-grey-6 text-body2">
                Expected format of the input data.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-select
                v-model="dataSource.format"
                :options="formatOptions"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>

        <!-- Refresh Interval -->
        <div class="col-12">
          <div class="row">
            <div class="col-12 col-md-6">
              <div class="text-subtitle1 text-weight-medium q-mb-xs">
                Refresh Interval (minutes)
              </div>
              <div class="text-grey-6 text-body2">
                How often to fetch new data from the source.
              </div>
            </div>
            <div class="col-12 col-md-6">
              <q-input
                v-model.number="dataSource.refreshInterval"
                type="number"
                min="1"
                max="1440"
                outlined
                dense
                class="q-mt-sm q-mt-md-none"
              />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部操作按钮 -->
    <div
      class="row justify-end q-gutter-sm q-mt-xl q-pt-xl"
      style="border-top: 1px solid #e0e0e0"
    >
      <q-btn flat label="Reset" @click="resetForm" :loading="loading" />
      <q-btn
        color="primary"
        label="Save changes"
        @click="saveChanges"
        :loading="loading"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();

const canisterId = ref(route.params.canisterId as string);

const loading = ref(false);

const statusOptions = ["Running", "Stopped", "Maintenance"];
const algorithmOptions = [
  "Random Forest",
  "Neural Network",
  "SVM",
  "Linear Regression",
];
const formatOptions = ["JSON", "CSV", "XML"];

const canisterData = reactive({
  name: "Prediction Model v1.0",
  status: "Running",
});

const modelConfig = reactive({
  algorithm: "Random Forest",
  confidenceThreshold: 0.85,
  maxPredictions: 100,
  timeout: 30,
});

const dataSource = reactive({
  url: "",
  format: "JSON",
  refreshInterval: 60,
});

const goBack = () => {
  router.back();
};

const resetForm = () => {
  canisterData.name = "Prediction Model v1.0";
  canisterData.status = "Running";
  modelConfig.algorithm = "Random Forest";
  modelConfig.confidenceThreshold = 0.85;
  modelConfig.maxPredictions = 100;
  modelConfig.timeout = 30;
  dataSource.url = "";
  dataSource.format = "JSON";
  dataSource.refreshInterval = 60;
};

const saveChanges = async () => {
  loading.value = true;
  try {
    await new Promise((resolve) => setTimeout(resolve, 1000));
    console.log("Changes saved successfully");
  } catch (error) {
    console.error("Error saving changes:", error);
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.q-page {
  max-width: 1200px;
  margin: 0 auto;
}
</style>
