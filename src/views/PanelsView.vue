<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
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
const ok = ref("");

// 弹窗状态：editingId 为 null 表示新建
const showModal = ref(false);
const editingId = ref<string | null>(null);
const editingVersion = ref(0);
const formRemark = ref("");
const formItems = ref<PanelItem[]>([]);
const formError = ref("");

const typeSuggestions: { value: string; label: string }[] = [
  { value: "link", label: "跳转链接" },
  { value: "command", label: "命令" },
];

let uid = 0;
function emptyItem(): PanelItem {
  uid += 1;
  return { name: "", desc: "", type: "link", link: "", only_admin: false };
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
  refresh();
}

function openCreate() {
  editingId.value = null;
  editingVersion.value = 0;
  formRemark.value = "";
  formItems.value = [emptyItem()];
  formError.value = "";
  showModal.value = true;
}

function openEdit(rec: PanelRecord) {
  editingId.value = rec.panel_id;
  editingVersion.value = rec.panel.version;
  formRemark.value = rec.panel.remark;
  formItems.value = rec.panel.items.map((i) => ({ ...i }));
  formError.value = "";
  showModal.value = true;
}

function closeModal() {
  showModal.value = false;
}

function addItem() {
  formItems.value.push(emptyItem());
}

function removeItem(index: number) {
  formItems.value.splice(index, 1);
}

function moveItem(index: number, dir: -1 | 1) {
  const target = index + dir;
  if (target < 0 || target >= formItems.value.length) return;
  const items = formItems.value;
  [items[index], items[target]] = [items[target], items[index]];
}

function validate(): string {
  if (!formItems.value.length) return "面板至少需要一个条目";
  const names = new Set<string>();
  for (let i = 0; i < formItems.value.length; i++) {
    const item = formItems.value[i];
    const name = item.name.trim();
    if (!name) return `第 ${i + 1} 个条目缺少名称`;
    if (names.has(name)) return `条目名称「${name}」重复`;
    names.add(name);
    if (item.type === "link" && !item.link.trim()) {
      return `条目「${name}」缺少跳转链接`;
    }
  }
  return "";
}

async function submitForm() {
  formError.value = validate();
  if (formError.value) return;

  const panel: Panel = {
    items: formItems.value.map((i) => ({
      ...i,
      name: i.name.trim(),
      desc: i.desc.trim(),
      link: i.link.trim(),
    })),
    remark: formRemark.value.trim(),
    version: editingId.value ? editingVersion.value : 0,
  };
  try {
    if (editingId.value) {
      await updatePanel(editingId.value, panel);
      ok.value = "面板已更新";
    } else {
      await addPanel({
        panel,
        scope: scope.value,
        target_type: null,
        user_openids: null,
        group_openids: null,
      });
      ok.value = "面板已创建";
    }
    showModal.value = false;
    await refresh();
  } catch (e) {
    formError.value = String(e);
  }
}

async function onDelete(rec: PanelRecord) {
  const label = rec.panel.remark || rec.panel_id;
  if (!window.confirm(`确定删除面板「${label}」吗？此操作不可恢复。`)) return;
  error.value = "";
  try {
    await deletePanel(rec.panel_id);
    ok.value = "面板已删除";
    await refresh();
  } catch (e) {
    error.value = String(e);
  }
}

async function copyPanelId(id: string) {
  try {
    await navigator.clipboard.writeText(id);
    ok.value = "已复制 Panel ID";
  } catch {
    error.value = "复制失败，请手动复制";
  }
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape" && showModal.value) closeModal();
}

onMounted(() => {
  window.addEventListener("keydown", onKeydown);
  refresh();
});
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <section class="view">
    <h2>面板管理</h2>
    <p class="hint">
      管理 QQ 开放平台 v2 消息面板（<code>/v2/panels</code>），
      面板条目将展示在聊天窗口供用户点击。
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
        <span v-if="records.length" class="hint">共 {{ records.length }} 个{{ isEnd ? "" : "+" }}</span>
        <button class="ghost" :disabled="loading" @click="refresh">
          {{ loading ? "刷新中…" : "刷新" }}
        </button>
        <button :disabled="showModal" @click="openCreate">新建面板</button>
      </div>
    </div>

    <p v-if="error" class="msg err">{{ error }}</p>
    <p v-else-if="ok" class="msg ok">{{ ok }}</p>

    <div v-if="records.length" class="panel-grid">
      <div v-for="rec in records" :key="rec.panel_id" class="panel-card">
        <div class="card-head">
          <span class="card-title">{{ rec.panel.remark || "未命名面板" }}</span>
          <span class="card-version">v{{ rec.version }}</span>
        </div>

        <div class="card-items">
          <div v-for="(item, idx) in rec.panel.items" :key="idx" class="item-preview">
            <code>{{ item.name }}</code>
            <span v-if="item.desc" class="item-desc-text">{{ item.desc }}</span>
            <span v-if="item.only_admin" class="admin-badge">仅管理员</span>
          </div>
          <span v-if="!rec.panel.items.length" class="hint">无条目</span>
        </div>

        <div class="card-meta">
          <span class="hint">{{ rec.updated_at || "无更新时间" }}</span>
          <button class="copy-btn" title="复制 Panel ID" @click="copyPanelId(rec.panel_id)">
            <code>{{ rec.panel_id }}</code>
            复制
          </button>
        </div>

        <div class="card-actions">
          <button class="ghost" @click="openEdit(rec)">编辑</button>
          <button class="danger" @click="onDelete(rec)">删除</button>
        </div>
      </div>
    </div>

    <div v-if="!isEnd && nextCursor && records.length" class="load-more">
      <button class="ghost" :disabled="loadingMore" @click="fetchPanels(false)">
        {{ loadingMore ? "加载中…" : "加载更多" }}
      </button>
    </div>

    <p v-if="!loading && !records.length && !error" class="hint">
      当前范围下暂无面板，点击「新建面板」创建一个吧。
    </p>

    <Teleport to="body">
      <div v-if="showModal" class="modal-mask" @click.self="closeModal">
        <div class="modal">
          <h3>{{ editingId ? "编辑面板" : "新建面板" }}</h3>

          <label class="field">
            <span>备注</span>
            <input v-model="formRemark" placeholder="面板备注（可选）" />
          </label>

          <div class="items-head">
            <span>条目（{{ formItems.length }}）</span>
            <button class="ghost small" @click="addItem">＋ 添加条目</button>
          </div>

          <div v-if="!formItems.length" class="hint">
            还没有条目，点击「添加条目」开始吧。
          </div>

          <div class="items-editor">
            <div v-for="(item, idx) in formItems" :key="idx" class="item-card">
              <div class="item-card-head">
                <span class="item-index">{{ idx + 1 }}</span>
                <input
                  v-model="item.name"
                  class="item-name"
                  placeholder="名称（必填）"
                />
                <div class="item-ops">
                  <button
                    class="ghost icon"
                    title="上移"
                    :disabled="idx === 0"
                    @click="moveItem(idx, -1)"
                  >
                    ↑
                  </button>
                  <button
                    class="ghost icon"
                    title="下移"
                    :disabled="idx === formItems.length - 1"
                    @click="moveItem(idx, 1)"
                  >
                    ↓
                  </button>
                  <button class="danger icon" title="删除条目" @click="removeItem(idx)">✕</button>
                </div>
              </div>
              <div class="item-card-body">
                <input v-model="item.desc" class="item-desc" placeholder="说明（可选）" />
                <select v-model="item.type" class="item-type" title="类型">
                  <option v-for="t in typeSuggestions" :key="t.value" :value="t.value">
                    {{ t.label }}
                  </option>
                </select>
                <input
                  v-if="item.type === 'link'"
                  v-model="item.link"
                  class="item-link"
                  placeholder="https:// 跳转链接"
                />
                <label class="item-admin">
                  <input v-model="item.only_admin" type="checkbox" />
                  仅管理员
                </label>
              </div>
            </div>
          </div>

          <p v-if="formError" class="msg err">{{ formError }}</p>

          <div class="modal-actions">
            <button class="ghost" @click="closeModal">取消</button>
            <button :disabled="!formItems.length" @click="submitForm">保存</button>
          </div>
        </div>
      </div>
    </Teleport>
  </section>
</template>

<style scoped>
.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}
.scope-tabs {
  display: flex;
  gap: 6px;
}
.scope-tab {
  background: transparent;
  color: inherit;
  border-color: rgba(128, 128, 128, 0.5);
}
.scope-tab.active {
  background: rgba(100, 108, 255, 0.15);
  color: #646cff;
  border-color: rgba(100, 108, 255, 0.4);
}
.toolbar-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.panel-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 12px;
}
.panel-card {
  border: 1px solid rgba(128, 128, 128, 0.35);
  border-radius: 10px;
  padding: 14px 16px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.card-head {
  display: flex;
  align-items: center;
  gap: 8px;
}
.card-title {
  font-weight: 600;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.card-version {
  color: rgba(128, 128, 128, 1);
  font-size: 0.85em;
}
.card-items {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-height: 22px;
}
.item-preview {
  display: flex;
  gap: 6px;
  align-items: baseline;
  flex-wrap: wrap;
}
.item-desc-text {
  color: rgba(128, 128, 128, 1);
  font-size: 0.9em;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.admin-badge {
  font-size: 0.75em;
  color: #b8860b;
  border: 1px solid rgba(184, 134, 11, 0.5);
  border-radius: 4px;
  padding: 0 4px;
  white-space: nowrap;
}
.card-meta {
  display: flex;
  align-items: center;
  gap: 8px;
}
.card-meta .hint {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.copy-btn {
  background: transparent;
  color: inherit;
  border-color: rgba(128, 128, 128, 0.5);
  padding: 2px 8px;
  font-size: 0.85em;
  display: flex;
  align-items: center;
  gap: 4px;
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

.modal-mask {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.modal {
  background: #f6f7f9;
  border-radius: 12px;
  padding: 20px 24px;
  width: min(640px, calc(100vw - 48px));
  max-height: calc(100vh - 64px);
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
}
@media (prefers-color-scheme: dark) {
  .modal {
    background: #252628;
  }
}
.modal h3 {
  margin: 0;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.field span {
  font-weight: 600;
}
.items-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: 600;
}
button.small {
  padding: 2px 10px;
  font-size: 0.85em;
}
.items-editor {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.item-card {
  border: 1px solid rgba(128, 128, 128, 0.35);
  border-radius: 8px;
  padding: 8px 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.item-card-head {
  display: flex;
  gap: 8px;
  align-items: center;
}
.item-index {
  color: rgba(128, 128, 128, 1);
  font-size: 0.85em;
  min-width: 16px;
  text-align: center;
}
.item-name {
  flex: 1;
}
.item-ops {
  display: flex;
  gap: 4px;
}
button.icon {
  padding: 2px 8px;
  font-size: 0.9em;
  line-height: 1.4;
}
.item-card-body {
  display: flex;
  gap: 6px;
  align-items: center;
  flex-wrap: wrap;
  padding-left: 24px;
}
.item-desc {
  flex: 1;
  min-width: 120px;
}
select.item-type {
  width: 100px;
  border-radius: 8px;
  border: 1px solid rgba(128, 128, 128, 0.4);
  padding: 0.5em 0.9em;
  font-size: 1em;
  font-family: inherit;
  color: inherit;
  background-color: transparent;
  outline: none;
}
.item-link {
  flex: 1;
  min-width: 160px;
}
.item-admin {
  display: flex;
  align-items: center;
  gap: 4px;
  white-space: nowrap;
  cursor: pointer;
}
.modal-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  margin-top: 4px;
}
</style>
