#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::dbg_macro)]

use axum::{Json, Router, routing::post};
use core::search_papers;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/search", post(search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn search(Json(payload): Json<SearchRequest>) -> Json<serde_json::Value> {
    match search_papers(&payload.query).await {
        Ok(results) => Json(serde_json::json!({ "results": results })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}
