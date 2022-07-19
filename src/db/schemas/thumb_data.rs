use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "camelCase")]
pub struct ThumbData {
    id: String,
    title: String,
    author_name: String,
    author_id: String,
    time: NaiveDateTime,
    views: i32,
    active: bool,
}

impl TryFrom<String> for ThumbData {
    type Error = String;
    fn try_from(val: String) -> Result<Self, Self::Error> {
        match serde_json::from_str::<ThumbData>(val.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;

    use super::ThumbData;

    #[test]
    fn deserialize() {
        let target = ThumbData {
            id: "testArtworkId".into(),
            title: "testArtworkTitle".into(),
            author_name: "testAuthorName".into(),
            author_id: "testAuthorId".into(),
            time: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            views: 123,
            active: false,
        };
        let json = r#"{
            "id":"testArtworkId",
            "title":"testArtworkTitle",
            "authorName":"testAuthorName",
            "authorId":"testAuthorId",
            "time":"2015-09-05T23:56:04",
            "views":123,
            "active":false
        }"#;
        assert_eq!(target, serde_json::from_str::<ThumbData>(json).unwrap());
    }
}
