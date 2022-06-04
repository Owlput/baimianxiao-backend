// #![allow(unused)]
pub mod access_code;
mod db;
mod routes;
pub mod macros;
pub mod hcaptcha;

use hyper_tls::HttpsConnector;
use routes::construct_router;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use tracing::info;

use crate::access_code::setup_code;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let access_code = Arc::new(AtomicU64::new(0));
    let hyper_client = hyper::Client::builder().build::<_,hyper::Body>(HttpsConnector::new());
    setup_code(access_code.clone());
    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr).serve(construct_router(&db_pool,access_code,hyper_client).into_make_service());
    info!("Server started.");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
