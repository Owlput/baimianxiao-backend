pub mod artwork;
pub mod thumbs;

pub use artwork::get_artwork;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum LicenseType {
    CCBYNCND,
    CCBYNCSA,
    CC0,
    CUSTOM,
}

#[cfg(test)]
mod test{
    use serde::{Serialize, Deserialize};

    use super::LicenseType;

    #[derive(PartialEq,Eq,Debug,Serialize,Deserialize)]
    struct TestBody{
        license_type:LicenseType
    }
    #[test]
    fn serde_license_type(){
        let json = r#"
        {
            "license_type":"CC0"
        }
        "#;
        let res:TestBody = serde_json::from_str(json).unwrap();
        assert_eq!(res,TestBody{
            license_type:LicenseType::CC0
        });
    }
}