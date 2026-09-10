<script setup lang="ts">
import { onMounted, ref } from "vue";
import {
  loadConfig,
  saveConfig,
  type AppConfig,
} from "../lib/api";

const emit = defineEmits<{ saved: [config: AppConfig] }>();

const appId = ref("");
const appSecret = ref("");
const message = ref("");
const messageOk = ref(false);
const saving = ref(false);

onMounted(async () => {
  try {
    const cfg = await loadConfig();
    appId.value = cfg.app_id;
    appSecret.value = cfg.app_secret;
  } catch (e) {
    message.value = `读取配置失败: ${e}`;
  }
});

async function onSave() {
  message.value = "";
  if (!appId.value.trim() || !appSecret.value.trim()) {
    message.value = "AppID 和 AppSecret 均不能为空";
    return;
  }
  saving.value = true;
  try {
    const cfg: AppConfig = {
      app_id: appId.value.trim(),
      app_secret: appSecret.value.trim(),
    };
    await saveConfig(cfg);
    message.value = "已保存";
    messageOk.value = true;
    emit("saved", cfg);
  } catch (e) {
    message.value = `保存失败: ${e}`;
    messageOk.value = false;
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <section class="view">
    <h2>机器人密钥设置</h2>
    <p class="hint">
      在
      <a href="https://q.qq.com" target="_blank">QQ 开放平台</a>
      「机器人管理 → 开发设置」中获取 AppID 和 AppSecret。
    </p>

    <form class="form" @submit.prevent="onSave">
      <label class="field">
        <span>AppID</span>
        <input v-model="appId" placeholder="例如 102542802" autocomplete="off" />
      </label>

      <label class="field">
        <span>AppSecret</span>
        <input
          v-model="appSecret"
          type="password"
          placeholder="机器人密钥"
          autocomplete="new-password"
        />
      </label>

      <div class="row">
        <button type="submit" :disabled="saving">
          {{ saving ? "保存中…" : "保存" }}
        </button>
        <span v-if="message" :class="['msg', messageOk ? 'ok' : 'err']">{{ message }}</span>
      </div>
    </form>
  </section>
</template>

<style scoped>
.form {
  max-width: 460px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.field span {
  font-weight: 600;
}
.row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.hint a {
  color: #646cff;
}
</style>
