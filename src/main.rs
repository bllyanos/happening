use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

mod core;
mod users;

#[tokio::main]
async fn main() {
    let db = core::database::connect();
    // .await
    // .expect("cannot connect to database");
    let router: Router = Router::new().route("/", get(handle_index));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn handle_index() -> Json<Value> {
    let app_name = "happening-api";
    let server_time = chrono::offset::Utc::now();
    Json(json!({ "app_name" : app_name, "server_time": server_time }))
}
