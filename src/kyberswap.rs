use reqwest::header::USER_AGENT;
use std::{env, error::Error};

use crate::{
    header,
    params::{BuildRouteParams, BuildRouteParamsWithoutChain, GetRouteEncodeParams},
    response::{BuildRouteResponseData, GenericResponse, GetRoutesResponseData},
};

pub struct Kyberswap {
    client: reqwest::Client,
}

impl Kyberswap {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_routes(
        &self,
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

        let http_response = self
            .client
            .get(api_url)
            .query(&query)
            .header(USER_AGENT, header::USER_AGENT_VALUE)
            .send()
            .await?;

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
                eprintln!("fail to get routes: {:?}", other);
            }
        };

        Ok(GetRoutesResponseData::default())
    }

    pub async fn build_route(
        &self,
        params: BuildRouteParams,
    ) -> Result<BuildRouteResponseData, Box<dyn Error>> {
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

        println!("api_url : {}", api_url);

        let http_response = self
            .client
            .post(api_url)
            .json(&p)
            .header(USER_AGENT, header::USER_AGENT_VALUE)
            .send()
            .await?;

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
                eprintln!("fail to build route: {:?}", other);
            }
        };

        Ok(BuildRouteResponseData::default())
    }
}
