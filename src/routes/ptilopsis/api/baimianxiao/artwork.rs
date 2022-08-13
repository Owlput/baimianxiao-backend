use axum::{extract::Path, Extension};
use hyper::{Body, Request, Response, StatusCode};
use redis::{aio::ConnectionManager, AsyncCommands};
use sqlx::{Pool, Postgres};
use tracing::{debug, error, info, warn};
use url::Url;

use crate::db::schemas::ArtworkData;

pub async fn get_artwork(
    Path(path): Path<String>,
    Extension(mut redis_client): Extension<Option<ConnectionManager>>,
    Extension(mut pg_pool): Extension<Option<Pool<Postgres>>>,
    request: Request<Body>,
) -> Response<Body> {
    let mut redis_client = match redis_client {
        Some(client) => client,
        None => {
            return Response::builder()
                .status(500)
                .body(Body::from("Redis not connected"))
                .unwrap();
        }
    };
    let origin = request
        .headers()
        .get("x-real-ip")
        .unwrap()
        .to_str()
        .unwrap_or("0.0.0.0");
    info!("accepted request from {}", origin);
    match redis_client
        .get::<String, String>(format!("{:#?}/hcaptcha_credit", origin))
        // Check whether the client has completed CAPTCHA.
        .await
    {
        Ok(result) => {
            debug!("Successfully verify CAPTCHA status for {}", origin);
            if result.parse::<u8>().unwrap_or(0) <= 0 {
                return Response::builder()
                    .status(302)
                    .header("Location", "/hcaptcha")
                    .body(Body::empty())
                    .unwrap();
            }
        }
        Err(e) => {
            warn!(
                "Failed to verify CAPTCHA status with error from redis: {}",
                e
            );
            return Response::builder()
                .status(500)
                .body(Body::from("Failed to verify your hCAPTCHA status."))
                .unwrap();
        }
    }
    let target_work = match Url::parse(&path).unwrap().query_pairs().next() {
        Some(q) => {
            if q.0 == "work" {
                if q.1.len() != 8 {
                    return Response::builder()
                        .status(400)
                        .body(Body::from("Bad Request: Illegal work ID."))
                        .unwrap();
                }
                q.1.to_string()
            } else {
                debug!(
                    r#"Bad request from {} on path {} : "work" is not found as the first query field "#,
                    origin, path
                );
                return Response::builder()
                    .status(400)
                    .body(Body::from(
                        r#"Bad request: Unable to extract field "work" from the first query."#,
                    ))
                    .unwrap();
            }
        }
        None => {
            debug!(
                r#"Bad request from {} on path {} : no query field is specified, expected "work" as the first query field "#,
                origin, path
            );
            return Response::builder().status(400).body(Body::from(r#"Bad request: no query field was found, expected "work" as the first query field "#)).unwrap();
        }
    };
    let mut pg_pool = match pg_pool {
        Some(pool) => pool,
        None => {
            return Response::builder()
                .status(500)
                .body(Body::from("Posgresql not connected"))
                .unwrap();
        }
    };
    let work_data = match sqlx::query_as::<_, ArtworkData>("SELECT")
        .fetch_one(&pg_pool)
        .await
    {
        Ok(data) => {
            debug!(r#"artwork `{}` found in database "#, target_work);
            data
        }
        Err(e) => {
            info!(
                r#"Failed to query for artwork `{}` with error from sqlx: {}"#,
                target_work, e
            );
            return Response::builder()
                .status(404)
                .body(Body::from(
                    "Work specified was not found or is temporarily not available ",
                ))
                .unwrap();
        }
    };
    Response::builder()
        .status(200)
        .body(Body::from(serde_json::to_string(&work_data).unwrap()))
        .unwrap()
}

pub async fn put_artwork() {}
