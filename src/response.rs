use serde::{Deserialize, Serialize};

use crate::model::RouteSummary;

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponse<T> {
    pub code: usize,
    pub message: String,
    pub data: T,
    pub request_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRoutesResponseData {
    pub route_summary: RouteSummary,
    pub router_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputChangeLevel {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputChange {
    pub amount: String,
    pub percent: f64,
    pub level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildRouteResponseData {
    pub amount_in: String,
    pub amount_in_usd: String,
    pub amount_out: String,
    pub amount_out_usd: String,
    pub gas: String,
    pub gas_usd: String,
    pub output_change: OutputChange,
    pub data: String,
    pub router_address: String,
}
