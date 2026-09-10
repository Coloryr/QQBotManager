use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{bot, net};

const URL: &str = "https://api.bot.qq.com/v2/panels";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DeletePanelResponse {}

impl Default for DeletePanelResponse {
    fn default() -> Self {
        Self {}
    }
}

pub async fn delete_panel(panel_id: &str) -> Result<DeletePanelResponse, String> {
    let clinet = net::get_work_client();

    let auth = format!("QQBot {}", bot::get_access_token().await?);

    let res = clinet
        .start(Method::DELETE, &format!("{}/{}", URL, panel_id))
        .header("Authorization", auth)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    net::handle_response(res).await
}
