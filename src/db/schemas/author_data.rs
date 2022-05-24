use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthorData {
    id: String,
    name: String,
    permit: String,
    contact: Vec<(String,String)>,
}
