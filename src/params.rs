use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt, PickFirst};

use crate::model::RouteSummary;

#[serde_as]
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRouteEncodeParams {
    pub chain: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    #[serde_as(as = "PickFirst<(_,BoolFromInt)>")]
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

pub struct SimulateTenderlyParams {
    /* Simulation Configuration */
    pub save: bool,
    pub save_if_fails: bool,
    pub simulation_type: String,

    network_id: String,

    /* Standard EVM Transaction object */
    from: String,
    to: String,
    input: String,
    gas: u64,
    gas_price: u64,
    value: u64,
}
