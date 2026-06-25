## 2026-06-25 (v0.1.2)

### New service modules (7)
- broker (8 methods, 25 types)
- crypto_loan (28 methods, 53 types)
- earn (69 methods, 293 types)
- fiat (26 methods, 123 types)
- fmart_bot (1 method, 2 types)
- spot_margin (4 methods, 10 types)
- user (19 methods, 40 types)

### Extended existing modules (4)
- account: +132 lines of new methods + types
- market:  +157 lines of new methods + types
- position: +90 lines of new methods + types
- trade:   +25 lines of new methods + types

### Notes
- All shared model types now derive both Serialize and Deserialize so leg/item
  structs can be reused across request and response contexts.
- 33 service-method signatures fell back to serde_json::Value (untyped body
  / return) where the OpenAPI spec referenced response or request types that
  the generation workflow could not auto-resolve. Marked with FIXME comments
  in source — typed signatures can be added in a follow-up.

## 2026-06-24

### New Functions (15)
- accept_non_lp_quote
- cancel_all_quotes
- cancel_all_rfqs
- cancel_quote
- cancel_rfq
- create_quote
- create_rfq
- execute_quote
- get_public_trades
- get_quotes
- get_quotes_realtime
- get_rfq_config
- get_rfqs
- get_rfqs_realtime
- get_trade_history

### New Types (53)
- AcceptNonLpQuoteParams
- AcceptNonLpQuoteResponse
- AcceptNonLpQuoteResult
- CancelAllQuotesParams
- CancelAllQuotesResponse
- CancelAllQuotesResultItem
- CancelAllRfqsParams
- CancelAllRfqsResponse
- CancelAllRfqsResult
- CancelQuoteParams
- CancelQuoteResponse
- CancelQuoteResult
- CancelRfqParams
- CancelRfqResponse
- CancelRfqResult
- CreateQuoteParams
- QuoteLeg
- CreateQuoteResponse
- CreateQuoteResult
- CreateRfqParams
- CreateRfqLeg
- CreateRfqResponse
- CreateRfqResult
- ExecuteQuoteParams
- ExecuteQuoteResponse
- ExecuteQuoteResult
- GetPublicTradesResponse
- GetPublicTradesResult
- PublicTrade
- PublicTradeLeg
- GetQuotesRealtimeResponse
- GetQuotesRealtimeResult
- QuoteRealtimeItem
- QuoteItemLeg
- GetQuotesResponse
- GetQuotesResult
- QuoteItem
- GetRfqConfigResponse
- GetRfqConfigResult
- RfqStrategyType
- RfqCounterparty
- GetRfqsRealtimeResponse
- GetRfqsRealtimeResult
- GetRfqsRealtimeItem
- GetRfqsRealtimeLeg
- GetRfqsResponse
- GetRfqsResult
- GetRfqsListItem
- GetRfqsLeg
- GetTradeHistoryResponse
- GetTradeHistoryResult
- GetTradeHistoryTrade
- GetTradeHistoryLeg
