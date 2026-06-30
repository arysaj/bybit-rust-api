use serde::{Deserialize, Serialize};

// NOTE: Bybit V5 endpoints universally use camelCase on the wire (e.g.
// `orderId`, `accountType`). The container-level `rename_all = "camelCase"`
// attribute drives wire serialization here. The previous version of this
// file also had per-field `#[serde(rename = "snake_case")]` attributes
// which silently *overrode* the container, sending snake_case keys —
// inconsistent with the rest of the V5 SDK. They have been removed.

/// Response payload for `POST /v5/fmartingalebot/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFMartBotResponse {
    #[serde(default)]
    pub status_code: Option<i32>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub ban_reason_text: Option<String>,
    #[serde(default)]
    pub bot_id: Option<i64>,
}

/// Request body for `POST /v5/fmartingalebot/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFMartBotParams {
    pub symbol: String,
    pub martingale_mode: String,
    pub leverage: String,
    pub price_float_percent: String,
    pub add_position_percent: String,
    pub add_position_num: i32,
    pub init_margin: String,
    pub round_tp_percent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cycle_toggle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_percent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followed_bot_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_bonus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}
