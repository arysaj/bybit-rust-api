//! RFQ (Request For Quote) models.

use serde::{Deserialize, Serialize};

/// Accept non-LP quote request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptNonLpQuoteParams {
    pub rfq_id: String,
}

/// Accept non-LP quote response.
pub type AcceptNonLpQuoteResponse = AcceptNonLpQuoteResult;

/// Accept non-LP quote result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptNonLpQuoteResult {
    pub rfq_id: Option<String>,
}

/// Cancel all quotes request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllQuotesParams {}

/// Cancel all quotes response.
pub type CancelAllQuotesResponse = Vec<CancelAllQuotesResultItem>;

/// Cancel all quotes result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllQuotesResultItem {
    pub rfq_id: Option<String>,
    pub quote_id: Option<String>,
    pub quote_link_id: Option<String>,
    pub code: Option<String>,
    pub msg: Option<String>,
}

/// Cancel all RFQs request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllRfqsParams {}

/// Cancel all RFQs response.
pub type CancelAllRfqsResponse = Vec<CancelAllRfqsResult>;

/// Cancel all RFQs result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllRfqsResult {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub code: Option<String>,
    pub msg: Option<String>,
}

/// Cancel quote request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelQuoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_link_id: Option<String>,
}

/// Cancel quote response.
pub type CancelQuoteResponse = CancelQuoteResult;

/// Cancel quote result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelQuoteResult {
    pub rfq_id: Option<String>,
    pub quote_id: Option<String>,
    pub quote_link_id: Option<String>,
}

/// Cancel RFQ request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRfqParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_link_id: Option<String>,
}

/// Cancel RFQ response.
pub type CancelRfqResponse = CancelRfqResult;

/// Cancel RFQ result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRfqResult {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
}

/// Create quote request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuoteParams {
    pub rfq_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_in: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_buy_list: Option<Vec<QuoteLeg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_sell_list: Option<Vec<QuoteLeg>>,
}

/// Quote leg used in create quote request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteLeg {
    pub category: String,
    pub symbol: String,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
}

/// Create quote response.
pub type CreateQuoteResponse = CreateQuoteResult;

/// Create quote result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuoteResult {
    pub rfq_id: Option<String>,
    pub quote_id: Option<String>,
    pub quote_link_id: Option<String>,
    pub expires_at: Option<String>,
    pub desk_code: Option<String>,
    pub status: Option<String>,
}

/// Create RFQ request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRfqParams {
    pub counterparties: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<String>,
    pub list: Vec<CreateRfqLeg>,
}

/// RFQ leg used in create RFQ request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRfqLeg {
    pub category: String,
    pub symbol: String,
    pub side: String,
    pub qty: String,
}

/// Create RFQ response.
pub type CreateRfqResponse = CreateRfqResult;

/// Create RFQ result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRfqResult {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub status: Option<String>,
    pub expires_at: Option<String>,
    pub desk_code: Option<String>,
}

/// Execute quote request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteParams {
    pub rfq_id: String,
    pub quote_id: String,
    pub quote_side: String,
}

/// Execute quote response.
pub type ExecuteQuoteResponse = ExecuteQuoteResult;

/// Execute quote result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteResult {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub quote_id: Option<String>,
    pub status: Option<String>,
}

/// Get public trades response.
pub type GetPublicTradesResponse = GetPublicTradesResult;

/// Get public trades result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicTradesResult {
    pub cursor: Option<String>,
    pub list: Option<Vec<PublicTrade>>,
}

/// Public trade entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    pub rfq_id: Option<String>,
    pub strategy_type: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<PublicTradeLeg>>,
}

/// Public trade leg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTradeLeg {
    pub category: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
    pub mark_price: Option<String>,
}

/// Get quotes realtime response.
pub type GetQuotesRealtimeResponse = GetQuotesRealtimeResult;

/// Get quotes realtime result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuotesRealtimeResult {
    pub list: Option<Vec<QuoteRealtimeItem>>,
}

/// Quote realtime item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRealtimeItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub quote_id: Option<String>,
    pub quote_link_id: Option<String>,
    pub expires_at: Option<String>,
    pub status: Option<String>,
    pub desk_code: Option<String>,
    pub exec_quote_side: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub quote_buy_list: Option<Vec<QuoteItemLeg>>,
    pub quote_sell_list: Option<Vec<QuoteItemLeg>>,
}

/// Quote item leg (response).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteItemLeg {
    pub category: Option<String>,
    pub symbol: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
}

/// Get quotes response.
pub type GetQuotesResponse = GetQuotesResult;

/// Get quotes result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuotesResult {
    pub cursor: Option<String>,
    pub list: Option<Vec<QuoteItem>>,
}

/// Quote item (history).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub quote_id: Option<String>,
    pub quote_link_id: Option<String>,
    pub expires_at: Option<String>,
    pub desk_code: Option<String>,
    pub status: Option<String>,
    pub exec_quote_side: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub quote_buy_list: Option<Vec<QuoteItemLeg>>,
    pub quote_sell_list: Option<Vec<QuoteItemLeg>>,
}

/// Get RFQ config response.
pub type GetRfqConfigResponse = GetRfqConfigResult;

/// Get RFQ config result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRfqConfigResult {
    pub desk_code: Option<String>,
    pub max_legs: Option<i32>,
    #[serde(rename = "maxLP")]
    pub max_lp: Option<i32>,
    pub max_active_rfq: Option<i32>,
    pub rfq_expire_time: Option<i32>,
    pub min_limit_qty_spot_order: Option<i64>,
    pub min_limit_qty_contract_order: Option<i64>,
    pub min_limit_qty_option_order: Option<i64>,
    pub strategy_types: Option<Vec<RfqStrategyType>>,
    pub counterparties: Option<Vec<RfqCounterparty>>,
}

/// RFQ strategy type definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqStrategyType {
    pub strategy_name: Option<String>,
}

/// RFQ counterparty definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqCounterparty {
    pub trader_name: Option<String>,
    pub desk_code: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// Get RFQs realtime response.
pub type GetRfqsRealtimeResponse = GetRfqsRealtimeResult;

/// Get RFQs realtime result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRfqsRealtimeResult {
    pub list: Option<Vec<GetRfqsRealtimeItem>>,
}

/// RFQ realtime item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRfqsRealtimeItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub counterparties: Option<Vec<String>>,
    pub expires_at: Option<String>,
    pub strategy_type: Option<String>,
    pub status: Option<String>,
    pub accept_other_quote_status: Option<String>,
    pub desk_code: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<GetRfqsRealtimeLeg>>,
}

/// RFQ realtime leg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRfqsRealtimeLeg {
    pub category: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub qty: Option<String>,
}

/// Get RFQs response.
pub type GetRfqsResponse = GetRfqsResult;

/// Get RFQs result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRfqsResult {
    pub cursor: Option<String>,
    pub list: Option<Vec<GetRfqsListItem>>,
}

/// RFQ history list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRfqsListItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub counterparties: Option<Vec<String>>,
    pub strategy_type: Option<String>,
    pub expires_at: Option<String>,
    pub status: Option<String>,
    pub accept_other_quote_status: Option<String>,
    pub desk_code: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<GetRfqsLeg>>,
}

/// RFQ history leg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRfqsLeg {
    pub category: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub qty: Option<String>,
}

/// Get trade history response.
pub type GetTradeHistoryResponse = GetTradeHistoryResult;

/// Get trade history result payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeHistoryResult {
    pub cursor: Option<String>,
    pub list: Option<Vec<GetTradeHistoryTrade>>,
}

/// Trade history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeHistoryTrade {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub quote_id: Option<String>,
    pub quote_link_id: Option<String>,
    pub quote_side: Option<String>,
    pub strategy_type: Option<String>,
    pub status: Option<String>,
    pub rfq_desk_code: Option<String>,
    pub quote_desk_code: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<GetTradeHistoryLeg>>,
}

/// Trade history leg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeHistoryLeg {
    pub category: Option<String>,
    pub order_id: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
    pub mark_price: Option<String>,
    pub exec_fee: Option<String>,
    pub exec_id: Option<String>,
    pub result_code: Option<i32>,
    pub result_message: Option<String>,
    pub reject_party: Option<String>,
}

/// Cancel all RFQs result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllRfqsResultItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub code: Option<String>,
    pub msg: Option<String>,
}

/// Public trade item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTradeItem {
    pub rfq_id: Option<String>,
    pub strategy_type: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<PublicTradeLeg>>,
}

/// Strategy type entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyType {
    pub strategy_name: Option<String>,
}

/// Counterparty entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counterparty {
    pub trader_name: Option<String>,
    pub desk_code: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// RFQ realtime item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqRealtimeItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub counterparties: Option<Vec<String>>,
    pub expires_at: Option<String>,
    pub strategy_type: Option<String>,
    pub status: Option<String>,
    pub accept_other_quote_status: Option<String>,
    pub desk_code: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<RfqRealtimeLeg>>,
}

/// RFQ realtime leg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqRealtimeLeg {
    pub category: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub qty: Option<String>,
}

/// RFQ item (history).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub counterparties: Option<Vec<String>>,
    pub strategy_type: Option<String>,
    pub expires_at: Option<String>,
    pub status: Option<String>,
    pub accept_other_quote_status: Option<String>,
    pub desk_code: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<RfqLeg>>,
}

/// RFQ leg (history).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqLeg {
    pub category: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub qty: Option<String>,
}

/// Trade history item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistoryItem {
    pub rfq_id: Option<String>,
    pub rfq_link_id: Option<String>,
    pub quote_id: Option<String>,
    pub quote_link_id: Option<String>,
    pub quote_side: Option<String>,
    pub strategy_type: Option<String>,
    pub status: Option<String>,
    pub rfq_desk_code: Option<String>,
    pub quote_desk_code: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub legs: Option<Vec<TradeHistoryLeg>>,
}

/// Trade history leg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistoryLeg {
    pub category: Option<String>,
    pub order_id: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
    pub mark_price: Option<String>,
    pub exec_fee: Option<String>,
    pub exec_id: Option<String>,
    pub result_code: Option<i32>,
    pub result_message: Option<String>,
    pub reject_party: Option<String>,
}
