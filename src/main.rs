// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

mod handler;
mod model;
mod respose;
mod route;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;
use tracing::{debug, info, instrument};

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("simple_api_rust_axum=trace")
        .init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    debug!("Creating router (with CORS layer: {:#?})", cors);
    let app = create_router().layer(cors);

    info!("🚀 Starting server at localhost:3000 🚀");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
