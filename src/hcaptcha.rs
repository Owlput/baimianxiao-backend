use std::env;

use axum::body::HttpBody;
use hyper::{client::HttpConnector, Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use tracing::{error, info};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct HcaptchaResponse {
    pub success: bool,
    pub challenge_ts: String,
    hostname: String,
    credit: Option<bool>,
    #[serde(alias = r#"error-codes"#)]
    error_codes: Option<Vec<String>>,
    score: Option<f64>,
    #[serde(alias = r#"score-reason"#)]
    score_reason: Option<Vec<String>>,
}

async fn verify_hcaptcha<T>(
    hyper_client: hyper::Client<HttpsConnector<HttpConnector>>,
    token: &String,
) -> Result<(), ()> {
    let token_hash = xxhash_rust::xxh64::xxh64(token.as_bytes(), 2048);
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
            if res.body().size_hint().upper().unwrap_or(512) < 513 {
                let res_msg: HcaptchaResponse = match serde_json::from_slice::<HcaptchaResponse>(
                    &match hyper::body::to_bytes(res.into_body()).await {
                        Ok(b) => b,
                        Err(_) => todo!(),
                    },
                ) {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                };

                if res_msg.success {
                    info!("token {} verified with response:{:?}", token_hash, res_msg);
                    Ok(())
                } else{
                    Err(())
                }
            } else {
                Err(())
            }
        }
        Err(e) => {
            error!("Failed to verify hCaptcha: {} with error:{}", token_hash, e);
            Err(())
        }
    }
}
