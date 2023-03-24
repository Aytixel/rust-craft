mod chat;

use std::fmt::Debug;

use datapack_macro::DeserializeFolder;
use serde::Deserialize;

#[derive(Debug, Deserialize, DeserializeFolder)]
pub struct ChatType {
    chat: chat::Chat,
    narration: chat::Chat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            ChatType::deserialize_folder("./data/minecraft/chat_type/").unwrap()
        );
    }
}
