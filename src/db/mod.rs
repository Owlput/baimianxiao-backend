use axum::http::StatusCode;

pub mod schemas;
pub mod postgres;

pub fn internal_error<E>(err:E) ->(StatusCode,String)
where E:std::error::Error
{
    (StatusCode::INTERNAL_SERVER_ERROR,err.to_string())
}