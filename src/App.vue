<script setup lang="ts">
import { ref } from "vue";
import SettingsView from "./views/SettingsView.vue";
import BotInfoView from "./views/BotInfoView.vue";
import PanelsView from "./views/PanelsView.vue";
import { useTheme, type ThemeMode } from "./lib/theme";
import { useToast } from "./lib/toast";

const { message: toastMessage } = useToast();

const { mode } = useTheme();

const pages = [
  { key: "settings", label: "密钥设置", icon: "key" },
  { key: "botinfo", label: "机器人信息", icon: "bot" },
  { key: "panels", label: "面板管理", icon: "panel" },
] as const;

type PageKey = (typeof pages)[number]["key"];

const currentPage = ref<PageKey>("settings");

const themeOptions: { key: ThemeMode; label: string }[] = [
  { key: "light", label: "亮色" },
  { key: "dark", label: "暗色" },
  { key: "system", label: "系统" },
];
</script>

<template>
  <div class="layout">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-mark">Q</div>
        <span class="brand-name">QQBotManager</span>
      </div>

      <nav>
        <button
          v-for="p in pages"
          :key="p.key"
          :class="['nav-item', { active: currentPage === p.key }]"
          @click="currentPage = p.key"
        >
          <svg
            v-if="p.icon === 'key'"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <circle cx="8" cy="15" r="4" />
            <path d="M10.8 12.2 20 3" />
            <path d="m16.5 6.5 3 3" />
            <path d="m14 9 2.5 2.5" />
          </svg>
          <svg
            v-else-if="p.icon === 'bot'"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <rect x="4" y="8" width="16" height="12" rx="3" />
            <path d="M12 8V4" />
            <circle cx="12" cy="3" r="1" />
            <circle cx="9" cy="13.5" r="1" fill="currentColor" stroke="none" />
            <circle cx="15" cy="13.5" r="1" fill="currentColor" stroke="none" />
            <path d="M9.5 17h5" />
          </svg>
          <svg
            v-else
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <rect x="3" y="3" width="18" height="18" rx="3" />
            <path d="M3 10h18" />
            <path d="M10 10v11" />
          </svg>
          {{ p.label }}
        </button>
      </nav>

      <div class="theme-switch">
        <button
          v-for="t in themeOptions"
          :key="t.key"
          :class="['theme-item', { active: mode === t.key }]"
          @click="mode = t.key"
        >
          {{ t.label }}
        </button>
      </div>
    </aside>

    <main class="content">
      <SettingsView v-if="currentPage === 'settings'" />
      <BotInfoView v-else-if="currentPage === 'botinfo'" />
      <PanelsView v-else />
    </main>

    <Transition name="toast">
      <div v-if="toastMessage" class="toast">
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="m5 12.5 4.5 4.5L19 7.5" />
        </svg>
        {{ toastMessage }}
      </div>
    </Transition>
  </div>
</template>

<style>
:root {
  font-family: Inter, "Microsoft YaHei", Avenir, Helvetica, Arial, sans-serif;
  font-size: 15px;
  line-height: 1.6;

  --bg: #f3f4f8;
  --surface: #ffffff;
  --text: #1a1d23;
  --muted: #6f7683;
  --border: #e4e7ee;
  --border-strong: #d3d7e0;
  --input-bg: #f4f5f9;
  --accent: #2f7cf6;
  --accent-hover: #1e6af0;
  --accent-soft: rgba(47, 124, 246, 0.12);
  --accent-soft-text: #2f7cf6;
  --ring: rgba(47, 124, 246, 0.22);
  --shadow-sm: 0 1px 2px rgba(16, 24, 40, 0.05);
  --shadow-md: 0 4px 14px rgba(16, 24, 40, 0.08);
  --shadow-lg: 0 12px 40px rgba(16, 24, 40, 0.18);
  --danger: #e5484d;

  color: var(--text);
  background-color: var(--bg);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
}

:root[data-theme="dark"] {
  --bg: #141519;
  --surface: #1e2025;
  --text: #e8eaed;
  --muted: #9aa0aa;
  --border: #2d3037;
  --border-strong: #3a3e46;
  --input-bg: #26282e;
  --accent: #3b82f6;
  --accent-hover: #2f6fe0;
  --accent-soft: rgba(96, 165, 250, 0.16);
  --accent-soft-text: #7cabf8;
  --ring: rgba(96, 165, 250, 0.28);
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 14px rgba(0, 0, 0, 0.35);
  --shadow-lg: 0 12px 40px rgba(0, 0, 0, 0.5);
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  user-select: none;
}

input,
select,
button {
  border-radius: 8px;
  border: 1px solid var(--border-strong);
  padding: 0.45em 0.85em;
  font-size: 1em;
  font-family: inherit;
  color: inherit;
  background-color: var(--input-bg);
  outline: none;
  transition:
    background-color 0.15s,
    border-color 0.15s,
    color 0.15s,
    box-shadow 0.15s;
}

input {
  user-select: text;
}

input::placeholder {
  color: var(--muted);
  opacity: 0.7;
}

input:focus,
select:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--ring);
}

button {
  cursor: pointer;
  background-color: var(--accent);
  border-color: var(--accent);
  color: #fff;
  font-weight: 550;
  box-shadow: var(--shadow-sm);
}

button:hover {
  background-color: var(--accent-hover);
  border-color: var(--accent-hover);
}

button:focus-visible {
  box-shadow: 0 0 0 3px var(--ring);
}

button:disabled {
  opacity: 0.55;
  cursor: default;
}

button.ghost {
  background: var(--surface);
  color: inherit;
  border-color: var(--border-strong);
  box-shadow: var(--shadow-sm);
}

button.ghost:hover {
  background: var(--input-bg);
  border-color: var(--accent);
  color: var(--accent-soft-text);
}

button.danger {
  background: var(--surface);
  color: var(--danger);
  border-color: var(--danger);
  box-shadow: none;
}

button.danger:hover {
  background: var(--danger);
  border-color: var(--danger);
  color: #fff;
}

code {
  background: var(--input-bg);
  border-radius: 4px;
  padding: 1px 5px;
  font-size: 0.92em;
}

.msg {
  font-weight: 600;
}

.msg.ok {
  color: #2da44e;
}

.msg.err {
  color: var(--danger);
  white-space: pre-wrap;
}
</style>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
}

/* ---------- 侧边栏 ---------- */
.sidebar {
  width: 216px;
  flex-shrink: 0;
  background: linear-gradient(180deg, #1d2028 0%, #171921 100%);
  color: #a7adbb;
  display: flex;
  flex-direction: column;
  gap: 22px;
  padding: 18px 12px 14px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 2px 8px;
}

.brand-mark {
  width: 30px;
  height: 30px;
  border-radius: 9px;
  background: linear-gradient(135deg, #5ea0fb, #2563eb);
  color: #fff;
  font-weight: 700;
  font-size: 15px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 8px rgba(37, 99, 235, 0.45);
  flex-shrink: 0;
}

.brand-name {
  color: #f2f4f8;
  font-weight: 650;
  font-size: 0.98em;
  letter-spacing: 0.01em;
}

.sidebar nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  text-align: left;
  background: transparent;
  color: inherit;
  border: none;
  border-radius: 9px;
  padding: 8px 12px;
  font-size: 0.95em;
  box-shadow: none;
}

.nav-item svg {
  width: 17px;
  height: 17px;
  flex-shrink: 0;
  opacity: 0.85;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.07);
  color: #fff;
}

.nav-item.active {
  background: var(--accent);
  color: #fff;
}

.nav-item.active svg {
  opacity: 1;
}

/* ---------- 主题切换 ---------- */
.theme-switch {
  margin-top: auto;
  display: flex;
  gap: 3px;
  background: rgba(255, 255, 255, 0.06);
  border-radius: 9px;
  padding: 3px;
}

.theme-item {
  flex: 1;
  background: transparent;
  border: none;
  color: #a7adbb;
  border-radius: 7px;
  padding: 5px 0;
  font-size: 0.82em;
  box-shadow: none;
}

.theme-item:hover {
  background: transparent;
  color: #fff;
}

.theme-item.active {
  background: rgba(255, 255, 255, 0.14);
  color: #fff;
}

/* ---------- 内容区 ---------- */
.content {
  flex: 1;
  overflow-y: auto;
  padding: 32px 40px;
}

/* ---------- 全局弹窗提示 ---------- */
.toast {
  position: fixed;
  top: 24px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 200;
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-left: 3px solid #2da44e;
  border-radius: 10px;
  padding: 10px 18px;
  font-weight: 600;
  box-shadow: var(--shadow-lg);
  color: var(--text);
}

.toast svg {
  width: 16px;
  height: 16px;
  color: #2da44e;
  flex-shrink: 0;
}

.toast-enter-active,
.toast-leave-active {
  transition:
    opacity 0.2s,
    transform 0.2s;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-8px);
}

.view > h2 {
  margin: 0 0 4px;
  font-size: 1.35em;
  font-weight: 650;
}

.hint {
  color: var(--muted);
  font-size: 0.92em;
}

.hint a {
  color: var(--accent-soft-text);
  text-decoration: none;
}

.hint a:hover {
  text-decoration: underline;
}

.card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 14px;
  padding: 22px 24px;
  box-shadow: var(--shadow-sm);
}
</style>
