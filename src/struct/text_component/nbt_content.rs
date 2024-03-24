use serde::{Deserialize, Serialize};

use super::TextComponent;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NBTContent {
    pub nbt: String,
    #[serde(default)]
    pub interpret: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<Box<TextComponent>>,
    pub block: String,
    pub entity: String,
    pub storage: String,
}
