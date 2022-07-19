use axum::{extract::Form, Extension};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use tracing::info;

use crate::hcaptcha::verify_hcaptcha;

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

pub async fn ptilomsg(
    Extension(hyper_client): Extension<hyper::Client<HttpsConnector<HttpConnector>>>,
    Form(input): Form<Input>,
) {
    if let Ok(()) = verify_hcaptcha(hyper_client, &input.h_captcha_response).await {
        info!("Message received:{} from {}", input.msg, input.email);
    }
}
