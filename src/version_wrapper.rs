use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use semver::Version;

// 自定义结构体，包含一个 Version 类型的字段
#[derive(Debug,Clone,PartialEq, Eq,Hash)]
pub struct VersionWrapper {
    pub version: Version,
}
impl VersionWrapper {
    pub fn new (v : Version) -> VersionWrapper {
        VersionWrapper { version: v }
    }
    
}
// 
impl Serialize for VersionWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 在这里编写序列化逻辑
        serializer.serialize_str(&self.version.to_string())
    }
}
// 实现 Deserialize trait
impl<'de> Deserialize<'de> for VersionWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 在这里编写反序列化逻辑
        let s = String::deserialize(deserializer)?;
        match semver::Version::from_str(&s) {
            Ok(v) => Ok(VersionWrapper { version: v }),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
        
    }
}