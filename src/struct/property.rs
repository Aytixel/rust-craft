use packet::{DeserializeStruct, SerializeStruct};

#[derive(Debug, Clone, DeserializeStruct, SerializeStruct)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    #[option(is_signed)]
    pub signature: Option<String>,
}
