//! Broker models for broker earnings, account info, API limits, and voucher operations.

use serde::{Deserialize, Serialize};

/// Response for querying broker account info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBrokerAccountInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<BrokerAccountInfoResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

/// Broker account info result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokerAccountInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sub_acct_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee_rebate_rate: Option<BaseFeeRebateRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup_fee_rebate_rate: Option<MarkupFeeRebateRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
}

/// Base fee rebate rates by product line.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseFeeRebateRate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivatives: Option<String>,
}

/// Markup fee rebate rates by product line.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkupFeeRebateRate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivatives: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert: Option<String>,
}

/// Response for querying broker all UID details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBrokerAllUidDetailsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<QueryBrokerAllUidResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

/// Result payload for querying broker all UID details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBrokerAllUidResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ApiLimitInfo>>,
}

/// API limit info entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLimitInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
}

/// Response for querying broker capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBrokerCapResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<QueryBrokerCapResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

/// Result payload for querying broker capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBrokerCapResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<BrokerCapInfo>>,
}

/// Broker capacity info entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokerCapInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eb_cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_cap: Option<String>,
}

/// Request parameters for setting broker API limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBrokerApiLimitParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ApiLimitInfoRequest>>,
}

/// API limit info request entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLimitInfoRequest {
    pub uids: String,
    pub biz_type: String,
    pub rate: i64,
}

/// Response for setting broker API limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBrokerApiLimitResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<SetApiLimitResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

/// Result payload for setting broker API limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetApiLimitResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ApiLimitInfoResult>>,
}

/// API limit info result entry returned after setting limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLimitInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

/// Response for querying broker earnings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBrokerEarningResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<BrokerEarningResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

/// Result payload for broker earnings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokerEarningResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_earning_cat: Option<TotalEarningCat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<RebateDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

/// Total earnings categorized by product line.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalEarningCat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot: Option<Vec<TotalEarning>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivatives: Option<Vec<TotalEarning>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<TotalEarning>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert: Option<Vec<TotalEarning>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Vec<TotalEarning>>,
}

/// Total earning entry by coin.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalEarning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earning: Option<String>,
}

/// Rebate detail entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RebateDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup_earning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee_earning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_id: Option<String>,
}

/// Request parameters for fetching award info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAwardInfoParams {
    pub id: String,
}

/// Response for fetching award info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAwardInfoResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount_unit: String,
    #[serde(default)]
    pub product_line: String,
    #[serde(default)]
    pub sub_product_line: String,
    #[serde(default)]
    pub total_amount: String,
    #[serde(default)]
    pub used_amount: String,
}

/// Request parameters for distributing an award to a sub-account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistributeAwardParams {
    pub account_id: String,
    pub award_id: String,
    pub spec_code: String,
    pub amount: String,
    pub broker_id: String,
}

/// Request parameters for retrieving an award distribution record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDistributionRecordParams {
    pub account_id: String,
    pub award_id: String,
    pub spec_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_used_amount: Option<bool>,
}

/// Response for retrieving an award distribution record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDistributionRecordResponse {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub award_id: String,
    #[serde(default)]
    pub spec_code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub is_claimed: bool,
    #[serde(default)]
    pub start_at: String,
    #[serde(default)]
    pub end_at: String,
    #[serde(default)]
    pub effective_at: String,
    #[serde(default)]
    pub ineffective_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_amount: Option<String>,
}
