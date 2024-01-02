use serde_derive::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq,Hash, Deserialize,Serialize)]
pub struct Version {
    pub version : String,
}
impl Version {
    pub fn new (version : String) -> Version{
        return Version{ version : version};
    }
    pub fn version(&self) -> &String {
        return &self.version;
    }
}