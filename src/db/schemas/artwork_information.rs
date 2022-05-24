use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Debug)]
pub enum ArtworkLicense {
    Custom,
    CCBYNCND4,
    CC0,
    Unrecognized,
}
impl From<String> for ArtworkLicense {
    fn from(val: String) -> Self {
        match val.as_str() {
            "Custom" => Self::Custom,
            "CCBYNCND4" => Self::CCBYNCND4,
            "CC0" => Self::CC0,
            _ => Self::Unrecognized,
        }
    }
}
impl Serialize for ArtworkLicense {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ArtworkLicense::Custom => serializer.serialize_str("Custom"),
            ArtworkLicense::CCBYNCND4 => serializer.serialize_str("CCBYNCND4"),
            ArtworkLicense::CC0 => serializer.serialize_str("CC0"),
            ArtworkLicense::Unrecognized => Err(serde::ser::Error::custom("Unrecognized Value")),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ArtworkInformation {
    id: String,
    title: String,
    author_id: String,
    tags: Vec<(String, u64)>,
    source_this: String,
    source_other: Vec<(String, String)>,
    license: ArtworkLicense,
}
impl TryFrom<String> for ArtworkInformation {
    type Error = String;
    fn try_from(val: String) -> Result<Self, Self::Error> {
        match serde_json::from_str::<ArtworkInformation>(val.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn try_parse_artwork_information_from_string() {
        let artwork_information_string:String = r#"{
            "id":"testid1234",
            "title":"testTitle",
            "author_id":"testAuthorId",
            "tags":[["testTag1",114514],["testTag2",1919],["testTag3",810]],
            "source_this":"https://example.com/randomart/testid1234",
            "source_other":[["source1","http://example.com/source1"],["source2","http://example.com/source2"],["source3","http://example.com/source3"]],
            "license":"CC0"
        }"#.into();
        let acutal = ArtworkInformation {
            id: "testid1234".into(),
            title: "testTitle".into(),
            author_id: "testAuthorId".into(),
            tags: vec![
                ("testTag1".into(), 114514),
                ("testTag2".into(), 1919),
                ("testTag3".into(), 810),
            ],
            source_this: "https://example.com/randomart/testid1234".into(),
            source_other: vec![
                ("source1".into(), "http://example.com/source1".into()),
                ("source2".into(), "http://example.com/source2".into()),
                ("source3".into(), "http://example.com/source3".into()),
            ],
            license: ArtworkLicense::CC0,
        };
        let converted: ArtworkInformation = artwork_information_string.try_into().unwrap();
        debug_assert_eq!(acutal, converted)
    }
    #[test]
    fn try_seralize_artwork_information() {
        let actual:String = r#"{"id":"testid1234","title":"testTitle","author_id":"testAuthorId","tags":[["testTag1",114514],["testTag2",1919],["testTag3",810]],"source_this":"https://example.com/randomart/testid1234","source_other":[["source1","http://example.com/source1"],["source2","http://example.com/source2"],["source3","http://example.com/source3"]],"license":"CC0"}"#.into();
        let input = ArtworkInformation {
            id: "testid1234".into(),
            title: "testTitle".into(),
            author_id: "testAuthorId".into(),
            tags: vec![
                ("testTag1".into(), 114514),
                ("testTag2".into(), 1919),
                ("testTag3".into(), 810),
            ],
            source_this: "https://example.com/randomart/testid1234".into(),
            source_other: vec![
                ("source1".into(), "http://example.com/source1".into()),
                ("source2".into(), "http://example.com/source2".into()),
                ("source3".into(), "http://example.com/source3".into()),
            ],
            license: ArtworkLicense::CC0,
        };
        let seralized = serde_json::to_string(&input).unwrap();
        assert_eq!(actual, seralized)
    }
}
