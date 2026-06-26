use serde::{Deserialize, Serialize};

use crate::models::earn::PriceQuote;

pub type CreatePayResponse = CreatePayResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePayResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<PayOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

pub type PayResultResponse = PayResultResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayResultResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<PayOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

pub type MockStatusResponse = MockStatusResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockStatusResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<PayOrder>,
}

pub type FxConvertResponse = FxConvertResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FxConvertResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_amount: Option<String>,
}

pub type RefundResponse = RefundResponseResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundResponseResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_orders: Option<Vec<RefundOrder>>,
}

pub type PayoutResponse = PayoutResponseResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResponseResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<PayOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_orders: Option<Vec<RefundOrder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_refund_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kyc_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub terminal_type: String,
    pub device: String,
    pub browser_version: String,
    pub ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Good {
    pub shopping_name: String,
    pub mcc_code: String,
    pub goods_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskInfo {
    pub terminal_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundOrderItem {
    pub refund_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
    pub merchant_refund_no: String,
    pub refund_amount: String,
    pub env: Env,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_info: Option<RiskInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payee {
    pub uid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementBaseResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementSignResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementSignData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementSignData {
    #[serde(rename = "sign_order_id", skip_serializing_if = "Option::is_none")]
    pub sign_order_id: Option<String>,
    #[serde(rename = "sign_url", skip_serializing_if = "Option::is_none")]
    pub sign_url: Option<String>,
    #[serde(rename = "qr_code", skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<String>,
    #[serde(rename = "qr_code_url", skip_serializing_if = "Option::is_none")]
    pub qr_code_url: Option<String>,
    #[serde(rename = "expire_time", skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementUnsignResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementUnsignData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementUnsignData {
    #[serde(rename = "agreement_no", skip_serializing_if = "Option::is_none")]
    pub agreement_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "unsign_time", skip_serializing_if = "Option::is_none")]
    pub unsign_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementPayResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementPayData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementPayData {
    #[serde(rename = "order_no", skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
    #[serde(rename = "trade_no", skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    #[serde(rename = "out_trade_no", skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "crypto_payment", skip_serializing_if = "Option::is_none")]
    pub crypto_payment: Option<String>,
    #[serde(rename = "pay_time", skip_serializing_if = "Option::is_none")]
    pub pay_time: Option<i64>,
    #[serde(rename = "failure_reason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementPayWithSignResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementPayWithSignData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementPayWithSignData {
    #[serde(rename = "sign_result", skip_serializing_if = "Option::is_none")]
    pub sign_result: Option<AgreementSignData>,
    #[serde(rename = "pay_result", skip_serializing_if = "Option::is_none")]
    pub pay_result: Option<AgreementPayData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementQueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementQueryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementQueryData {
    #[serde(rename = "agreement_no", skip_serializing_if = "Option::is_none")]
    pub agreement_no: Option<String>,
    #[serde(
        rename = "external_agreement_no",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_agreement_no: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "merchant_user_id", skip_serializing_if = "Option::is_none")]
    pub merchant_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sign_time", skip_serializing_if = "Option::is_none")]
    pub sign_time: Option<i64>,
    #[serde(rename = "valid_time", skip_serializing_if = "Option::is_none")]
    pub valid_time: Option<i64>,
    #[serde(rename = "single_limit", skip_serializing_if = "Option::is_none")]
    pub single_limit: Option<String>,
    #[serde(rename = "period_limits", skip_serializing_if = "Option::is_none")]
    pub period_limits: Option<String>,
    #[serde(rename = "used_quota", skip_serializing_if = "Option::is_none")]
    pub used_quota: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementListData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementListData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "page_no", skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AgreementQueryData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementPayQueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementPayQueryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementPayQueryData {
    #[serde(rename = "trade_no", skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    #[serde(rename = "out_trade_no", skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "crypto_payment", skip_serializing_if = "Option::is_none")]
    pub crypto_payment: Option<String>,
    #[serde(rename = "pay_time", skip_serializing_if = "Option::is_none")]
    pub pay_time: Option<i64>,
    #[serde(rename = "refund_amount", skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<String>,
    #[serde(rename = "refund_no", skip_serializing_if = "Option::is_none")]
    pub refund_no: Option<String>,
    #[serde(rename = "out_refund_no", skip_serializing_if = "Option::is_none")]
    pub out_refund_no: Option<String>,
    #[serde(rename = "refund_time", skip_serializing_if = "Option::is_none")]
    pub refund_time: Option<i64>,
    #[serde(rename = "failure_reason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementPayListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementPayListData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementPayListData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "page_no", skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AgreementPayQueryData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementRefundResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgreementRefundData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementRefundData {
    #[serde(rename = "refund_no", skip_serializing_if = "Option::is_none")]
    pub refund_no: Option<String>,
    #[serde(rename = "out_refund_no", skip_serializing_if = "Option::is_none")]
    pub out_refund_no: Option<String>,
    #[serde(rename = "trade_no", skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "refund_amount", skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<String>,
    #[serde(rename = "refund_time", skip_serializing_if = "Option::is_none")]
    pub refund_time: Option<i64>,
    #[serde(rename = "failure_reason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyQuoteParams {
    pub from_coin: String,
    pub from_coin_type: String,
    pub to_coin: String,
    pub to_coin_type: String,
    pub request_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_coin_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmQuoteParams {
    pub quote_tx_id: String,
    pub sub_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdsParams {
    pub token_id: String,
    pub currency_id: String,
    pub side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAdParams {
    pub token_id: String,
    pub currency_id: String,
    pub side: String,
    pub price_type: String,
    pub premium: String,
    pub price: String,
    pub min_amount: String,
    pub max_amount: String,
    pub remark: String,
    pub trading_preference_set: serde_json::Value,
    pub payment_ids: Vec<String>,
    pub quantity: String,
    pub payment_period: String,
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAdParams {
    pub item_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdParams {
    pub id: String,
    pub price_type: String,
    pub premium: String,
    pub price: String,
    pub min_amount: String,
    pub max_amount: String,
    pub remark: String,
    pub trading_preference_set: serde_json::Value,
    pub payment_ids: Vec<String>,
    pub action_type: String,
    pub quantity: String,
    pub payment_period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMyAdsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMyAdDetailsParams {
    pub item_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllOrdersParams {
    pub page: i32,
    pub size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDetailParams {
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPendingOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    pub page: i32,
    pub size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkOrderAsPaidParams {
    pub order_id: String,
    pub payment_type: String,
    pub payment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAssetsParams {
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendChatMessageParams {
    pub message: String,
    pub content_type: String,
    pub order_id: String,
    pub msg_uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadChatFileParams {
    #[serde(rename = "upload_file")]
    pub upload_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatMessagesParams {
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<String>,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCounterpartyUserInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementMonetaryAmount {
    pub total: String,
    pub currency: String,
    #[serde(rename = "currency_type")]
    pub currency_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementLimitConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "currency_type", skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementPeriodLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "currency_type", skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(rename = "period_type")]
    pub period_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementCryptoPaymentInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(rename = "exchange_rate", skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    #[serde(rename = "rate_time", skip_serializing_if = "Option::is_none")]
    pub rate_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementOrderInfo {
    #[serde(rename = "order_title")]
    pub order_title: String,
    #[serde(rename = "order_desc", skip_serializing_if = "Option::is_none")]
    pub order_desc: Option<String>,
    #[serde(rename = "goods_name", skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    #[serde(rename = "goods_id", skip_serializing_if = "Option::is_none")]
    pub goods_id: Option<String>,
    #[serde(rename = "goods_category", skip_serializing_if = "Option::is_none")]
    pub goods_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementSceneInfo {
    #[serde(rename = "device_id", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "device_ip", skip_serializing_if = "Option::is_none")]
    pub device_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementRiskInfo {
    #[serde(rename = "user_ip", skip_serializing_if = "Option::is_none")]
    pub user_ip: Option<String>,
    #[serde(rename = "device_fingerprint", skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    #[serde(rename = "user_agent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementSignParams {
    #[serde(rename = "merchant_user_id", skip_serializing_if = "Option::is_none")]
    pub merchant_user_id: Option<String>,
    #[serde(rename = "scene_code", skip_serializing_if = "Option::is_none")]
    pub scene_code: Option<String>,
    #[serde(rename = "product_code", skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(
        rename = "external_agreement_no",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_agreement_no: Option<String>,
    #[serde(rename = "sign_valid_time", skip_serializing_if = "Option::is_none")]
    pub sign_valid_time: Option<String>,
    #[serde(rename = "single_limit", skip_serializing_if = "Option::is_none")]
    pub single_limit: Option<AgreementLimitConfig>,
    #[serde(rename = "period_limits", skip_serializing_if = "Option::is_none")]
    pub period_limits: Option<Vec<AgreementPeriodLimit>>,
    #[serde(rename = "sign_notify_url", skip_serializing_if = "Option::is_none")]
    pub sign_notify_url: Option<String>,
    #[serde(rename = "return_url", skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(
        rename = "sign_expire_minutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub sign_expire_minutes: Option<i32>,
    #[serde(rename = "extra_params", skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementPayParams {
    #[serde(rename = "agreement_no", skip_serializing_if = "Option::is_none")]
    pub agreement_no: Option<String>,
    #[serde(rename = "out_trade_no")]
    pub out_trade_no: String,
    #[serde(rename = "scene_code")]
    pub scene_code: String,
    pub amount: AgreementMonetaryAmount,
    #[serde(rename = "order_info")]
    pub order_info: AgreementOrderInfo,
    #[serde(rename = "scene_info", skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<AgreementSceneInfo>,
    #[serde(rename = "pay_notify_url")]
    pub pay_notify_url: String,
    #[serde(rename = "risk_info", skip_serializing_if = "Option::is_none")]
    pub risk_info: Option<AgreementRiskInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatCoin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_night: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_from_min_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_from_max_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoCoin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_night: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_from_min_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_from_max_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiats: Option<Vec<FiatCoin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptos: Option<Vec<CryptoCoin>>,
}

pub type QueryCoinListResponse = CoinListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferencePriceResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buys: Option<Vec<PriceQuote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sells: Option<Vec<PriceQuote>>,
}

pub type GetReferencePriceResponse = ReferencePriceResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteApplyRequest {
    pub from_coin: String,
    pub from_coin_type: String,
    pub to_coin: String,
    pub to_coin_type: String,
    pub request_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_coin_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteApplyResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
}

pub type ApplyQuoteResponse = QuoteApplyResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeExecuteRequest {
    pub quote_tx_id: String,
    pub sub_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeExecuteResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_request_id: Option<String>,
}

pub type ConfirmQuoteResponse = TradeExecuteResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_user_id: Option<String>,
}

pub type QueryTradeResponse = TradeInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TradeInfo>>,
}

pub type QueryTradeHistoryResponse = TradeHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyInfo>,
}

pub type QueryBalanceResponse = serde_json::Value;

pub type MarkOrderAsPaidResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPreferenceSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_un_post_ad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_kyc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_register_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time_threshold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_finish_number_day30: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_rate_day30: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_order_finish_number_day30: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_complete_rate_day30: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_national_limit: Option<String>,
}

pub type GetCoinBalanceResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdsRequest {
    pub token_id: String,
    pub currency_id: String,
    pub side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

pub type GetAdsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAdRequest {
    pub token_id: String,
    pub currency_id: String,
    pub side: String,
    pub price_type: String,
    pub premium: String,
    pub price: String,
    pub min_amount: String,
    pub max_amount: String,
    pub remark: String,
    pub trading_preference_set: TradingPreferenceSet,
    pub payment_ids: Vec<String>,
    pub quantity: String,
    pub payment_period: String,
    pub item_type: String,
}

pub type PostAdResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAdRequest {
    pub item_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdRequest {
    pub id: String,
    pub price_type: String,
    pub premium: String,
    pub price: String,
    pub min_amount: String,
    pub max_amount: String,
    pub remark: String,
    pub trading_preference_set: TradingPreferenceSet,
    pub payment_ids: Vec<String>,
    pub action_type: String,
    pub quantity: String,
    pub payment_period: String,
}

pub type UpdateAdResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMyAdsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_preference_set: Option<TradingPreferenceSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<Vec<serde_json::Value>>,
}

pub type GetMyAdsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdDetailRequest {
    pub item_id: String,
}

pub type GetMyAdDetailsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_last_seconds: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllOrdersRequest {
    pub page: i32,
    pub size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
}

pub type GetPendingOrdersResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPendingOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    pub page: i32,
    pub size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDetailRequest {
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTermItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qrcode: Option<String>,
}

pub type GetOrderDetailResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkOrderPaidRequest {
    pub order_id: String,
    pub payment_type: String,
    pub payment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAssetsRequest {
    pub order_id: String,
}

pub type UploadChatFileResponse = UploadFileResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatMessageParams {
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<String>,
    pub size: String,
}

pub type GetChatMessagesResponse = GetChatMessageResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatMessageResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<ChatMessageItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_nick_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kyc_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_logout_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_finish_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_finish_sell_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_finish_buy_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_finish_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_release_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_transfer_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_create_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_trade_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_trade_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_trade_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kyc_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_appraise_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_appraise_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_appraise_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCounterpartyInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

pub type GetCounterpartyUserInfoResponse = UserInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qrcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clabe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_card_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concept: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ext1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ext2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ext3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ext4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ext5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ext6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_template_version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_payment_template_changed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_config_vo: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_balance: Option<Vec<String>>,
}

pub type GetUserPaymentResponse = Vec<PaymentMethod>;

pub type QueryFundingDetailApiResponse = FundingDetailApiResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingDetailApiResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FundingDetailApiBO>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingDetailApiBO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_amt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_amt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_busi_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_busi_type_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_en: Option<String>,
}
