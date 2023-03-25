use fastnbt::value::Value;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    properties: Option<Value>,
}
