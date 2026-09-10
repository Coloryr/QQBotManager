use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    bot::{self, get_panel_list::Panel},
    net,
};

const URL: &str = "https://api.bot.qq.com/v2/panels";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CreatePanel {
    pub panel: Panel,
    pub target_type: Option<String>,
    pub user_openids: Option<Vec<String>>,
    pub group_openids: Option<Vec<String>>,
    pub scope: String,
}

impl Default for CreatePanel {
    fn default() -> Self {
        Self {
            panel: Default::default(),
            target_type: Default::default(),
            user_openids: Default::default(),
            group_openids: Default::default(),
            scope: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CreatePanelResponse {
    pub panel_id: String,
}

impl Default for CreatePanelResponse {
    fn default() -> Self {
        Self {
            panel_id: Default::default(),
        }
    }
}

pub async fn create_panel(panel: CreatePanel) -> Result<CreatePanelResponse, String> {
    let clinet = net::get_work_client();

    let auth = format!("QQBot {}", bot::get_access_token().await?);

    let res = clinet
        .start(Method::POST, URL)
        .header("Authorization", auth)
        .json(&panel)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    net::handle_response(res).await
}
