<script setup lang="ts">
import { ref } from "vue";
import SettingsView from "./views/SettingsView.vue";
import BotInfoView from "./views/BotInfoView.vue";
import PanelsView from "./views/PanelsView.vue";

const pages = [
  { key: "settings", label: "密钥设置" },
  { key: "botinfo", label: "机器人信息" },
  { key: "panels", label: "面板管理" },
] as const;

type PageKey = (typeof pages)[number]["key"];

const currentPage = ref<PageKey>("settings");
</script>

<template>
  <div class="layout">
    <aside class="sidebar">
      <h1 class="brand">QQBotManager</h1>
      <nav>
        <button
          v-for="p in pages"
          :key="p.key"
          :class="['nav-item', { active: currentPage === p.key }]"
          @click="currentPage = p.key"
        >
          {{ p.label }}
        </button>
      </nav>
    </aside>

    <main class="content">
      <SettingsView v-if="currentPage === 'settings'" />
      <BotInfoView v-else-if="currentPage === 'botinfo'" />
      <PanelsView v-else />
    </main>
  </div>
</template>

<style>
:root {
  font-family: Inter, "Microsoft YaHei", Avenir, Helvetica, Arial, sans-serif;
  font-size: 15px;
  line-height: 1.6;

  color: #1f2328;
  background-color: #f6f7f9;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #e6e6e6;
    background-color: #1e1f22;
  }
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  user-select: none;
}

input,
button {
  border-radius: 8px;
  border: 1px solid rgba(128, 128, 128, 0.4);
  padding: 0.5em 0.9em;
  font-size: 1em;
  font-family: inherit;
  color: inherit;
  background-color: transparent;
  outline: none;
}

input {
  user-select: text;
}

input:focus {
  border-color: #646cff;
}

button {
  cursor: pointer;
  background-color: #646cff;
  border-color: #646cff;
  color: #fff;
}

button:hover {
  filter: brightness(1.1);
}

button:disabled {
  opacity: 0.5;
  cursor: default;
}

button.ghost {
  background: transparent;
  color: inherit;
  border-color: rgba(128, 128, 128, 0.5);
}

button.danger {
  background: transparent;
  color: #e5484d;
  border-color: #e5484d;
}

code {
  background: rgba(128, 128, 128, 0.15);
  border-radius: 4px;
  padding: 1px 5px;
}

.msg {
  font-weight: 600;
}

.msg.ok {
  color: #2da44e;
}

.msg.err {
  color: #e5484d;
  white-space: pre-wrap;
}
</style>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
}

.sidebar {
  width: 200px;
  flex-shrink: 0;
  border-right: 1px solid rgba(128, 128, 128, 0.25);
  padding: 18px 12px;
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.brand {
  font-size: 1.05em;
  text-align: center;
  margin: 0;
}

.sidebar nav {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.nav-item {
  text-align: left;
  background: transparent;
  color: inherit;
  border-color: transparent;
}

.nav-item:hover {
  filter: none;
  background: rgba(128, 128, 128, 0.15);
}

.nav-item.active {
  background: rgba(100, 108, 255, 0.15);
  color: #646cff;
  border-color: rgba(100, 108, 255, 0.4);
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 28px 32px;
}

.view {
  max-width: 860px;
}

h2 {
  margin-top: 0;
}

.hint {
  color: rgba(128, 128, 128, 1);
  font-size: 0.92em;
}
</style>
