use serde::{Deserialize, Serialize};

use crate::model::RouteSummary;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRouteEncodeParams {
    pub chain: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub save_gas: bool,
    pub recipient: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuildRouteParams {
    pub chain: String,
    pub route_summary: RouteSummary,
    // sender address of sender wallet
    // pub sender: Option<String>,
    // recipient address of recipient wallet
    pub recipient: String,
    // pub deadline: Option<u64>,
    // pub slippage_tolerance: Option<u64>,
    // pub referral: Option<String>,
    // pub source: Option<String>,
    // permit allows user to swap without approving token beforehand
    // pub permit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuildRouteParamsWithoutChain {
    pub route_summary: RouteSummary,
    // sender address of sender wallet
    // pub sender: Option<String>,
    // recipient address of recipient wallet
    pub recipient: String,
    // pub deadline: Option<u64>,
    // pub slippage_tolerance: Option<u64>,
    // pub referral: Option<String>,
    // pub source: Option<String>,
    // permit allows user to swap without approving token beforehand
    // pub permit: Option<String>,
}
