
#[derive(Clone, Debug, PartialEq, Eq,Hash)]
pub struct Version {
    version : String,
}
impl Version {
    pub fn new (version : String) -> Version{
        return Version{ version : version};
    }
}