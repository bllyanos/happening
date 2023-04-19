mod auth;
mod core;
mod users;
mod workspaces;

use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    println!("connecting to database..");
    let db = core::database::connect()
        .await
        .expect("cannot connect to database");
    println!("connected  to database..");

    let auth_router = auth::router::create(db.clone());
    let router: Router = Router::new()
        .route("/", get(handle_index))
        .merge(auth_router);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("RUNNING ON http://{}", addr);

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
