<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 应用信息
const appInfo = ref({ version: "", name: "" });
// 更新状态
const updateStatus = ref({
  available: false,
  version: "",
  body: "",
  downloading: false,
});
const message = ref("");

// 获取应用信息
async function fetchAppInfo() {
  try {
    const info = await invoke("get_app_info");
    appInfo.value = info;
  } catch (error) {
    console.error("获取应用信息失败:", error);
  }
}

// 检查更新
async function checkUpdates() {
  try {
    message.value = "正在检查更新...";
    const result = await invoke("check_for_updates");
    if (result.update_available) {
      updateStatus.value = {
        available: true,
        version: result.new_version,
        body: result.body,
        downloading: false,
      };
      message.value = `发现新版本: ${result.new_version}`;
    } else {
      message.value = "当前已是最新版本";
      updateStatus.value.available = false;
    }
  } catch (error) {
    message.value = `检查更新失败: ${error}`;
  }
}

// 组件挂载时获取应用信息
onMounted(() => {
  fetchAppInfo();
});
</script>

<template>
  <div class="main-container">
    <h1>{{ appInfo.name }}</h1>
    <p>当前版本: {{ appInfo.version }}</p>

    <div class="update-section">
      <button @click="checkUpdates" :disabled="updateStatus.downloading">
        检查更新
      </button>

      <p v-if="message" class="message">{{ message }}</p>
    </div>
  </div>
</template>

<style>
.main-container {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
  font-family: sans-serif;
}

.update-section {
  margin-top: 30px;
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
}

.update-info {
  margin-top: 20px;
  padding: 15px;
  background-color: #f0f8ff;
  border-radius: 6px;
}

.update-body {
  color: #555;
  font-size: 0.9em;
}

.message {
  margin-top: 15px;
  color: #666;
}

button {
  padding: 8px 16px;
  background-color: #4285f4;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover:not(:disabled) {
  background-color: #3367d6;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}
</style>
