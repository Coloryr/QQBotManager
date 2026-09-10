use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

use reqwest::{Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

/// 默认 HTTP 超时时间（秒）
const DEFAULT_TIMEOUT: u64 = 10;

/// 将 reqwest 错误映射为项目统一的 ErrorType
fn map_err(error: reqwest::Error) -> String {
    error.to_string()
}

/// HTTP 客户端封装
///
/// 对 `reqwest::Client` 的二次包装，增加了超时配置、默认请求头、代理支持
/// 以及可选的请求速率限制。
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    /// 创建 HTTP 客户端
    ///
    /// # 参数
    ///
    /// - `proxy`: 代理策略
    ///   - `Auto` / `User` — 使用系统代理或（后续）配置代理
    ///   - `None` — 显式禁用代理
    pub fn new() -> Self {
        let builder = reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .connect_timeout(Duration::from_secs(DEFAULT_TIMEOUT));

        Client {
            inner: builder.build().unwrap(),
        }
    }

    /// 发送自定义 HTTP 请求
    pub fn start(&self, method: Method, url: &str) -> RequestBuilder {
        self.inner.request(method, url)
    }

    /// 发送 GET 请求，失败时自动重试一次
    ///
    /// 连接池中的空闲连接可能已被服务端/代理关闭，复用时会立即报
    /// "error sending request"，此时换新连接重试一次即可。
    async fn get_with_retry(&self, url: &str) -> Result<Response, String> {
        match self.inner.get(url).send().await {
            Ok(resp) => Ok(resp),
            Err(_) => self.inner.get(url).send().await.map_err(map_err),
        }
    }

    /// 发送 GET 请求，返回原始响应
    pub async fn get(&self, url: &str) -> Result<Response, String> {
        self.get_with_retry(url).await
    }

    /// 发送 GET 请求，返回响应体文本
    pub async fn get_text(&self, url: &str) -> Result<String, String> {
        self.get_with_retry(url)
            .await?
            .text()
            .await
            .map_err(map_err)
    }

    /// 发送 GET 请求，返回响应体字节
    pub async fn get_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        self.get_with_retry(url)
            .await?
            .bytes()
            .await
            .map_err(map_err)
            .map(|data| data.to_vec())
    }

    /// 发送 GET 请求，返回反序列化的 JSON
    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, String> {
        let resp = self.get_with_retry(url).await?;
        handle_response(resp).await
    }

    /// 发送带有 Range 头的 GET 请求（断点续传）
    ///
    /// # 参数
    ///
    /// - `url`: 请求地址
    /// - `pos`: 已下载的字节数，从该位置继续下载
    pub async fn get_ranges(&self, url: &str, pos: u64) -> Result<Response, String> {
        self.inner
            .get(url)
            .header("Range", format!("bytes={}-", pos))
            .send()
            .await
            .map_err(map_err)
    }

    /// 发送 POST 请求，JSON 请求体，返回原始响应
    pub async fn post_json_get_req<B: Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<reqwest::Response, String> {
        self.inner
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(map_err)
    }

    /// 发送 POST 请求，JSON 请求体，返回响应文本
    pub async fn post_json_get_text<B: Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<String, String> {
        self.inner
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(map_err)?
            .text()
            .await
            .map_err(map_err)
    }

    /// 发送 POST 请求，JSON 请求体，返回响应字节
    pub async fn post_json_get_bytes<B: Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<Vec<u8>, String> {
        self.inner
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(map_err)?
            .bytes()
            .await
            .map_err(map_err)
            .map(|data| data.to_vec())
    }

    /// 发送 POST 请求，JSON 请求体，返回反序列化的 JSON
    pub async fn post_json_get_json<B: Serialize, T: DeserializeOwned>(
        &self,
        url: &str,
        json: &B,
    ) -> Result<T, String> {
        let resp = self
            .inner
            .post(url)
            .json(json)
            .send()
            .await
            .map_err(map_err)?;
        handle_response(resp).await
    }

    /// 发送 POST 请求，表单请求体，返回反序列化的 JSON
    pub async fn post_form_get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> Result<T, String> {
        let resp = self
            .inner
            .post(url)
            .form(params)
            .send()
            .await
            .map_err(map_err)?;
        handle_response(resp).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// 处理 HTTP 响应：检查状态码并解析 JSON
///
/// 如果状态码表示失败（非 2xx），返回 `HttpReadError`。
/// 成功时反序列化 JSON 为指定类型。
pub async fn handle_response<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, String> {
    let status = resp.status();
    if !status.is_success() {
        let url = resp.url().to_string();
        let error = resp.text().await.unwrap_or_default();
        return Err(format!("err:{error} url:{url}"));
    }
    let bytes = resp.bytes().await.map_err(map_err)?;
    serde_json::from_slice::<T>(&bytes).map_err(|err| err.to_string())
}

/// 全局通用 HTTP 客户端（下载资源、一般 API 调用）
static WORK_CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| Arc::new(Client::new()));

/// 获取全局通用 HTTP 客户端（用于资源下载和一般 API 请求）
pub fn get_work_client() -> Arc<Client> {
    WORK_CLIENT.clone()
}

/// 公网 IP 回显服务（返回纯文本本机公网 IP）
const PUBLIC_IP_SERVICES: [&str; 2] = ["https://api.ipify.org", "https://ipv4.icanhazip.com"];

/// 公网 IP 专用客户端（与全局客户端一致，均直连）
static IP_CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| Arc::new(Client::new()));

/// 获取本机公网 IP（强制直连，依次尝试多个回显服务）
#[tauri::command]
pub async fn get_public_ip() -> Result<String, String> {
    let client = &*IP_CLIENT;
    let mut last_err = String::new();
    for url in PUBLIC_IP_SERVICES {
        match client.get_text(url).await {
            Ok(text) => {
                let ip = text.trim().to_string();
                // 简单校验：非空且长度合理（IPv4 最长 15，IPv6 最长 45）
                if !ip.is_empty() && ip.len() <= 45 {
                    return Ok(ip);
                }
                last_err = format!("服务 {url} 返回了异常内容");
            }
            Err(e) => last_err = e,
        }
    }
    Err(format!("无法获取公网 IP：{last_err}"))
}
