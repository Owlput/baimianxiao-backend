use axum::{extract::Path, Extension};
use hyper::{Body, Request, StatusCode};
use redis::{aio::ConnectionManager, AsyncCommands};
use tracing::info;
use url::Url;

pub async fn get_artwork(
    Path(path): Path<String>,
    Extension(mut redis_client): Extension<ConnectionManager>,
    request: Request<Body>,
) -> Result<String, (axum::http::StatusCode, String)> {
    
    match redis_client
            .get::<String, String>(format!("{}",request.headers()))
            .await{
                Ok(_) => {},
                Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR,e.to_string())),
            }
            
    let target_work = match Url::parse(&path)
        .unwrap()
        .query_pairs()
        .next()
    {
        Some(q) => {
            if q.0 == "work" {
                q.1.to_string()
            } else {
                return Err((StatusCode::BAD_REQUEST, "Bad Request".into()));
            }
        }
        None => return Err((StatusCode::BAD_REQUEST, "Bad Request".into())),
    };
    Ok("Ok".into())
}

pub async fn put_artwork(

) {

}
