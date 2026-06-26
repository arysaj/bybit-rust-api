//! HTTP client for Bybit REST API.

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, warn};

use crate::auth::{generate_signature, get_timestamp};
use crate::config::ClientConfig;
use crate::constants::*;
use crate::error::{BybitError, Result};

/// JSON keys whose values must be masked before being written to debug logs.
///
/// Match is case-insensitive on the unquoted key text. We deliberately do not
/// try to be exhaustive — anything that looks credential-shaped goes here.
const SENSITIVE_JSON_KEYS: &[&str] = &[
    "secret",
    "password",
    "apiSecret",
    "api_secret",
    "apiKey",
    "api_key",
    "privateKey",
    "private_key",
    "token",
    "sign",
];

/// Mask credential values in a JSON body string before logging.
///
/// This is a defensive last line of defense — typed credential fields should
/// already use [`crate::models::RedactedString`]. The mask is purely textual
/// (substring of a JSON dump), so it works on both serialized request bodies
/// and raw HTTP response text.
fn mask_sensitive(json_body: &str) -> String {
    let mut out = json_body.to_owned();
    for key in SENSITIVE_JSON_KEYS {
        // Match `"<key>":"<anything-not-double-quote>"` (the most common JSON shape).
        // Use a simple linear scan: find `"<key>":"`, then replace up to the next `"`.
        let needle = format!("\"{}\":\"", key);
        let needle_lower = needle.to_lowercase();
        let mut cursor = 0usize;
        loop {
            // Case-insensitive find: scan windows of out from cursor.
            let lower_remaining = out[cursor..].to_lowercase();
            let Some(rel) = lower_remaining.find(&needle_lower) else {
                break;
            };
            let start = cursor + rel + needle.len();
            // Find the closing quote (no escape support — Bybit secrets do not contain `"`).
            let Some(end_rel) = out[start..].find('"') else {
                break;
            };
            let end = start + end_rel;
            out.replace_range(start..end, "***REDACTED***");
            cursor = start + "***REDACTED***".len();
        }
    }
    out
}

#[cfg(test)]
mod mask_tests {
    use super::mask_sensitive;

    #[test]
    fn masks_secret_field() {
        let input = r#"{"apiKey":"k1","secret":"superSecretValue","other":"plain"}"#;
        let out = mask_sensitive(input);
        assert!(out.contains("***REDACTED***"));
        assert!(!out.contains("superSecretValue"));
        assert!(!out.contains("k1"));
        assert!(out.contains("plain"));
    }

    #[test]
    fn masks_password_case_insensitively() {
        let input = r#"{"Password":"p@ss","apisecret":"x"}"#;
        let out = mask_sensitive(input);
        assert!(!out.contains("p@ss"));
        assert!(!out.contains("\"x\""));
    }

    #[test]
    fn leaves_unrelated_keys_intact() {
        let input = r#"{"category":"linear","symbol":"BTCUSDT"}"#;
        assert_eq!(mask_sensitive(input), input);
    }
}

/// API response wrapper from Bybit.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// Return code (0 = success)
    pub ret_code: i32,
    /// Return message
    pub ret_msg: String,
    /// Response data
    pub result: T,
    /// Extended info
    #[serde(default)]
    #[allow(dead_code)]
    pub ret_ext_info: serde_json::Value,
    /// Server time
    #[allow(dead_code)]
    pub time: u64,
}

/// Bybit HTTP API client.
#[derive(Debug, Clone)]
pub struct BybitClient {
    config: ClientConfig,
    http: reqwest::Client,
}

impl BybitClient {
    /// Create a new client with the given configuration.
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(BybitError::Http)?;

        Ok(Self { config, http })
    }

    /// Create a new client with API credentials using default settings.
    pub fn with_credentials(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
    ) -> Result<Self> {
        let config = ClientConfig::builder(api_key, api_secret).build();
        Self::new(config)
    }

    /// Create a new client for testnet.
    pub fn testnet(api_key: impl Into<String>, api_secret: impl Into<String>) -> Result<Self> {
        let config = ClientConfig::builder(api_key, api_secret)
            .base_url(TESTNET)
            .build();
        Self::new(config)
    }

    /// Create a new client for demo environment.
    pub fn demo(api_key: impl Into<String>, api_secret: impl Into<String>) -> Result<Self> {
        let config = ClientConfig::builder(api_key, api_secret)
            .base_url(DEMO)
            .build();
        Self::new(config)
    }

    /// Get the client configuration.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Send a public GET request (no authentication).
    pub async fn get_public<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, endpoint);

        let response = tokio::time::timeout(
            self.config.timeout,
            self.http.get(&url).query(params).send(),
        )
        .await
        .map_err(|_| BybitError::Timeout)?
        .map_err(BybitError::Http)?;

        self.parse_response(response).await
    }

    /// Send an authenticated GET request.
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, endpoint);
        let timestamp = get_timestamp();

        // Build query string for signature
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let signature = generate_signature(
            &self.config.api_secret,
            timestamp,
            &self.config.api_key,
            self.config.recv_window,
            &query_string,
        );

        let headers = self.build_auth_headers(timestamp, &signature);

        let response = tokio::time::timeout(
            self.config.timeout,
            self.http.get(&url).query(params).headers(headers).send(),
        )
        .await
        .map_err(|_| BybitError::Timeout)?
        .map_err(BybitError::Http)?;

        self.parse_response(response).await
    }

    /// Send an authenticated POST request.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, endpoint);
        let timestamp = get_timestamp();

        let body_str = serde_json::to_string(body).map_err(|e| BybitError::Parse(e.to_string()))?;

        let signature = generate_signature(
            &self.config.api_secret,
            timestamp,
            &self.config.api_key,
            self.config.recv_window,
            &body_str,
        );

        let headers = self.build_auth_headers(timestamp, &signature);

        if self.config.debug {
            debug!("POST {} body: {}", url, mask_sensitive(&body_str));
        }

        let response = tokio::time::timeout(
            self.config.timeout,
            self.http
                .post(&url)
                .headers(headers)
                .header(CONTENT_TYPE, "application/json")
                .body(body_str)
                .send(),
        )
        .await
        .map_err(|_| BybitError::Timeout)?
        .map_err(BybitError::Http)?;

        self.parse_response(response).await
    }

    /// Build authentication headers.
    fn build_auth_headers(&self, timestamp: u64, signature: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            HEADER_API_KEY,
            HeaderValue::from_str(&self.config.api_key)
                .unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        headers.insert(
            HEADER_TIMESTAMP,
            HeaderValue::from_str(&timestamp.to_string())
                .unwrap_or_else(|_| HeaderValue::from_static("0")),
        );
        headers.insert(
            HEADER_SIGN,
            HeaderValue::from_str(signature).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        headers.insert(HEADER_SIGN_TYPE, HeaderValue::from_static("2"));
        headers.insert(
            HEADER_RECV_WINDOW,
            HeaderValue::from_str(&self.config.recv_window.to_string())
                .unwrap_or_else(|_| HeaderValue::from_static("5000")),
        );

        headers
    }

    /// Parse API response and handle errors.
    async fn parse_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let text = response.text().await.map_err(BybitError::Http)?;

        if self.config.debug {
            debug!(
                "Response status: {}, body: {}",
                status,
                mask_sensitive(&text)
            );
        }

        if !status.is_success() {
            // Try to parse as API error
            if let Ok(api_resp) = serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                return Err(BybitError::Api {
                    code: api_resp.ret_code,
                    msg: api_resp.ret_msg,
                });
            }
            return Err(BybitError::Parse(format!(
                "HTTP {} - {}",
                status.as_u16(),
                text
            )));
        }

        // Parse successful response
        let api_resp: ApiResponse<T> = serde_json::from_str(&text).map_err(|e| {
            warn!("Failed to parse response: {}, body: {}", e, text);
            BybitError::Parse(format!(
                "JSON parse error: {} - body: {}",
                e,
                &text[..text.len().min(200)]
            ))
        })?;

        // Check for API-level errors
        if api_resp.ret_code != 0 {
            return Err(BybitError::Api {
                code: api_resp.ret_code,
                msg: api_resp.ret_msg,
            });
        }

        Ok(api_resp.result)
    }
}
