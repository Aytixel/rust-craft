mod style;

use std::fmt::Debug;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chat {
    parameters: Vec<String>,
    style: Option<style::Style>,
    translation_key: String,
}

impl Debug for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Chat");

        s.field("parameters", &self.parameters);

        if let Some(style) = &self.style {
            s.field("style", &style);
        }

        s.field("translation_key", &self.translation_key).finish()
    }
}
