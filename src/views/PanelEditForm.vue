<script setup lang="ts">
import type { PanelItem } from "../lib/api";
import { inputValue, truncateWeighted, weightedLen } from "../lib/text";

defineProps<{ items: PanelItem[] }>();
defineEmits<{ add: []; remove: [index: number]; move: [index: number, dir: -1 | 1] }>();

const typeSuggestions: { value: string; label: string }[] = [
  { value: "link", label: "跳转链接" },
  { value: "command", label: "命令" },
];

// 文档限制：name 最多 14 字符（约 7 个汉字），desc 最多 30 字符（约 15 个汉字）
const NAME_MAX = 14;
const DESC_MAX = 30;
</script>

<template>
  <div class="items-head">
    <span>条目（{{ items.length }}）</span>
  </div>

  <div class="items-editor">
    <div v-for="(item, idx) in items" :key="idx" class="item-card">
      <div class="item-card-head">
        <span class="item-index">{{ idx + 1 }}</span>
        <input
          :value="item.name"
          class="item-name"
          placeholder="元素名称（必填）"
          @input="item.name = truncateWeighted(inputValue($event), NAME_MAX)"
        />
        <span
          class="char-count"
          :class="{ over: weightedLen(item.name) > NAME_MAX }"
        >
          {{ weightedLen(item.name) }}/{{ NAME_MAX }}
        </span>
        <div class="item-ops">
          <button
            class="ghost icon"
            title="上移"
            :disabled="idx === 0"
            @click="$emit('move', idx, -1)"
          >
            ↑
          </button>
          <button
            class="ghost icon"
            title="下移"
            :disabled="idx === items.length - 1"
            @click="$emit('move', idx, 1)"
          >
            ↓
          </button>
          <button class="danger icon" title="删除条目" @click="$emit('remove', idx)">✕</button>
        </div>
      </div>
      <div class="item-card-body">
        <div class="item-desc-wrap">
          <input
            :value="item.desc"
            class="item-desc"
            placeholder="元素描述（必填，补充说明功能）"
            @input="item.desc = truncateWeighted(inputValue($event), DESC_MAX)"
          />
          <span
            class="char-count"
            :class="{ over: weightedLen(item.desc) > DESC_MAX }"
          >
            {{ weightedLen(item.desc) }}/{{ DESC_MAX }}
          </span>
        </div>
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

  <button class="add-item" @click="$emit('add')">＋ 添加条目</button>
</template>

<style scoped>
.items-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: 600;
  font-size: 0.95em;
}
.items-editor {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.add-item {
  width: 100%;
  border-style: dashed;
  background: transparent;
  color: var(--accent-soft-text);
  font-weight: 550;
  padding: 9px 0;
  box-shadow: none;
}
.add-item:hover {
  background: var(--accent-soft);
  border-color: var(--accent);
}
.item-card {
  background: var(--input-bg);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 10px 12px;
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
  color: var(--accent-soft-text);
  font-size: 0.8em;
  min-width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--accent-soft);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  font-weight: 600;
}
.item-name {
  flex: 1;
  min-width: 0;
  background: var(--surface);
}
.char-count {
  color: var(--muted);
  font-size: 0.75em;
  white-space: nowrap;
  flex-shrink: 0;
}
.char-count.over {
  color: var(--danger);
}
.item-desc-wrap {
  flex: 1;
  min-width: 150px;
  display: flex;
  align-items: center;
  gap: 5px;
}
.item-desc-wrap .item-desc {
  flex: 1;
  min-width: 0;
}
.item-ops {
  display: flex;
  gap: 4px;
}
button.icon {
  padding: 2px 8px;
  font-size: 0.9em;
  line-height: 1.4;
  box-shadow: none;
}
.item-card-body {
  display: flex;
  gap: 6px;
  align-items: center;
  flex-wrap: wrap;
  padding-left: 26px;
}
.item-desc {
  flex: 1;
  min-width: 120px;
  background: var(--surface);
}
select.item-type {
  width: 128px;
  cursor: pointer;
  background: var(--surface);
}
.item-link {
  flex: 1;
  min-width: 160px;
  background: var(--surface);
}
.item-admin {
  display: flex;
  align-items: center;
  gap: 5px;
  white-space: nowrap;
  cursor: pointer;
  font-size: 0.9em;
}
.item-admin input {
  accent-color: var(--accent);
  cursor: pointer;
}
</style>
