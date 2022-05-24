#![allow(unused)]
pub mod access_code;
mod db;
mod routes;

use axum::http::{Method, HeaderValue};
use axum::routing::get;
use axum::{Extension, Router};
use routes::thumbs::thumb_data;
// use hyper::service::Service;
// use hyper::{Body, Request, Response, Server};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Database, Pool, Postgres};
use std::env;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_http::cors::CorsLayer;
use tower_service::Service;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr).serve(construct_router(&db_pool).into_make_service());
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn construct_router(pool: &Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/thumbs", get(thumb_data))
        .layer(Extension(pool.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
}

// struct Svc {
//     pool:Pool<Postgres>,
// }
// impl Service<Request<Body>> for Svc {
//     type Response = Response<Body>;
//     type Error = hyper::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
//     fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         fn mk_response(s: String) -> Result<Response<Body>, hyper::Error> {
//             Ok(Response::builder().body(Body::from(s)).unwrap())
//         }
//         let res = match req.uri(){
//                 "/api/baimianxiao/thumb"=>{

//                 }
//         };
//         Box::pin(async { res })
//     }
// }

// struct MakeSvc {
//     pool: Pool<Postgres>,
// }

// impl<T> Service<T> for MakeSvc {
//     type Response = Svc;
//     type Error = hyper::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

//     fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, _: T) -> Self::Future {
//         let pool = self.pool.clone();
//         let fut = async move {
//             Ok(Svc { pool })
//         };
//         Box::pin(fut)
//     }
// }
