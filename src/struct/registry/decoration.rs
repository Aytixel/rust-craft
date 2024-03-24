use serde::{Deserialize, Serialize};

use crate::r#struct::TextStyle;

#[derive(Debug, Deserialize, Serialize)]
pub struct Decoration {
    translation_key: String,
    style: Option<TextStyle>,
    parameters: Vec<String>,
}
