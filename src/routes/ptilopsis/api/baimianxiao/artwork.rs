use axum::{extract::Path, Extension};
use hyper::{Body, Request, Response, StatusCode};
use redis::{aio::ConnectionManager, AsyncCommands};
use sqlx::{FromRow, Pool, Postgres, QueryBuilder};
use tracing::{debug, error, info, warn};
use url::Url;

use crate::{db::schemas::ArtworkData, hcaptcha::check_hcaptcha,helpers::response::*};

pub async fn get_artwork(
    Path(path): Path<String>,
    Extension(mut redis_client): Extension<Option<ConnectionManager>>,
    Extension(mut pg_pool): Extension<Option<Pool<Postgres>>>,
    request: Request<Body>,
) -> Response<Body> {
    let mut redis_client = match redis_client {
        Some(client) => client,
        None => {
            warn!("Redis service not connected.");
            return internal_server_error();
        }
    };
    let origin = request
        .headers()
        .get("x-real-ip")
        .unwrap()
        .to_str()
        .unwrap_or("0.0.0.0")
        .to_string();
    info!("accepted request from {}", origin);
    match check_hcaptcha(&mut redis_client, &origin).await {
        Ok(()) => (),
        Err(e) => return e,
    }
    let target_work = match helpers::get_work_id(&path, &origin) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let mut pg_pool = match pg_pool {
        Some(pool) => pool,
        None => {
            warn!("Posgresql not connected");
            return internal_server_error()
        }
    };
    let mut query = QueryBuilder::new("SELECT");
    query
        .push_bind("*")
        .push(r#"FROM "artworkData""#)
        .push("WITH")
        .push_bind(format!("id = {}", target_work));
    let query_string = query.sql().to_string();
    let work: ArtworkData = match query.build_query_as().fetch_optional(&pg_pool).await {
        Ok(v) => match v {
            Some(v) => v,
            None => {
                debug!("Work {} not found in database.", target_work);
                return not_found()
            }
        },
        Err(e) => {
            info!(
                "Failed to perform query {} from {}: {}",
                query_string, origin, e
            );
            return internal_server_error()
        }
    };
    Response::builder()
        .status(200)
        .body(Body::from(serde_json::to_string(&work).unwrap()))
        .unwrap()
}

pub async fn put_artwork() {}

mod helpers {
    use hyper::{Body, Response};
    use tracing::{debug, info};
    use url::Url;

    pub fn get_work_id(path: &String, origin: &String) -> Result<i64, Response<Body>> {
        match Url::parse(&path).unwrap().query_pairs().next() {
            Some(q) => {
                if q.0 == "work" {
                    match q.1.parse::<i64>() {
                        Ok(v) => return Ok(v),
                        Err(e) => {
                            info!(
                                "Failed to resolve work id from {} on path {}: {}",
                                origin, path, e
                            );
                            return Err(Response::builder()
                                .status(400)
                                .body(Body::from("Bad Request: Illegal work ID"))
                                .unwrap());
                        }
                    }
                } else {
                    debug!(
                        r#"Bad request from {} on path {} : "work" is not found as the first query field "#,
                        origin, path
                    );
                    return Err(Response::builder()
                        .status(400)
                        .body(Body::from(
                            r#"Bad request: Unable to extract field "work" from the first query."#,
                        ))
                        .unwrap());
                }
            }
            None => {
                debug!(
                    r#"Bad request from {} on path {} : no query field is specified, expected "work" as the first query field "#,
                    origin, path
                );
                return Err(Response::builder().status(400).body(Body::from(r#"Bad request: no query field was found, expected "work" as the first query field "#)).unwrap());
            }
        };
    }
}
