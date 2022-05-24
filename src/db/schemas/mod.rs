use serde::{ser::SerializeTuple, Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub mod artwork_information;
pub mod author_data;
pub mod thumb;
