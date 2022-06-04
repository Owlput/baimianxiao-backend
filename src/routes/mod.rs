pub mod api_ptilomsg;

use std::sync::{atomic::AtomicU64, Arc};

use api_ptilomsg::api_ptilomsg;
use axum::{
    http::{HeaderValue, Method},
    routing::*,
    Extension, Router,
};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;

use crate::{db::schemas::Thumb, simple_get_route};

pub fn construct_router(
    pool: &Pool<Postgres>,
    code: Arc<AtomicU64>,
    hyper_client: hyper::Client<HttpsConnector<HttpConnector>>,
) -> Router {
    simple_get_route!(api_baimianxiao_thumbs, "SELECT * FROM thumb_data;", Thumb);
    Router::new()
        .route("/api/baimianxiao/thumbs", get(api_baimianxiao_thumbs))
        .route("/api/ptilomsg", post(api_ptilomsg))
        .layer(Extension(pool.clone()))
        .layer(Extension(code))
        .layer(Extension(hyper_client))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
}

// async fn test_tokenuse(code:Arc<AtomicU64>)->Result<String,(StatusCode,String)>{

// }
