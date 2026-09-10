<script setup lang="ts">
import { onMounted, ref } from "vue";
import {
  getPublicIp,
  loadConfig,
  saveConfig,
  type AppConfig,
} from "../lib/api";
import { showToast } from "../lib/toast";

const emit = defineEmits<{ saved: [config: AppConfig] }>();

const appId = ref("");
const appSecret = ref("");
const message = ref("");
const saving = ref(false);

const publicIp = ref("");
const ipError = ref("");
const ipLoading = ref(false);

onMounted(async () => {
  try {
    const cfg = await loadConfig();
    appId.value = cfg.app_id;
    appSecret.value = cfg.app_secret;
  } catch (e) {
    message.value = `读取配置失败: ${e}`;
  }
  fetchIp();
});

async function fetchIp() {
  ipLoading.value = true;
  ipError.value = "";
  try {
    publicIp.value = await getPublicIp();
  } catch (e) {
    ipError.value = String(e);
  } finally {
    ipLoading.value = false;
  }
}

async function copyIp() {
  try {
    await navigator.clipboard.writeText(publicIp.value);
  } catch {
    ipError.value = "复制失败，请手动复制";
  }
}

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
    message.value = "";
    showToast("已保存");
    emit("saved", cfg);
  } catch (e) {
    message.value = `保存失败: ${e}`;
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

    <div class="card form-card">
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

        <div class="field">
          <span>IP 白名单</span>
          <p class="hint inline-hint">
            需要在开放平台「开发设置 → IP 白名单」中添加本机的公网 IP，否则接口调用会被拒绝：
          </p>
          <div class="ip-row">
            <code class="ip-value">
              <template v-if="ipLoading">获取中…</template>
              <template v-else-if="publicIp">{{ publicIp }}</template>
              <template v-else>获取失败</template>
            </code>
            <button
              v-if="publicIp"
              type="button"
              class="ghost small"
              @click="copyIp"
            >
              复制
            </button>
            <button type="button" class="ghost small" :disabled="ipLoading" @click="fetchIp">
              {{ ipLoading ? "获取中…" : "刷新" }}
            </button>
          </div>
          <p v-if="ipError" class="msg err">{{ ipError }}</p>
        </div>

        <div class="row">
          <button type="submit" :disabled="saving">
            {{ saving ? "保存中…" : "保存" }}
          </button>
        </div>
        <p v-if="message" class="msg err save-msg">{{ message }}</p>
      </form>
    </div>
  </section>
</template>

<style scoped>
.view {
  max-width: 880px;
  margin: 0 auto;
}
h2 {
  text-align: center;
}
.view > .hint {
  text-align: center;
}
.form-card {
  margin: 20px auto 0;
  max-width: 480px;
}
.form {
  display: flex;
  flex-direction: column;
  gap: 18px;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.field > span {
  font-weight: 600;
  font-size: 0.92em;
}
.inline-hint {
  margin: 0;
}
.ip-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}
.ip-value {
  padding: 4px 10px;
  font-size: 1em;
}
button.small {
  padding: 3px 12px;
  font-size: 0.85em;
}
.row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.row button {
  min-width: 96px;
}
.save-msg {
  margin: -8px 0 0;
}
</style>
