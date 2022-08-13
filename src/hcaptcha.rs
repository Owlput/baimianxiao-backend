use std::env;

use axum::body::HttpBody;
use hyper::{client::HttpConnector, Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use tracing::{info, warn};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct HcaptchaResponse {
    pub success: bool,
    pub challenge_ts: Option<String>,
    hostname: Option<String>,
    credit: Option<bool>,
    #[serde(alias = "error-codes")]
    error_codes: Option<Vec<String>>,
    score: Option<f64>,
    #[serde(alias = "score-reason")]
    score_reason: Option<Vec<String>>,
}

pub async fn verify_hcaptcha(
    hyper_client: hyper::Client<HttpsConnector<HttpConnector>>,
    token: &String,
) -> Result<(), ()> {
    let token_hash = xxhash_rust::xxh64::xxh64(token.as_bytes(), 255);
    info!("token received with hash: {}",token_hash);
    match hyper_client
        .request(
            Request::builder()
                .method(Method::POST)
                .uri("https://hcaptcha.com/siteverify")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(format!(
                    "response={}&secret={}",
                    token,
                    env::var("HCAPTCHA_SECRET").expect("hCaptcha secret not set")
                )))
                .unwrap(),
        )
        .await
    {
        Ok(res) => {
            if res.body().size_hint().upper().unwrap_or(513) < 512 {
                match serde_json::from_slice::<HcaptchaResponse>(
                    &match hyper::body::to_bytes(res.into_body()).await {
                        Ok(b) => b,
                        Err(e) => {
                            warn!("Failed to parse verification response body for {} with {}",token_hash,e);
                            return Err(())
                        },
                    },
                ) {
                    Ok(val) => {
                        if val.success {
                            return Ok(())
                        } else {
                            info!("Failed to verify {}",token_hash);
                            return Err(())
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse response from hCaptcha server for {} with {}",token_hash,e);
                        return Err(())
                    },
                };
            } else {
                warn!("Failed to verify {}: Response from hCaptcha server is longer than expected.",token_hash);
                Err(())
            }
        }
        Err(e) => {
            warn!("Failed to verify token: {} with error:{}", token_hash, e);
            Err(())
        }
    }
}
