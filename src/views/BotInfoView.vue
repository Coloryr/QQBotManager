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
    <p class="hint">
      使用已保存的 AppID / AppSecret 调用 QQ 开放平台接口
      <code>GET /users/@me</code>。
    </p>

    <div class="toolbar">
      <button :disabled="loading" @click="onFetch">
        {{ loading ? "获取中…" : "获取机器人信息" }}
      </button>
    </div>
    <p v-if="error" class="msg err fetch-error">{{ error }}</p>

    <div v-if="info" class="card profile-card">
      <div class="profile-head">
        <img
          v-if="info.avatar"
          :src="info.avatar"
          alt="头像"
          class="avatar"
          referrerpolicy="no-referrer"
        />
        <div v-else class="avatar avatar-placeholder">{{ (info.username || "?").slice(0, 1) }}</div>
        <div class="profile-titles">
          <div class="profile-name">
            {{ info.username || "-" }}
            <span v-if="info.bot" class="badge badge-bot">机器人</span>
          </div>
          <div class="profile-id">{{ info.id || "-" }}</div>
        </div>
      </div>

      <div class="kv">
        <div class="kv-row">
          <span class="kv-label">机器人 ID</span>
          <span class="kv-value">{{ info.id || "-" }}</span>
        </div>
        <div class="kv-row">
          <span class="kv-label">名称</span>
          <span class="kv-value">{{ info.username || "-" }}</span>
        </div>
        <div class="kv-row">
          <span class="kv-label">是否机器人</span>
          <span class="kv-value">{{ info.bot ? "是" : "否" }}</span>
        </div>
        <div v-if="info.union_openid" class="kv-row">
          <span class="kv-label">union_openid</span>
          <span class="kv-value">{{ info.union_openid }}</span>
        </div>
        <div v-if="info.union_user_account" class="kv-row">
          <span class="kv-label">union_user_account</span>
          <span class="kv-value">{{ info.union_user_account }}</span>
        </div>
      </div>
    </div>

    <div v-else-if="!loading && !error" class="card empty-card">
      <p class="hint">点击「获取机器人信息」查看机器人资料。</p>
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
.toolbar {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 12px;
  margin: 20px 0 0;
}
.fetch-error {
  margin: 12px 0 0;
  text-align: center;
}
.profile-card {
  max-width: 560px;
  margin: 16px auto 0;
  padding: 0;
  overflow: hidden;
}
.profile-head {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 22px 24px;
  background: linear-gradient(135deg, var(--accent-soft), transparent 70%);
}
.avatar {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  box-shadow: var(--shadow-sm);
}
.avatar-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--accent);
  color: #fff;
  font-size: 1.5em;
  font-weight: 650;
}
.profile-titles {
  min-width: 0;
}
.profile-name {
  font-size: 1.15em;
  font-weight: 650;
  display: flex;
  align-items: center;
  gap: 8px;
}
.profile-id {
  color: var(--muted);
  font-size: 0.9em;
  word-break: break-all;
}
.badge {
  font-size: 0.72em;
  font-weight: 600;
  border-radius: 5px;
  padding: 1px 7px;
  white-space: nowrap;
}
.badge-bot {
  color: var(--accent-soft-text);
  background: var(--accent-soft);
}
.kv {
  padding: 8px 24px 14px;
}
.kv-row {
  display: flex;
  gap: 16px;
  padding: 9px 0;
  border-bottom: 1px solid var(--border);
  font-size: 0.95em;
}
.kv-row:last-child {
  border-bottom: none;
}
.kv-label {
  width: 170px;
  flex-shrink: 0;
  color: var(--muted);
}
.kv-value {
  word-break: break-all;
}
.empty-card {
  max-width: 560px;
  margin: 16px auto 0;
  padding: 28px 24px;
  text-align: center;
}
.empty-card .hint {
  margin: 0;
}
</style>
