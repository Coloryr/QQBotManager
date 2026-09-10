use serde::{Deserialize, Serialize};

use crate::net;

const URL: &str = "https://api.bot.qq.com/app/getAppAccessToken";

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
struct GetToken {
    #[serde(rename = "appId")]
    app_id: String,
    #[serde(rename = "clientSecret")]
    client_secret: String,
}

impl Default for GetToken {
    fn default() -> Self {
        Self {
            app_id: Default::default(),
            client_secret: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct GetTokenResponse {
    pub access_token: String,
    pub expires_in: String,
}

impl Default for GetTokenResponse {
    fn default() -> Self {
        Self {
            access_token: Default::default(),
            expires_in: Default::default(),
        }
    }
}

pub async fn get_token(appid: &str, secret: &str) -> Result<GetTokenResponse, String> {
    let client = net::get_work_client();
    client
        .post_json_get_json::<_, GetTokenResponse>(
            URL,
            &GetToken {
                app_id: appid.to_string(),
                client_secret: secret.to_string(),
            },
        )
        .await
}
