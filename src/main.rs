// #![allow(unused)]
pub mod access_code;
mod db;
mod routes;
pub mod macros;

use routes::construct_router;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr).serve(construct_router(&db_pool).into_make_service());
    info!("Server started.");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
