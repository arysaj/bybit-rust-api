//! Fiat API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::fiat::*;
use crate::models::AccountType;

impl BybitClient {
    /// Request a Quote.
    pub async fn apply_quote(&self, params: ApplyQuoteParams) -> Result<ApplyQuoteResponse> {
        self.post("/v5/fiat/quote-apply", &params).await
    }

    /// Confirm Quote and Execute Trade.
    pub async fn confirm_quote(&self, params: ConfirmQuoteParams) -> Result<ConfirmQuoteResponse> {
        self.post("/v5/fiat/trade-execute", &params).await
    }

    /// Get Account Information.
    // FIXME(gen-sdk-rust): plan agent referenced GetAccountInfoResponse without
    // emitting the type to models/fiat.rs. Falling back to serde_json::Value.
    pub async fn get_account_info_v2(&self) -> Result<serde_json::Value> {
        self.post("/v5/p2p/user/personal/info", &serde_json::json!({}))
            .await
    }

    /// Get Ads.
    pub async fn get_ads(&self, params: GetAdsParams) -> Result<GetAdsResponse> {
        self.post("/v5/p2p/item/online", &params).await
    }

    /// Get All Orders.
    // FIXME(gen-sdk-rust): plan agent referenced GetAllOrdersParams +
    // GetAllOrdersResponse without emitting the types to models/fiat.rs.
    // Falling back to serde_json::Value for both.
    pub async fn get_all_orders(&self, params: serde_json::Value) -> Result<serde_json::Value> {
        self.post("/v5/p2p/order/simplifyList", &params).await
    }

    /// Get Chat Message.
    pub async fn get_chat_messages(
        &self,
        params: GetChatMessagesParams,
    ) -> Result<GetChatMessagesResponse> {
        self.post("/v5/p2p/order/message/listpage", &params).await
    }

    /// Get Coin Balance.
    pub async fn get_coin_balance(
        &self,
        account_type: AccountType,
        member_id: Option<&str>,
        coin: Option<&str>,
        with_bonus: Option<i64>,
    ) -> Result<GetCoinBalanceResponse> {
        let account_type_str = account_type.to_string();
        let with_bonus_str = with_bonus.map(|v| v.to_string());
        let mut params = vec![("accountType", account_type_str.as_str())];
        if let Some(m) = member_id {
            params.push(("memberId", m));
        }
        if let Some(c) = coin {
            params.push(("coin", c));
        }
        if let Some(ref wb) = with_bonus_str {
            params.push(("withBonus", wb.as_str()));
        }
        self.get("/v5/asset/transfer/query-account-coins-balance", &params)
            .await
    }

    /// Get Counterparty User Info.
    pub async fn get_counterparty_user_info(
        &self,
        params: GetCounterpartyUserInfoParams,
    ) -> Result<GetCounterpartyUserInfoResponse> {
        self.post("/v5/p2p/user/order/personal/info", &params).await
    }

    /// Get My Ad Details.
    pub async fn get_my_ad_details(
        &self,
        params: GetMyAdDetailsParams,
    ) -> Result<GetMyAdDetailsResponse> {
        self.post("/v5/p2p/item/info", &params).await
    }

    /// Get My Ads.
    pub async fn get_my_ads(&self, params: GetMyAdsParams) -> Result<GetMyAdsResponse> {
        self.post("/v5/p2p/item/personal/list", &params).await
    }

    /// Get Order Detail.
    pub async fn get_order_detail(
        &self,
        params: GetOrderDetailParams,
    ) -> Result<GetOrderDetailResponse> {
        self.post("/v5/p2p/order/info", &params).await
    }

    /// Get Pending Orders.
    pub async fn get_pending_orders(
        &self,
        params: GetPendingOrdersParams,
    ) -> Result<GetPendingOrdersResponse> {
        self.post("/v5/p2p/order/pending/simplifyList", &params)
            .await
    }

    /// Get Reference Price.
    pub async fn get_reference_price(
        &self,
        symbol: &str,
        payment_method: Option<&str>,
    ) -> Result<GetReferencePriceResponse> {
        let mut params = vec![("symbol", symbol)];
        if let Some(pm) = payment_method {
            params.push(("paymentMethod", pm));
        }
        self.get("/v5/fiat/reference-price", &params).await
    }

    /// Get User Payment.
    pub async fn get_user_payment(&self) -> Result<GetUserPaymentResponse> {
        self.post("/v5/p2p/user/payment/list", &serde_json::json!({}))
            .await
    }

    /// Mark Order as Paid.
    pub async fn mark_order_as_paid(
        &self,
        params: MarkOrderAsPaidParams,
    ) -> Result<MarkOrderAsPaidResponse> {
        self.post("/v5/p2p/order/pay", &params).await
    }

    /// Post Ad.
    pub async fn post_ad(&self, params: PostAdParams) -> Result<PostAdResponse> {
        self.post("/v5/p2p/item/create", &params).await
    }

    /// Query Account Balance.
    pub async fn query_balance(
        &self,
        account_category: Option<&str>,
        currency: Option<&str>,
    ) -> Result<QueryBalanceResponse> {
        let mut params = vec![];
        if let Some(ac) = account_category {
            params.push(("accountCategory", ac));
        }
        if let Some(c) = currency {
            params.push(("currency", c));
        }
        self.get("/v5/fiat/balance-query", &params).await
    }

    /// Get Trading Pairs.
    pub async fn query_coin_list(&self, side: Option<i64>) -> Result<QueryCoinListResponse> {
        let side_str = side.map(|v| v.to_string());
        let mut params = vec![];
        if let Some(ref s) = side_str {
            params.push(("side", s.as_str()));
        }
        self.get("/v5/fiat/query-coin-list", &params).await
    }

    /// Get Funding History.
    pub async fn query_funding_detail_api(
        &self,
        create_time_from: Option<&str>,
        create_time_to: Option<&str>,
        limit: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<QueryFundingDetailApiResponse> {
        let mut params = vec![];
        if let Some(v) = create_time_from {
            params.push(("createTimeFrom", v));
        }
        if let Some(v) = create_time_to {
            params.push(("createTimeTo", v));
        }
        if let Some(v) = limit {
            params.push(("limit", v));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }
        self.get("/v5/asset/fundinghistory", &params).await
    }

    /// Query Trade Status.
    pub async fn query_trade(
        &self,
        trade_no: Option<&str>,
        merchant_request_id: Option<&str>,
    ) -> Result<QueryTradeResponse> {
        let mut params = vec![];
        if let Some(v) = trade_no {
            params.push(("tradeNo", v));
        }
        if let Some(v) = merchant_request_id {
            params.push(("merchantRequestId", v));
        }
        self.get("/v5/fiat/trade-query", &params).await
    }

    /// Query Trade History.
    pub async fn query_trade_history(
        &self,
        index: Option<i64>,
        limit: Option<i64>,
        start_time: Option<&str>,
        end_time: Option<&str>,
    ) -> Result<QueryTradeHistoryResponse> {
        let index_str = index.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());
        let mut params = vec![];
        if let Some(ref v) = index_str {
            params.push(("index", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = start_time {
            params.push(("startTime", v));
        }
        if let Some(v) = end_time {
            params.push(("endTime", v));
        }
        self.get("/v5/fiat/query-trade-history", &params).await
    }

    /// Release Assets.
    pub async fn release_assets(&self, params: ReleaseAssetsParams) -> Result<serde_json::Value> {
        self.post("/v5/p2p/order/finish", &params).await
    }

    /// Remove Ad.
    pub async fn remove_ad(&self, params: RemoveAdParams) -> Result<serde_json::Value> {
        self.post("/v5/p2p/item/cancel", &params).await
    }

    /// Send Chat Message.
    pub async fn send_chat_message(
        &self,
        params: SendChatMessageParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/p2p/order/message/send", &params).await
    }

    /// Update / Relist Ad.
    pub async fn update_ad(&self, params: UpdateAdParams) -> Result<UpdateAdResponse> {
        self.post("/v5/p2p/item/update", &params).await
    }

    /// Upload Chat File.
    pub async fn upload_chat_file(
        &self,
        params: UploadChatFileParams,
    ) -> Result<UploadChatFileResponse> {
        self.post("/v5/p2p/oss/upload_file", &params).await
    }
}
