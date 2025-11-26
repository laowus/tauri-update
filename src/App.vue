<script setup>
import { check } from "@tauri-apps/plugin-updater";
import { onMounted } from "vue";

// 使用onMounted钩子处理异步操作
onMounted(async () => {
  console.log("开始检查更新...");
  console.log("当前时间戳:", new Date().toISOString());

  try {
    console.log("正在发送请求获取更新信息...");
    const startTime = Date.now();

    const update = await check();
    const endTime = Date.now();

    console.log(`请求完成，耗时: ${endTime - startTime}ms`);
    console.log("原始update对象:", update);

    // 验证是否成功获取JSON内容
    if (update && typeof update === "object") {
      console.log("✓ 成功获取到JSON对象");

      // 检查是否有可用更新
      console.log(`是否有更新可用: ${update.available}`);

      // 检查版本信息
      if (update.version) {
        console.log(`远程版本号: ${update.version}`);
      } else {
        console.warn("! 未找到版本号字段");
      }

      // 检查平台信息
      if (update.platforms) {
        console.log("✓ 包含platforms对象");
        console.log("支持的平台:", Object.keys(update.platforms).join(", "));

        // 检查当前平台(Windows-x86_64)的配置
        const currentPlatform = "windows-x86_64";
        if (update.platforms[currentPlatform]) {
          console.log(`✓ 找到当前平台(${currentPlatform})的配置`);
          const platformData = update.platforms[currentPlatform];
          console.log(`下载URL: ${platformData.url || "未提供"}`);
          console.log(`签名: ${platformData.signature ? "已提供" : "未提供"}`);
        } else {
          console.warn(`! 未找到当前平台(${currentPlatform})的配置`);
        }
      } else {
        console.warn("! 未找到platforms字段");
      }
    } else {
      console.error("! 获取的不是有效的JSON对象");
    }
  } catch (error) {
    console.error("更新检查失败:", error);
    console.error("错误类型:", error.constructor.name);

    // 尝试获取更详细的错误信息
    if (error.message) {
      console.error("错误消息:", error.message);
    }

    if (error.cause) {
      console.error("错误原因:", error.cause);
    }

    if (error.code) {
      console.error("错误代码:", error.code);
    }

    // 常见错误类型提示
    if (
      (error.message && error.message.includes("network")) ||
      error.message.includes("url")
    ) {
      console.error("💡 可能是网络连接问题或URL不正确");
    } else if (error.message && error.message.includes("parse")) {
      console.error("💡 可能是JSON格式错误");
    } else if (error.message && error.message.includes("signature")) {
      console.error("💡 可能是签名验证问题，考虑启用skipVerify: true");
    }
  }
});
</script>

<template>
  <div class="main-container">
    <h1>更新检查演示</h1>
    <div class="message">请查看控制台输出以了解更新检查状态</div>
  </div>
</template>

<style>
/* 保持现有样式 */
.main-container {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
  font-family: sans-serif;
}

/* 其他样式保持不变 */
</style>
