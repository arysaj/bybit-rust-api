use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip_product: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount_quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital_quote: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital_base: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DualAssetsProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceEarnProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetAdvanceProductResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceQuote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOffer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_low_price: Option<Vec<PriceQuote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_high_price: Option<Vec<PriceQuote>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductExtraInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ProductOffer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceEarnProductExtraInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetProductExtraInfoResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsExtra {
    pub order_direction: String,
    pub select_price: String,
    pub apy_e8: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageStakeExtra {
    pub initial_price: String,
    pub breakeven_price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageRedeemExtra {
    pub position_id: String,
    pub est_redeem_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_slippage_protected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAdvanceEarnOrderParams {
    pub category: String,
    pub product_id: i64,
    pub order_type: String,
    pub amount: String,
    pub account_type: String,
    pub coin: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_assets_extra: Option<DualAssetsExtra>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_card: Option<InterestCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_leverage_stake_extra: Option<SmartLeverageStakeExtra>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_leverage_redeem_extra: Option<SmartLeverageRedeemExtra>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_win_stake_extra: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_win_redeem_extra: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_buy_extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAdvanceOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAdvanceEarnOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PlaceAdvanceOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy_e8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_return_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_return_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_end_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvancePositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DualAssetsPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceEarnPositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetAdvancePositionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_apy_e8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_apy_e8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_bonus_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_bonus_pnl: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DualAssetsOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceEarnOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetAdvanceOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<SmartLeverageProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetSmartLeverageProductResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageProductExtraInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakeven_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageProductExtraInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<SmartLeverageProductExtraInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeveragePosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakeven_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeveragePositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<SmartLeveragePosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeveragePositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetSmartLeveragePositionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakeven_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<SmartLeverageOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetSmartLeverageOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRedeemEstAmountListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRedeemEstAmountListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetRedeemEstAmountListResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinLeverageResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinLeverageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetDoubleWinLeverageResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTokenOrderParams {
    pub coin: String,
    pub order_link_id: String,
    pub order_type: String,
    pub amount: String,
    pub account_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTokenOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTokenOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PlaceTokenOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenOrderListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenOrderListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TokenOrderListResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_purchase_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TokenProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TokenPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDailyYield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDailyYieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenDailyYield>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDailyYieldResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TokenDailyYieldResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenHourlyYield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenHourlyYieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenHourlyYield>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenHourlyYieldResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TokenHourlyYieldResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAprHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAprHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenAprHistory>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAprHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TokenAprHistoryResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedTermProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FixedTermProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetFixedTermProductResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemEstItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_rate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageRedeemEstAmountListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetRedeemEstAmountListResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rfq_product: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price_buffer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price_buffer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_deviation_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_deviation_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_tick_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetDoubleWinProductResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DoubleWinProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinProductExtraInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinProductExtraInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<DoubleWinProductExtraInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinStakeExtra {
    pub leverage: String,
    pub initial_price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinRedeemExtra {
    pub position_id: String,
    pub est_redeem_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_slippage_protected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinPositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetDoubleWinPositionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinPositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DoubleWinPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetDoubleWinOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DoubleWinOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvDiscountBuyProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip_product: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetDiscountBuyProductResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AdvDiscountBuyProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBuyPriceOfferItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_coupon_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_uid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBuyProductExtraInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<DiscountBuyPriceOfferItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyProductExtraInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<DiscountBuyProductExtraInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBuyExtra {
    pub initial_price: String,
    pub purchase_price: String,
    pub knockout_price: String,
    pub knockout_coupon_e8: i64,
    pub inst_uid: i64,
    pub settle_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvDiscountBuyPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_coupon_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyPositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetDiscountBuyPositionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyPositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AdvDiscountBuyPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvDiscountBuyOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_coupon_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetDiscountBuyOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AdvDiscountBuyOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceFixedTermOrderParams {
    pub product_id: String,
    pub category: String,
    pub coin: String,
    pub amount: String,
    pub account_type: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_invest: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceFixedTermOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PlaceFixedTermOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceFixedTermOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemFixedTermParams {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemFixedTermResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RedeemFixedTermResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemFixedTermResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermPositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetFixedTermPositionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermPositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FixedTermPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedTermPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_invest: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetFixedTermOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FixedTermOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedTermOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFixedTermAutoInvestParams {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaProductListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RwaProductListResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaProductListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRwaOrderParams {
    pub product_id: i64,
    pub order_type: String,
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRwaOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PlaceRwaOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRwaOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaPositionListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RwaPositionListResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaPositionListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaOrderListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RwaOrderListResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaOrderListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaNavChartResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RwaNavChartResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaNavChartResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaNavPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaNavPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetProductResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<EarnProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceEarnOrderParams {
    pub category: String,
    pub order_type: String,
    pub account_type: String,
    pub amount: String,
    pub coin: String,
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_card: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PlaceOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<OrderHistoryResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<EarnOrderHistoryItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnOrderHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PositionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<EarnPositionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnPositionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<YieldHistoryResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<YieldHistoryItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyEarnPositionParams {
    pub category: String,
    pub product_id: i64,
    pub position_id: i64,
    pub auto_reinvest: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiquidityParams {
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLiquidityParams {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReinvestLiquidityParams {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimLiquidityInterestParams {
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyYieldResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: HourlyYieldResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyYieldResult {
    #[serde(default)]
    pub list: Vec<HourlyYieldRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyYieldRecord {
    #[serde(default)]
    pub effective_amount: String,
    #[serde(default, rename = "yield")]
    pub yield_: String,
    #[serde(default)]
    pub reward_type: i32,
    #[serde(default)]
    pub apr_e8: i64,
    #[serde(default)]
    pub hourly_date: i64,
    #[serde(default)]
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AprHistoryResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: AprHistoryResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AprHistoryResult {
    #[serde(default)]
    pub list: Vec<AprRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AprRecord {
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub apr_e8: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCouponsResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: ListCouponsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCouponsResult {
    #[serde(default)]
    pub list: Vec<EarnCoupon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnCoupon {
    #[serde(default)]
    pub coupon_id: String,
    #[serde(default)]
    pub coupon_type: String,
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHoldToEarnProductResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: GetHoldToEarnProductResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHoldToEarnProductResult {
    #[serde(default)]
    pub list: Vec<HoldToEarnProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldToEarnProduct {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub min_stake_amount: String,
    #[serde(default)]
    pub max_stake_amount: String,
    #[serde(default)]
    pub precision: i32,
    #[serde(default)]
    pub apy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHoldToEarnYieldHistoryResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: GetHoldToEarnYieldHistoryResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHoldToEarnYieldHistoryResult {
    #[serde(default)]
    pub list: Vec<HoldToEarnYieldRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldToEarnYieldRecord {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default, rename = "yield")]
    pub yield_: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMProductsResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: GetLMProductsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMProductsResult {
    #[serde(default)]
    pub list: Vec<LMProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMProduct {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub min_leverage: String,
    #[serde(default)]
    pub max_leverage: String,
    #[serde(default)]
    pub min_quote_amount: String,
    #[serde(default)]
    pub max_quote_amount: String,
    #[serde(default)]
    pub min_base_amount: String,
    #[serde(default)]
    pub max_base_amount: String,
    #[serde(default)]
    pub apy: String,
    #[serde(default)]
    pub precision: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMOrderResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: LMOrderResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMPositionsResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: GetLMPositionsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMPositionsResult {
    #[serde(default)]
    pub list: Vec<LMPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMPosition {
    #[serde(default)]
    pub position_id: String,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub base_amount: String,
    #[serde(default)]
    pub quote_amount: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub margin: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMOrdersResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: GetLMOrdersResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMOrdersResult {
    #[serde(default)]
    pub list: Vec<LMOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMOrder {
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub order_link_id: String,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub position_id: String,
    #[serde(default)]
    pub order_type: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub base_amount: String,
    #[serde(default)]
    pub quote_amount: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub created_time: i64,
    #[serde(default)]
    pub updated_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMYieldRecordsResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: GetLMYieldRecordsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMYieldRecordsResult {
    #[serde(default)]
    pub list: Vec<LMYieldRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMYieldRecord {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub position_id: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenOrderListResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: TokenOrderListResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenProductResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: TokenProduct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenPositionResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: TokenPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyYieldRecord {
    #[serde(default, rename = "yield")]
    pub yield_: String,
    #[serde(default)]
    pub bonus_yield: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenDailyYieldResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: TokenDailyYieldResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenHourlyYieldResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: TokenHourlyYieldResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenHistoricalAprResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
    pub result: TokenAprHistoryResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TieredApy {
    #[serde(default)]
    pub min: String,
    #[serde(default)]
    pub max: String,
    #[serde(default)]
    pub apy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestCoinApy {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub apy: String,
    #[serde(default)]
    pub expect_unit_earning: String,
    #[serde(default)]
    pub current_price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInterestCoinApy {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub apy: String,
    #[serde(default)]
    pub expect_return_earning: String,
    #[serde(default)]
    pub price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldInfo {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub apy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarlyRedeemInfo {
    #[serde(default)]
    pub allow_early_redeem: bool,
    #[serde(default)]
    pub early_redeem_earning: String,
    #[serde(default)]
    pub return_coin: String,
    #[serde(default)]
    pub redemption_limit_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstSettleProfitParams {
    pub fund_id: String,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateFundParams {
    pub fund_name: String,
    pub coin: String,
    pub profit_share_rate: String,
    pub management_fee_rate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_introduction: Option<String>,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateInvestmentPlanParams {
    pub account_uid: String,
    pub plan_name: String,
    pub plan_type: String,
    pub investment_distribution: Vec<InvestmentDistributionItem>,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentDistributionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstManageInvestmentPlanParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_funds: Option<Vec<InvestmentDistributionItem>>,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstManageOrderParams {
    pub order_id: String,
    pub action: String,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateSubAccountParams {
    pub fund_id: String,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundTransferParams {
    pub transfer_id: String,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub amount: String,
    pub coin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMLiquidationRecordsResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstListFundsResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstSettleProfitResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateFundResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateInvestmentPlanResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstGetInvestmentPlansResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstManageInvestmentPlanResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstListOrdersResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstManageOrderResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateSubAccountResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundTransferResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmQueryFundTransferResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmListInvestmentPlansResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmGetPlanDetailResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmGetNewPlanDetailResponse {
    #[serde(default)]
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    #[serde(default)]
    pub time: i64,
    #[serde(default)]
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemFixedTermRequest {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoInvestRequest {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRwaProductListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRwaOrderRequest {
    pub product_id: i64,
    pub order_type: String,
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRwaPositionListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRwaOrderListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRwaNavChartResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub category: String,
    pub order_type: String,
    pub account_type: String,
    pub amount: String,
    pub coin: String,
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_card: Option<InterestCardRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestCardRef {
    pub award_id: i64,
    pub spec_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmClaimParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmClaimResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmAssetTrendResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundNavResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmSubscribeParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmSubscribeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestMoreParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub category: String,
    pub product_id: String,
    pub amount: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestMoreResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmRedeemParams {
    pub plan_id: String,
    pub category: String,
    pub product_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmRedeemResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmListOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmListProductCardsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub products: Vec<serde_json::Value>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceEarnOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_order_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_redeem_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_stake_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnOrderHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimable_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_redeem_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_stake_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_interest_calculation_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reinvest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnPositionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_staking_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnYieldHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnHourlyYieldHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyEarnPositionRequest {
    pub category: String,
    pub product_id: i64,
    pub position_id: i64,
    pub auto_reinvest: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnAprHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_pnl_percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEarnCouponsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldToEarnYieldCoin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirdropDailyPnl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_coin_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinApy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<i64>,
    #[serde(rename = "apy_e8", skip_serializing_if = "Option::is_none")]
    pub apy_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiquidityMiningProductsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiquidityRequest {
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLiquidityRequest {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReinvestRequest {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginRequest {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
    pub amount: String,
    pub quote_account_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiquidityResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimInterestRequest {
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiquidityMiningPositionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiquidityMiningOrdersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiquidityMiningYieldRecordsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMLiquidationRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiquidityMiningLiquidationRecordsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstFundInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_equity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentAPR")]
    pub current_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_share_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncollected_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_loan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstInvestmentDistributionItem {
    pub product_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstInvestmentDistribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstInvestmentPlanInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_asset_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment_distribution: Option<Vec<PwmInstInvestmentDistribution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimLiquidityInterestResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCoinAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestmentDistribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmPositionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reinvest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCategoryPositionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_investment_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_avg_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PwmPositionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmEquityFundPositionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holding_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr30d: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_total: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharp_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_drawdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmEquityFundPositionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_investment_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_avg_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PwmEquityFundPositionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmPlanPositions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_coins_earning: Option<PwmCategoryPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_yield: Option<PwmCategoryPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_funds: Option<PwmEquityFundPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onchain_earn: Option<PwmCategoryPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_account: Option<Vec<PwmCoinAmount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundIntroduction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_yield_rate_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_yield_rate_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharp_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_drawback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockup_period: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmConfiguredProductItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<PwmFundIntroduction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmConfiguredProductGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_amount_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PwmConfiguredProductItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmNewPlanProducts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_coins_earning: Option<PwmConfiguredProductGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_yield: Option<PwmConfiguredProductGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_funds: Option<PwmConfiguredProductGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onchain_earn: Option<PwmConfiguredProductGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestmentPlanSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_asset_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment_distribution: Option<Vec<PwmInvestmentDistribution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstUpdateFundItem {
    pub fund_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstOrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundTransferRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmQueryFundTransferResultResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmPlanDetailResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_asset_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_avg_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_assets: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmNewPlanDetailResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_configured_amount_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmClaimResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmAssetDataPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_value_usd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmAssetTrendResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Vec<PwmAssetDataPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmNavDataPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundNavResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Vec<PwmNavDataPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmSubscribeRequest {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmSubscribeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestMoreRequest {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub category: String,
    pub product_id: String,
    pub amount: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestMoreResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmRedeemRequest {
    pub plan_id: String,
    pub category: String,
    pub product_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmRedeemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmOrderDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmListOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<PwmOrderDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmProductCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_range_low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_range_high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_drawdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharp_ratio: Option<String>,
    #[serde(rename = "estAPR", skip_serializing_if = "Option::is_none")]
    pub est_apr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmProductTypeGroup {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<PwmProductCard>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmListProductCardsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<PwmProductTypeGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanProductItem {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub products: Vec<PwmCreateCustomPlanProductItem>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmClaimRequest {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    pub order_link_id: String,
}
