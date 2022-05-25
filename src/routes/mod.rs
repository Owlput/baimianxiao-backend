use axum::{
    http::{HeaderValue, Method},
    routing::*,
    Extension, Router,
};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;

use crate::{simple_get_route, db::schemas::Thumb};

pub fn construct_router(pool: &Pool<Postgres>) -> Router {
    simple_get_route!(api_baimianxiao_thumbs, "SELECT * FROM thumb_data;", Thumb);
    Router::new()
        .route("/api/baimianxiao/thumbs", get(api_baimianxiao_thumbs))
        .layer(Extension(pool.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
}

