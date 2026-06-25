//! Broker API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::broker::*;

impl BybitClient {
    /// Get Broker Account Info.
    pub async fn query_broker_account_info(&self) -> Result<QueryBrokerAccountInfoResponse> {
        self.get("/v5/broker/account-info", &[]).await
    }

    /// Query Broker All UID Rate Limits.
    pub async fn query_broker_all_uid_details(
        &self,
        uids: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<QueryBrokerAllUidDetailsResponse> {
        let limit_str = limit.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(u) = uids {
            params.push(("uids", u));
        }
        if let Some(ref l) = limit_str {
            params.push(("limit", l.as_str()));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        self.get("/v5/broker/apilimit/query-all", &params).await
    }

    /// Query Broker Rate Limit Cap.
    pub async fn query_broker_cap(&self) -> Result<QueryBrokerCapResponse> {
        self.get("/v5/broker/apilimit/query-cap", &[]).await
    }

    /// Get Broker Earnings Info.
    pub async fn query_broker_earning(
        &self,
        biz_type: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        uid: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<QueryBrokerEarningResponse> {
        let limit_str = limit.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(b) = biz_type {
            params.push(("bizType", b));
        }
        if let Some(b) = begin {
            params.push(("begin", b));
        }
        if let Some(e) = end {
            params.push(("end", e));
        }
        if let Some(u) = uid {
            params.push(("uid", u));
        }
        if let Some(ref l) = limit_str {
            params.push(("limit", l.as_str()));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        self.get("/v5/broker/earnings-info", &params).await
    }

    /// Set Broker API Rate Limit.
    pub async fn set_broker_api_limit(
        &self,
        params: SetBrokerApiLimitParams,
    ) -> Result<SetBrokerApiLimitResponse> {
        self.post("/v5/broker/apilimit/set", &params).await
    }

    /// Distribute voucher.
    pub async fn distribute_award(
        &self,
        params: DistributeAwardParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/broker/award/distribute-award", &params)
            .await
    }

    /// Get voucher details.
    pub async fn get_award_info(&self, params: GetAwardInfoParams) -> Result<GetAwardInfoResponse> {
        self.post("/v5/broker/award/info", &params).await
    }

    /// Query voucher distribution record.
    pub async fn get_distribution_record(
        &self,
        params: GetDistributionRecordParams,
    ) -> Result<GetDistributionRecordResponse> {
        self.post("/v5/broker/award/distribution-record", &params)
            .await
    }
}
