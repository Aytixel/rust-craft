use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

use super::{ClickEvent, HoverEvent, TextColor, TextFont};

#[serde_as]
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<TextColor>,
    #[serde_as(as = "Option<BoolFromInt>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde_as(as = "Option<BoolFromInt>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde_as(as = "Option<BoolFromInt>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde_as(as = "Option<BoolFromInt>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde_as(as = "Option<BoolFromInt>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<TextFont>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    #[serde(rename = "clickEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(rename = "hoverEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            font: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }
    }

    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn underlined(mut self, underlined: bool) -> Self {
        self.underlined = Some(underlined);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = Some(obfuscated);
        self
    }

    pub fn font(mut self, font: TextFont) -> Self {
        self.font = Some(font);
        self
    }

    pub fn insertion<T: Into<String>>(mut self, insertion: T) -> Self {
        self.insertion = Some(insertion.into());
        self
    }

    pub fn click_event(mut self, click_event: ClickEvent) -> Self {
        self.click_event = Some(click_event);
        self
    }

    pub fn hover_event(mut self, hover_event: HoverEvent) -> Self {
        self.hover_event = Some(hover_event);
        self
    }

    pub fn reset(mut self) -> Self {
        self.color = None;
        self.bold = None;
        self.italic = None;
        self.underlined = None;
        self.strikethrough = None;
        self.obfuscated = None;
        self.font = None;
        self.insertion = None;
        self.click_event = None;
        self.hover_event = None;
        self
    }
}

#[cfg(test)]
mod tests {
    use click_event::ClickEvent;
    use serde_json::json;

    use crate::r#struct::{
        text_component::click_event, HoverEvent, TextColor, TextFont, TextStyle,
    };

    #[test]
    fn text_style() {
        assert_eq!(
            serde_json::from_value::<TextStyle>(json!({
                "color": "black",
                "bold": 1,
                "obfuscated": 0,
                "font": "minecraft:alt",
                "clickEvent": {
                    "action": "change_page",
                    "value": 1
                },
                "hoverEvent": {
                    "action": "show_item",
                    "value": "{ \"id\": \"minecraft:air\", \"count\": 2 }"
                }
            }))
            .unwrap(),
            TextStyle::new()
                .color(TextColor::Black)
                .bold(true)
                .obfuscated(false)
                .font(TextFont::StandardGalacticAlphabet)
                .click_event(ClickEvent::ChangePage(1))
                .hover_event(HoverEvent::ShowItem(
                    "{ \"id\": \"minecraft:air\", \"count\": 2 }".to_string()
                ))
        );
    }
}
