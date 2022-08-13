use axum::{Extension, extract::Json};
use hyper::{client::HttpConnector, Response, Body};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use tracing::info;

use crate::hcaptcha::verify_hcaptcha;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Input {
    msg: String,
    email: Option<String>,
    #[serde(alias = r#"captcha-token"#)]
    captcha_token: String,
}

pub async fn ptilomsg(
    Extension(hyper_client): Extension<hyper::Client<HttpsConnector<HttpConnector>>>,
    Json(input): Json<Input>,
)->Response<Body> {
    if let Ok(()) = verify_hcaptcha(hyper_client, &input.captcha_token).await {
        info!("Message received:{} from {:#?}", input.msg, input.email);
        return Response::builder().header("Access-Control-Allow-Origin", "localhost:5500").header("Content-Type", "application/json").status(200).body(Body::from(r#"{"success":1}"#)).unwrap()
    
    }else{
        return Response::builder().header("Access-Control-Allow-Origin", "localhost:5500").status(401).body(Body::empty()).unwrap()
    }
}
