use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    bot::{self, Scope},
    net,
};

const URL: &str = "https://api.bot.qq.com/v2/panels";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PanelRecordList {
    pub records: Vec<PanelRecord>,
    pub next_cursor: String,
    pub is_end: bool,
}

impl Default for PanelRecordList {
    fn default() -> Self {
        Self {
            records: Default::default(),
            next_cursor: Default::default(),
            is_end: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PanelRecord {
    pub panel_id: String,
    pub scope: String,
    pub target_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub version: i32,
    pub panel: Panel,
}

impl Default for PanelRecord {
    fn default() -> Self {
        Self {
            panel_id: Default::default(),
            scope: Default::default(),
            target_type: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            version: Default::default(),
            panel: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Panel {
    pub items: Vec<PanelItem>,
    pub remark: String,
    pub version: i32,
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            items: Default::default(),
            remark: Default::default(),
            version: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PanelItem {
    pub name: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub panel_type: String,
    pub link: String,
    pub only_admin: bool,
}

impl Default for PanelItem {
    fn default() -> Self {
        Self {
            name: Default::default(),
            desc: Default::default(),
            panel_type: Default::default(),
            link: Default::default(),
            only_admin: Default::default(),
        }
    }
}

pub async fn get_panel_list(scope: Scope, cursor: Option<String>) -> Result<PanelRecordList, String> {
    let clinet = net::get_work_client();

    let auth = format!("QQBot {}", bot::get_access_token().await?);

    let mut url = format!("{}?scope={}&limit=50", URL, scope.to_string());
    if let Some(cursor) = cursor.as_deref().filter(|c| !c.is_empty()) {
        url.push_str(&format!("&cursor={cursor}"));
    }

    let res = clinet
        .start(Method::GET, &url)
        .header("Authorization", auth)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    net::handle_response(res).await
}
