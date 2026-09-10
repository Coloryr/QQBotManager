<script setup lang="ts">
import { onMounted, ref } from "vue";
import {
  addPanel,
  deletePanel,
  listPanels,
  updatePanel,
  type Panel,
  type PanelItem,
  type PanelRecord,
  type Scope,
} from "../lib/api";
import PanelEditForm from "./PanelEditForm.vue";
import { showToast } from "../lib/toast";
import { weightedLen } from "../lib/text";

const scopes: { value: Scope; label: string }[] = [
  { value: "c2c", label: "单聊" },
  { value: "group", label: "群聊" },
  { value: "channel", label: "频道" },
  { value: "dm", label: "群私信" },
];

const scope = ref<Scope>("group");
const records = ref<PanelRecord[]>([]);
const loading = ref(false);
const loadingMore = ref(false);
const nextCursor = ref("");
const isEnd = ref(false);
const error = ref("");

// 新建面板的草稿（卡片默认就是编辑状态，直接填、直接存）
const draft = ref<Panel | null>(null);
// 每张卡片的保存错误信息
const errors = ref<Record<string, string>>({});

let uid = 0;
function emptyItem(): PanelItem {
  uid += 1;
  return { name: "", desc: "", type: "command", link: "", only_admin: false };
}

function newPanel(): Panel {
  return { items: [emptyItem()], remark: "", version: 0 };
}

async function fetchPanels(reset: boolean) {
  error.value = "";
  if (reset) {
    loading.value = true;
  } else {
    loadingMore.value = true;
  }
  try {
    const res = await listPanels(scope.value, reset ? null : nextCursor.value);
    records.value = reset ? res.records : [...records.value, ...res.records];
    nextCursor.value = res.next_cursor;
    isEnd.value = res.is_end;
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
    loadingMore.value = false;
  }
}

function refresh() {
  return fetchPanels(true);
}

function switchScope(s: Scope) {
  if (scope.value === s) return;
  scope.value = s;
  records.value = [];
  nextCursor.value = "";
  isEnd.value = false;
  draft.value = null;
  errors.value = {};
  refresh();
}

function openCreate() {
  delete errors.value["__draft"];
  draft.value = newPanel();
}

function cancelDraft() {
  delete errors.value["__draft"];
  draft.value = null;
}

function addDraftItem() {
  draft.value?.items.push(emptyItem());
}

function removeDraftItem(index: number) {
  draft.value?.items.splice(index, 1);
}

function moveDraftItem(index: number, dir: -1 | 1) {
  if (draft.value) moveItems(draft.value.items, index, dir);
}

function moveItems(items: PanelItem[], index: number, dir: -1 | 1) {
  const target = index + dir;
  if (target < 0 || target >= items.length) return;
  [items[index], items[target]] = [items[target], items[index]];
}

function validate(panel: Panel): string {
  if (!panel.items.length) return "面板至少需要一个条目";
  const names = new Set<string>();
  for (let i = 0; i < panel.items.length; i++) {
    const item = panel.items[i];
    const name = item.name.trim();
    if (!name) return `第 ${i + 1} 个条目缺少元素名称`;
    if (weightedLen(name) > 14) return `条目「${name}」名称过长（最多 14 字符，约 7 个汉字）`;
    if (names.has(name)) return `条目名称「${name}」重复`;
    names.add(name);
    const desc = item.desc.trim();
    if (!desc) return `条目「${name}」缺少元素描述`;
    if (weightedLen(desc) > 30) return `条目「${name}」描述过长（最多 30 字符，约 15 个汉字）`;
    if (item.type === "link" && !item.link.trim()) {
      return `条目「${name}」缺少跳转链接`;
    }
  }
  return "";
}

function cleanPanel(panel: Panel): Panel {
  return {
    items: panel.items.map((i) => ({
      ...i,
      name: i.name.trim(),
      desc: i.desc.trim(),
      link: i.link.trim(),
    })),
    remark: panel.remark.trim(),
    version: panel.version,
  };
}

async function saveDraft() {
  if (!draft.value) return;
  const err = validate(draft.value);
  if (err) {
    errors.value["__draft"] = err;
    return;
  }
  try {
    await addPanel({
      panel: cleanPanel(draft.value),
      scope: scope.value,
      target_type: null,
      user_openids: null,
      group_openids: null,
    });
    draft.value = null;
    delete errors.value["__draft"];
    showToast("面板已创建");
    delete errors.value["__draft"];
    await refresh();
  } catch (e) {
    errors.value["__draft"] = String(e);
  }
}

async function saveRecord(rec: PanelRecord) {
  const err = validate(rec.panel);
  if (err) {
    errors.value[rec.panel_id] = err;
    return;
  }
  try {
    await updatePanel(rec.panel_id, cleanPanel(rec.panel));
    delete errors.value[rec.panel_id];
    showToast("面板已更新");
    await refresh();
  } catch (e) {
    errors.value[rec.panel_id] = String(e);
  }
}

async function onDelete(rec: PanelRecord) {
  const label = rec.panel.remark || rec.panel_id;
  if (!window.confirm(`确定删除面板「${label}」吗？此操作不可恢复。`)) return;
  error.value = "";
  try {
    await deletePanel(rec.panel_id);
    showToast("面板已删除");
    await refresh();
  } catch (e) {
    error.value = String(e);
  }
}

async function copyPanelId(id: string) {
  try {
    await navigator.clipboard.writeText(id);
    showToast("已复制 Panel ID");
  } catch {
    error.value = "复制失败，请手动复制";
  }
}

onMounted(() => {
  refresh();
});
</script>

<template>
  <section class="view wide">
    <h2>面板管理</h2>
    <p class="hint">
      管理 QQ 开放平台 v2 消息面板（<code>/v2/panels</code>），
      面板条目将展示在聊天窗口供用户点击。所有字段直接编辑，改完点「保存」。
    </p>

    <div class="toolbar">
      <div class="scope-tabs">
        <button
          v-for="s in scopes"
          :key="s.value"
          :class="['scope-tab', { active: scope === s.value }]"
          @click="switchScope(s.value)"
        >
          {{ s.label }}
        </button>
      </div>
      <div class="toolbar-actions">
        <span v-if="records.length" class="hint">
          共 {{ records.length }} 个{{ isEnd ? "" : "+" }}
          <template v-if="!draft">（QQ 限制每个范围只能有一个面板）</template>
        </span>
        <button class="ghost" :disabled="loading" @click="refresh">
          {{ loading ? "刷新中…" : "刷新" }}
        </button>
        <!-- 当前范围已有面板时隐藏创建入口 -->
        <button v-if="!records.length && !draft" :disabled="!!draft" @click="openCreate">
          新建面板
        </button>
      </div>
    </div>

    <p v-if="error" class="msg err">{{ error }}</p>

    <div v-if="records.length || draft" class="panel-grid">
      <!-- 新建面板草稿卡片，默认就是编辑状态 -->
      <div v-if="draft" class="panel-card draft-card">
        <div class="card-head">
          <input
            v-model="draft.remark"
            class="remark-input"
            placeholder="面板备注（可选）"
          />
          <span class="card-version">新</span>
        </div>

        <PanelEditForm
          :items="draft.items"
          @add="addDraftItem"
          @remove="removeDraftItem"
          @move="moveDraftItem"
        />

        <p v-if="errors['__draft']" class="msg err">{{ errors["__draft"] }}</p>

        <div class="card-actions">
          <button class="ghost" @click="cancelDraft">取消</button>
          <button :disabled="!draft.items.length" @click="saveDraft">保存</button>
        </div>
      </div>

      <!-- 已有面板：卡片本身就是编辑状态 -->
      <div v-for="rec in records" :key="rec.panel_id" class="panel-card">
        <div class="card-head">
          <input
            v-model="rec.panel.remark"
            class="remark-input"
            placeholder="面板备注（可选）"
          />
          <span class="card-version">v{{ rec.version }}</span>
        </div>

        <PanelEditForm
          :items="rec.panel.items"
          @add="rec.panel.items.push(emptyItem())"
          @remove="(i) => rec.panel.items.splice(i, 1)"
          @move="(i, d) => moveItems(rec.panel.items, i, d)"
        />

        <p v-if="errors[rec.panel_id]" class="msg err">{{ errors[rec.panel_id] }}</p>

        <div class="card-meta">
          <span class="hint">{{ rec.updated_at || "无更新时间" }}</span>
          <button class="copy-btn" title="复制 Panel ID" @click="copyPanelId(rec.panel_id)">
            <code>{{ rec.panel_id }}</code>
            复制
          </button>
        </div>

        <div class="card-actions">
          <button class="danger" @click="onDelete(rec)">删除</button>
          <button :disabled="!rec.panel.items.length" @click="saveRecord(rec)">保存</button>
        </div>
      </div>
    </div>

    <div v-if="!isEnd && nextCursor && records.length" class="load-more">
      <button class="ghost" :disabled="loadingMore" @click="fetchPanels(false)">
        {{ loadingMore ? "加载中…" : "加载更多" }}
      </button>
    </div>

    <div v-if="!loading && !records.length && !draft && !error" class="empty-state">
      <p class="hint">当前范围下暂无面板，点击「新建面板」创建一个吧。</p>
    </div>
  </section>
</template>

<style scoped>
/* ---------- 工具栏 ---------- */
.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin: 20px 0 18px;
  flex-wrap: wrap;
}
.scope-tabs {
  display: flex;
  gap: 3px;
  background: var(--input-bg);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 3px;
}
.scope-tab {
  background: transparent;
  color: var(--muted);
  border: none;
  border-radius: 7px;
  padding: 4px 14px;
  font-size: 0.9em;
  box-shadow: none;
}
.scope-tab:hover {
  background: transparent;
  color: var(--text);
}
.scope-tab.active {
  background: var(--surface);
  color: var(--accent-soft-text);
  box-shadow: var(--shadow-sm);
}
.toolbar-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

/* ---------- 面板卡片（始终为编辑状态） ---------- */
.panel-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: 14px;
  align-items: start;
}
.panel-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 14px;
  padding: 16px 18px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  box-shadow: var(--shadow-sm);
  transition:
    box-shadow 0.15s,
    border-color 0.15s;
}
.panel-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--border-strong);
}
.draft-card {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--ring), var(--shadow-md);
}
.remark-input {
  font-weight: 650;
}

.card-head {
  display: flex;
  align-items: center;
  gap: 8px;
}
.card-head input {
  flex: 1;
  min-width: 0;
}
.card-version {
  color: var(--accent-soft-text);
  background: var(--accent-soft);
  font-size: 0.75em;
  font-weight: 600;
  border-radius: 5px;
  padding: 1px 7px;
  flex-shrink: 0;
}
.card-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-top: 10px;
  border-top: 1px dashed var(--border);
}
.card-meta .hint {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.82em;
}
.copy-btn {
  background: transparent;
  color: var(--muted);
  border-color: var(--border);
  box-shadow: none;
  padding: 2px 8px;
  font-size: 0.82em;
  display: flex;
  align-items: center;
  gap: 4px;
}
.copy-btn:hover {
  color: var(--accent-soft-text);
  border-color: var(--accent);
  background: var(--accent-soft);
}
.card-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
.card-actions button {
  padding: 4px 14px;
  font-size: 0.9em;
}
.load-more {
  display: flex;
  justify-content: center;
  margin-top: 16px;
}

.empty-state {
  border: 1px dashed var(--border-strong);
  border-radius: 14px;
  padding: 44px 24px;
  text-align: center;
}
.empty-state .hint {
  margin: 0;
}
</style>
