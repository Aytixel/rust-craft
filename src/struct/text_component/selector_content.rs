use serde::{Deserialize, Serialize};

use super::TextComponent;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SelectorContent {
    pub selector: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<Box<TextComponent>>,
}

impl SelectorContent {
    pub fn new<T: Into<String>>(selector: T, separator: Option<TextComponent>) -> Self {
        Self {
            selector: selector.into(),
            separator: separator.map(Box::new),
        }
    }
}
