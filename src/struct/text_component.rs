mod click_event;
mod hover_event;
mod nbt_content;
mod score_content;
mod selector_content;
mod text_color;
mod text_font;
mod text_style;
mod text_translation;

use serde::{Deserialize, Serialize};

pub use click_event::*;
pub use hover_event::*;
pub use nbt_content::*;
pub use score_content::*;
pub use selector_content::*;
pub use text_color::*;
pub use text_font::*;
pub use text_style::*;
pub use text_translation::*;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TextComponent {
    Text(String),
    Component(Component),
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Component {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<TextComponent>>,
    #[serde(flatten)]
    pub style: TextStyle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<TextTranslation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreContent>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<SelectorContent>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbt: Option<NBTContent>,
}

impl TextComponent {
    pub fn from_text<T: Into<String>>(text: T, style: TextStyle) -> Self {
        Self::Component(Component {
            r#type: None,
            extra: None,
            style,
            text: Some(text.into()),
            translation: None,
            keybind: None,
            score: None,
            selector: None,
            nbt: None,
        })
    }

    pub fn from_score<T: Into<String>>(name: T, objective: T, style: TextStyle) -> Self {
        Self::Component(Component {
            r#type: None,
            extra: None,
            style,
            text: None,
            translation: None,
            keybind: None,
            score: Some(ScoreContent::new(name, objective)),
            selector: None,
            nbt: None,
        })
    }

    pub fn from_selector<T: Into<String>>(
        selector: T,
        separator: Option<TextComponent>,
        style: TextStyle,
    ) -> Self {
        Self::Component(Component {
            r#type: None,
            extra: None,
            style,
            text: None,
            translation: None,
            keybind: None,
            score: None,
            selector: Some(SelectorContent::new(selector, separator)),
            nbt: None,
        })
    }

    pub fn from_translation<T: Into<String>>(translate: T, with: Vec<TextComponent>) -> Self {
        Self::Component(Component {
            r#type: None,
            extra: None,
            style: TextStyle::new(),
            text: None,
            translation: Some(TextTranslation::new(translate, with)),
            keybind: None,
            score: None,
            selector: None,
            nbt: None,
        })
    }

    pub fn from_keybind<T: Into<String>>(keybind: T) -> Self {
        Self::Component(Component {
            r#type: None,
            extra: None,
            style: TextStyle::new(),
            text: None,
            translation: None,
            keybind: Some(keybind.into()),
            score: None,
            selector: None,
            nbt: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::r#struct::{ClickEvent, HoverEvent, TextComponent, TextStyle};

    #[test]
    fn text_component() {
        assert_eq!(
            serde_json::from_value::<TextComponent>(json!({
                "translate": "chat.type.text",
                "with": [
                    {
                        "text": "Herobrine",
                        "clickEvent": {
                            "action": "suggest_command",
                            "value": "/msg Herobrine"
                        },
                        "hoverEvent": {
                            "action": "show_entity",
                            "value": "{id:f84c6a79-0a4e-45e0-879b-cd49ebd4c4e2,name:Herobrine}"
                        },
                        "insertion": "Herobrine"
                    },
                    {
                        "text": "I don't exist"
                    }
                ]
            }))
            .unwrap(),
            TextComponent::from_translation(
                "chat.type.text",
                vec![
                    TextComponent::from_text(
                        "Herobrine",
                        TextStyle::new()
                            .click_event(ClickEvent::SuggestCommand("/msg Herobrine".to_string()))
                            .hover_event(HoverEvent::ShowEntity(
                                "{id:f84c6a79-0a4e-45e0-879b-cd49ebd4c4e2,name:Herobrine}"
                                    .to_string()
                            ))
                            .insertion("Herobrine"),
                    ),
                    TextComponent::from_text("I don't exist", TextStyle::new())
                ]
            )
        );
    }
}
