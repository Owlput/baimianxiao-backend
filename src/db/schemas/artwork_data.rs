use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, PartialEq, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "camelCase")]
pub struct ArtworkData {
    id: String,
    title: String,
    author_id: String,
    permit_id: String,
    tags: Vec<(String, i64)>,
    source_other: Vec<(String, String)>,
    license: String,
    time_origin: NaiveDateTime,
    time_this: NaiveDateTime,
    views: i64,
    active: bool,
}
impl TryFrom<String> for ArtworkData {
    type Error = String;
    fn try_from(val: String) -> Result<Self, Self::Error> {
        match serde_json::from_str::<ArtworkData>(val.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let target = ArtworkData {
            id: "testArtworkId".into(),
            title: "testArtworkTitle".into(),
            author_id: "testAuthorId".into(),
            permit_id: "testPermitId".into(),
            tags: vec![("tag1".into(), 123), ("tag2".into(), 456)],
            source_other: vec![("plat1".into(), "example.com".into())],
            license: "CC0".to_string(),
            time_origin: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            time_this: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            views: 3,
            active: false,
        };
        let json = r#"{
            "id": "testArtworkId",
            "title": "testArtworkTitle",
            "authorId": "testAuthorId",
            "permitId": "testPermitId",
            "tags": [
              ["tag1", 123],
              ["tag2", 456]
            ],
            "sourceOther": [["plat1", "example.com"]],
            "license": "CC0",
            "timeOrigin": "2015-09-05T23:56:04",
            "timeThis": "2015-09-05T23:56:04",
            "views": 3,
            "active": false
          }"#;
        assert_eq!(target, serde_json::from_str::<ArtworkData>(json).unwrap())
    }
}
