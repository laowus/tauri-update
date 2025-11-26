<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const appVersion = ref("");
const appName = ref("");
const updateAvailable = ref(false);
const newVersion = ref("");
const updateStatus = ref("");
const checkingUpdate = ref(false);
const installingUpdate = ref(false);

// 在组件挂载时获取应用信息
onMounted(async () => {
  try {
    const appInfo = await invoke("get_app_info");
    appVersion.value = appInfo?.version || "未知版本";
    appName.value = appInfo?.name || "应用";
  } catch (error) {
    console.error("获取应用信息失败:", error);
  }
});

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

// 检查更新
async function checkUpdate() {
  checkingUpdate.value = true;
  updateStatus.value = "正在检查更新...";
  try {
    const result = await invoke("check_for_updates");
    updateAvailable.value = result.update_available;
    if (result.update_available) {
      newVersion.value = result.new_version;
      updateStatus.value = `发现新版本: ${newVersion.value}`;
    } else {
      updateStatus.value = "当前已是最新版本";
    }
  } catch (error) {
    console.error("检查更新失败:", error);
    updateStatus.value = `检查更新失败: ${error}`;
  } finally {
    checkingUpdate.value = false;
  }
}

// 安装更新
async function installUpdate() {
  if (!updateAvailable.value) return;

  installingUpdate.value = true;
  updateStatus.value = "正在下载并安装更新...";
  try {
    await invoke("install_update");
    updateStatus.value = "更新成功，应用将重启";
  } catch (error) {
    console.error("安装更新失败:", error);
    updateStatus.value = `安装更新失败: ${error}`;
  } finally {
    installingUpdate.value = false;
  }
}
</script>

<template>
  <main class="container">
    <h1>{{ appName }}</h1>
    <p>当前版本: {{ appVersion }}</p>

    <div class="update-section">
      <h3>更新检查</h3>
      <button
        @click="checkUpdate"
        :disabled="checkingUpdate || installingUpdate"
      >
        {{ checkingUpdate ? "检查中..." : "检查更新" }}
      </button>

      <p v-if="updateStatus" class="update-status">{{ updateStatus }}</p>

      <button
        v-if="updateAvailable"
        @click="installUpdate"
        :disabled="installingUpdate"
        class="update-button"
      >
        {{ installingUpdate ? "安装中..." : `安装版本 ${newVersion}` }}
      </button>
    </div>

    <div class="greet-section">
      <h3>问候功能</h3>
      <input v-model="name" placeholder="输入你的名字" />
      <button @click="greet">问候</button>
      <p v-if="greetMsg">{{ greetMsg }}</p>
    </div>
  </main>
</template>

<style>
.container {
  padding: 20px;
  text-align: center;
  max-width: 600px;
  margin: 0 auto;
}

h1 {
  color: #333;
  margin-bottom: 10px;
}

h3 {
  color: #555;
  margin-top: 30px;
  margin-bottom: 15px;
}

p {
  color: #666;
  font-size: 16px;
  margin-bottom: 15px;
}

.update-section {
  background-color: #f5f5f5;
  padding: 20px;
  border-radius: 8px;
  margin: 20px 0;
}

.update-status {
  color: #0066cc;
  font-weight: bold;
  margin: 15px 0;
}

.update-button {
  background-color: #4caf50;
  color: white;
  font-weight: bold;
}

.greet-section {
  margin-top: 30px;
}

input {
  padding: 8px 12px;
  margin-right: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

button {
  padding: 8px 16px;
  background-color: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover:not(:disabled) {
  opacity: 0.9;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}
</style>
