use serde::{Deserialize, Serialize};

use crate::models::crypto_loan::CollateralRatioTier;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoricalInterestRateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<InterestRateRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_borrow_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<PositionTierItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTierItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_tiers_ratio_list: Option<Vec<PositionTierRatio>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTierRatio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_limit: Option<String>,
    #[serde(rename = "positionMMR", skip_serializing_if = "Option::is_none")]
    pub position_mmr: Option<String>,
    #[serde(rename = "positionIMR", skip_serializing_if = "Option::is_none")]
    pub position_imr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTieredCollateralRatioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CollateralTierItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralTierItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_ratio_list: Option<Vec<CollateralRatioTier>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVipMarginDataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_coin_list: Option<Vec<VipCoinGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipCoinGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<VipCoinDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipCoinDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_borrow_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_collateral: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_borrowing_amount: Option<String>,
}
