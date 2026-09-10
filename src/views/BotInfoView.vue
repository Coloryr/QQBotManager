<script setup lang="ts">
import { ref } from "vue";
import { fetchBotInfo, type BotInfo } from "../lib/api";

const info = ref<BotInfo | null>(null);
const loading = ref(false);
const error = ref("");

async function onFetch() {
  loading.value = true;
  error.value = "";
  info.value = null;
  try {
    info.value = await fetchBotInfo();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <section class="view">
    <h2>机器人信息</h2>
    <div class="row">
      <button :disabled="loading" @click="onFetch">
        {{ loading ? "获取中…" : "获取机器人信息" }}
      </button>
    </div>

    <p v-if="error" class="msg err">{{ error }}</p>

    <div v-if="info" class="card">
      <div v-if="info.avatar" class="avatar-row">
        <img :src="info.avatar" alt="头像" class="avatar" referrerpolicy="no-referrer" />
      </div>
      <table>
        <tbody>
          <tr><th>机器人 ID</th><td>{{ info.id || "-" }}</td></tr>
          <tr><th>名称</th><td>{{ info.username || "-" }}</td></tr>
          <tr><th>是否机器人</th><td>{{ info.bot ? "是" : "否" }}</td></tr>
          <tr v-if="info.union_openid"><th>union_openid</th><td>{{ info.union_openid }}</td></tr>
          <tr v-if="info.union_user_account"><th>union_user_account</th><td>{{ info.union_user_account }}</td></tr>
        </tbody>
      </table>
    </div>

    <p v-else-if="!loading && !error" class="hint">
      点击按钮后，将使用已保存的 AppID / AppSecret 通过 curl 调用 QQ 开放平台接口
      <code>GET /users/@me</code>。
    </p>
  </section>
</template>

<style scoped>
.row {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}
.card {
  max-width: 560px;
  border: 1px solid rgba(128, 128, 128, 0.35);
  border-radius: 10px;
  padding: 16px 20px;
}
.avatar-row {
  display: flex;
  margin-bottom: 12px;
}
.avatar {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  object-fit: cover;
}
table {
  width: 100%;
  border-collapse: collapse;
}
th,
td {
  text-align: left;
  padding: 6px 8px;
  border-bottom: 1px solid rgba(128, 128, 128, 0.2);
  word-break: break-all;
}
th {
  width: 160px;
  font-weight: 600;
  white-space: nowrap;
}
</style>
