use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFMartBotResponse {
    #[serde(rename = "status_code")]
    #[serde(default)]
    pub status_code: Option<i32>,
    #[serde(rename = "debug_msg")]
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(rename = "ban_reason_text")]
    #[serde(default)]
    pub ban_reason_text: Option<String>,
    #[serde(rename = "bot_id")]
    #[serde(default)]
    pub bot_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFMartBotParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "martingale_mode")]
    pub martingale_mode: String,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(rename = "price_float_percent")]
    pub price_float_percent: String,
    #[serde(rename = "add_position_percent")]
    pub add_position_percent: String,
    #[serde(rename = "add_position_num")]
    pub add_position_num: i32,
    #[serde(rename = "init_margin")]
    pub init_margin: String,
    #[serde(rename = "round_tp_percent")]
    pub round_tp_percent: String,
    #[serde(rename = "auto_cycle_toggle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cycle_toggle: Option<String>,
    #[serde(rename = "sl_percent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_percent: Option<String>,
    #[serde(rename = "entry_price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "followed_bot_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followed_bot_id: Option<i64>,
    #[serde(rename = "block_source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_source: Option<String>,
    #[serde(rename = "create_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_type: Option<String>,
    #[serde(rename = "init_bonus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_bonus: Option<String>,
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}
