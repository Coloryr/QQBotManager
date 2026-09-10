use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    bot::{self, get_panel_list::Panel},
    net,
};

const URL: &str = "https://api.bot.qq.com/v2/panels";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct EditPanelObj {
    panel: Panel,
}

impl Default for EditPanelObj {
    fn default() -> Self {
        Self {
            panel: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EditPanelResponse {
    pub version: u32,
}

impl Default for EditPanelResponse {
    fn default() -> Self {
        Self {
            version: Default::default(),
        }
    }
}

pub async fn edit_panel(panel_id: &str, panel: Panel) -> Result<EditPanelResponse, String> {
    let clinet = net::get_work_client();

    let auth = format!("QQBot {}", bot::get_access_token().await?);

    let res = clinet
        .start(Method::PUT, &format!("{}/{}", URL, panel_id))
        .header("Authorization", auth)
        .json(&EditPanelObj { panel: panel })
        .send()
        .await
        .map_err(|err| err.to_string())?;

    net::handle_response(res).await
}
