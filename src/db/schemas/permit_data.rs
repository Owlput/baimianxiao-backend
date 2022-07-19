use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct PermitData {
    id: String,
    source: Vec<String>,
}
impl TryFrom<String> for PermitData {
    type Error = String;
    fn try_from(val: String) -> Result<Self, Self::Error> {
        match serde_json::from_str::<PermitData>(val.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::PermitData;

    #[test]
    fn deserialize() {
        let target = PermitData {
            id: "testPermitId".into(),
            source: vec!["1.jpg".into(), "2.jpg".into()],
        };
        let json = r#"{
            "id":"testPermitId",
            "source":["1.jpg","2.jpg"]
        }"#;
        assert_eq!(target, serde_json::from_str::<PermitData>(json).unwrap())
    }
}
