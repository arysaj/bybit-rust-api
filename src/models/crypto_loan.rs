//! Crypto loan models.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustLtvParams {
    pub currency: String,
    pub amount: String,
    pub direction: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustLtvResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdjustmentHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_time: Option<i64>,
    #[serde(rename = "preLTV", skip_serializing_if = "Option::is_none")]
    pub pre_ltv: Option<String>,
    #[serde(rename = "afterLTV", skip_serializing_if = "Option::is_none")]
    pub after_ltv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralDataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyLiquidationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralRatioConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_ratio_list: Option<Vec<CollateralRatioTier>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralRatioTier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_ratio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoanableDataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanableDataItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_borrowable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_borrowing_accuracy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_flexible_borrowing_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_annualized_interest_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_borrowable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_borrowing_accuracy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_fixed_borrowing_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_borrowing_amount: Option<String>,
    #[serde(
        rename = "annualizedInterestRate7D",
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_interest_rate7_d: Option<String>,
    #[serde(
        rename = "annualizedInterestRate14D",
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_interest_rate14_d: Option<String>,
    #[serde(
        rename = "annualizedInterestRate30D",
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_interest_rate30_d: Option<String>,
    #[serde(
        rename = "annualizedInterestRate60D",
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_interest_rate60_d: Option<String>,
    #[serde(
        rename = "annualizedInterestRate90D",
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_interest_rate90_d: Option<String>,
    #[serde(
        rename = "annualizedInterestRate180D",
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_interest_rate180_d: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxCollateralAmountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateMaxLoanParams {
    pub currency: String,
    pub collateral_list: Vec<CollateralInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInput {
    pub ccy: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateMaxLoanResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoanPositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_total_debt: Option<String>,
    #[serde(
        rename = "flexibleTotalDebtUSD",
        skip_serializing_if = "Option::is_none"
    )]
    pub flexible_total_debt_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_hourly_interest_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_total_debt: Option<String>,
    #[serde(rename = "fixedTotalDebtUSD", skip_serializing_if = "Option::is_none")]
    pub fixed_total_debt_usd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "amountUSD", skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "amountUSD", skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col_res: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelFixedBorrowOrderParams {
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelFixedBorrowOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedBorrowOrderQuoteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFixedBorrowParams {
    pub order_currency: String,
    pub order_amount: String,
    pub annual_rate: String,
    pub term: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<String>,
    pub collateral_list: Vec<CollateralInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedBorrowResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullyRepayFixedLoanParams {
    pub loan_id: String,
    pub loan_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFixedFullyRepayResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenewFixedLoanParams {
    pub loan_id: String,
    pub collateral_list: Vec<CollateralInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowQuote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedBorrowRequest {
    pub order_currency: String,
    pub order_amount: String,
    pub annual_rate: String,
    pub term: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<String>,
    pub collateral_list: Vec<crate::models::account::Collateral>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFixedBorrowResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullyRepayFixedLoanResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CryptoLoanFixedFullyRepayResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFixedFullyRepayResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFixedRepayCollateralResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CryptoLoanFixedRepayCollateralResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFixedRepayCollateralResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFixedRepayCollateralParams {
    pub loan_id: i64,
    pub loan_currency: String,
    pub collateral_coin: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFixedSupplyOrderCancelResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFixedSupplyOrderCancelParams {
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_account: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCryptoLoanFixedSupplyOrderQuoteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetCryptoLoanFixedSupplyOrderQuoteResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCryptoLoanFixedSupplyOrderQuoteResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<GetCryptoLoanFixedSupplyOrderQuoteListItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCryptoLoanFixedSupplyOrderQuoteListItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFixedSupplyParams {
    pub order_currency: String,
    pub order_amount: String,
    pub annual_rate: String,
    pub term: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_source: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCryptoLoanFlexibleBorrowHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CryptoLoanFlexibleBorrowHistoryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFlexibleBorrowHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CryptoLoanFlexibleBorrowHistoryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFlexibleBorrowHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleBorrowRequest {
    pub loan_currency: String,
    pub loan_amount: String,
    pub collateral_list: Vec<CollateralItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFlexibleBorrowResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFlexibleBorrowParams {
    pub loan_currency: String,
    pub loan_amount: String,
    pub collateral_list: Vec<CollateralItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFlexibleRepayCollateralResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CryptoLoanFlexibleRepayCollateralResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFlexibleRepayCollateralResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFlexibleRepayCollateralParams {
    pub loan_currency: String,
    pub collateral_coin: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCryptoLoanFlexibleRepayParams {
    pub loan_currency: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCryptoLoanFlexibleRepaymentHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CryptoLoanFlexibleRepaymentHistoryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFlexibleRepaymentHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CryptoLoanFlexibleRepaymentHistoryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanFlexibleRepaymentHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
