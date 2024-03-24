use serde::{Deserialize, Serialize};

use super::TextComponent;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "action", content = "value")]
pub enum HoverEvent {
    #[serde(rename = "show_text")]
    ShowText(Box<TextComponent>),
    #[serde(rename = "show_item")]
    ShowItem(String),
    #[serde(rename = "show_entity")]
    ShowEntity(String),
}
