use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Thumb {
    id: String,
    title: String,
    author_name: String,
    author_img: String,
    time: NaiveDateTime,
    views: i32,
    active: bool,
}
