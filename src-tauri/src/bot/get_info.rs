use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{bot, net};

const URL: &str = "https://api.bot.qq.com/users/@me";

/// 机器人基础信息（GET /users/@me）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BotInfo {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub bot: bool,
    pub union_openid: String,
    pub union_user_account: String,
}

impl Default for BotInfo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            username: Default::default(),
            avatar: Default::default(),
            bot: Default::default(),
            union_openid: Default::default(),
            union_user_account: Default::default(),
        }
    }
}

pub async fn get_info() -> Result<BotInfo, String> {
    let clinet = net::get_work_client();

    let auth = format!("QQBot {}", bot::get_access_token().await?);

    let res = clinet
        .start(Method::GET, URL)
        .header("Authorization", auth)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    net::handle_response(res).await
}
