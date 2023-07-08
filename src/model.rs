use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Swap {
    pub pool: String,
    pub token_in: String,
    pub token_out: String,
    pub limit_return_amount: String,
    pub swap_amount: String,
    pub amount_out: String,
    pub exchange: String,
    pub pool_length: usize,
    pub pool_type: String,
    // pub pool_extra: Option<HashMap<String, String>>,
    pub extra: Option<HashMap<String, String>>,
}
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtraFee {
    pub fee_amount: String,
    pub charge_fee_by: String,
    pub is_in_bps: bool,
    pub fee_receiver: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteSummary {
    pub token_in: String,
    pub amount_in: String,
    pub amount_in_usd: String,
    pub token_in_market_price_available: bool,

    pub token_out: String,
    pub amount_out: String,
    pub amount_out_usd: String,
    pub token_out_market_price_available: bool,

    pub gas: String,
    pub gas_price: String,
    pub gas_usd: String,

    pub extra_fee: ExtraFee,
    pub route: Vec<Vec<Swap>>,
}
