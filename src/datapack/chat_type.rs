mod chat;

use std::fmt::Debug;

use datapack_macro::DeserializeJsonFolder;
use serde::Deserialize;

#[derive(Debug, Deserialize, DeserializeJsonFolder)]
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
            ChatType::deserialize_json_folder("./data/minecraft/chat_type/").unwrap()
        );
    }
}
