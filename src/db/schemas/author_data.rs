use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, PartialEq, Debug, FromRow)]
pub struct AuthorData {
    id: String,
    name: String,
    contact: Vec<(String, String)>,
    recent: Vec<String>,
}
impl TryFrom<String> for AuthorData {
    type Error = String;
    fn try_from(val: String) -> Result<Self, Self::Error> {
        match serde_json::from_str::<AuthorData>(val.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::AuthorData;

    #[test]
    fn deserialize() {
        let target = AuthorData {
            id: "testAuthorId".into(),
            name: "testAuthorName".into(),
            contact: vec![("plat1".into(), "example.com".into())],
            recent: vec!["artworkId1".into(), "artworkId2".into()],
        };
        let json = r#"
        {
            "id":"testAuthorId",
            "name":"testAuthorName",
            "contact":[["plat1","example.com"]],
            "recent":["artworkId1","artworkId2"]
        }
        "#;
        assert_eq!(target, serde_json::from_str::<AuthorData>(json).unwrap())
    }
}
