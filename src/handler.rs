use axum::{extract::Query, response::IntoResponse, Json};

use crate::{
    kyberswap::Kyberswap,
    params::{BuildRouteParams, GetRouteEncodeParams},
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Health check ok";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn get_route_encode_handler(
    opts: Option<Query<GetRouteEncodeParams>>,
) -> impl IntoResponse {
    let Query(params) = opts.unwrap_or_default();

    let kyberswap = Kyberswap::new();
    println!("params : {:?}", params);

    let get_routes_response_data = kyberswap.get_routes(&params).await.unwrap_or_default();

    let build_route_params = BuildRouteParams {
        chain: params.chain.to_owned(),
        route_summary: get_routes_response_data.route_summary,
        recipient: params.recipient.to_owned(),
    };

    let build_route_response_data = kyberswap
        .build_route(build_route_params)
        .await
        .unwrap_or_default();

    let json_response = serde_json::json!({
        "status": "success",
        "message": "Encode route handler",
        "data": build_route_response_data.data,
    });

    Json(json_response)
}
