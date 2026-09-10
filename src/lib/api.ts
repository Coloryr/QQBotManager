import { invoke } from "@tauri-apps/api/core";

export interface AppConfig {
  app_id: string;
  app_secret: string;
}

export interface BotInfo {
  id: string;
  username: string;
  avatar: string;
  bot: boolean;
  union_openid: string;
  union_user_account: string;
}

/** 面板作用范围 */
export type Scope = "c2c" | "group" | "channel" | "dm";

/** 面板内的单个条目 */
export interface PanelItem {
  name: string;
  desc: string;
  /** 条目类型，如 link */
  type: string;
  /** 跳转链接（type 为 link 时使用） */
  link: string;
  /** 仅管理员可用 */
  only_admin: boolean;
}

/** 面板内容 */
export interface Panel {
  items: PanelItem[];
  remark: string;
  version: number;
}

/** 面板记录（服务端返回） */
export interface PanelRecord {
  panel_id: string;
  scope: string;
  target_type: string;
  created_at: string;
  updated_at: string;
  version: number;
  panel: Panel;
}

/** 面板列表（服务端返回） */
export interface PanelRecordList {
  records: PanelRecord[];
  next_cursor: string;
  is_end: boolean;
}

/** 创建面板的请求体 */
export interface CreatePanelObj {
  panel: Panel;
  scope: string;
  target_type: string | null;
  user_openids: string[] | null;
  group_openids: string[] | null;
}

export function loadConfig(): Promise<AppConfig> {
  return invoke("load_config");
}

export function saveConfig(config: AppConfig): Promise<void> {
  return invoke("save_config", { config });
}

export function fetchBotInfo(): Promise<BotInfo> {
  return invoke("fetch_bot_info");
}

export function listPanels(scope: Scope, cursor?: string | null): Promise<PanelRecordList> {
  return invoke("list_panels", { scope, cursor: cursor ?? null });
}

export function addPanel(panel: CreatePanelObj): Promise<string> {
  return invoke("add_panel", { panel });
}

export function updatePanel(panelId: string, panel: Panel): Promise<string> {
  return invoke("update_panel", { panelId, panel });
}

export function deletePanel(panelId: string): Promise<string> {
  return invoke("delete_panel", { panelId });
}
