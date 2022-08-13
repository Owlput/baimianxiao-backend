#[macro_export]
macro_rules! simple_get_route {
    ($route:ident,$sql:expr,$data_type:ty) => {
        async fn $route(
            axum::extract::Extension(pool): axum::extract::Extension<Option<Pool<Postgres>>>,
        ) -> hyper::Response<hyper::Body> {
            let mut pool = match pool {
                Some(pool) => pool,
                None => {
                    return hyper::Response::builder()
                        .status(500)
                        .body(hyper::Body::from("Postgresql not connected"))
                        .unwrap();
                }
            };
            match sqlx::query_as::<_, $data_type>($sql).fetch_all(&pool).await {
                Ok(thumbs) => match serde_json::to_string(&thumbs) {
                    Ok(json) => {
                        tracing::debug!("successfully served route {}", stringify!($route));
                        hyper::Response::builder().status(200).body(hyper::Body::from(json)).unwrap()
                    }
                    Err(e) => {
                        tracing::info!(
                            "failed to serve route {} with error {}",
                            stringify!($route),
                            e
                        );
                        hyper::Response::builder().status(500).body(hyper::Body::from("unknown error")).unwrap()
                    }
                },
                Err(e) => {
                    tracing::info!(
                        "failed to serve route {} with error from sqlx: {}",
                        stringify!($route),
                        e
                    );
                    hyper::Response::builder().status(500).body(hyper::Body::from("unknown error")).unwrap()
                }
            }
        }
    };
}
