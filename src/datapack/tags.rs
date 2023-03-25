use datapack_macro::DeserializeJsonFolder;
use serde::Deserialize;

#[derive(Debug, Deserialize, DeserializeJsonFolder)]
pub struct Tag {
    values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            Tag::deserialize_json_folder("./data/minecraft/tags/").unwrap()
        );
    }
}
