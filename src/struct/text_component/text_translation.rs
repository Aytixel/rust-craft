use serde::{Deserialize, Serialize};

use super::TextComponent;

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct TextTranslation {
    pub translate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Vec<TextComponent>>,
}

impl TextTranslation {
    pub fn new<T: Into<String>>(translate: T, with: Vec<TextComponent>) -> Self {
        Self {
            translate: translate.into(),
            with: if with.is_empty() { None } else { Some(with) },
        }
    }
}
