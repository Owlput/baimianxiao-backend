use std::env;

use axum::{body::HttpBody, extract::Form, Extension};
use hyper::{client::HttpConnector, Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use tracing::{error, info};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Input {
    msg: String,
    email: String,
    #[serde(alias = r#"g-recaptcha-response"#)] //use alias and string literals to deal with "-"
    g_recaptcha_response: String,
    #[serde(alias = r#"h-captcha-response"#)]
    h_captcha_response: String,
}

pub async fn api_ptilomsg(
    Extension(hyper_client): Extension<hyper::Client<HttpsConnector<HttpConnector>>>,
    Form(input): Form<Input>,
) {
    // info!("request from user:{:?}", input); //for debug purpose
    match hyper_client
        .request(
            Request::builder()
                .method(Method::POST)
                .uri("https://hcaptcha.com/siteverify")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(format!(
                    "response={}&secret={}",
                    &input.h_captcha_response,
                    env::var("HCAPTCHA_SECRET").expect("hCaptcha secret not set")
                )))
                .unwrap(),
        )
        .await
    {
        Ok(response) => {
            let response_content_length = match response.body().size_hint().upper() {
                Some(v) => v,
                None => 512,
            };

            if response_content_length < 513 {
                info!(
                    "verified token with response:{:?}",
                    String::from_utf8(
                        hyper::body::to_bytes(response.into_body())
                            .await
                            .unwrap()
                            .to_ascii_lowercase()
                    )
                )
            }
        }
        Err(err) => {
            error!("failed to verify token:{}", err)
        }
    };
}
