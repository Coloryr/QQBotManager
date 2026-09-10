use crate::{
    bot::{
        create_panel::CreatePanel,
        edit_panel::EditPanelResponse,
        get_info::BotInfo,
        get_panel_list::{Panel, PanelRecordList},
    },
    config::{self, ConfigObj},
};
use std::{
    sync::{LazyLock, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

pub mod create_panel;
pub mod delete_panel;
pub mod edit_panel;
pub mod get_info;
pub mod get_panel_list;
pub mod get_token;

static TOKEN: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(Default::default()));
static TIME_OUT: RwLock<u64> = RwLock::new(0);

fn get_time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("系统时间早于 UNIX_EPOCH")
        .as_secs()
}

/// 用 AppID / AppSecret 换取 access_token
async fn update_access_token(config: &ConfigObj) -> Result<(), String> {
    if config.app_id.trim().is_empty() || config.app_secret.trim().is_empty() {
        return Err("请先在设置页填写 AppID 和 AppSecret".into());
    }

    let token = get_token::get_token(&config.app_id, &config.app_secret).await?;
    if token.access_token.trim().is_empty() || token.expires_in.trim().is_empty() {
        return Err("校验失败，请检查KEY".into());
    }
    
    *TOKEN.write().unwrap() = token.access_token;
    *TIME_OUT.write().unwrap() = get_time_now() + token.expires_in.parse::<u64>().unwrap();
    Ok(())
}

pub async fn get_access_token() -> Result<String, String> {
    let token = TOKEN.read().unwrap().clone();
    let time_out = TIME_OUT.read().unwrap().clone();
    if token.is_empty() || time_out == 0 || get_time_now() > (time_out - 60) {
        let config = config::get_config();
        update_access_token(&config).await?;
    }

    Ok(TOKEN.read().unwrap().clone())
}

pub enum Scope {
    C2C,
    Group,
    Channel,
    Dm,
}

impl Scope {
    pub fn to_string(&self) -> String {
        match self {
            Scope::C2C => String::from("c2c"),
            Scope::Group => String::from("group"),
            Scope::Channel => String::from("channel"),
            Scope::Dm => String::from("dm"),
        }
    }

    pub fn from_string(str: &str) -> Option<Scope> {
        if str == "c2c" {
            return Some(Scope::C2C);
        } else if str == "group" {
            return Some(Scope::Group);
        } else if str == "channel" {
            return Some(Scope::Channel);
        } else if str == "dm" {
            return Some(Scope::Dm);
        }

        None
    }
}

/// 获取机器人基础信息
#[tauri::command]
pub async fn fetch_bot_info() -> Result<BotInfo, String> {
    get_info::get_info().await
}

/// 获取面板列表
#[tauri::command]
pub async fn list_panels(scope: String, cursor: Option<String>) -> Result<PanelRecordList, String> {
    let scope = Scope::from_string(&scope).ok_or_else(|| format!("未知的 scope: {scope}"))?;
    get_panel_list::get_panel_list(scope, cursor).await
}

/// 创建面板
#[tauri::command]
pub async fn add_panel(panel: CreatePanel) -> Result<create_panel::CreatePanelResponse, String> {
    create_panel::create_panel(panel).await
}

/// 编辑面板
#[tauri::command]
pub async fn update_panel(panel_id: String, panel: Panel) -> Result<EditPanelResponse, String> {
    edit_panel::edit_panel(&panel_id, panel).await
}

/// 删除面板
#[tauri::command]
pub async fn delete_panel(panel_id: String) -> Result<delete_panel::DeletePanelResponse, String> {
    delete_panel::delete_panel(&panel_id).await
}
