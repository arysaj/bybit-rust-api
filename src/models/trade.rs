//! Trade models for orders.

use crate::error::{BybitError, Result};
use crate::models::common::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Place order request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderParams {
    /// Product category
    pub category: Category,
    /// Symbol name
    pub symbol: String,
    /// Order side
    pub side: Side,
    /// Order type
    pub order_type: OrderType,
    /// Order quantity
    pub qty: String,
    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Position index (for hedge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
    /// User-defined order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Take profit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Reduce only flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Close on trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// Trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
}

impl PlaceOrderParams {
    /// Create a new market order.
    pub fn market(category: Category, symbol: &str, side: Side, qty: &str) -> Self {
        Self {
            category,
            symbol: symbol.to_string(),
            side,
            order_type: OrderType::Market,
            qty: qty.to_string(),
            price: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            reduce_only: None,
            close_on_trigger: None,
            trigger_price: None,
            trigger_by: None,
        }
    }

    /// Create a new limit order.
    pub fn limit(category: Category, symbol: &str, side: Side, qty: &str, price: &str) -> Self {
        Self {
            category,
            symbol: symbol.to_string(),
            side,
            order_type: OrderType::Limit,
            qty: qty.to_string(),
            price: Some(price.to_string()),
            time_in_force: Some(TimeInForce::GTC),
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            reduce_only: None,
            close_on_trigger: None,
            trigger_price: None,
            trigger_by: None,
        }
    }

    /// Set position index (for hedge mode).
    pub fn with_position_idx(mut self, idx: i32) -> Self {
        self.position_idx = Some(idx);
        self
    }

    /// Set user-defined order ID.
    pub fn with_order_link_id(mut self, id: &str) -> Self {
        self.order_link_id = Some(id.to_string());
        self
    }

    /// Set take profit price.
    pub fn with_take_profit(mut self, price: &str) -> Self {
        self.take_profit = Some(price.to_string());
        self
    }

    /// Set stop loss price.
    pub fn with_stop_loss(mut self, price: &str) -> Self {
        self.stop_loss = Some(price.to_string());
        self
    }

    /// Set reduce only flag.
    pub fn with_reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Validate parameters before sending.
    pub fn validate(&self) -> Result<()> {
        if self.symbol.is_empty() {
            return Err(BybitError::InvalidParam("symbol cannot be empty".into()));
        }

        if self.qty.is_empty() {
            return Err(BybitError::InvalidParam("qty cannot be empty".into()));
        }

        // Parse and validate qty is positive
        let qty: Decimal = self
            .qty
            .parse()
            .map_err(|_| BybitError::InvalidParam("qty must be a valid number".into()))?;
        if qty <= Decimal::ZERO {
            return Err(BybitError::InvalidParam("qty must be positive".into()));
        }

        // Limit orders require price
        if self.order_type == OrderType::Limit {
            match &self.price {
                None => {
                    return Err(BybitError::InvalidParam(
                        "price is required for limit orders".into(),
                    ))
                }
                Some(p) => {
                    let price: Decimal = p.parse().map_err(|_| {
                        BybitError::InvalidParam("price must be a valid number".into())
                    })?;
                    if price <= Decimal::ZERO {
                        return Err(BybitError::InvalidParam("price must be positive".into()));
                    }
                }
            }
        }

        Ok(())
    }
}

/// Amend order request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderParams {
    /// Product category
    pub category: Category,
    /// Symbol name
    pub symbol: String,
    /// Order ID (either order_id or order_link_id required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User-defined order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// New order quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    /// New order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// New take profit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// New stop loss price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
}

impl AmendOrderParams {
    /// Create amend params by order ID.
    pub fn by_order_id(category: Category, symbol: &str, order_id: &str) -> Self {
        Self {
            category,
            symbol: symbol.to_string(),
            order_id: Some(order_id.to_string()),
            order_link_id: None,
            qty: None,
            price: None,
            take_profit: None,
            stop_loss: None,
        }
    }

    /// Create amend params by order link ID.
    pub fn by_order_link_id(category: Category, symbol: &str, order_link_id: &str) -> Self {
        Self {
            category,
            symbol: symbol.to_string(),
            order_id: None,
            order_link_id: Some(order_link_id.to_string()),
            qty: None,
            price: None,
            take_profit: None,
            stop_loss: None,
        }
    }

    /// Set new price.
    pub fn with_price(mut self, price: &str) -> Self {
        self.price = Some(price.to_string());
        self
    }

    /// Set new quantity.
    pub fn with_qty(mut self, qty: &str) -> Self {
        self.qty = Some(qty.to_string());
        self
    }
}

/// Cancel order request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams {
    /// Product category
    pub category: Category,
    /// Symbol name
    pub symbol: String,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User-defined order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

impl CancelOrderParams {
    /// Create cancel params by order ID.
    pub fn by_order_id(category: Category, symbol: &str, order_id: &str) -> Self {
        Self {
            category,
            symbol: symbol.to_string(),
            order_id: Some(order_id.to_string()),
            order_link_id: None,
        }
    }

    /// Create cancel params by order link ID.
    pub fn by_order_link_id(category: Category, symbol: &str, order_link_id: &str) -> Self {
        Self {
            category,
            symbol: symbol.to_string(),
            order_id: None,
            order_link_id: Some(order_link_id.to_string()),
        }
    }
}

/// Cancel all orders request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersParams {
    /// Product category
    pub category: Category,
    /// Symbol name (optional, cancel all if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Settle coin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
}

/// Order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    /// Order ID
    pub order_id: String,
    /// User-defined order ID
    #[serde(default)]
    pub order_link_id: String,
}

/// Orders list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersList {
    /// Category
    pub category: String,
    /// List of orders
    pub list: Vec<OrderInfo>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Order info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    /// Order ID
    pub order_id: String,
    /// User-defined order ID
    #[serde(default)]
    pub order_link_id: String,
    /// Symbol
    pub symbol: String,
    /// Side
    pub side: String,
    /// Order type
    pub order_type: String,
    /// Price
    #[serde(default)]
    pub price: String,
    /// Quantity
    pub qty: String,
    /// Time in force
    #[serde(default)]
    pub time_in_force: String,
    /// Order status
    pub order_status: String,
    /// Cumulative executed qty
    #[serde(default)]
    pub cum_exec_qty: String,
    /// Cumulative executed value
    #[serde(default)]
    pub cum_exec_value: String,
    /// Average price
    #[serde(default)]
    pub avg_price: String,
    /// Created time
    pub created_time: String,
    /// Updated time
    pub updated_time: String,
    /// Take profit price
    #[serde(default)]
    pub take_profit: String,
    /// Stop loss price
    #[serde(default)]
    pub stop_loss: String,
    /// Position index
    #[serde(default)]
    pub position_idx: i32,
    /// Reduce only
    #[serde(default)]
    pub reduce_only: bool,
}

/// Batch order request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderRequest {
    /// Product category
    pub category: Category,
    /// List of orders
    pub request: Vec<PlaceOrderParams>,
}

/// Batch order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResponse {
    /// List of results
    pub list: Vec<BatchOrderResult>,
}

/// Single batch order result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResult {
    /// Category
    pub category: String,
    /// Symbol
    pub symbol: String,
    /// Order ID
    #[serde(default)]
    pub order_id: String,
    /// User-defined order ID
    #[serde(default)]
    pub order_link_id: String,
    /// Create type
    #[serde(default)]
    pub create_type: String,
}

/// Cancel all orders response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllResponse {
    /// List of cancelled orders
    pub list: Vec<CancelledOrder>,
}

/// Cancelled order info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelledOrder {
    /// Order ID
    pub order_id: String,
    /// User-defined order ID
    #[serde(default)]
    pub order_link_id: String,
}

pub type DcpSetTimewindowResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcpSetTimewindowParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    pub time_window: i32,
}

pub type SetDcpResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDcpParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    pub time_window: i32,
}

pub type PreCheckOrderResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreCheckOrderParams {
    pub category: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
}

pub type GetSpotBorrowQuotaResponse = SpotBorrowQuotaResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotBorrowQuotaResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_trade_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_trade_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_max_trade_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_max_trade_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_coin: Option<String>,
}

pub type AmendOrderResponse = AmendOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrdersRequest {
    pub category: String,
    pub request: Vec<BatchAmendOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderItem {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
}

pub type BatchAmendOrdersResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrdersRequest {
    pub category: String,
    pub request: Vec<BatchCancelOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderItem {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

pub type BatchCancelOrdersResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateOrdersRequest {
    pub category: String,
    pub request: Vec<BatchOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderItem {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
}

pub type BatchCreateOrdersResponse = serde_json::Value;

pub type CancelAllOrdersResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: String,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
}

pub type CancelOrderResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    pub category: String,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,
    pub side: String,
    pub order_type: String,
    pub qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbo_side_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbo_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpi_taker_access: Option<bool>,
}

pub type CreateOrderResponse = serde_json::Value;

pub type GetOpenOrdersResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_trade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_exec_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_exec_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_exec_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price_on_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_group: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oco_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price: Option<String>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_fee_detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

pub type GetOrderHistoryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_trade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_exec_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_exec_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_exec_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price_on_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_group: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpi_taker_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpi_matched_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oco_trigger_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance: Option<String>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cum_fee_detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fees: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeExecutionDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_fee_v2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_maker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_trade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fees: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreCheckOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_imr_e4: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_mmr_e4: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_imr_e4: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_mmr_e4: Option<i64>,
}
