use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Result {
    count: Option<u32>,
    #[serde(alias = "tag")]
    item: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResultVariant {
    Item(String),
    Result(Result),
}
