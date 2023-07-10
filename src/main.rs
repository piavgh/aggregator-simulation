mod handler;
mod model;
mod params;
mod response;
mod route;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenvy::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = create_router().layer(cors);

    println!("🚀 Server started successfully");
    Ok(router.into())
}
