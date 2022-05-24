use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{http::{Error, StatusCode}, Extension};
use serde::Serialize;
use serde_json::Serializer;
use sqlx::{Pool, Postgres};
use tower_service::Service;
use tracing::{error, info};

use crate::db::{schemas::thumb::Thumb, internal_error};

pub struct ServeArtworkThumbs {
    thumbs: Vec<Thumb>,
}
// impl Service<Request<Body>> for ServeArtworkThumbs {
//     type Response = Response<Body>;
//     type Error = hyper::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
//     fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, _req: Request<Body>) -> Self::Future {
//         let thumbs = self.thumbs.clone();
//         Box::pin(async move {
//             Ok(Response::builder()
//                 .body(Body::from(serde_json::to_string(&thumbs).unwrap()))
//                 .unwrap())
//         })
//     }
// }

// pub struct MakeArtworkThumbService {
//     pool: Pool<Postgres>,
// }
// impl MakeArtworkThumbService {
//     pub fn new(pool: Pool<Postgres>) -> Self {
//         MakeArtworkThumbService { pool }
//     }
// }

pub async fn thumb_data(Extension(pool): Extension<Pool<Postgres>>) -> Result<String, (StatusCode, String)> {
    match sqlx::query_as::<_, Thumb>("SELECT * FROM thumb_data LIMIT 5")
        .fetch_all(&pool)
        .await
    {
        Ok(thumbs) => match serde_json::to_string(&thumbs) {
            Ok(json) => Ok(json),
            Err(e) => {
                error!("{}",e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }},
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
