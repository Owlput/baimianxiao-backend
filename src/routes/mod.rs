pub mod ptilopsis;


use std::sync::{atomic::AtomicU64, Arc};

use ptilopsis::api::ptilomsg::ptilomsg;
use axum::{
    http::{HeaderValue, Method},
    routing::*,
    Extension, Router,
};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use redis::aio::ConnectionManager;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;

use crate::{db::schemas::ThumbData, simple_get_route, routes::ptilopsis::api::baimianxiao::get_artwork};
use crate::routes::ptilopsis::*;

pub fn construct_router(
    pg_pool_read: &Pool<Postgres>,
    code: Arc<AtomicU64>,
    hyper_client: hyper::Client<HttpsConnector<HttpConnector>>,
    redis_client: ConnectionManager,
) -> Router {
    simple_get_route!(api_baimianxiao_thumbs, r#"SELECT * FROM "thumbData";"#, ThumbData);
    Router::new()
        .route("/api/baimianxiao/thumbData/all", get(api_baimianxiao_thumbs))
        .route("/api/ptilomsg", post(ptilomsg))
        .route("/api/baimianxiao/artwork/:id", get(api::baimianxiao::get_artwork))
        .layer(Extension(pg_pool_read.clone()))
        .layer(Extension(code))
        .layer(Extension(hyper_client))
        .layer(Extension(redis_client))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
}

// async fn test_tokenuse(code:Arc<AtomicU64>)->Result<String,(StatusCode,String)>{

// }
