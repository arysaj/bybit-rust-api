//! RFQ API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::rfq::*;

impl BybitClient {
    /// Accept Non-LP Quote.
    pub async fn accept_non_lp_quote(
        &self,
        params: AcceptNonLpQuoteParams,
    ) -> Result<AcceptNonLpQuoteResponse> {
        self.post("/v5/rfq/accept-other-quote", &params).await
    }

    /// Cancel All Quotes.
    pub async fn cancel_all_quotes(
        &self,
        params: CancelAllQuotesParams,
    ) -> Result<CancelAllQuotesResponse> {
        self.post("/v5/rfq/cancel-all-quotes", &params).await
    }

    /// Cancel All RFQs.
    pub async fn cancel_all_rfqs(
        &self,
        params: CancelAllRfqsParams,
    ) -> Result<CancelAllRfqsResponse> {
        self.post("/v5/rfq/cancel-all-rfq", &params).await
    }

    /// Cancel Quote.
    pub async fn cancel_quote(&self, params: CancelQuoteParams) -> Result<CancelQuoteResponse> {
        self.post("/v5/rfq/cancel-quote", &params).await
    }

    /// Cancel RFQ.
    pub async fn cancel_rfq(&self, params: CancelRfqParams) -> Result<CancelRfqResponse> {
        self.post("/v5/rfq/cancel-rfq", &params).await
    }

    /// Create Quote.
    pub async fn create_quote(&self, params: CreateQuoteParams) -> Result<CreateQuoteResponse> {
        self.post("/v5/rfq/create-quote", &params).await
    }

    /// Create RFQ.
    pub async fn create_rfq(&self, params: CreateRfqParams) -> Result<CreateRfqResponse> {
        self.post("/v5/rfq/create-rfq", &params).await
    }

    /// Execute Quote.
    pub async fn execute_quote(&self, params: ExecuteQuoteParams) -> Result<ExecuteQuoteResponse> {
        self.post("/v5/rfq/execute-quote", &params).await
    }

    /// Get Public Trades.
    pub async fn get_public_trades(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GetPublicTradesResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(ref s) = start_time_str {
            params.push(("startTime", s.as_str()));
        }
        if let Some(ref s) = end_time_str {
            params.push(("endTime", s.as_str()));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }

        self.get("/v5/rfq/public-trades", &params).await
    }

    /// Get Quotes.
    pub async fn get_quotes(
        &self,
        rfq_id: Option<&str>,
        quote_id: Option<&str>,
        quote_link_id: Option<&str>,
        trader_type: Option<&str>,
        status: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GetQuotesResponse> {
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = rfq_id {
            params.push(("rfqId", s));
        }
        if let Some(s) = quote_id {
            params.push(("quoteId", s));
        }
        if let Some(s) = quote_link_id {
            params.push(("quoteLinkId", s));
        }
        if let Some(s) = trader_type {
            params.push(("traderType", s));
        }
        if let Some(s) = status {
            params.push(("status", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(s) = cursor {
            params.push(("cursor", s));
        }

        self.get("/v5/rfq/quote-list", &params).await
    }

    /// Get Quotes Realtime.
    pub async fn get_quotes_realtime(
        &self,
        rfq_id: Option<&str>,
        quote_id: Option<&str>,
        quote_link_id: Option<&str>,
        trader_type: Option<&str>,
    ) -> Result<GetQuotesRealtimeResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = rfq_id {
            params.push(("rfqId", s));
        }
        if let Some(s) = quote_id {
            params.push(("quoteId", s));
        }
        if let Some(s) = quote_link_id {
            params.push(("quoteLinkId", s));
        }
        if let Some(s) = trader_type {
            params.push(("traderType", s));
        }

        self.get("/v5/rfq/quote-realtime", &params).await
    }

    /// Get RFQ Config.
    pub async fn get_rfq_config(&self) -> Result<GetRfqConfigResponse> {
        self.get("/v5/rfq/config", &[]).await
    }

    /// Get RFQs.
    pub async fn get_rfqs(
        &self,
        rfq_id: Option<&str>,
        rfq_link_id: Option<&str>,
        trader_type: Option<&str>,
        status: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GetRfqsResponse> {
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = rfq_id {
            params.push(("rfqId", s));
        }
        if let Some(s) = rfq_link_id {
            params.push(("rfqLinkId", s));
        }
        if let Some(s) = trader_type {
            params.push(("traderType", s));
        }
        if let Some(s) = status {
            params.push(("status", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(s) = cursor {
            params.push(("cursor", s));
        }

        self.get("/v5/rfq/rfq-list", &params).await
    }

    /// Get RFQs Realtime.
    pub async fn get_rfqs_realtime(
        &self,
        rfq_id: Option<&str>,
        rfq_link_id: Option<&str>,
        trader_type: Option<&str>,
    ) -> Result<GetRfqsRealtimeResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = rfq_id {
            params.push(("rfqId", s));
        }
        if let Some(s) = rfq_link_id {
            params.push(("rfqLinkId", s));
        }
        if let Some(s) = trader_type {
            params.push(("traderType", s));
        }

        self.get("/v5/rfq/rfq-realtime", &params).await
    }

    /// Get Trade History.
    pub async fn get_trade_history(
        &self,
        rfq_id: Option<&str>,
        rfq_link_id: Option<&str>,
        quote_id: Option<&str>,
        quote_link_id: Option<&str>,
        trader_type: Option<&str>,
        status: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GetTradeHistoryResponse> {
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = rfq_id {
            params.push(("rfqId", s));
        }
        if let Some(s) = rfq_link_id {
            params.push(("rfqLinkId", s));
        }
        if let Some(s) = quote_id {
            params.push(("quoteId", s));
        }
        if let Some(s) = quote_link_id {
            params.push(("quoteLinkId", s));
        }
        if let Some(s) = trader_type {
            params.push(("traderType", s));
        }
        if let Some(s) = status {
            params.push(("status", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(s) = cursor {
            params.push(("cursor", s));
        }

        self.get("/v5/rfq/trade-list", &params).await
    }
}
