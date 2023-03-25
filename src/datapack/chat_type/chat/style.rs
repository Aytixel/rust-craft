use std::fmt::Debug;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Style {
    color: Option<String>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    font: Option<String>,
}

impl Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Style");

        if let Some(color) = &self.color {
            s.field("color", &color);
        }

        if let Some(bold) = &self.bold {
            s.field("bold", &bold);
        }

        if let Some(italic) = &self.italic {
            s.field("italic", &italic);
        }

        if let Some(underlined) = &self.underlined {
            s.field("underlined", &underlined);
        }

        if let Some(strikethrough) = &self.strikethrough {
            s.field("strikethrough", &strikethrough);
        }

        if let Some(obfuscated) = &self.obfuscated {
            s.field("obfuscated", &obfuscated);
        }

        if let Some(font) = &self.font {
            s.field("font", &font);
        }

        s.finish()
    }
}
