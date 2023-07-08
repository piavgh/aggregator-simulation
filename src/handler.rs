use axum::{extract::Query, response::IntoResponse, Json};
use std::{env, error::Error};

use crate::{
    params::{BuildRouteParams, BuildRouteParamsWithoutChain, GetRouteEncodeParams},
    response::{BuildRouteResponseData, GenericResponse, GetRoutesResponseData},
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

    let get_routes_response_data = get_routes(&params).await.unwrap_or_default();

    let build_route_params = BuildRouteParams {
        chain: params.chain.to_owned(),
        route_summary: get_routes_response_data.route_summary,
        recipient: params.recipient.to_owned(),
    };

    let build_route_response_data = build_route(build_route_params).await.unwrap_or_default();

    let json_response = serde_json::json!({
        "status": "success",
        "message": "Encode route handler",
        "data": build_route_response_data.data,
    });

    Json(json_response)
}

async fn get_routes(
    params: &GetRouteEncodeParams,
) -> Result<GetRoutesResponseData, Box<dyn Error>> {
    let base_url = env::var("AGGREGATOR_API_URL").unwrap_or_default();
    let api_url = format!("{}/{}/api/v1/routes", base_url, params.chain);

    let save_gas = &params.save_gas.to_owned().to_string();

    let query = vec![
        ("tokenIn", &params.token_in),
        ("tokenOut", &params.token_out),
        ("amountIn", &params.amount_in),
        ("saveGas", save_gas),
    ];

    let client = reqwest::Client::new();
    let http_response = client.get(api_url).query(&query).send().await?;

    match http_response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an GetRoutesReponse
            match http_response
                .json::<GenericResponse<GetRoutesResponseData>>()
                .await
            {
                Ok(get_routes_response) => return Ok(get_routes_response.data),
                Err(err) => {
                    println!(
                        "Hm, the response didn't match the shape we expected: {:?}",
                        err
                    );

                    return Err(Box::new(err));
                }
            };
        }
        other => {
            eprintln!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(GetRoutesResponseData::default())
}

async fn build_route(params: BuildRouteParams) -> Result<BuildRouteResponseData, Box<dyn Error>> {
    let chain = params.chain;
    let route_summary = params.route_summary;
    // let sender = params.sender;
    let recipient = params.recipient;
    // let deadline = params.deadline;
    // let slippage_tolerance = params.slippage_tolerance;
    // let referral = params.referral;
    // let source = params.source;
    // let permit = params.permit;

    let base_url = env::var("AGGREGATOR_API_URL").unwrap_or_default();
    let api_url = format!("{}/{}/api/v1/route/build", base_url, chain);

    let p = BuildRouteParamsWithoutChain {
        route_summary,
        // sender,
        recipient,
        // deadline,
        // slippage_tolerance,
        // referral,
        // source,
        // permit,
    };

    let client = reqwest::Client::new();
    let http_response = client.post(api_url).json(&p).send().await?;

    match http_response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an GetRoutesReponse
            match http_response
                .json::<GenericResponse<BuildRouteResponseData>>()
                .await
            {
                Ok(build_route_response) => return Ok(build_route_response.data),
                Err(err) => {
                    println!(
                        "Hm, the response didn't match the shape we expected: {:?}",
                        err
                    );

                    return Err(Box::new(err));
                }
            };
        }
        other => {
            eprintln!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(BuildRouteResponseData::default())
}
