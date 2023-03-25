use serde::Deserialize;

use crate::datapack::{items, translate};

#[derive(Debug, Deserialize)]
pub struct Display {
    announce_to_chat: bool,
    description: translate::Translate,
    frame: String,
    hidden: bool,
    icon: items::Item,
    show_toast: bool,
    title: translate::Translate,
}
