// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthchecker", get(health_cheacker_handler));

    println!("🚀 Server started successfully");
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_cheacker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_res = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_res)
}
