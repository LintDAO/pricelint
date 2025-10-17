<template>
  <q-card class="terminal-panel">
    <q-card-section class="q-pa-none">
      <pre
        ref="terminalOutput"
      ><code><span v-for="(log, index) in logs" :key="index" class="log-entry"><span class="canister-id">[{{ log.canisterId }}]</span><span class="timestamp">[{{ log.timestamp }}]</span><span class="message">{{ log.message }}</span></span></code></pre>
    </q-card-section>
  </q-card>
</template>

<script setup lang="ts">
import { nextTick, ref } from "vue";
import { useRoute } from "vue-router";

// 获取 canisterId 从路由参数
const route = useRoute();
const canisterId = ref(route.params.canisterId as string);

// 日志数据结构
const logs = ref([
  {
    canisterId: canisterId.value || "Unknown",
    timestamp: new Date().toISOString(),
    message: "Training started...",
  },
  {
    canisterId: canisterId.value || "Unknown",
    timestamp: new Date().toISOString(),
    message: "Epoch 1/10: Loss = 0.532",
  },
]);

// 添加日志
const addLog = (message, timestamp = new Date().toISOString()) => {
  logs.value.push({
    canisterId: canisterId.value || "Unknown",
    timestamp,
    message,
  });
  // 限制日志数量
  if (logs.value.length > 100) logs.value.shift();
  // // 自动滚动到底部
  // nextTick(() => {
  //   const output = terminalOutput.value;
  //   output.scrollTop = output.scrollHeight;
  // });
};

const terminalOutput = ref(null);
</script>

<style scoped>
.terminal-panel {
  background-color: #0c0c0c; /* Campbell 背景色 */
  color: #cccccc; /* Campbell 默认文本色 */
  font-family: "Consolas", "Courier New", monospace;
  max-height: 400px;
  overflow: hidden;
  border-radius: 4px;
}

pre {
  height: 400px; /* 固定高度，模拟终端面板 */
  overflow-y: auto;
  margin: 0;
  padding: 16px;
  white-space: pre-wrap;
  line-height: 1.2; /* 紧凑行高 */
  box-sizing: border-box; /* 确保 padding 不影响高度 */
}
code {
  color: #cccccc;
}
.log-entry {
  display: block;
  margin: 0;
}
.canister-id {
  color: #4caf50; /* Campbell 绿色 */
}
.timestamp {
  color: #ffc107; /* Campbell 亮黄色 */
}
.message {
  color: #cccccc; /* Campbell 浅灰色 */
}
</style>
