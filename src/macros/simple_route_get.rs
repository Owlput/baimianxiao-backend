#[macro_export]
macro_rules! simple_get_route {
    ($route:ident,$sql:expr,$data_type:ty) => (
        async fn $route(
            axum::extract::Extension(pool): axum::extract::Extension<Pool<Postgres>>,
        ) -> Result<String, (axum::http::StatusCode, String)> {
            match sqlx::query_as::<_, $data_type>($sql)
            .fetch_all(&pool)
            .await
        {
            Ok(thumbs) => match serde_json::to_string(&thumbs) {
                Ok(json) => {
                    tracing::info!("successfully served route {}",stringify!($route));
                    Ok(json)
                }
                Err(e) => {
                    tracing::error!("{}", e);
                    Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
                }
            },
            Err(e) => {
                tracing::error!("{}", e);
                Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
        }
    );
}