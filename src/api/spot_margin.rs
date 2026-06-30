//! Spot Margin API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::spot_margin::*;

impl BybitClient {
    /// Get Historical Interest Rate.
    pub async fn get_historical_interest_rate(
        &self,
        currency: &str,
        vip_level: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<GetHistoricalInterestRateResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let mut params = vec![("currency", currency)];
        if let Some(v) = vip_level {
            params.push(("vipLevel", v));
        }
        if let Some(s) = start_time_str.as_deref() {
            params.push(("startTime", s));
        }
        if let Some(s) = end_time_str.as_deref() {
            params.push(("endTime", s));
        }
        self.get("/v5/spot-margin-trade/interest-rate-history", &params)
            .await
    }

    /// Get Position Tiers.
    pub async fn get_position_tiers(
        &self,
        currency: Option<&str>,
    ) -> Result<GetPositionTiersResponse> {
        let mut params = vec![];
        if let Some(c) = currency {
            params.push(("currency", c));
        }
        self.get("/v5/spot-margin-trade/position-tiers", &params)
            .await
    }

    /// Get Tiered Collateral Ratio.
    pub async fn get_tiered_collateral_ratio(
        &self,
        currency: Option<&str>,
    ) -> Result<GetTieredCollateralRatioResponse> {
        let mut params = vec![];
        if let Some(c) = currency {
            params.push(("currency", c));
        }
        self.get_public("/v5/spot-margin-trade/collateral", &params)
            .await
    }

    /// Get VIP Margin Data.
    pub async fn get_vip_margin_data(
        &self,
        vip_level: Option<&str>,
        currency: Option<&str>,
    ) -> Result<GetVipMarginDataResponse> {
        let mut params = vec![];
        if let Some(v) = vip_level {
            params.push(("vipLevel", v));
        }
        if let Some(c) = currency {
            params.push(("currency", c));
        }
        self.get_public("/v5/spot-margin-trade/data", &params).await
    }
}
